// Future security improvements:
// - Account lockout after enough failed attempts
// - 2-factor authentication
// - Password reset functionality
// - Email verification
// - Password strength validation
// - Log auth attempts

use actix_session::{Session, SessionMiddleware};
use actix_web::{cookie::Key, get, post, web, HttpResponse, Responder};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use data::*;
use once_cell::sync::Lazy;
use password_hash::{rand_core::OsRng, SaltString};
use permission::*;
use pstd::{anyhow::AnyResult, prelude::*};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Row, Sqlite, SqlitePool};
use std::env;

use crate::Applet;

pub mod data;
pub mod permission;
pub mod session;

fn argon2() -> Argon2<'static> {
    Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(
            128 * 1024, /* Memory (128 MiB) */
            4,          /* Passes */
            3,          /* Threads */
            None,
        )
        .unwrap(), // Unwrap will never fail because the values are valid
    )
}

/// Derives a 64-byte cookie key from the JWT_SECRET by cycling the secret's bytes.
fn derive_cookie_key(secret: &str) -> [u8; 64] {
    let mut key = [0u8; 64];
    let bytes = secret.as_bytes();
    for (i, b) in key.iter_mut().enumerate() {
        *b = bytes[i % bytes.len()];
    }
    key
}

#[derive(Clone)]
pub struct AccountApp {
    pool: Pool<Sqlite>,
    cookie_key_bytes: [u8; 64],
}

impl AccountApp {
    /// Builds the session middleware. Called from main.rs since `.wrap()` changes
    /// the `App<T>` type and cannot be done inside `Applet::build`.
    pub fn session_middleware(&self) -> SessionMiddleware<session::SqliteSessionStore> {
        let store = session::SqliteSessionStore::new(self.pool.clone());
        let key = Key::from(&self.cookie_key_bytes);
        SessionMiddleware::builder(store, key)
            .cookie_secure(false)
            .build()
    }
}

impl Applet for AccountApp {
    async fn init() -> AnyResult<Self>
    where
        Self: Sized,
    {
        let jwt_secret = env::var("JWT_SECRET")?;
        let cookie_key_bytes = derive_cookie_key(&jwt_secret);

        let pool = SqlitePoolOptions::new()
            .connect(&env::var("DATABASE_URL")?)
            .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            version integer NOT NULL,
            data BLOB NOT NULL,
            stream_key TEXT
        )",
        )
        .execute(&pool)
        .await?;

        // Migration: add stream_key column for existing databases.
        // Silently ignored if the column already exists.
        // Note: ALTER TABLE ADD COLUMN does not support UNIQUE in SQLite;
        // uniqueness is enforced by the index below instead.
        let _ = sqlx::query("ALTER TABLE users ADD COLUMN stream_key TEXT")
            .execute(&pool)
            .await;

        // Ensure the unique index exists (idempotent).
        // The WHERE clause allows multiple rows with NULL stream_key.
        sqlx::query(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_users_stream_key \
             ON users (stream_key) WHERE stream_key IS NOT NULL",
        )
        .execute(&pool)
        .await?;

        session::SqliteSessionStore::init(&pool).await?;

        // Background task: purge expired sessions every hour
        let cleanup_pool = pool.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(60 * 60)).await;
                let now = pstd::time::Utc::now().timestamp();
                let _ = sqlx::query("DELETE FROM sessions WHERE expiry < ?")
                    .bind(now)
                    .execute(&cleanup_pool)
                    .await;
            }
        });

        Ok(Self {
            pool,
            cookie_key_bytes,
        })
    }

    fn build<T>(self, app: actix_web::App<T>) -> actix_web::App<T>
    where
        T: actix_web::dev::ServiceFactory<
            actix_web::dev::ServiceRequest,
            Config = (),
            Error = actix_web::error::Error,
            InitError = (),
        >,
    {
        app.app_data(web::Data::new(self.pool))
            .service(register)
            .service(login)
            .service(logout)
            .service(me)
            .service(get_stream_key)
            .service(regenerate_stream_key)
    }
}

#[derive(Serialize, Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[post("/register")]
async fn register(db: web::Data<SqlitePool>, req: web::Json<RegisterRequest>) -> impl Responder {
    if req.password.len() < 8 || req.password.len() > 64 {
        return HttpResponse::BadRequest().body("Password must be between 8 and 64 characters");
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match argon2().hash_password(req.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            error!("Hashing error: {}", e);
            return HttpResponse::InternalServerError().body("Error registering user");
        }
    };

    static DATA: Lazy<(u16, Vec<u8>)> = Lazy::new(|| {
        let data = UserData::latest(USER_PERMISSIONS.clone());
        (data.version(), bitcode::encode(&data))
    });

    // Insert user
    let res = sqlx::query(
        "INSERT INTO users (username, password_hash, version, data) VALUES (?, ?, ?, ?)",
    )
    .bind(&req.username)
    .bind(&password_hash)
    .bind(DATA.0)
    .bind(DATA.1.as_slice())
    .execute(db.get_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Ok().body("User registered"),
        Err(e) => {
            if e.to_string().contains("UNIQUE constraint failed") {
                HttpResponse::Conflict().body("Username already exists")
            } else {
                error!("DB error: {}", e);
                HttpResponse::InternalServerError().body("Error registering user")
            }
        }
    }
}

#[post("/login")]
async fn login(
    db: web::Data<SqlitePool>,
    session: Session,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    // Compute a fake hash on the unknown-user path so timing doesn't reveal whether a username exists
    static FAKE_HASH: Lazy<String> = Lazy::new(|| {
        argon2()
            .hash_password(&[74, 65, 73, 74], &SaltString::generate(&mut OsRng))
            .unwrap() // Unwrap will never fail
            .to_string()
    });

    let password_row = sqlx::query("SELECT password_hash FROM users WHERE username = ?")
        .bind(&req.username)
        .fetch_one(db.get_ref())
        .await;

    let password_hash: String = match &password_row {
        Ok(record) => record.get(0),
        Err(_) => FAKE_HASH.to_owned(),
    };
    let parsed_hash = match password_hash::PasswordHash::new(&password_hash) {
        Ok(hash) => hash,
        Err(e) => {
            error!("Hash parsing error: {}", e);
            return HttpResponse::InternalServerError().body("Error logging in");
        }
    };

    if argon2()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .is_ok()
        && password_row.is_ok()
    {
        if let Err(e) = session.insert("username", &req.username) {
            error!("Session insert error: {}", e);
            return HttpResponse::InternalServerError().body("Error logging in");
        }
        return HttpResponse::Ok().json(serde_json::json!({"username": req.username}));
    }

    HttpResponse::Unauthorized().body("Invalid username or password")
}

#[post("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().body("Logged out")
}

#[get("/me")]
async fn me(session: Session) -> impl Responder {
    match session.get::<String>("username") {
        Ok(Some(username)) => HttpResponse::Ok().json(serde_json::json!({"username": username})),
        Ok(None) => HttpResponse::Unauthorized().body("Not logged in"),
        Err(e) => {
            error!("Session read error: {}", e);
            HttpResponse::InternalServerError().body("Error reading session")
        }
    }
}

/// Generates a cryptographically random 48-character hex stream key.
fn generate_stream_key() -> String {
    use password_hash::rand_core::RngCore;
    let mut bytes = [0u8; 24];
    password_hash::rand_core::OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Checks that the session has a logged-in user with the given permission.
/// Returns the username on success, or an error `HttpResponse` on failure.
async fn require_permission(
    session: &Session,
    db: &SqlitePool,
    permission_name: &str,
) -> Result<String, HttpResponse> {
    let username = match session.get::<String>("username") {
        Ok(Some(u)) => u,
        Ok(None) => return Err(HttpResponse::Unauthorized().body("Not logged in")),
        Err(e) => {
            error!("Session read error: {}", e);
            return Err(HttpResponse::InternalServerError().body("Session error"));
        }
    };

    let row = sqlx::query("SELECT data FROM users WHERE username = ?")
        .bind(&username)
        .fetch_one(db)
        .await;

    let row = match row {
        Ok(r) => r,
        Err(e) => {
            error!("DB error fetching user data: {}", e);
            return Err(HttpResponse::InternalServerError().body("DB error"));
        }
    };

    let data_bytes: &[u8] = row.get(0);
    let user_data: UserData = match bitcode::decode(data_bytes) {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to decode user data: {}", e);
            return Err(HttpResponse::InternalServerError().body("Data error"));
        }
    };

    let permission: Permission = match permission_name.parse() {
        Ok(p) => p,
        Err(_) => {
            error!("Unknown permission: {}", permission_name);
            return Err(HttpResponse::InternalServerError().body("Unknown permission"));
        }
    };

    if !user_data.has_permission(&permission) {
        return Err(HttpResponse::Forbidden().body("Insufficient permissions"));
    }

    Ok(username)
}

#[get("/stream-key")]
async fn get_stream_key(db: web::Data<SqlitePool>, session: Session) -> impl Responder {
    let username = match require_permission(&session, db.get_ref(), "StreamKey::ViewOwn").await {
        Ok(u) => u,
        Err(resp) => return resp,
    };

    let row = sqlx::query("SELECT stream_key FROM users WHERE username = ?")
        .bind(&username)
        .fetch_one(db.get_ref())
        .await;

    match row {
        Ok(r) => {
            let key: Option<String> = r.get(0);
            HttpResponse::Ok().json(serde_json::json!({ "stream_key": key }))
        }
        Err(e) => {
            error!("DB error fetching stream key: {}", e);
            HttpResponse::InternalServerError().body("DB error")
        }
    }
}

#[post("/stream-key/regenerate")]
async fn regenerate_stream_key(db: web::Data<SqlitePool>, session: Session) -> impl Responder {
    let username =
        match require_permission(&session, db.get_ref(), "StreamKey::RegenerateOwn").await {
            Ok(u) => u,
            Err(resp) => return resp,
        };

    let new_key = generate_stream_key();

    let result = sqlx::query("UPDATE users SET stream_key = ? WHERE username = ?")
        .bind(&new_key)
        .bind(&username)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "stream_key": new_key })),
        Err(e) => {
            error!("DB error updating stream key: {}", e);
            HttpResponse::InternalServerError().body("DB error")
        }
    }
}

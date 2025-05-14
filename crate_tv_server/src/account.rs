// Future security improvements:
// - Account lockout after enough failed attempts
// - 2-factor authentication
// - Password reset functionality
// - Email verification
// - Password strength validation
// - Better JWT header algorithm
// - Log auth attempts
// - Generate fake hash at startup instead of it being a constant and randomize salt per login attempt

use actix_web::{post, web, HttpResponse, Responder};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use password_hash::{rand_core::OsRng, SaltString};
use pstd::{anyhow::AnyResult, prelude::*};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Row, Sqlite, SqlitePool};
use std::env;

use crate::Applet;

const JWT_EXPIRATION: Duration = Duration::from_secs(24 * 60 * 60); // 24 hours

fn argon2() -> Argon2<'static> {
    // Remember to change fake hash when updating hash settings
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

#[derive(Clone)]
pub struct AccountApp(Pool<Sqlite>);

impl Applet for AccountApp {
    async fn init() -> AnyResult<Self>
    where
        Self: Sized,
    {
        assert!(env::var("JWT_SECRET").is_ok(), "JWT_SECRET must be set");

        let pool = SqlitePoolOptions::new()
            .connect(&env::var("DATABASE_URL")?)
            .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        )",
        )
        .execute(&pool)
        .await?;

        Ok(Self(pool))
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
        app.app_data(web::Data::new(self.0))
            .service(register)
            .service(login)
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
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
    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match argon2().hash_password(req.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            error!("Hashing error: {}", e);
            return HttpResponse::InternalServerError().body("Error registering user");
        }
    };

    // Insert user
    let res = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(&req.username)
        .bind(&password_hash)
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
async fn login(db: web::Data<SqlitePool>, req: web::Json<LoginRequest>) -> impl Responder {
    // Fetch user
    let user = sqlx::query("SELECT password_hash FROM users WHERE username = ?")
        .bind(&req.username)
        .fetch_one(db.get_ref())
        .await;

    // Hello itsa me the PROGRAMMER!!! FEAR ME!!!! I'd like to demonstrate my endless knowledge of cybersecurity by computing a hash whether or not the user exists so that the HACKERS can't figure out if a username exists or not by logging in.
    const FAKE_HASH_TO_DECEIVE_THE_HACKERS: &str = "$argon2id$v=19$m=131072,t=4,p=3$NA0/bDQ8TrHCgiRcFKaMbQ$uLu4H0j1fw+f4f0J+Uq9pl2l6TBYInrwi57rH9YnGe4";

    let password_hash = match &user {
        Ok(record) => record.get(0),
        Err(_) => FAKE_HASH_TO_DECEIVE_THE_HACKERS.to_owned(),
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
    {
        // Create JWT
        let secret = env::var("JWT_SECRET").unwrap(); // Asserted during initialization
        let expiration = pstd::time::Utc::now().timestamp() as u64 + JWT_EXPIRATION.as_secs();
        let claims = Claims {
            sub: req.username.clone(),
            exp: expiration as usize,
        };

        // Encode JWT
        let token = match encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(token) => token,
            Err(e) => {
                error!("JWT encoding error: {}", e);
                return HttpResponse::InternalServerError().body("Error logging in");
            }
        };

        if user.is_ok() {
            return HttpResponse::Ok().json(serde_json::json!({"token": token}));
        }
    }

    HttpResponse::Unauthorized().body("Invalid username or password")
}

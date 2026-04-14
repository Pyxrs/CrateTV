use actix_session::storage::{
    generate_session_key, LoadError, SaveError, SessionKey, SessionStore, UpdateError,
};
use actix_web::cookie::time::Duration;
use pstd::{anyhow::AnyResult, time::Utc};
use sqlx::SqlitePool;
use std::collections::HashMap;

type SessionState = HashMap<String, String>;

#[derive(Clone)]
pub struct SqliteSessionStore {
    pool: SqlitePool,
}

impl SqliteSessionStore {
    /// Creates the store. Call `init` first to create the sessions table.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates the sessions table if it doesn't exist. Must be called before use.
    pub async fn init(pool: &SqlitePool) -> AnyResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS sessions (
                session_key TEXT PRIMARY KEY,
                session_data TEXT NOT NULL,
                expiry INTEGER NOT NULL
            )",
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

impl SessionStore for SqliteSessionStore {
    async fn load(&self, session_key: &SessionKey) -> Result<Option<SessionState>, LoadError> {
        let row: Option<(String, i64)> =
            sqlx::query_as("SELECT session_data, expiry FROM sessions WHERE session_key = ?")
                .bind(session_key.as_ref())
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| LoadError::Other(e.into()))?;

        if let Some((data, expiry)) = row {
            if expiry < Utc::now().timestamp() {
                let _ = sqlx::query("DELETE FROM sessions WHERE session_key = ?")
                    .bind(session_key.as_ref())
                    .execute(&self.pool)
                    .await;
                return Ok(None);
            }

            let state: SessionState = serde_json::from_str(&data)
                .map_err(|e: serde_json::Error| LoadError::Deserialization(e.into()))?;
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    async fn save(
        &self,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, SaveError> {
        let session_key = generate_session_key();
        let expiry = Utc::now().timestamp() + ttl.whole_seconds();

        let data = serde_json::to_string(&session_state)
            .map_err(|e: serde_json::Error| SaveError::Serialization(e.into()))?;

        sqlx::query(
            "INSERT INTO sessions (session_key, session_data, expiry) VALUES (?, ?, ?) ON CONFLICT DO NOTHING",
        )
        .bind(session_key.as_ref())
        .bind(data)
        .bind(expiry)
        .execute(&self.pool)
        .await
        .map_err(|e| SaveError::Other(e.into()))?;

        Ok(session_key)
    }

    async fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, UpdateError> {
        let expiry = Utc::now().timestamp() + ttl.whole_seconds();

        let data = serde_json::to_string(&session_state)
            .map_err(|e: serde_json::Error| UpdateError::Serialization(e.into()))?;

        let result =
            sqlx::query("UPDATE sessions SET session_data = ?, expiry = ? WHERE session_key = ?")
                .bind(&data)
                .bind(expiry)
                .bind(session_key.as_ref())
                .execute(&self.pool)
                .await
                .map_err(|e| UpdateError::Other(e.into()))?;

        if result.rows_affected() == 0 {
            self.save(session_state, ttl).await.map_err(|e| match e {
                SaveError::Serialization(err) => UpdateError::Serialization(err),
                SaveError::Other(err) => UpdateError::Other(err),
            })
        } else {
            Ok(session_key)
        }
    }

    async fn update_ttl(&self, session_key: &SessionKey, ttl: &Duration) -> AnyResult<()> {
        let expiry = Utc::now().timestamp() + ttl.whole_seconds();

        sqlx::query("UPDATE sessions SET expiry = ? WHERE session_key = ?")
            .bind(expiry)
            .bind(session_key.as_ref())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete(&self, session_key: &SessionKey) -> AnyResult<()> {
        sqlx::query("DELETE FROM sessions WHERE session_key = ?")
            .bind(session_key.as_ref())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

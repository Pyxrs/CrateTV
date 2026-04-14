use std::collections::HashSet;

use bitcode::{Decode, Encode};
use thiserror::Error;
use sqlx::{Row, SqlitePool};

use super::permission::*;

#[non_exhaustive]
#[derive(Encode, Decode)]
pub enum UserData {
    /// Latest
    V1 { permissions: HashSet<Permission> },
}

impl Default for UserData {
    fn default() -> Self {
        Self::V1 {
            permissions: GUEST_PERMISSIONS.clone(),
        }
    }
}

impl UserData {
    pub fn latest(permissions: HashSet<Permission>) -> Self {
        Self::V1 { permissions }
    }

    pub fn version(&self) -> u16 {
        match self {
            Self::V1 { .. } => 1,
        }
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        match self {
            Self::V1 { permissions } => permissions.contains(permission),
        }
    }

    pub fn update(&mut self) {
        if self.version() < Self::default().version() {
            match self {
                Self::V1 { .. } => {}
            }
        }
    }
}

/// Fetches the UserData for a given username from the database.
pub async fn fetch(username: &str, db: &SqlitePool) -> Result<UserData, DataFetchError> {
    let data_row = sqlx::query("SELECT data FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(db)
        .await?;

    let data: &[u8] = data_row.get(0);
    let user_data: UserData = bitcode::decode(data)?;

    Ok(user_data)
}

#[derive(Error, Debug)]
pub enum DataFetchError {
    #[error("User not found")]
    UserNotFound(#[from] sqlx::Error),
    #[error("Failed to decode user data")]
    DataInvalid(#[from] bitcode::Error),
}

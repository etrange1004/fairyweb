use std::fmt::{Display, Formatter, Result};
use argon2::{Argon2, PasswordHash, password_hash::SaltString};
use tracing::debug;
use rand::thread_rng;

use crate::server::{models, errors};

#[derive(Clone, PartialEq)]
pub enum Role {
    User, 
    Admin,
}
impl Role {
    pub fn from_str(role: &str) -> Role {
        match role.to_lowercase().as_str() {
            "admin" => Role::Admin,
            _  => Role::User,
        }
    }
}
impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}
fn get_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET").unwrap().into_bytes()
}
pub async fn hashed_password(password: String) -> String {
    tokio::task::spawn_blocking(move || -> String {
        let salt = SaltString::generate(rand::thread_rng());
        PasswordHash::generate(Argon2::default(), password, salt.as_str())
            .map_err(|e| errors::CustomError::GeneratingPasswordHashError).unwrap().to_string()
    })
    .await
    .map_err(|e| errors::CustomError::GeneratingPasswordHashError)
    .unwrap()
}
pub async fn verify_password(password: String, password_hash: String) -> bool {
    tokio::task::spawn_blocking(move || -> bool {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|_| errors::CustomError::VerifyingPasswordHashError).unwrap();
        hash.verify_password(&[&Argon2::default()], password).is_ok()
    })
    .await
    .map_err(|_| errors::CustomError::VerifyingPasswordHashError)
    .unwrap()
}
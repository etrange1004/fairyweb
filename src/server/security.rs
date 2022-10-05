use argon2::{Argon2, PasswordHash, password_hash::SaltString};

use crate::server::errors;

pub async fn hashed_password(password: String) -> String {
    tokio::task::spawn_blocking(move || -> String {
        let salt = SaltString::generate(rand::thread_rng());
        PasswordHash::generate(Argon2::default(), password, salt.as_str())
            .map_err(|_| errors::CustomError::GeneratingPasswordHashError).unwrap().to_string()
    })
    .await
    .map_err(|_| errors::CustomError::GeneratingPasswordHashError)
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
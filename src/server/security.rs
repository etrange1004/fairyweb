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



/*
pub fn get_hashed_password(password: String) -> String {
    thread::spawn(move || -> String {
        let salt = SaltString::generate(&mut OsRng);
        Scrypt.hash_password_customized(password.as_bytes(), &salt).unwrap().to_string()                        
    }).join().unwrap().into()
}
pub fn verify_password(password: String, password_hash: String) -> bool  {
    thread::spawn(move || -> bool {
        let parsed_hash = PasswordHash::new(password_hash.as_str()).unwrap();
        Scrypt.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }).join().unwrap().into()
}
pub fn get_hashed_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Scrypt.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}
pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash).unwrap();
    Scrypt.verify_password(password.as_bytes(), &parsed_hash).is_ok()
}
pub fn get_jwt_for_user(user: &models::User) -> String {
    let expiration_time = OffsetDateTime::now_utc().checked_add(time::Duration::seconds(60)).expect("invalid timestamp").unix_timestamp();
    let user_claims = models::Claims { sub: user.username.clone(), role: user.role.clone(), exp: expiration_time as usize };
    let token = match encode(&Header::default(), &user_claims, &EncodingKey::from_secret(&get_secret())) {
        Ok(t) => t,
        Err(_) => panic!(),
    };
    token
}
fn is_authorized(required_role: Role, claims_role: &str) -> bool {
    let claims_role = Role::from_str(claims_role);
    tracing::debug!("************** needed role: {}, user role: {} ******************", required_role, claims_role);
    required_role == claims_role || claims_role == Role::Admin
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> std::result::Result<String, errors::CustomError> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(errors::CustomError::AuthHeaderRequiredError),
    };
    let auth_header = match std::str::from_utf8(headers.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(errors::CustomError::AuthHeaderRequiredError),
    };
    if !auth_header.starts_with("Bearer ") {
        return Err(errors::CustomError::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches("Bearer ").to_owned())
}
async fn authorize((role, headers): (Role, HeaderMap<HeaderValue>)) -> Result<String> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<models::Claims>(&jwt, &DecodingKey::from_secret(&get_secret()), &Validation::default())
                .map_err(|_| errors::CustomError::InvalidJasonWebTokenError.to_string())?;
            tracing::debug!("************************** decoded claims: {:?}", &decoded.claims);
            if !is_authorized(role, &decoded.claims.role) {
                return Err(errors::CustomError::NotAuthorizedError.to_string());
            }
            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(e.to_string())
    }
}
*/
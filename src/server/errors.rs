use axum::{
    response::{IntoResponse, Response, Html},
    http::StatusCode,
};
use std::fmt::{Display, Debug};
use thiserror::Error;
use argon2::password_hash::rand_core::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("invalid credentials")]
    InvalidCredentialsError,
    #[error("user exists")]
    UserExistsError(String),
    #[error("user not exist")]
    UserNotExistError(String),
    #[error("invalid password")]
    InvalidPasswordError,
    #[error("invalid password hash")]
    InvalidPasswordHashError,
    #[error("jwt creation error")]
    JWTCreationError,
    #[error("authorization header required")]
    AuthHeaderRequiredError,
    #[error("invalid authorization header")]
    InvalidAuthHeaderError,
    #[error("not authorized")]
    NotAuthorizedError,
    #[error("error generating password hash")]
    GeneratingPasswordHashError,
    #[error("error verifying password hash")]
    VerifyingPasswordHashError,
    #[error("failed to send email alert")]
    SendVerifyEmailError,
    #[error("Database Error")]
    SqlxDatabaseError(String),
}
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {        
        let home_url: String = std::env::var("HOME_URL").unwrap_or("http://localhost:8080".to_string());
        match self {
            CustomError::InvalidCredentialsError => {
                return (
                    StatusCode::FORBIDDEN, Html(
                    format!("<script>alert(\"{}\");location.href=\"{}/login\";</script>", self.to_string(), home_url))
                ).into_response();
            }
            CustomError::UserExistsError(username) => {
                return (
                    StatusCode::BAD_REQUEST, Html(
                    format!("<script>alert(\"{} already exists!\");location.href=\"{}/login\";</script>", username, home_url))
                ).into_response();
            }
            CustomError::UserNotExistError(username) => {
                return (
                    StatusCode::BAD_REQUEST, Html(
                    format!("<script>alert(\"{} not exist.\");location.href=\"{}/login\";</script>", username, home_url))
                ).into_response();
            }
            CustomError::InvalidPasswordError => {
                return (
                    StatusCode::BAD_REQUEST, Html(
                    format!("<script>alert(\"invalid password !!!\");location.href=\"{}/login\";</script>", home_url))
                ).into_response();
            }
            CustomError::NotAuthorizedError => {
                return (
                    StatusCode::UNAUTHORIZED, Html(
                    format!("<script>alert(\"{}\");location.href=\"{}/login\";</script>", self.to_string(), home_url))
                ).into_response();
            }
            CustomError::InvalidPasswordHashError => {
                return (
                    StatusCode::UNAUTHORIZED, Html(
                    format!("<script>alert(\"{}\");location.href=\"{}/login\";</script>", self.to_string(), home_url))
                ).into_response();
            }
            CustomError::JWTCreationError => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR, Html(
                    format!("<script>alert(\"Internal Server Error: {}\");location.href=\"{}/login\";</script>", self.to_string(), home_url))
                ).into_response();
            }
            CustomError::GeneratingPasswordHashError => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR, Html(
                    format!(
                        "<script>alert(\"Internal Server Error: {}\");location.href=\"{}/login\";</script>", 
                        self.to_string(), home_url
                    ))
                ).into_response();
            }
            CustomError::VerifyingPasswordHashError => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR, Html(
                    format!(
                        "<script>alert(\"Internal Server Error: {}\");location.href=\"{}/login\";</script>", 
                        self.to_string(), home_url
                    ))
                ).into_response();
            }
            CustomError::SendVerifyEmailError => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR, Html(
                    format!("<script>alert(\"{}\");location.href=\"{}/login\";</script>", self.to_string(), home_url))
                ).into_response();
            }
            CustomError::SqlxDatabaseError(ref err) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR, Html(
                    format!("<script>alert(\"{} : {}\");location.href=\"{}/login\";</script>", self.to_string(), err.to_string(), home_url))
                ).into_response();
            }
            _ => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR, Html(
                    format!("<script>alert(\"Internal Server Error\");location.href=\"{}/login\";</script>", home_url))
                ).into_response();
            }
        }
    }
}
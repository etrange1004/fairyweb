use axum::{
    response::{IntoResponse, Response, Html},
    http::StatusCode,
};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(unreachable_patterns)]
pub enum CustomError {
    #[error("admin id is 'admin' and password is '0000'.")]
    AdminLoginError,
    #[error("user exists")]
    UserExistsError(String),
    #[error("user not exist")]
    UserNotExistError(String),
    #[error("invalid password")]
    InvalidPasswordError,
    #[error("invalid password !!!")]
    EditInvalidPasswordError((String, String)),
    #[error("error generating password hash")]
    GeneratingPasswordHashError,
    #[error("error verifying password hash")]
    VerifyingPasswordHashError,
    #[error("profile picture file upload error")]
    FileUploadError,
    #[error("failed to send email alert")]
    SendVerifyEmailError,
    #[error("Database Error")]
    SqlxDatabaseError(String),
}
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {        
        let home_url: String = std::env::var("HOME_URL").unwrap_or("https://localhost:8080".to_string());
        match self {
            CustomError::AdminLoginError => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR, Html(
                    format!("<script>alert(\"admin login error!!!\");location.href=\"{}/init\";</script>", home_url))
                ).into_response();
            }
            CustomError::UserExistsError(username) => {
                return (
                    StatusCode::BAD_REQUEST, Html(
                    format!("<script>alert(\"{} already exists!\");location.href=\"{}/signin\";</script>", username, home_url))
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
            CustomError::EditInvalidPasswordError((no, id)) => {
                return (
                    StatusCode::BAD_REQUEST, Html(
                    format!("<script>alert(\"invalid password !!!\");location.href=\"{}/edit?number={}&id={}&page_no=1\";</script>", home_url, no, id))
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
            CustomError::FileUploadError => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR, Html(
                    format!("<script>alert(\"{}\");location.href=\"{}/signin\";</script>", self.to_string(), home_url))
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
        }
    }
}
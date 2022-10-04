use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub user_id: usize,
    pub username: String,
    pub password: String,
    pub role: String,
}
#[derive(Clone, Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub role: String,
}
#[derive(Clone, Debug, Deserialize)]
pub struct LoginUser {
    pub username: Option<String>,
    pub password: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct UpdateUser {
    pub id: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct UpdatePassword {
    pub id: Option<String>,
    pub password: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
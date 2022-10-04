pub struct Config {
    pub database_url: String,
    pub hmac_key: String,
    pub home_url: String,
}
impl Config {
    pub fn new() -> Self {
        Config {
            database_url: std::env::var("DATABASE_URL").unwrap_or("mysql://chachafairy:0000@localhost/fairydb".to_string()),
            hmac_key: std::env::var("JWT_SECRET").unwrap_or("123456".to_string()), 
            home_url: std::env::var("HOME_URL").unwrap_or("http://localhost:8080".to_string()),
        }
    }
}

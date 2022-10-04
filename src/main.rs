use std::error::Error;
use sqlx::mysql::MySqlPoolOptions;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use fairyweb::config::Config;
use fairyweb::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).init();
    let config = Config::new();
    let db = MySqlPoolOptions::new().max_connections(100).connect(&config.database_url).await.expect("could not connect to database url.");
    server::start(config, db).await?;
    Ok(())
}
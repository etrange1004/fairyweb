use sqlx::MySqlPool;
use tokio::sync::broadcast;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::error::Error;
use axum::{
    error_handling::HandleErrorLayer,
    extract::Extension,
    http::StatusCode,
    Router,
};
use tower::ServiceBuilder;
use tower_http::{trace::{TraceLayer, DefaultMakeSpan}, add_extension::AddExtensionLayer};
use tower_cookies::CookieManagerLayer;

use crate::config::Config;

mod home;
mod init;
mod board;
mod chat;
mod errors;
mod models;
mod script;
mod security;
mod style;
mod user;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>, 
    db: MySqlPool,
}
struct ChatState {
    user_set: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
}

pub async fn start(config: Config, db: MySqlPool) -> Result<(), Box<dyn Error>> {
    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(5000);
    let chatstate = Arc::new(ChatState { user_set, tx });
    let app = api_router()
    .layer(CookieManagerLayer::new())
    .layer(
        ServiceBuilder::new()
            .layer(AddExtensionLayer::new(ApiContext {
                config: Arc::new(config),
                db,
            }))
            .layer(Extension(chatstate))
            .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true))),
    )
    .layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|error: tower::BoxError| async move {
                if error.is::<tower::timeout::error::Elapsed>() {
                    Ok(StatusCode::REQUEST_TIMEOUT)
                } else {
                    Err((StatusCode::INTERNAL_SERVER_ERROR, format!("unhandled internal error: {}", error)))
                }
            }))
            .timeout(tokio::time::Duration::from_secs(10))
            .into_inner(),
    );
    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
fn api_router() -> Router {
    Router::new()
        .merge(init::router())
        .merge(home::router())
        .merge(user::router())
        .merge(board::router())
        .merge(chat::router())
}
async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("expect shutdown signal handler");
    tracing::debug!("**************************signal shutdown*********************************");
}
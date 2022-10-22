use sqlx::MySqlPool;
use tokio::sync::broadcast;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::error::Error;
use std::net::SocketAddr;
use axum::{
    error_handling::HandleErrorLayer, extract::Extension, http::StatusCode, Router,
};
use axum_server::{
    AddrIncomingConfig, Handle, HttpConfig, tls_rustls::RustlsConfig,
};
use axum_server_dual_protocol::ServerExt;

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

    let cert = rcgen::generate_simple_self_signed(["UncleOppa".to_string(), "Fairy".to_string()]).unwrap();
    let tls_config = RustlsConfig::from_der(vec![cert.serialize_der()?], cert.serialize_private_key_der()).await.unwrap();
    let srv_addr_port = SocketAddr::from(([0, 0, 0, 0], 8080));
    let http_cfg = HttpConfig::new().http1_only(true).http2_only(false).max_buf_size(8192).build();
    let incoming_cfg = AddrIncomingConfig::new().tcp_nodelay(true).tcp_sleep_on_accept_errors(true).build();
    
    axum_server_dual_protocol::bind_dual_protocol(srv_addr_port, tls_config)
        .addr_incoming_config(incoming_cfg)
        .http_config(http_cfg)
        .set_upgrade(true)        
        .serve(app.into_make_service())
        //.with_graceful_shutdown(shutdown_signal())
        .await.unwrap();
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
/*
async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("expect shutdown signal handler");
    tracing::debug!("**************************signal shutdown*********************************");
}
*/
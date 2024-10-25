use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use config::get_app_config;
use log::{info, LevelFilter};
use logger::SimpleLogger;
use router_common::RouterGlobalState;

mod auth;
mod config;
mod logger;
mod router_common;
mod router_middleware;

#[tokio::main]
async fn main() {
    let config = get_app_config().unwrap();
    config.init();

    let pool = config.get_pool().await.unwrap();
    let router_global_state = RouterGlobalState { pool };

    info!("Configuring routers...");

    let main_router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/v1/login", post(auth::api::login_router))
        .route("/api/v1/sign_up", post(auth::api::sign_up_router))
        .route(
            "/api/v1/refresh_token",
            get(auth::api::refresh_token_router),
        )
        .layer(from_fn(router_middleware::trace_time))
        .with_state(router_global_state);

    info!("Starting server on port {}", &config.port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();
    axum::serve(listener, main_router).await.unwrap();
}

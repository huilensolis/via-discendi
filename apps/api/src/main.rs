use axum::{
    middleware::{self, from_fn},
    routing::{get, post},
    Router,
};
use log::{info, Level, LevelFilter, SetLoggerError};
use logger::SimpleLogger;
use router_common::RouterGlobalState;
use sqlx::postgres::PgPoolOptions;

mod auth;
mod logger;
mod router_common;
mod router_middleware;

static LOGGER: SimpleLogger = SimpleLogger {
    allowed_level: Level::Debug,
};

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}

#[tokio::main]
async fn main() {
    let port = 3000;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@localhost/mydatabase")
        .await
        .unwrap();

    init().unwrap();
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

    info!("Starting server on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, main_router).await.unwrap();
}

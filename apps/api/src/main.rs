use axum::{
    routing::{get, post},
    Router,
};
use router_config::RouterGlobalState;
use sqlx::postgres::PgPoolOptions;

mod auth;
mod router_config;

#[tokio::main]
async fn main() {

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://myuser:mypassword@localhost/mydatabase").await.unwrap();

    let router_global_state= RouterGlobalState{
        pool: pool
    };

    let main_router = Router::new()
        .route("/", get(|| async { "Hello, World!" })) 
        .route("/api/v1/login", post(auth::api::login_router))
        .route("/api/v1/sign_up", post(auth::api::sign_up_router))
        .route("/api/v1/refresh_token", post(auth::api::refresh_token_router))
        .with_state(router_global_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, main_router).await.unwrap();
}

use sqlx::PgPool;
use serde::Serialize;

#[derive(Clone)]
pub struct RouterGlobalState {
    pub pool: PgPool
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub is_successful: bool,
    pub message: String
}

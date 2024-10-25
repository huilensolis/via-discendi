use crate::logger::SimpleLogger;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone)]
pub struct RouterGlobalState {
    pub pool: PgPool,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub is_successful: bool,
    pub message: String,
}

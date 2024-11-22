use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone)]
pub struct RouterGlobalState {
    pub pool: PgPool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateResponse {
    pub is_successful: bool,
    pub message: String,
    pub id: Option<String>,
}

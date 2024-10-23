use sqlx::PgPool;

pub struct RouterGlobalState {
    pub pool: PgPool
}

pub struct CreateResponse {
    pub is_successful: bool,
    pub message: String
}

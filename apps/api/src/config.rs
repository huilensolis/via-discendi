use config::{Config, ConfigError};
use log::Level;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

#[derive(Deserialize)]
pub struct AppConfig {
    database_url: String,
    pub logging_level: Level,
    pub port: u16,
}

pub fn get_app_config() -> Result<AppConfig, ConfigError> {
    Config::builder()
        .set_default(
            "database_url",
            "postgres://myuser:mypassword@localhost/mydatabase",
        )?
        .set_default("port", 4269)?
        .set_default("logging_level", "DEBUG")?
        .build()?
        .try_deserialize::<AppConfig>()
}

impl AppConfig {
    pub async fn get_pool(&self) -> Result<Pool<Postgres>, Error> {
        PgPoolOptions::new().connect(&self.database_url).await
    }
}

use config::{Config, ConfigError};
use log::{Level, LevelFilter};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

use crate::logger::SimpleLogger;

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
    pub fn init(&self) {
        let logger: SimpleLogger = SimpleLogger {
            allowed_level: self.logging_level,
        };

        log::set_boxed_logger(Box::new(logger))
            .map(|()| log::set_max_level(self.logging_level.to_level_filter()))
            .unwrap();
    }

    pub async fn get_pool(&self) -> Result<Pool<Postgres>, Error> {
        PgPoolOptions::new().connect(&self.database_url).await
    }
}

use sea_orm::{Database, DatabaseConnection, ConnectOptions, DbErr};
use crate::config::Settings;

pub async fn connect(settings: &Settings) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(settings.database.url.clone());
    opt.max_connections(settings.database.max_connection);
    opt.min_connections(1);
    opt.sqlx_logging(false);
    Database::connect(opt).await
}

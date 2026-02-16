use sea_orm::{Database, DatabaseConnection};
use crate::config::Settings;

pub async fn connect(settings: &Settings) -> Result<DatabaseConnection, sea_orm::DbErr> {
    Database::connect(&settings.database.url).await
}

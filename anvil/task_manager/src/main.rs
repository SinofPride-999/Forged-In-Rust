mod config;
mod db;

use config::load_config;
use db::connection::connect;

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let settings = load_config().expect("Failed to load configuration");

    let _db = connect(&settings).await?;

    println!(
        "{} running in {} mode",
        settings.app.name,
        settings.app.environment,
    );
    println!("Connected to MySQL successfully!");

    Ok(())
}

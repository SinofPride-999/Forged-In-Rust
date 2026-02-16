mod config;
mod db;
mod entities;
mod services;
mod cli;

use config::load_config;
use db::connection::connect;
use cli::{Cli, Commands};
use service::tasks_service as tasks;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let settings = load_config()
        .expect("Failed to load configuration");
    let _db = connect(&settings)
        .await?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title, description } => {
            tasks::add_task(&db, &title, description.as_deref()).await?;
            println!("✅ Task added!");
        }
        Commands::List => {
            let tasks_list = tasks::list_tasks(&db).await?;
            for t in tasks_list {
                println!("#{} [{}] {}", t.id, t.status, t.title);
            }
        }
        Commands::Complete { id } => {
            tasks::complete_task(&db, id).await?;
            println!("✅ Task marked as done!");
        }
        Commands::Delete { id } => {
            tasks::delete_task(&db, id).await?;
            println!("✅ Task deleted!");
        }
    }

    Ok(())
}

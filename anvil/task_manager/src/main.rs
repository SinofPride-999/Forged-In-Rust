mod cli;
mod config;
mod db;
mod entities;
mod services;

use clap::Parser;
use cli::{Cli, Commands};
use config::load_config;
use db::connection::connect;
use services::task_service as tasks;

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let settings = load_config().expect("Failed to load configuration");
    let db = connect(&settings).await?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Add { input } => {
            let parts: Vec<&str> = input.split('|').map(|s| s.trim()).collect();
            let title = parts.get(0).unwrap_or(&"Untitled");
            let description = parts.get(1).map(|s| *s);
            
            tasks::add_task(&db, title, description).await?;
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

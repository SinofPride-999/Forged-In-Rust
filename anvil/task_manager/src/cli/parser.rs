use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Task Manager", version = "1.0", author = "Jhay")]
#[command(about = "Manage tasks from the CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        title: String,
        description: Option<String>,
    },

    /// List all tasks
    List,

    /// Mark a task as done
    Complete {
        id: i32,
    },

    /// Delete a task
    Delete {
        id: i32,
    },
}

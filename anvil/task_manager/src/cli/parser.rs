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
        /// Titile and optional description seperated by a '|'
        input: String,
    },

    /// List all tasks
    List {
        /// Optional filter: all | open | done
        filter: Option<String>,
    },

    /// Mark a task as done
    Complete { id: i32 },

    /// Delete a task
    Delete { id: i32 },
}

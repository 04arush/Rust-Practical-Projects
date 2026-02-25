use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI Task Manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    // Add a task
    Add {
        description: String,
    },
    // List all tasks
    List,
    // Mark a task as completed
    Done {
        id: u32,
    },
}
mod cli;
mod storage;
mod task;

use clap::Parser;
use cli::{Cli, Commands};
use task::Task;

fn main() {

    let cli = Cli::parse();

    // Load existing tasks from disk
    let mut tasks = storage::load_tasks();

    // Match against the enum defined in cli.rs
    match &cli.command {

        Some(Commands::Add { description }) => {
            // Find the highest ID and add 1, or start at 1
            let id = tasks.last().map(|t| t.id + 1).unwrap_or(1);

            tasks.push(Task {
                id,
                description: description.clone(),
                completed: false,
            });

            storage::save_tasks(&tasks);
            println!("Added task: {}", description);
        }

        Some(Commands::List) | None => {
            if tasks.is_empty() {
                println!("No tasks found. Enjoy your day!");
            } else {
                for task in &tasks {
                    let status = if task.completed { "[x]" } else { "[ ]" };
                    println!("{} {} - {}", task.id, status, task.description);
                }
            }
        }

        Some(Commands::Done { id }) => {
            // Find the specific task and mutate it
            if let Some(task) = tasks.iter_mut().find(|t| t.id == *id) {
                task.completed = true;
                storage::save_tasks(&tasks);
                println!("Marked task {} as completed!", id);
            } else {
                println!("Task {} not found.", id);
            }
        }
    }
}
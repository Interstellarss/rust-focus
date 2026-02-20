mod cli;
mod error;
mod store;
mod task;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};
use store::Store;
use ui::*;

fn main() {
    let cli = Cli::parse();
    let mut store = match Store::load("tasks.json") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    match cli.command {
        Commands::Add { title } => match store.add_task(title) {
            Ok(task) => print_added(task.id, &task.title),
            Err(e) => eprint!("Error: {}", e),
        },
        Commands::List => print_tasks(store.list_tasks()),

        Commands::Done { id } => match store.done_task(id) {
            Ok(_) => print_done(id),
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Delete { id } => match store.delete_task(id) {
            Ok(_) => print_deleted(id),
            Err(e) => println!("Error: {}", e),
        },
    }
}

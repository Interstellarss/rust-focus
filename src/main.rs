mod cli;
mod task;
mod store;

use clap::Parser;
use cli::{Cli, Commands};
use store::Store;

fn main() {
    let cli = Cli::parse();
    let mut store = Store::new();


    match cli.command {
        Commands::Add{title} => {
            let task = store.add_task(title);
            println!("Added task #{}: {}", task.id, task.title);
        }
        Commands::List => {
            let tasks = store.list_tasks();
            if tasks.is_empty() {
                println!("no tasks yet.");
            }else {
                for t in tasks{
                    let status = if t.done {"✅"} else {"Not"};
                    println!("#{} {} {}", t.id, status, t.title);
                }
            }
        }
        Commands::Done{id} => match store.done_task(id) {
            Ok(_) => println!("Task #{} marked as done.", id),
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Delete{id} => match store.delete_task(id){
            Ok(_) => println!("Task #{} deleted.", id),
            Err(e) => println!("Error: {}", e),
        },
    }

}

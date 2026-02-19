mod cli;
mod task;
mod store;

use cli::{parse_args, print_help, Command};
use std::env;
use store::Store;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = match parse_args(&args){
        Ok(cmd) => cmd,
        Err(e) => {
            eprint!("Error: {}", e);
            print_help();
            return;
        }
    };

    let mut store = Store::new();

    match command {
        Command::Add(title) => {
            let task = store.add_task(title);
            println!("Added task #{}: {}", task.id, task.title);
        }
        Command::List => {
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
        Command::Done(id) => match store.done_task(id) {
            Ok(_) => println!("Task #{} marked as done.", id),
            Err(e) => eprintln!("Error: {}", e),
        },
        Command::Delete(id) => match store.delete_task(id){
            Ok(_) => println!("Task #{} deleted.", id),
            Err(e) => println!("Error: {}", e),
        },
        Command::Help => print_help(),
    }

}

pub enum Command {
    Add(String),
    List,
    Done(u64),
    Delete(u64),
    Help,
}

pub fn parse_args(args: &[String]) -> Result<Command, String>{
    //parse the cli
    if args.len() < 2{
        return Ok(Command::Help);
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3{
                return Err("Usage: add <title>".to_string());
            }
            let title = args[2..].join(" ");
            Ok(Command::Add(title))
        }
        "list" => Ok(Command::List),
        "done" => {
            if args.len() < 3 {
                return Err("Usage: done <id>".to_string());
            }
            let id = args[2]
            .parse::<u64>()
            .map_err(|_| "id must be a positive integer".to_string())?;
            Ok(Command::Done(id))
        }
        "delete" => {
            if args.len() < 3 {
                return Err("Usage: delete <id>".to_string());
            }
            let id = args[2]
            .parse::<u64>()
            .map_err(|_| "id must be a positive integer".to_string())?;
            Ok(Command::Delete(id))
        }
        _ => Ok(Command::Help),
    }
}

pub fn print_help(){
    println!("rust-focus commands:");
    println!(" add <title> Add a task");
    println!(" list List tasks");
    println!(" done <id> Mark task as done");
    println!(" delete <id> Delete task");
}
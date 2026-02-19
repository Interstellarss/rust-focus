mod cli;
mod task;
mod store;

use task::Task;

fn main() {

    let mut t = Task::new(1, "Learn Rust lifecycle".to_string());

    println!("Before done: {t:#?}");

    t.mark_done();

    println!("After done: {t:#?}");

}

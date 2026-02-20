use crate::task::Task;

pub fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks yet.");
        return;
    }

    for t in tasks {
        let status = if t.done { "✅" } else { "⬜️" };
        println!("#{} {} {}", t.id, status, t.title);
    }
}

pub fn print_added(id: u64, title: &str) {
    println!("Added task #{}: {}", id, title);
}

pub fn print_done(id: u64) {
    println!("Task #{} mared as done.", id);
}

pub fn print_deleted(id: u64) {
    println!("Task #{} deleted.", id);
}

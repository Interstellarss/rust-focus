use crate::task::Task;


#[derive(Default)]
pub struct Store {
    pub tasks: Vec<Task>,
    next_id: u64,
}

impl Store {
    pub fn new() -> Self {
        Self { tasks: Vec::new(), next_id: 1, }
    }

    pub fn add_task(&mut self, title: String) -> &Task{
        let task = Task::new(self.next_id, title);
        self.tasks.push(task);
        self.next_id += 1;
        self.tasks.last().unwrap()
    }

    pub fn list_tasks(&self) -> &[Task]{
        &self.tasks
    }

    pub fn done_task(&mut self, id: u64) -> Result<(), String>{
        let task = self
        .tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| format!("Task with id {} not found", id))?;


        task.mark_done();
        Ok(())
    }

    pub fn delete_task(&mut self, id: u64) -> Result<(), String> {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() == before {
            return Err(format!("Task with id {} not found", id));
        }

        Ok(())
    }
    

}


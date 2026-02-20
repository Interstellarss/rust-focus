use crate::error::{AppError, AppResult};
use crate::task::Task;
use std::fs;
use std::path::PathBuf;

#[derive(Default)]
pub struct Store {
    pub tasks: Vec<Task>,
    next_id: u64,
    path: PathBuf,
}

impl Store {
    pub fn load(path: impl Into<PathBuf>) -> AppResult<Self> {
        let path = path.into();

        if !path.exists() {
            return Ok(Self {
                tasks: Vec::new(),
                next_id: 1,
                path,
            });
        }

        let content = fs::read_to_string(&path)?;

        let tasks: Vec<Task> = serde_json::from_str(&content).unwrap_or_default();
        let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        Ok(Self {
            tasks,
            next_id,
            path,
        })
    }

    fn save(&self) -> AppResult<()> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    pub fn add_task(&mut self, title: String) -> AppResult<&Task> {
        if title.trim().is_empty() {
            return Err(AppError::InvalidInput(
                "task title cannot be empty".to_string(),
            ));
        }

        let task = Task::new(self.next_id, title);
        self.tasks.push(task);
        self.next_id += 1;
        self.save()?;
        Ok(self.tasks.last().unwrap())
    }

    pub fn list_tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn done_task(&mut self, id: u64) -> AppResult<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(AppError::TaskNotFound(id))?;
        task.mark_done();
        self.save()
    }

    pub fn delete_task(&mut self, id: u64) -> AppResult<()> {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() == before {
            return Err(AppError::TaskNotFound(id));
        }
        self.save()
    }
}

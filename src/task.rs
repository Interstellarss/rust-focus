use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub done: bool,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(id: u64, title: String) -> Self {
        Self { id, title, done: false, created_at: Utc::now(), completed_at: None, }
    }

    pub fn mark_done(&mut self){
        self.done = true;
        self.completed_at = Some(Utc::now());
    }
}



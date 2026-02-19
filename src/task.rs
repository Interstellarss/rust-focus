#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub done: bool,
}
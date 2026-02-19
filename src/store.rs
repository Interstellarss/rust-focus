use crate::task::Task;


#[derive(Default)]
pub struct Store {
    pub tasks: Vec<Task>,
}
use std::sync::{Arc, Mutex};
use tracing::instrument;

#[derive(Clone, Default)]
pub struct TaskManager {
    inner: Arc<Mutex<Vec<Task>>>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[instrument]
    pub fn create_task(&self, title: String) -> Task {
        let mut tasks = self.inner.lock().unwrap();
        let id = tasks.len() as i32 + 1;
        let task = Task { id, title };
        tasks.push(task.clone());
        task
    }

    #[instrument]
    pub fn list_tasks(&self) -> Vec<Task> {
        let tasks = self.inner.lock().unwrap();
        tasks.clone()
    }
}

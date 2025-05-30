use crate::task_manager::{Task, TaskManager};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use tracing::instrument;

#[derive(serde::Deserialize)]
struct CreateTaskPayload {
    title: String,
}

pub fn router(manager: TaskManager) -> Router {
    Router::new()
        .route("/tasks", post(create_task).get(list_tasks))
        .with_state(manager)
}

#[instrument]
async fn create_task(
    State(manager): State<TaskManager>,
    Json(payload): Json<CreateTaskPayload>,
) -> Json<Task> {
    let task = manager.create_task(payload.title);
    Json(task)
}

#[instrument]
async fn list_tasks(State(manager): State<TaskManager>) -> Json<Vec<Task>> {
    let tasks = manager.list_tasks();
    Json(tasks)
}

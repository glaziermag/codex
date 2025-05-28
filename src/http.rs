use crate::task_manager::{Task, TaskManager};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use opentelemetry_prometheus::PrometheusHandle;
use tracing::instrument;

#[derive(serde::Deserialize)]
struct CreateTaskPayload {
    title: String,
}

#[derive(Clone)]
pub struct AppState {
    pub manager: TaskManager,
    pub metrics: PrometheusHandle,
}

pub fn router(manager: TaskManager, metrics: PrometheusHandle) -> Router {
    let state = AppState { manager, metrics };
    Router::new()
        .route("/tasks", post(create_task).get(list_tasks))
        .route("/metrics", get(metrics_handler))
        .with_state(state)
}

#[instrument]
async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskPayload>,
) -> Json<Task> {
    let task = state.manager.create_task(payload.title);
    Json(task)
}

#[instrument]
async fn list_tasks(State(state): State<AppState>) -> Json<Vec<Task>> {
    let tasks = state.manager.list_tasks();
    Json(tasks)
}

async fn metrics_handler(State(state): State<AppState>) -> String {
    state.metrics.render()
}

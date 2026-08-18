use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, patch};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

#[derive(Debug)]
enum ApiError {
    InvalidTitle,
    NotFound(u64),
    StateUnavailable,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Self::InvalidTitle => (
                StatusCode::UNPROCESSABLE_ENTITY,
                String::from("任务标题不能为空"),
            ),
            Self::NotFound(id) => (StatusCode::NOT_FOUND, format!("任务 {id} 不存在")),
            Self::StateUnavailable => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("服务状态暂时不可用"),
            ),
        };
        (status, Json(ErrorBody { error })).into_response()
    }
}

#[derive(Debug, Default)]
struct TodoStore {
    next_id: u64,
    tasks: BTreeMap<u64, Task>,
}

#[derive(Debug, Clone, Default)]
pub struct AppState(Arc<RwLock<TodoStore>>);

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/{id}", get(get_task).delete(delete_task))
        .route("/tasks/{id}/complete", patch(complete_task))
        .with_state(state)
}

async fn list_tasks(State(state): State<AppState>) -> Result<Json<Vec<Task>>, ApiError> {
    let store = state.0.read().map_err(|_| ApiError::StateUnavailable)?;
    Ok(Json(store.tasks.values().cloned().collect()))
}

async fn create_task(
    State(state): State<AppState>,
    Json(input): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>), ApiError> {
    let title = input.title.trim();
    if title.is_empty() {
        return Err(ApiError::InvalidTitle);
    }

    let mut store = state.0.write().map_err(|_| ApiError::StateUnavailable)?;
    store.next_id += 1;
    let task = Task {
        id: store.next_id,
        title: title.to_string(),
        completed: false,
    };
    store.tasks.insert(task.id, task.clone());
    Ok((StatusCode::CREATED, Json(task)))
}

async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Task>, ApiError> {
    let store = state.0.read().map_err(|_| ApiError::StateUnavailable)?;
    store
        .tasks
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(ApiError::NotFound(id))
}

async fn complete_task(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Task>, ApiError> {
    let mut store = state.0.write().map_err(|_| ApiError::StateUnavailable)?;
    let task = store.tasks.get_mut(&id).ok_or(ApiError::NotFound(id))?;
    task.completed = true;
    Ok(Json(task.clone()))
}

async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, ApiError> {
    let mut store = state.0.write().map_err(|_| ApiError::StateUnavailable)?;
    store
        .tasks
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or(ApiError::NotFound(id))
}

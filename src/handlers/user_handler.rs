use axum::{
    Json,
    extract::{State, Path},
    response::{IntoResponse, Response},
    http::StatusCode,
};
use crate::models::user::{User, NewUser};
use std::sync::{Arc, Mutex};

pub type UserDb = Arc<Mutex<Vec<User>>>;


// GET ハンドラー（変更なし）
pub async fn get_users(State(db): State<UserDb>) -> Json<Vec<User>> {
    let users = db.lock().unwrap();
    Json(users.clone())
}

// POST ハンドラー
pub async fn create_user(State(db): State<UserDb>, Json(new_user): Json<NewUser>) -> Json<User> {
    let mut users = db.lock().unwrap();
    let new_id = users.last().map(|u| u.id + 1).unwrap_or(1);
    let user = User { id: new_id, name: new_user.name };
    users.push(user.clone());
    Json(user)
}

// PUT ハンドラー
#[axum::debug_handler]
pub async fn update_user(
    Path(id): Path<u64>,
    State(db): State<UserDb>,
    Json(payload): Json<NewUser>,
) -> Response {
    let mut users = db.lock().unwrap();

    if let Some(user) = users.iter_mut().find(|u| u.id == id) {
        user.name = payload.name.clone();
        Json(user.clone()).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

// DELETE ハンドラー
#[axum::debug_handler]
pub async fn delete_user(
    Path(id): Path<u64>,
    State(db): State<UserDb>,
) -> Response {
    let mut users = db.lock().unwrap();
    // 指定IDのユーザーを削除して返す
    if let Some(pos) = users.iter().position(|u| u.id == id) {
        Json(users.remove(pos)).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
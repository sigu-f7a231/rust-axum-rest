use axum::{Json, extract::{State, Path}, http::StatusCode};
use super::UserDb;
use crate::models::user::{User, NewUser};

#[axum::debug_handler]
pub async fn update_user(
    Path(id): Path<u64>,
    State(db): State<UserDb>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let mut users = db.lock().unwrap();

    if let Some(user) = users.iter_mut().find(|u| u.id == id) {
        user.name = payload.name.clone();
        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

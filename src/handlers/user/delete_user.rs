use axum::{extract::{State, Path}, Json, http::StatusCode};
use super::UserDb;
use crate::models::user::User;

#[axum::debug_handler]
pub async fn delete_user(
    Path(id): Path<u64>,
    State(db): State<UserDb>,
) -> Result<Json<User>, StatusCode> {
    let mut users = db.lock().unwrap();

    if let Some(pos) = users.iter().position(|u| u.id == id) {
        let user = users.remove(pos);
        Ok(Json(user))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

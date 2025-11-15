use axum::{extract::State, extract::Path, Json};
use crate::handlers::user::UserDb;
use crate::handlers::user::get_user::service_get::{get_users_service, get_user_by_id_service};
use serde_json::json;

#[axum::debug_handler]
pub async fn get_users(State(db): State<UserDb>) -> Json<Vec<serde_json::Value>> {
    let users = get_users_service(&db);
    Json(users.into_iter().map(|u| json!({"id": u.id, "name": u.name})).collect())
}

#[axum::debug_handler]
pub async fn get_user_by_id(
    State(db): State<UserDb>,
    Path(id): Path<u64>,
) -> Json<serde_json::Value> {
    match get_user_by_id_service(&db, id) {
        Some(user) => Json(json!({"id": user.id, "name": user.name})),
        None => Json(json!({"error": "user not found"})),
    }
}

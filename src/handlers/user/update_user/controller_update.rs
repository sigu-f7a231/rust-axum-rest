use axum::{extract::State, extract::Path, Json};
use crate::handlers::user::{UserDb};
use crate::handlers::user::update_user::service_update::update_user_service;
use crate::models::user::NewUser;
use serde_json::json;

#[axum::debug_handler]
pub async fn update_user(
    State(db): State<UserDb>,
    Path(id): Path<u64>,
    Json(payload): Json<NewUser>,
) -> Json<serde_json::Value> {
    match update_user_service(&db, id, payload) {
        Some(user) => Json(json!({ "status": "updated", "user": user })),
        None => Json(json!({ "status": "not found" })),
    }
}

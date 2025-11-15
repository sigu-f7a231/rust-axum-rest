use axum::{extract::State, extract::Path, Json};
use crate::handlers::user::UserDb;
use crate::handlers::user::delete_user::service_delete::delete_user_service; // ← 修正
use serde_json::json;


#[axum::debug_handler]
pub async fn delete_user(
    State(db): State<UserDb>,
    Path(id): Path<u64>,
) -> Json<serde_json::Value> {
    match delete_user_service(&db, id) {
        Some(_) => Json(json!({ "status": "deleted" })),
        None => Json(json!({ "status": "not found" })),
    }
}

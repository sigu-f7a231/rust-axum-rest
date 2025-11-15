use axum::{routing::get, Router};
use crate::handlers::user::get_user::controller_get::{get_users, get_user_by_id};
use crate::handlers::user::UserDb;

pub fn get_user_router() -> Router<UserDb> {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user_by_id))  // 個別取得
}

use axum::{routing::put, Router};
use crate::handlers::user::update_user::controller_update::update_user;
use crate::handlers::user::UserDb;

pub fn update_user_router() -> Router<UserDb> {
    Router::new().route("/:id", put(update_user))
}

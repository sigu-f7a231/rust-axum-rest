use axum::{routing::delete, Router};
use crate::handlers::user::delete_user::controller_delete::delete_user;
use crate::handlers::user::UserDb;

pub fn delete_user_router() -> Router<UserDb> {
    Router::new().route("/:id", delete(delete_user))
}

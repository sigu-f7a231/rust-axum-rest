use axum::{routing::post, Router};
use crate::handlers::user::create_user::controller_create::create_user;
use crate::handlers::user::UserDb;

pub fn create_user_router() -> Router<UserDb> {
    Router::new().route("/users", post(create_user))
}

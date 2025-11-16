use axum::Router;

use crate::handlers::user::{UserDb, route::user_crud_router};

pub fn user_router() -> Router<UserDb> {
    Router::new().nest("/users", user_crud_router())
}

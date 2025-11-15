use axum::Router;
use crate::router::user_router::user_router;
use crate::handlers::user::UserDb;

pub fn api_router() -> Router<UserDb> {
    Router::new().merge(user_router())
}

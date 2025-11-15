use axum::Router;

use crate::handlers::user::{
    get_user::router::get_user_router,
    create_user::router::create_user_router,
    update_user::router::update_user_router,
    delete_user::router::delete_user_router,
};

use crate::handlers::user::UserDb;

pub fn user_router() -> Router<UserDb> {
    Router::new()
        .merge(get_user_router())
        .merge(create_user_router())
        .merge(update_user_router())
        .merge(delete_user_router())
}

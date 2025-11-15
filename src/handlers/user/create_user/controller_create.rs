use axum::{Json, extract::State};
use crate::handlers::user::UserDb;
use crate::handlers::user::create_user::service_create::create_user_service;
use crate::models::user::{User, NewUser};

pub async fn create_user(
    State(db): State<UserDb>,
    Json(new_user): Json<NewUser>,
) -> Json<User> {
    let created = create_user_service(&db, new_user);
    Json(created)
}

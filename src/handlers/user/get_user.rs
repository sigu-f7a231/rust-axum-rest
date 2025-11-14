use axum::{Json, extract::State};
use super::UserDb;
use crate::models::user::User;

pub async fn get_users(State(db): State<UserDb>) -> Json<Vec<User>> {
    let users = db.lock().unwrap();
    Json(users.clone())
}

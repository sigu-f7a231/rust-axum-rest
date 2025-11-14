use axum::{Json, extract::State};

use super::UserDb;
use crate::models::user::{User, NewUser};

pub async fn create_user(
    State(db): State<UserDb>,
    Json(new_user): Json<NewUser>,
) -> Json<User> {
    let mut users = db.lock().unwrap();
    let new_id = users.last().map(|u| u.id + 1).unwrap_or(1);

    let user = User {
        id: new_id,
        name: new_user.name,
    };

    users.push(user.clone());
    Json(user)
}

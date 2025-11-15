use crate::handlers::user::UserDb;
use crate::models::user::{User, NewUser};

pub fn create_user_service(db: &UserDb, new: NewUser) -> User {
    let mut users = db.lock().unwrap();
    let new_id = users.last().map(|u| u.id + 1).unwrap_or(1);

    let user = User {
        id: new_id,
        name: new.name,
    };

    users.push(user.clone());
    user
}

use crate::handlers::user::UserDb;
use crate::models::user::{User, NewUser};

pub fn update_user_service(db: &UserDb, id: u64, new: NewUser) -> Option<User> {
    let mut users = db.lock().unwrap();

    if let Some(user) = users.iter_mut().find(|u| u.id == id) {
        user.name = new.name;
        return Some(user.clone());
    }
    None
}

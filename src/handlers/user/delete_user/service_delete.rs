use crate::handlers::user::UserDb;
use crate::models::user::User;

pub fn delete_user_service(db: &UserDb, id: u64) -> Option<User> {
    let mut users = db.lock().unwrap();
    if let Some(pos) = users.iter().position(|u| u.id == id) {
        Some(users.remove(pos))
    } else {
        None
    }
}

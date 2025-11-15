use crate::handlers::user::UserDb;
use crate::models::user::User;

pub fn get_users_service(db: &UserDb) -> Vec<User> {
    let db = db.lock().unwrap();
    db.clone()
}

// 個別取得サービス
pub fn get_user_by_id_service(db: &UserDb, id: u64) -> Option<User> {
    let db = db.lock().unwrap();
    db.iter().find(|u| u.id == id).cloned()
}

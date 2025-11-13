use crate::models::user::User;
use std::sync::{Arc, Mutex};

pub type Db = Arc<Mutex<Vec<User>>>;

pub fn init_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}

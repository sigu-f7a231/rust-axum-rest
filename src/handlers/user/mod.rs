pub mod get_user;
pub mod create_user;
pub mod update_user;
pub mod delete_user;
pub mod route;   // ← ★ 追加！

use std::sync::{Arc, Mutex};
use crate::models::user::User;

pub type UserDb = Arc<Mutex<Vec<User>>>;

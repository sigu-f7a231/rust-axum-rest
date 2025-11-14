pub mod get_user;
pub mod create_user;
pub mod update_user;
pub mod delete_user;

use crate::models::user::User;
use std::sync::{Arc, Mutex};

pub type UserDb = Arc<Mutex<Vec<User>>>;

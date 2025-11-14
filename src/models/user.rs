use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewUser {
    pub name: String,
}

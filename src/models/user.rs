#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
}

// POST用にクライアントが送る構造体
#[derive(Clone, Debug, serde::Deserialize)]
pub struct NewUser {
    pub name: String,
}

use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod models;
mod handlers;

use handlers::user_handler::{get_users, create_user, UserDb};
use crate::models::user::User;

#[tokio::main]
async fn main() {
    // 共有データベース
    let db: UserDb = Arc::new(Mutex::new(vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
    ]));

    // Router
    let app = Router::new()
        .route("/users", get(get_users).post(create_user))
        .with_state(db);  // 共有状態を渡す

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

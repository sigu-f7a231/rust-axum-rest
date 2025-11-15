use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod models;
mod handlers;
mod router;

use crate::router::api_router::api_router;
use crate::models::user::User;
use crate::handlers::user::UserDb;

#[tokio::main]
async fn main() {
    let db: UserDb = Arc::new(Mutex::new(vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
    ]));

    // 全体ルーターだけ呼ぶ
    let app = api_router().with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

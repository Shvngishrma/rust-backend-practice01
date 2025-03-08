use axum::{routing::get, Router, extract::Path, Json};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize)] // ✅ Enable automatic JSON serialization
struct User {
    id: String,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust Backend!" }))
        .route("/users/{id}", get(get_user)) // ✅ Returns JSON now
        .route("/users", get(get_all_users)); // ✅ New route for multiple users

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// ✅ Return JSON instead of plain text
async fn get_user(Path(id): Path<String>) -> Json<User> {
    let user = User {
        id: id.clone(),
        name: format!("User {}", id),
        email: format!("user{}@example.com", id),
    };
    Json(user) // ✅ Returns JSON response
}

// ✅ New route to list multiple users
async fn get_all_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: "1".to_string(), name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: "2".to_string(), name: "Bob".to_string(), email: "bob@example.com".to_string() },
    ];
    Json(users)
}

use axum::{routing::{get, post}, Router, extract::Path, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Create the router with routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust Backend!" }))
        .route("/users/{id}", get(get_user)) // ✅ GET single user
        .route("/users", get(get_all_users)) // ✅ GET all users
        .route("/users", post(create_user)); // ✅ POST new user

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    // Bind and serve the application
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

// Handler for GET /users/{id}
async fn get_user(Path(id): Path<String>) -> String {
    format!("Hello, user with id: {}", id)
}

// Handler for GET /users (Returns an empty list for now)
async fn get_all_users() -> Json<Vec<User>> {
    let users = vec![]; // Placeholder empty list
    Json(users)
}

// Struct for user creation request
#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

// Struct for user response
#[derive(Debug, Serialize)]
struct User {
    id: String,
    name: String,
    email: String,
}

// Handler for POST /users
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    let user = User {
        id: "100".to_string(), // Simulated ID
        name: payload.name,
        email: payload.email,
    };
    Json(user) // Return JSON response
}

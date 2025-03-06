use axum::{routing::get, Router, extract::Path};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Create a basic router with GET routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust Backend!" }))
        .route("/users/:id", get(get_user)); // ✅ Use function reference

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    // Bind the server to the address
    let listener = TcpListener::bind(addr).await.unwrap();

    // Serve the application
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// ✅ Correct handler function
async fn get_user(Path(id): Path<String>) -> String {
    format!("Hello, user with id: {}", id)
}

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Create a basic router with a simple GET route
    let app = Router::new().route("/", get(|| async { "Hello, Rust Backend!" }));

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

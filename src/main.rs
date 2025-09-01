use axum_server::Server;
use std::net::SocketAddr;
use rust_server::app;  // Import the app from lib.rs

#[tokio::main]
async fn main() {
    let app = app();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {}", addr);
    Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

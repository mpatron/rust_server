use axum::http::StatusCode;
use axum_server::Server;
use reqwest::Client;
use rust_server::hello::hello_routers;
use std::net::SocketAddr; // Import the app from lib.rs
use tracing::info;

#[tokio::test]
async fn test_hello_world_integration() {
    info!("Running test intégration test_hello_world_integration");

    // Set up the app
    let app = hello_routers();

    // Start the server in a background task
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let server = Server::bind(addr).serve(app.into_make_service());
    tokio::spawn(server);

    // Use reqwest to send a GET request to the server
    let client = Client::new();
    let res = client.get("http://localhost:8000/").send().await.unwrap();

    // Check if the response is as expected
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().await.unwrap();
    assert_eq!(body, "Hello, World! 🤣");
    info!("test_hello_world_integration passed");
}

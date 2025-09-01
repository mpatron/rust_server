use axum::http::StatusCode;
use axum_server::Server;
use reqwest::Client;
use std::net::SocketAddr;
use rust_server::app; // Import the app from lib.rs

#[tokio::test]
async fn test_hello_world_integration() {
    // Set up the app
    let app = app();

    // Start the server in a background task
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let server = Server::bind(addr)
        .serve(app.into_make_service());
    tokio::spawn(server);

    // Use reqwest to send a GET request to the server
    let client = Client::new();
    let res = client.get("http://localhost:8000/")
        .send()
        .await
        .unwrap();

    // Check if the response is as expected
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().await.unwrap();
    assert_eq!(body, "Hello, World! 🤣");
}

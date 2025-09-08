use axum_server::Server;
use rust_server::app;
use std::net::SocketAddr; // Import the app from lib.rs
use tracing::info;
use tracing_subscriber;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting server...");
    let app = app();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    info!("Listening on {}", addr);
    Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    info!("Server stopped");
}

#[cfg(test)]
mod tests {
    use tracing::info;

    #[tokio::test]
    async fn test_main() {
        info!("Running unit test_main");
        assert_eq!(1, 1);
    }
}
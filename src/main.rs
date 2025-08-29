use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Crée une route qui répond à GET "/"
    let app = Router::new().route("/", get(handler));

    // Adresse du serveur
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Serveur en écoute sur {}", addr);

    // Lance le serveur
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Gestionnaire pour la route "/"
async fn handler() -> &'static str {
    "Bonjour, monde !"
}

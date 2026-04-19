mod hello;
mod upload;
use axum::{Json, Router, extract::ConnectInfo, routing::{get, post}};
use tower_http::trace::TraceLayer;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_logs() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
}

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {
    init_logs();

    let version = "APP_VERSION";
    match env::var(version) {
        Ok(val) => println!("{version}: {val:?}"),
        Err(e) => println!("La varible {version} est vide: {e}"),
    }

    info!("🚀 Server starting...");
    let app = Router::new()
        .merge(hello::hello_routers())
        .merge(upload::upload_routers())
        .merge(app_default())
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("🧱 Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    info!("🦀 Server stopped");
    Ok(())
}

fn app_default() -> Router {
    Router::new()
        //.route("/", get(|| async { "Hello, World!" }))
        .route(
            "/json",
            post(|payload: Json<serde_json::Value>| async move {
                Json(serde_json::json!({ "data": payload.0 }))
            }),
        )
        .route(
            "/requires-connect-info",
            get(|ConnectInfo(addr): ConnectInfo<SocketAddr>| async move { format!("Hi {addr}") }),
        )
        // We can still add middleware
        .layer(TraceLayer::new_for_http())
}

// https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{self, Request, StatusCode}};
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    use tracing::info;
    use serde_json::{json, Value};

    #[tokio::test]
    async fn test_main() {
        info!("Running unit test_main");
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn json_app_default() {
        let app = app_default();

        let response = app
            .oneshot(
                Request::post("/json")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
    }
}

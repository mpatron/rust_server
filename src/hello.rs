use axum::{Router, response::IntoResponse, routing::get};
use tracing::info;
// use apiresponse::{ApiResponse, ApiError};

async fn hello_world() -> &'static str {
    info!("Running hello_world({})", "Hello, World! 🤣");
    "Hello, World! 🤣"
}

async fn health_check() -> impl IntoResponse {
    "✅ Healthy"
}

pub fn hello_routers() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };    
    use tracing::info;
    use tower::util::ServiceExt;
    use http_body_util::BodyExt; // for `collect`

    #[tokio::test]
    async fn test_hello_world() {
        info!("Running unit test_hello_world");
        let response = hello_world().await;
        assert_eq!(response, "Hello, World! 🤣");
    }

    #[tokio::test]
    async fn test_hello_world2() {
        let app = hello_routers();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        info!("Response body: {:?}", body);
        assert_eq!(str::from_utf8(&body[..]).unwrap(), "Hello, World! 🤣");
    }    
}

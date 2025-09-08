use axum::{
    extract::{Multipart, Path},
    response::Html,
    routing::{get, post},
    Router,
};
use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define routes
    let app = Router::new()
        .route("/", get(upload_form))
        .route("/upload", post(upload_file));

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Serve a simple HTML form for file upload
async fn upload_form() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
            <form action="/upload" method="post" enctype="multipart/form-data">
                <input type="file" name="file" />
                <button type="submit">Upload</button>
            </form>
        </body>
        </html>
        "#,
    )
}

// Handle file upload
async fn upload_file(mut multipart: Multipart) -> String {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        let data = field.bytes().await.unwrap();

        // Save the file to the local filesystem
        let mut file = File::create(format!("./uploads/{}", file_name)).unwrap();
        file.write_all(&data).unwrap();

        return format!("File '{}' uploaded successfully!", file_name);
    }

    "No file uploaded!".to_string()
}
use std::fs;

use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use tower_http::services::ServeFile;

async fn notes() -> impl IntoResponse {
    let html = fs::read_to_string("notes.html")
        .unwrap_or_else(|_| "<h1>Uh oh where is my notes</h1>".to_string());
    Html(html)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(notes))
        .nest_service("/css", ServeFile::new("static/build.css"))
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Started server on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

use axum::{response::IntoResponse, routing::get, Json, Router};

mod domain;
mod errors;

async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Server is running";

    let json_response = serde_json::json!({
        "status": "OK",
        "message": MESSAGE
    });

    Json(json_response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/health", get(health_check_handler));
    println!("Server starting...");
    let listner = tokio::net::TcpListener::bind("0.0.0.0:42001")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}

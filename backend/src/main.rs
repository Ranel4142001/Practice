use axum::{
    routing::{get, post},
    Json, Router,
    http::{StatusCode, header, Method},
};
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;

mod greet;
mod users;

use greet::handler::greet;
use users::handler::list_users;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST, Method::GET])
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/greet", post(greet))
        .route("/users", get(list_users))
        .layer(cors);

    println!("🚀 Server running on http://localhost:3000");

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app).await.expect("Server crashed");
}

use axum::{
    routing::{get, post},
    Json, Router,
    http::{StatusCode, header, Method},
    Extension,
};
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;
use mysql::*;

// Declare modules (folders)
mod greet;
mod users;
mod db; // new module for database setup

// Import handler functions
use greet::handler::greet;
use users::handler::list_users;
// Import centralized DB initializer
use db::init_pool;

/// This function handles requests to `/`
/// When you open http://localhost:3000 in the browser,
/// this is what will be shown.
async fn root() -> &'static str {
    "🚀 Backend is running successfully!"
}

#[tokio::main]
async fn main() {
    // -----------------------------
    // CORS configuration
    // -----------------------------
    // This allows the frontend (localhost:5173)
    // to talk to the backend (localhost:3000)
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow requests from any origin
        .allow_methods([Method::GET, Method::POST]) // Allowed HTTP methods
        .allow_headers([header::CONTENT_TYPE]); // Allowed headers

    // -----------------------------
    // Initialize DB pool once
    // -----------------------------
    let pool = init_pool();

    // -----------------------------
    // Router configuration
    // -----------------------------
    let app = Router::new()
        // Root route (GET /)
        .route("/", get(root))

        // API routes
        .route("/greet", post(greet))     // POST /greet
        .route("/users", get(list_users)) // GET /users

        // Apply CORS middleware
        .layer(cors)

        // Share DB pool with all handlers
        .layer(Extension(pool));

    println!("🚀 Server running on http://localhost:3000");

    // -----------------------------
    // Start the server
    // -----------------------------
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Server crashed");
}

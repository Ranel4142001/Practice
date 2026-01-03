// Bring in Axum tools:
// - Json: lets us read JSON from requests and send JSON back
// - StatusCode: lets us send proper HTTP status codes (like 200, 500, etc.)
use axum::{Json, http::StatusCode};

// Bring in MySQL crate and helper traits for database work
use mysql::*;
use mysql::prelude::*;

// Bring in our request and response data models
use crate::greet::model::{GreetRequest, GreetResponse};

/// Handles POST requests to `/greet`
///
/// Example request body (JSON):
/// {
///   "name": "Ranel"
/// }
///
/// Example response (JSON):
/// {
///   "message": "Hello, Ranel! Welcome 👋"
/// }
pub async fn greet(
    // Take the JSON body from the request and turn it into a GreetRequest struct
    Json(payload): Json<GreetRequest>,
) -> Result<Json<GreetResponse>, (StatusCode, String)> {

    // -----------------------------
    // Database connection details
    // -----------------------------
    // Format: mysql://username:password@host:port/database
    let url = "mysql://root:Latterdaysaints1401!@localhost:3306/greeting_app";

    // -----------------------------
    // Turn the connection string into MySQL options
    // -----------------------------
    let opts = Opts::from_url(url)
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Invalid DB URL: {}", e),
        ))?;

    // -----------------------------
    // Create a pool of database connections
    // -----------------------------
    // A pool lets us reuse connections instead of opening a new one each time
    let pool = Pool::new(opts)
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB Pool error: {}", e),
        ))?;

    // -----------------------------
    // Get one connection from the pool
    // -----------------------------
    let mut conn = pool.get_conn()
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB Conn error: {}", e),
        ))?;

    // -----------------------------
    // Save the user's name into the database
    // -----------------------------
    // `?` is a placeholder so we don’t risk SQL injection
    conn.exec_drop(
        "INSERT INTO users (name) VALUES (?)",
        (payload.name.clone(),),
    )
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("DB Insert error: {}", e),
    ))?;

    // -----------------------------
    // Send back a JSON response
    // -----------------------------
    Ok(Json(GreetResponse {
        message: format!("Hello, {}! Welcome 👋", payload.name),
    }))
}

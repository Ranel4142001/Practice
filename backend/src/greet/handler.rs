// Bring in Axum tools
use axum::{Json, Extension, http::StatusCode};

// Bring in MySQL Pool type
use mysql::Pool;

// Import common MySQL traits (e.g. Queryable, FromRow) 
// so we can run queries like exec_drop and query_map
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
    // Get the shared DB pool from app state
    Extension(pool): Extension<Pool>,
    // Take the JSON body from the request and turn it into a GreetRequest struct
    Json(payload): Json<GreetRequest>,
) -> Result<Json<GreetResponse>, (StatusCode, String)> {
    // -----------------------------
    // Get a connection from the pool
    // -----------------------------
    let mut conn = pool.get_conn()
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB Conn error: {}", e),
        ))?;

    // -----------------------------
    // Save the user's name into the database
    // -----------------------------
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

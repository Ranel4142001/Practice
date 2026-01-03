//Bring in Axum tools
use axum::{Json, http::StatusCode};

//Bring in MySQL Pool type
use mysql::*;

// Import common MySQL traits (e.g. Queryable, FromRow) 
// so we can run queries like exec_drop and query_map
use mysql::prelude::*;

//Bring in our request and response data models
use crate::users::model::User;

/// Handles GET requests to `/users`
///
/// Response (JSON):
/// [
///   { "id": 1, "name": "Ranel" },
///   { "id": 2, "name": "Alice" }
/// ]
pub async fn list_users() -> Result<Json<Vec<User>>, (StatusCode, String)> {
    // -----------------------------
    // Database connection details
    // -----------------------------
    let url = "mysql://root:Latterdaysaints1401!@localhost:3306/greeting_app";

    // -----------------------------
    // Turn the connection string into MySQL options
    // -----------------------------
    let opts = Opts::from_url(url)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Invalid DB URL: {}", e)))?;

    // -----------------------------
    // Create a pool of database connections
    // -----------------------------
    // A pool lets us reuse connections instead of opening a new one each time
    let pool = Pool::new(opts)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Pool error: {}", e)))?;

    // -----------------------------
    // Get one connection from the pool
    // -----------------------------
    let mut conn = pool.get_conn()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Conn error: {}", e)))?;

    // -----------------------------
    // Run a query to get all users
    // -----------------------------
    // For each row (id, name), build a User struct
    let users: Vec<User> = conn
        .query_map("SELECT id, name FROM users", |(id, name)| User { id, name })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Query error: {}", e)))?;

    // -----------------------------
    // Send back the list of users as JSON
    // -----------------------------
    Ok(Json(users))
}

use axum::{
    routing::{get, post},
    Json, Router,
    http::{StatusCode, header, Method},
};
use serde::{Deserialize, Serialize};
use mysql::*;
use mysql::prelude::*;
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct GreetRequest {
    name: String,
}

#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

async fn greet(Json(payload): Json<GreetRequest>) -> Result<Json<GreetResponse>, (StatusCode, String)> {
    let url = "mysql://root:Latterdaysaints1401!@localhost:3306/greeting_app";
    let opts = Opts::from_url(url).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Invalid DB URL: {}", e)))?;
    let pool = Pool::new(opts).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Pool error: {}", e)))?;
    let mut conn = pool.get_conn().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Conn error: {}", e)))?;

    conn.exec_drop("INSERT INTO users (name) VALUES (?)", (payload.name.clone(),))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Insert error: {}", e)))?;

    Ok(Json(GreetResponse {
        message: format!("Hello, {}! Welcome 👋", payload.name),
    }))
}

async fn list_users() -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let url = "mysql://root:Latterdaysaints1401!@localhost:3306/greeting_app";
    let opts = Opts::from_url(url).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Invalid DB URL: {}", e)))?;
    let pool = Pool::new(opts).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Pool error: {}", e)))?;
    let mut conn = pool.get_conn().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Conn error: {}", e)))?;

    let users: Vec<User> = conn
        .query_map("SELECT id, name FROM users", |(id, name)| User { id, name })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Query error: {}", e)))?;

    Ok(Json(users))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST, Method::GET]) // ✅ allow GET for React
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/greet", post(greet))
        .route("/users", get(list_users)) // ✅ new endpoint
        .layer(cors);

    println!("🚀 Server running on http://localhost:3000");

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app).await.expect("Server crashed");
}

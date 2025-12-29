use axum::{
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use mysql::*;
use mysql::prelude::*;
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct GreetRequest {
    name: String,
}

#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

async fn greet(Json(payload): Json<GreetRequest>) -> Json<GreetResponse> {
    // Database connection
    let url = "mysql://root:Latterdaysaints1401!@localhost:3306/greeting_app";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // Insert name into DB
    conn.exec_drop(
        "INSERT INTO users (name) VALUES (?)",
        (payload.name.clone(),),
    ).unwrap();

    Json(GreetResponse {
        message: format!("Hello, {}! Welcome 👋", payload.name),
    })
}




#[tokio::main] async fn main() {
    let app = axum::Router::new() 
    .route("/greet", axum::routing::post(greet)) 
    .layer(tower_http::cors::CorsLayer::permissive());
    
    println!("🚀 Server running on http://localhost:3000"); 
    
    // Explicit type annotation fixes E0282 
    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap(); 
    axum::serve(listener, app).await.unwrap();
 }
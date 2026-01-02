use axum::{Json, http::StatusCode};
use mysql::*;
use mysql::prelude::*;
use crate::greet::model::{GreetRequest, GreetResponse};

pub async fn greet(Json(payload): Json<GreetRequest>) -> Result<Json<GreetResponse>, (StatusCode, String)> {
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

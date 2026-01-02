use axum::{Json, http::StatusCode};
use mysql::*;
use mysql::prelude::*;
use crate::users::model::User;

pub async fn list_users() -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let url = "mysql://root:Latterdaysaints1401!@localhost:3306/greeting_app";
    let opts = Opts::from_url(url).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Invalid DB URL: {}", e)))?;
    let pool = Pool::new(opts).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Pool error: {}", e)))?;
    let mut conn = pool.get_conn().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Conn error: {}", e)))?;

    let users: Vec<User> = conn
        .query_map("SELECT id, name FROM users", |(id, name)| User { id, name })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Query error: {}", e)))?;

    Ok(Json(users))
}

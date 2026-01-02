use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GreetRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct GreetResponse {
    pub message: String,
}

// Serde is a library that helps convert data
// between Rust structs and formats like JSON.
// (Deserialize = read JSON into a struct, Serialize = turn a struct into JSON)
use serde::{Deserialize, Serialize};

// -----------------------------
// Request model
// -----------------------------
// This struct describes the JSON data
// that the frontend sends TO the backend.
//
// Example request (JSON):
// {
//   "name": "Ranel"
// }
#[derive(Deserialize)]
pub struct GreetRequest {
    pub name: String,
}

// -----------------------------
// Response model
// -----------------------------
// This struct describes the JSON data
// that the backend sends BACK to the frontend.
//
// Example response (JSON):
// {
//   "message": "Hello, Ranel! Welcome 👋"
// }
#[derive(Serialize)]
pub struct GreetResponse {
    pub message: String,
}

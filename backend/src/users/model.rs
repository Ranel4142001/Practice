// Serde is a library that helps convert Rust structs into JSON (Serialize)
// so we can send them as responses in our API.
use serde::Serialize;

// -----------------------------
// User model
// -----------------------------
// This struct represents a single user record.
// It matches the data we store in the database
// and send back to the frontend as JSON.
//
// Example JSON response:
// {
//   "id": 1,
//   "name": "Ranel"
// }
#[derive(Serialize)]
pub struct User {
    pub id: i32,      // Unique ID number for the user
    pub name: String, // The user's name
}

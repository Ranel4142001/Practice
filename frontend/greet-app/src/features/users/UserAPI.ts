// frontend/greet-app/src/features/users/usersAPI.ts

// -----------------------------
// fetchUsers function
// -----------------------------
// This function asks the backend for the list of users.
// It sends a GET request to http://localhost:3000/users
// and returns the response as JSON.
//
// Example JSON response:
// [
//   { "id": 1, "name": "Ranel" },
//   { "id": 2, "name": "Alice" }
// ]
export async function fetchUsers() {
    // -----------------------------
    // Send a GET request to the backend
    // -----------------------------
    const response = await fetch("http://localhost:3000/users");

    // -----------------------------
    // Check if the request worked
    // -----------------------------
    // If not, throw an error so it can be handled elsewhere
    if (!response.ok) {
        throw new Error("Failed to fetch users");
    }

    // -----------------------------
    // Convert the response into JSON
    // -----------------------------
    // The result will be an array of user objects
    return response.json(); // [{ id: 1, name: "Ranel" }, ...]
}

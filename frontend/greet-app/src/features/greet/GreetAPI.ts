// frontend/greet-app/src/features/greet/greetAPI.ts

// This function sends the user's name to the backend
// and gets back a greeting message in JSON.
export async function sendGreeting(name: string) {
  // -----------------------------
  // Send a POST request to the backend
  // -----------------------------
  // - URL: http://localhost:3000/greet
  // - Method: POST
  // - Headers: tell the server we're sending JSON
  // - Body: convert { name } into JSON text
  const response = await fetch("http://localhost:3000/greet", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ name }),
  });

  // -----------------------------
  // Check if the request worked
  // -----------------------------
  // If not, throw an error so we can handle it elsewhere
  if (!response.ok) {
    throw new Error("Failed to send greeting");
  }

  // -----------------------------
  // Convert the response back into JSON
  // -----------------------------
  // Example response:
  // { "message": "Hello, Ranel! Welcome 👋" }
  return response.json();
}

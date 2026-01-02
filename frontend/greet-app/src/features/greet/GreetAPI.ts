// frontend/greet-app/src/features/greet/greetAPI.ts
export async function sendGreeting(name: string) {
  const response = await fetch("http://localhost:3000/greet", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ name }),
  });

  if (!response.ok) {
    throw new Error("Failed to send greeting");
  }

  return response.json(); // { message: "Hello, Ranel! Welcome 👋" }
}

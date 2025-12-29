"use strict";
// Grab elements from the DOM with proper type assertions
const input = document.getElementById("nameInput");
const button = document.getElementById("btn");
const output = document.getElementById("output");
// Add click event listener to the button
button.addEventListener("click", async () => {
    const name = input.value.trim();
    // Simple validation
    if (!name) {
        output.textContent = "⚠️ Please enter a name.";
        return;
    }
    try {
        // Send POST request to Rust backend
        const response = await fetch("http://localhost:3000/greet", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ name })
        });
        if (!response.ok) {
            output.textContent = `❌ Error: ${response.status}`;
            return;
        }
        // Parse JSON response
        const data = await response.json();
        output.textContent = data.message;
    }
    catch (err) {
        console.error(err);
        output.textContent = "🚫 Failed to connect to backend.";
    }
});

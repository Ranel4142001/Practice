// frontend/greet-app/src/features/greet/GreetForm.tsx

// Import React's useState hook for managing component state
import { useState } from "react";
// Import the function that talks to the backend
import { sendGreeting } from "./GreetAPI";

// -----------------------------
// GreetForm Component
// -----------------------------
// This component shows a simple form where the user
// can type their name, send it to the backend,
// and display the greeting message returned.
export default function GreetForm() {
    // -----------------------------
    // State variables
    // -----------------------------
    // name → stores the text typed into the input field
    // message → stores the greeting or error message from the backend
    const [name, setName] = useState("");
    const [message, setMessage] = useState("");

    // -----------------------------
    // Handle form submission
    // -----------------------------
    // Prevents the page from reloading,
    // sends the name to the backend,
    // and updates the message state with the response.
    async function handleSubmit(e: React.FormEvent) {
        e.preventDefault(); // stop default form behavior
        try {
            const data = await sendGreeting(name); // call backend API
            setMessage(data.message);              // show greeting message
        } catch (err) {
            setMessage("Error sending greeting");  // show error if request fails
        }
    }

    // -----------------------------
    // Render the form and message
    // -----------------------------
    // - Input field for the user's name
    // - Button to submit the form
    // - Paragraph to show the greeting or error message
    return (
        <div>
            <form onSubmit={handleSubmit}>
                <input
                    value={name}
                    onChange={(e) => setName(e.target.value)} // update state as user types
                    placeholder="Enter your name"
                />
                <button type="submit">Greet Me</button>
            </form>
            {message && <p>{message}</p>}
        </div>
    );
}

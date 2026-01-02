// frontend/greet-app/src/features/greet/GreetForm.tsx
import { useState } from "react";
import { sendGreeting } from "./GreetAPI";

export default function GreetForm() {
    const [name, setName] = useState("");
    const [message, setMessage] = useState("");

    async function handleSubmit(e: React.FormEvent) {
        e.preventDefault();
        try {
            const data = await sendGreeting(name);
            setMessage(data.message);
        } catch (err) {
            setMessage("Error sending greeting");
        }
    }

    return (
        <div>
            <form onSubmit={handleSubmit}>
                <input
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    placeholder="Enter your name"
                />
                <button type="submit">Greet Me</button>
            </form>
            {message && <p>{message}</p>}
        </div>
    );
}

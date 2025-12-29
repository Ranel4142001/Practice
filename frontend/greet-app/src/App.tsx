import { useState } from "react";

function App() {
  const [name, setName] = useState("");
  const [message, setMessage] = useState("");

  const greet = async () => {
    if (!name.trim()) {
      setMessage("⚠️ Please enter a name.");
      return;
    }

    try {
      const response = await fetch("http://localhost:3000/greet", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name }),
      });

      if (!response.ok) {
        setMessage(`❌ Error: ${response.status}`);
        return;
      }

      const data: { message: string } = await response.json();
      setMessage(data.message);
    } catch (err) {
      console.error(err);
      setMessage("🚫 Failed to connect to backend.");
    }
  };

  return (
    <div style={{ fontFamily: "sans-serif", padding: "2rem" }}>
      <h1>Greeting System</h1>
      <input
        type="text"
        placeholder="Enter your name"
        value={name}
        onChange={(e) => setName(e.target.value)}
      />
      <button onClick={greet}>Greet Me</button>
      <p>{message}</p>
    </div>
  );
}

export default App;

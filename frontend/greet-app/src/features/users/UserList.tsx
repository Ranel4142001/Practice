// frontend/greet-app/src/features/users/UserList.tsx

// Import React hooks for state and lifecycle
import { useEffect, useState } from "react";
// Import the function that fetches users from the backend
import { fetchUsers } from "./UserAPI";

// -----------------------------
// User interface
// -----------------------------
// Defines the shape of a user object
// Example: { id: 1, name: "Ranel" }
interface User {
    id: number;
    name: string;
}

// -----------------------------
// UserList Component
// -----------------------------
// This component shows a list of users.
// It loads data from the backend when the component first appears.
export default function UserList() {
    // -----------------------------
    // State: users
    // -----------------------------
    // Holds the array of user objects fetched from the backend
    const [users, setUsers] = useState<User[]>([]);

    // -----------------------------
    // useEffect: load users on mount
    // -----------------------------
    // Runs once when the component is first rendered.
    // Calls fetchUsers() and saves the result in state.
    useEffect(() => {
        async function loadUsers() {
            try {
                const data = await fetchUsers(); // get users from backend
                setUsers(data);                  // update state with data
            } catch (err) {
                console.error(err);              // log error if request fails
            }
        }
        loadUsers();
    }, []);

    // -----------------------------
    // Render the list
    // -----------------------------
    // Shows a heading and a list of user names
    return (
        <div>
            <h2>Guest List</h2>
            <ul>
                {users.map((u) => (
                    <li key={u.id}>{u.name}</li>
                ))}
            </ul>
        </div>
    );
}

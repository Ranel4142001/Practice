// frontend/greet-app/src/features/users/UserList.tsx
import { useEffect, useState } from "react";
import { fetchUsers } from "./UserAPI";

interface User {
    id: number;
    name: string;
}

export default function UserList() {
    const [users, setUsers] = useState<User[]>([]);

    useEffect(() => {
        async function loadUsers() {
            try {
                const data = await fetchUsers();
                setUsers(data);
            } catch (err) {
                console.error(err);
            }
        }
        loadUsers();
    }, []);

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

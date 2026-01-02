// frontend/greet-app/src/features/users/usersAPI.ts
export async function fetchUsers() {
    const response = await fetch("http://localhost:3000/users");

    if (!response.ok) {
        throw new Error("Failed to fetch users");
    }

    return response.json(); // [{ id: 1, name: "Ranel" }, ...]
}
    
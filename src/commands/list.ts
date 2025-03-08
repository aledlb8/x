import { loadStorage } from "../storage";

export function listItems() {
    const storage = loadStorage();

    if (Object.keys(storage).length === 0) {
        console.log("🔴 No stored items found.");
        return;
    }

    console.table(
        Object.entries(storage).map(([key, entry]) => ({
            Name: key,
            Type: entry.type,
        }))
    );
}
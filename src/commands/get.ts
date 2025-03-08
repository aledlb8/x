import inquirer from "inquirer";
import { loadStorage } from "../storage";
import { logger } from "../logger";

export async function getItem(name?: string) {
    const storage = loadStorage();
    const keys = Object.keys(storage);

    if (keys.length === 0) {
        logger.warn("🔴 No stored items found.");
        return;
    }

    if (!name) {
        const { selectedItem } = await inquirer.prompt([
            {
                type: "list",
                name: "selectedItem",
                message: "Select an item to retrieve:",
                choices: keys,
            },
        ]);
        name = selectedItem;
    }

    if (!name || !storage[name]) {
        logger.warn(`❌ No entry found for "${name}".`);
        return;
    }

    console.log("------------------------------------------------");
    console.log(`🔹 Type:  ${storage[name]?.type}`);
    console.table(storage[name]?.data);
    console.log("------------------------------------------------");
}
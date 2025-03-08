import inquirer from "inquirer";
import { loadStorage, saveStorage } from "../storage";
import { logger } from "../logger";

export async function deleteItem(name?: string) {
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
                message: "Select an item to delete:",
                choices: keys,
            },
        ]);
        name = selectedItem;
    }

    if (!name || !storage[name]) {
        logger.warn(`❌ No entry found for "${name}".`);
        return;
    }

    const { confirmDelete } = await inquirer.prompt([
        {
            type: "confirm",
            name: "confirmDelete",
            message: `⚠ Are you sure you want to delete "${name}"?`,
            default: false,
        },
    ]);

    if (!confirmDelete) {
        logger.info("❌ Deletion cancelled.");
        return;
    }

    delete storage[name];
    saveStorage(storage);
    logger.info(`✅ "${name}" has been deleted successfully.`);
}
import inquirer from "inquirer";
import { loadStorage, saveStorage } from "../storage";
import { logger } from "../logger";

export async function addItem() {
    const { itemType } = await inquirer.prompt([
        { type: "list", name: "itemType", message: "What would you like to store?", choices: ["Password", "Credit Card", "Secure Note"] },
    ]);

    const storage = loadStorage();
    let itemData: any = {};

    if (itemType === "Password") {
        const answers = await inquirer.prompt([
            { type: "input", name: "service", message: "Enter service name (e.g., Gmail):" },
            { type: "input", name: "website", message: "Enter website URL (optional):" },
            { type: "input", name: "email", message: "Enter email (optional):" },
            { type: "input", name: "username", message: "Enter username:" },
            { type: "password", name: "password", message: "Enter password:" },
        ]);
        itemData = answers;
    } else if (itemType === "Credit Card") {
        const answers = await inquirer.prompt([
            { type: "input", name: "cardholder", message: "Enter Cardholder Name:" },
            { type: "input", name: "number", message: "Enter Card Number:" },
            { type: "input", name: "expiry", message: "Enter Expiry Date (MM/YY):" },
            { type: "password", name: "cvv", message: "Enter CVV:" },
        ]);
        itemData = answers;
    } else if (itemType === "Secure Note") {
        const { title, note } = await inquirer.prompt([
            { type: "input", name: "title", message: "Enter note title:" },
            { type: "editor", name: "note", message: "Enter your secure note:" },
        ]);
        itemData = { title, note };
    }

    storage[itemData.service || itemData.title || itemData.cardholder] = { type: itemType, data: itemData };
    saveStorage(storage);
    logger.info(`✅ ${itemType} saved successfully.`);
}
import fs from "fs-extra";
import inquirer from "inquirer";
import { encrypt } from "./encryption";
import { logger } from "./logger";
import { CONFIG_FILE } from "./config";


export function loadConfig(): { sessionDuration: number } {
    if (!fs.existsSync(CONFIG_FILE)) {
        return { sessionDuration: 15 };
    }
    return JSON.parse(fs.readFileSync(CONFIG_FILE, "utf8"));
}

export function saveConfig(config: object): void {
    fs.writeFileSync(CONFIG_FILE, JSON.stringify(config));
}

export async function changeSessionDuration() {
    const { duration } = await inquirer.prompt([
        { type: "input", name: "duration", message: "Enter session duration in minutes:", validate: (input) => /^\d+$/.test(input) ? true : "Enter a valid number" },
    ]);

    const config = loadConfig();
    config.sessionDuration = parseInt(duration, 10);
    saveConfig(config);
    logger.info(`✅ Session duration updated to ${duration} minutes.`);
}

export async function changeMasterPassword() {
    const { newPassword } = await inquirer.prompt([
        { type: "password", name: "newPassword", message: "Enter new Master Password:" },
    ]);

    const encryptedPassword = encrypt(newPassword);
    fs.writeFileSync(CONFIG_FILE, JSON.stringify({ master: encryptedPassword }));
    logger.info("✅ Master Password changed successfully.");
}
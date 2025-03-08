import fs from "fs-extra";
import inquirer from "inquirer";
import { encrypt, decrypt } from "./encryption";
import { logger } from "./logger";
import { CONFIG_FILE, SESSION_DURATION, SESSION_FILE } from "./config";

export function isMasterPasswordSet(): boolean {
    return fs.existsSync(CONFIG_FILE);
}

export async function setMasterPassword(): Promise<void> {
    const { masterPassword } = await inquirer.prompt([
        { type: "password", name: "masterPassword", message: "Set a Master Password:" },
    ]);

    const encryptedPassword = encrypt(masterPassword);
    fs.writeFileSync(CONFIG_FILE, JSON.stringify({ master: encryptedPassword }));
    logger.info("Master Password set successfully! Use this password to unlock the CLI.");
}

function isSessionActive(): boolean {
    if (!fs.existsSync(SESSION_FILE)) return false;

    try {
        const sessionData = JSON.parse(fs.readFileSync(SESSION_FILE, "utf8"));
        const timestamp = sessionData.timestamp;
        return Date.now() - timestamp < SESSION_DURATION;
    } catch {
        return false;
    }
}

function createSession(): void {
    fs.writeFileSync(SESSION_FILE, JSON.stringify({ timestamp: Date.now() }));
}

export function logout(): void {
    if (fs.existsSync(SESSION_FILE)) {
        fs.unlinkSync(SESSION_FILE);
    }
    logger.info("🔓 Logged out. You will need to re-enter your Master Password next time.");
}

export async function verifyMasterPassword(): Promise<boolean> {
    if (isSessionActive()) {
        logger.info("🔓 Session active. No need to enter Master Password.");
        return true;
    }

    if (!isMasterPasswordSet()) {
        logger.warn("No Master Password found. Please set one first.");
        await setMasterPassword();
        return true;
    }

    const storedData = JSON.parse(fs.readFileSync(CONFIG_FILE, "utf8"));
    const correctPassword = decrypt(storedData.master);

    let attempts = 0;
    while (attempts < 3) {
        const { enteredPassword } = await inquirer.prompt([
            { type: "password", name: "enteredPassword", message: "Enter Master Password to Unlock:" },
        ]);

        if (enteredPassword === correctPassword) {
            logger.info("✅ Access Granted!");
            createSession();
            return true;
        } else {
            logger.warn(`❌ Incorrect Password! (${2 - attempts} attempts left)`);
            attempts++;
        }
    }

    logger.error("🚫 Too many incorrect attempts. Exiting...");
    process.exit(1);
}
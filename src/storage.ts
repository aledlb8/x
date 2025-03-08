import fs from "fs-extra";
import { encrypt, decrypt } from "./encryption";
import { PASSWORD_FILE } from "./config";

type Passwords = {
    [service: string]: {
        website: string;
        email: string;
        username: string;
        password: string;
    };
}

export interface StoredData {
    [key: string]: { type: string; data: any };
}

export function loadStorage(): StoredData {
    if (!fs.existsSync(PASSWORD_FILE)) return {};
    return JSON.parse(decrypt(fs.readFileSync(PASSWORD_FILE, "utf8")));
}

export function saveStorage(data: StoredData): void {
    fs.writeFileSync(PASSWORD_FILE, encrypt(JSON.stringify(data)));
}
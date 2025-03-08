"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.isMasterPasswordSet = isMasterPasswordSet;
exports.setMasterPassword = setMasterPassword;
exports.logout = logout;
exports.verifyMasterPassword = verifyMasterPassword;
const fs_extra_1 = __importDefault(require("fs-extra"));
const inquirer_1 = __importDefault(require("inquirer"));
const encryption_1 = require("./encryption");
const logger_1 = require("./logger");
const config_1 = require("./config");
function isMasterPasswordSet() {
    return fs_extra_1.default.existsSync(config_1.CONFIG_FILE);
}
function setMasterPassword() {
    return __awaiter(this, void 0, void 0, function* () {
        const { masterPassword } = yield inquirer_1.default.prompt([
            { type: "password", name: "masterPassword", message: "Set a Master Password:" },
        ]);
        const encryptedPassword = (0, encryption_1.encrypt)(masterPassword);
        fs_extra_1.default.writeFileSync(config_1.CONFIG_FILE, JSON.stringify({ master: encryptedPassword }));
        logger_1.logger.info("Master Password set successfully! Use this password to unlock the CLI.");
    });
}
function isSessionActive() {
    if (!fs_extra_1.default.existsSync(config_1.SESSION_FILE))
        return false;
    try {
        const sessionData = JSON.parse(fs_extra_1.default.readFileSync(config_1.SESSION_FILE, "utf8"));
        const timestamp = sessionData.timestamp;
        return Date.now() - timestamp < config_1.SESSION_DURATION;
    }
    catch (_a) {
        return false;
    }
}
function createSession() {
    fs_extra_1.default.writeFileSync(config_1.SESSION_FILE, JSON.stringify({ timestamp: Date.now() }));
}
function logout() {
    if (fs_extra_1.default.existsSync(config_1.SESSION_FILE)) {
        fs_extra_1.default.unlinkSync(config_1.SESSION_FILE);
    }
    logger_1.logger.info("🔓 Logged out. You will need to re-enter your Master Password next time.");
}
function verifyMasterPassword() {
    return __awaiter(this, void 0, void 0, function* () {
        if (isSessionActive()) {
            logger_1.logger.info("🔓 Session active. No need to enter Master Password.");
            return true;
        }
        if (!isMasterPasswordSet()) {
            logger_1.logger.warn("No Master Password found. Please set one first.");
            yield setMasterPassword();
            return true;
        }
        const storedData = JSON.parse(fs_extra_1.default.readFileSync(config_1.CONFIG_FILE, "utf8"));
        const correctPassword = (0, encryption_1.decrypt)(storedData.master);
        let attempts = 0;
        while (attempts < 3) {
            const { enteredPassword } = yield inquirer_1.default.prompt([
                { type: "password", name: "enteredPassword", message: "Enter Master Password to Unlock:" },
            ]);
            if (enteredPassword === correctPassword) {
                logger_1.logger.info("✅ Access Granted!");
                createSession();
                return true;
            }
            else {
                logger_1.logger.warn(`❌ Incorrect Password! (${2 - attempts} attempts left)`);
                attempts++;
            }
        }
        logger_1.logger.error("🚫 Too many incorrect attempts. Exiting...");
        process.exit(1);
    });
}

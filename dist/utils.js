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
exports.loadConfig = loadConfig;
exports.saveConfig = saveConfig;
exports.changeSessionDuration = changeSessionDuration;
exports.changeMasterPassword = changeMasterPassword;
const fs_extra_1 = __importDefault(require("fs-extra"));
const inquirer_1 = __importDefault(require("inquirer"));
const encryption_1 = require("./encryption");
const logger_1 = require("./logger");
const config_1 = require("./config");
function loadConfig() {
    if (!fs_extra_1.default.existsSync(config_1.CONFIG_FILE)) {
        return { sessionDuration: 15 };
    }
    return JSON.parse(fs_extra_1.default.readFileSync(config_1.CONFIG_FILE, "utf8"));
}
function saveConfig(config) {
    fs_extra_1.default.writeFileSync(config_1.CONFIG_FILE, JSON.stringify(config));
}
function changeSessionDuration() {
    return __awaiter(this, void 0, void 0, function* () {
        const { duration } = yield inquirer_1.default.prompt([
            { type: "input", name: "duration", message: "Enter session duration in minutes:", validate: (input) => /^\d+$/.test(input) ? true : "Enter a valid number" },
        ]);
        const config = loadConfig();
        config.sessionDuration = parseInt(duration, 10);
        saveConfig(config);
        logger_1.logger.info(`✅ Session duration updated to ${duration} minutes.`);
    });
}
function changeMasterPassword() {
    return __awaiter(this, void 0, void 0, function* () {
        const { newPassword } = yield inquirer_1.default.prompt([
            { type: "password", name: "newPassword", message: "Enter new Master Password:" },
        ]);
        const encryptedPassword = (0, encryption_1.encrypt)(newPassword);
        fs_extra_1.default.writeFileSync(config_1.CONFIG_FILE, JSON.stringify({ master: encryptedPassword }));
        logger_1.logger.info("✅ Master Password changed successfully.");
    });
}

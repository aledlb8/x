"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.loadStorage = loadStorage;
exports.saveStorage = saveStorage;
const fs_extra_1 = __importDefault(require("fs-extra"));
const encryption_1 = require("./encryption");
const config_1 = require("./config");
function loadStorage() {
    if (!fs_extra_1.default.existsSync(config_1.PASSWORD_FILE))
        return {};
    return JSON.parse((0, encryption_1.decrypt)(fs_extra_1.default.readFileSync(config_1.PASSWORD_FILE, "utf8")));
}
function saveStorage(data) {
    fs_extra_1.default.writeFileSync(config_1.PASSWORD_FILE, (0, encryption_1.encrypt)(JSON.stringify(data)));
}

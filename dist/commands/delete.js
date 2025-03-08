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
exports.deleteItem = deleteItem;
const inquirer_1 = __importDefault(require("inquirer"));
const storage_1 = require("../storage");
const logger_1 = require("../logger");
function deleteItem(name) {
    return __awaiter(this, void 0, void 0, function* () {
        const storage = (0, storage_1.loadStorage)();
        const keys = Object.keys(storage);
        if (keys.length === 0) {
            logger_1.logger.warn("🔴 No stored items found.");
            return;
        }
        if (!name) {
            const { selectedItem } = yield inquirer_1.default.prompt([
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
            logger_1.logger.warn(`❌ No entry found for "${name}".`);
            return;
        }
        const { confirmDelete } = yield inquirer_1.default.prompt([
            {
                type: "confirm",
                name: "confirmDelete",
                message: `⚠ Are you sure you want to delete "${name}"?`,
                default: false,
            },
        ]);
        if (!confirmDelete) {
            logger_1.logger.info("❌ Deletion cancelled.");
            return;
        }
        delete storage[name];
        (0, storage_1.saveStorage)(storage);
        logger_1.logger.info(`✅ "${name}" has been deleted successfully.`);
    });
}

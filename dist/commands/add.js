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
exports.addItem = addItem;
const inquirer_1 = __importDefault(require("inquirer"));
const storage_1 = require("../storage");
const logger_1 = require("../logger");
function addItem() {
    return __awaiter(this, void 0, void 0, function* () {
        const { itemType } = yield inquirer_1.default.prompt([
            { type: "list", name: "itemType", message: "What would you like to store?", choices: ["Password", "Credit Card", "Secure Note"] },
        ]);
        const storage = (0, storage_1.loadStorage)();
        let itemData = {};
        if (itemType === "Password") {
            const answers = yield inquirer_1.default.prompt([
                { type: "input", name: "service", message: "Enter service name (e.g., Gmail):" },
                { type: "input", name: "website", message: "Enter website URL (optional):" },
                { type: "input", name: "email", message: "Enter email (optional):" },
                { type: "input", name: "username", message: "Enter username:" },
                { type: "password", name: "password", message: "Enter password:" },
            ]);
            itemData = answers;
        }
        else if (itemType === "Credit Card") {
            const answers = yield inquirer_1.default.prompt([
                { type: "input", name: "cardholder", message: "Enter Cardholder Name:" },
                { type: "input", name: "number", message: "Enter Card Number:" },
                { type: "input", name: "expiry", message: "Enter Expiry Date (MM/YY):" },
                { type: "password", name: "cvv", message: "Enter CVV:" },
            ]);
            itemData = answers;
        }
        else if (itemType === "Secure Note") {
            const { title, note } = yield inquirer_1.default.prompt([
                { type: "input", name: "title", message: "Enter note title:" },
                { type: "editor", name: "note", message: "Enter your secure note:" },
            ]);
            itemData = { title, note };
        }
        storage[itemData.service || itemData.title || itemData.cardholder] = { type: itemType, data: itemData };
        (0, storage_1.saveStorage)(storage);
        logger_1.logger.info(`✅ ${itemType} saved successfully.`);
    });
}

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
exports.getItem = getItem;
const inquirer_1 = __importDefault(require("inquirer"));
const storage_1 = require("../storage");
const logger_1 = require("../logger");
function getItem(name) {
    return __awaiter(this, void 0, void 0, function* () {
        var _a, _b;
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
                    message: "Select an item to retrieve:",
                    choices: keys,
                },
            ]);
            name = selectedItem;
        }
        if (!name || !storage[name]) {
            logger_1.logger.warn(`❌ No entry found for "${name}".`);
            return;
        }
        console.log("------------------------------------------------");
        console.log(`🔹 Type:  ${(_a = storage[name]) === null || _a === void 0 ? void 0 : _a.type}`);
        console.table((_b = storage[name]) === null || _b === void 0 ? void 0 : _b.data);
        console.log("------------------------------------------------");
    });
}

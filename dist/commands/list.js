"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.listItems = listItems;
const storage_1 = require("../storage");
function listItems() {
    const storage = (0, storage_1.loadStorage)();
    if (Object.keys(storage).length === 0) {
        console.log("🔴 No stored items found.");
        return;
    }
    console.table(Object.entries(storage).map(([key, entry]) => ({
        Name: key,
        Type: entry.type,
    })));
}

#!/usr/bin/env node
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
Object.defineProperty(exports, "__esModule", { value: true });
const commander_1 = require("commander");
const add_1 = require("./commands/add");
const get_1 = require("./commands/get");
const list_1 = require("./commands/list");
const delete_1 = require("./commands/delete");
const utils_1 = require("./utils");
const auth_1 = require("./auth");
const logger_1 = require("./logger");
function ensureAuthentication() {
    return __awaiter(this, void 0, void 0, function* () {
        if (!(0, auth_1.isMasterPasswordSet)()) {
            logger_1.logger.warn("🔒 No Master Password set. You need to create one first.");
            yield (0, auth_1.setMasterPassword)();
        }
        yield (0, auth_1.verifyMasterPassword)();
    });
}
commander_1.program.command("add").description("Add a new item").action(() => __awaiter(void 0, void 0, void 0, function* () {
    yield ensureAuthentication();
    yield (0, add_1.addItem)();
}));
commander_1.program.command("get [name]").description("Retrieve an item").action((name) => __awaiter(void 0, void 0, void 0, function* () {
    yield ensureAuthentication();
    (0, get_1.getItem)(name);
}));
commander_1.program.command("list").description("List all stored items").action(() => __awaiter(void 0, void 0, void 0, function* () {
    yield ensureAuthentication();
    (0, list_1.listItems)();
}));
commander_1.program.command("delete [name]").description("Delete am item").action((name) => __awaiter(void 0, void 0, void 0, function* () {
    yield ensureAuthentication();
    (0, delete_1.deleteItem)(name);
}));
const configCommand = commander_1.program.command("config").description("Manage application settings");
configCommand.command("session <minutes>").description("Set session duration").action(utils_1.changeSessionDuration);
configCommand.command("password").description("Change Master Password").action(utils_1.changeMasterPassword);
commander_1.program.command("logout").description("Expire session (require Master Password on next use)").action(() => {
    (0, auth_1.logout)();
});
commander_1.program.parse(process.argv);

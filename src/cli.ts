#!/usr/bin/env node
import { program } from "commander";
import { addItem } from "./commands/add";
import { getItem } from "./commands/get";
import { listItems } from "./commands/list";
import { deleteItem } from "./commands/delete";
import { changeSessionDuration, changeMasterPassword } from "./utils";
import { isMasterPasswordSet, logout, setMasterPassword, verifyMasterPassword } from "./auth";
import { logger } from "./logger";

async function ensureAuthentication() {
    if (!isMasterPasswordSet()) {
        logger.warn("🔒 No Master Password set. You need to create one first.");
        await setMasterPassword();
    }

    await verifyMasterPassword();
}

program.command("add").description("Add a new item").action(async () => {
    await ensureAuthentication();
    await addItem();
});

program.command("get [name]").description("Retrieve an item").action(async (name) => {
    await ensureAuthentication();
    getItem(name);
});

program.command("list").description("List all stored items").action(async () => {
    await ensureAuthentication();
    listItems();
});

program.command("delete [name]").description("Delete am item").action(async (name) => {
    await ensureAuthentication();
    deleteItem(name);
});

const configCommand = program.command("config").description("Manage application settings");

configCommand.command("session <minutes>").description("Set session duration").action(changeSessionDuration);
configCommand.command("password").description("Change Master Password").action(changeMasterPassword);

program.command("logout").description("Expire session (require Master Password on next use)").action(() => {
    logout();
});

program.parse(process.argv);
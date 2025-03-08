"use strict";
// C:\Users\<user>\AppData\Roaming
Object.defineProperty(exports, "__esModule", { value: true });
exports.SESSION_DURATION = exports.SESSION_FILE = exports.CONFIG_FILE = exports.PASSWORD_FILE = void 0;
exports.PASSWORD_FILE = `${process.env.APPDATA || process.env.HOME}/x.json`;
exports.CONFIG_FILE = `${process.env.APPDATA || process.env.HOME}/x_config.json`;
exports.SESSION_FILE = `${process.env.APPDATA || process.env.HOME}/x_session.json`;
exports.SESSION_DURATION = 15 * 60 * 1000; // 15 minutes

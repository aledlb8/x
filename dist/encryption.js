"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.decrypt = exports.encrypt = void 0;
const crypto_1 = __importDefault(require("crypto"));
const dotenv_1 = __importDefault(require("dotenv"));
dotenv_1.default.config();
const algorithm = "aes-256-ctr";
const key = crypto_1.default
    .createHash("sha256")
    .update(String(process.env.ENCRYPT_KEY || "default_key"))
    .digest("base64")
    .substring(0, 32);
const encrypt = (text) => {
    const iv = crypto_1.default.randomBytes(16);
    const cipher = crypto_1.default.createCipheriv(algorithm, key, iv);
    const buffer = Buffer.concat([cipher.update(text, "utf8"), cipher.final()]);
    const hash = iv.toString("hex") + "**x**" + buffer.toString("hex");
    return Buffer.from(hash, "utf8").toString("base64");
};
exports.encrypt = encrypt;
const decrypt = (text) => {
    const buffer = Buffer.from(text, "base64").toString("utf8");
    const hash = buffer.split("**x**");
    if (hash.length !== 2 || !hash[0] || !hash[1]) {
        throw new Error("Invalid encrypted text format.");
    }
    const ivBuffer = Buffer.from(hash[0], "hex");
    const contentBuffer = Buffer.from(hash[1], "hex");
    const decipher = crypto_1.default.createDecipheriv(algorithm, key, ivBuffer);
    const concat = Buffer.concat([decipher.update(contentBuffer), decipher.final()]);
    return concat.toString("utf8");
};
exports.decrypt = decrypt;

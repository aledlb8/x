import crypto from "crypto";
import dotenv from "dotenv";

dotenv.config();

const algorithm = "aes-256-ctr";

const key: string = crypto
    .createHash("sha256")
    .update(String(process.env.ENCRYPT_KEY || "default_key"))
    .digest("base64")
    .substring(0, 32);

export const encrypt = (text: string): string => {
    const iv: Buffer = crypto.randomBytes(16);
    const cipher: crypto.Cipher = crypto.createCipheriv(algorithm, key, iv);
    const buffer: Buffer = Buffer.concat([cipher.update(text, "utf8"), cipher.final()]);

    const hash: string = iv.toString("hex") + "**x**" + buffer.toString("hex");
    return Buffer.from(hash, "utf8").toString("base64");
};

export const decrypt = (text: string): string => {
    const buffer: string = Buffer.from(text, "base64").toString("utf8");
    const hash: string[] = buffer.split("**x**");

    if (hash.length !== 2 || !hash[0] || !hash[1]) {
        throw new Error("Invalid encrypted text format.");
    }

    const ivBuffer = Buffer.from(hash[0], "hex");
    const contentBuffer = Buffer.from(hash[1], "hex");

    const decipher: crypto.Decipher = crypto.createDecipheriv(algorithm, key, ivBuffer);
    const concat: Buffer = Buffer.concat([decipher.update(contentBuffer), decipher.final()]);

    return concat.toString("utf8");
};
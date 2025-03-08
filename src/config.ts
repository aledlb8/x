 // C:\Users\<user>\AppData\Roaming

export const PASSWORD_FILE = `${process.env.APPDATA || process.env.HOME}/x.json`;

export const CONFIG_FILE = `${process.env.APPDATA || process.env.HOME}/x_config.json`;

export const SESSION_FILE = `${process.env.APPDATA || process.env.HOME}/x_session.json`;
export const SESSION_DURATION = 15 * 60 * 1000; // 15 minutes


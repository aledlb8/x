use crate::storage::database::DB;
use chrono::Utc;

const SESSION_STORAGE: &str = "session";
pub const SESSION_TIMEOUT_KEY: &str = "session_timeout";
const DEFAULT_SESSION_TIMEOUT: i64 = 15; // minutes

pub fn is_session_active(db: &DB) -> bool {
    if let Some(last_login) = db.get(SESSION_STORAGE).unwrap() {
        let last_login_time: i64 = String::from_utf8(last_login.to_vec())
            .unwrap()
            .parse()
            .unwrap();
        let now = Utc::now().timestamp();

        let session_timeout = db
                    .get(SESSION_TIMEOUT_KEY)
                    .unwrap()
                    .and_then(|v| String::from_utf8(v.to_vec()).ok())
                    .and_then(|s| s.parse::<i64>().ok())
                    .unwrap_or(DEFAULT_SESSION_TIMEOUT);

        if now - last_login_time < (session_timeout * 60) {
            return true;
        }
    }
    false
}

pub fn update_session(db: &DB) {
    let now = Utc::now().timestamp().to_string();
    db.insert(SESSION_STORAGE, now.as_bytes())
        .expect("Failed to update session");
    db.flush().expect("Failed to flush session update");
}
use crate::storage::database::DB;
use chrono::Utc;

const SESSION_STORAGE: &str = "session";
const SESSION_TIMEOUT: i64 = 15; // minutes

pub fn is_session_active(db: &DB) -> bool {
    if let Some(last_login) = db.get(SESSION_STORAGE).unwrap() {
        let last_login_time: i64 = String::from_utf8(last_login.to_vec())
            .unwrap()
            .parse()
            .unwrap();
        let now = Utc::now().timestamp();

        if now - last_login_time < (SESSION_TIMEOUT * 60) {
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
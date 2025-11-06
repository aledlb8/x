use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VaultEntry {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CloudResponse {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub vault: Option<Vec<VaultEntry>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CloudInfoResponse {
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub entry_count: usize,
}

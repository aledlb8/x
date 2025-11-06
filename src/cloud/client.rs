use crate::cloud::types::{CloudInfoResponse, CloudResponse, VaultEntry};
use aes_gcm::{Aes256Gcm, Key};
use blake3;
use hex;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;
use std::fmt;
use std::time::Duration;

const MASTER_HASH_HEADER: &str = "X-Master-Hash";
const AUTH_FAILURE: &str = "Authentication with the cloud host failed";

#[derive(Debug)]
pub enum CloudClientError {
    Http(reqwest::Error),
    Failure(String),
    AuthenticationFailed,
}

impl From<reqwest::Error> for CloudClientError {
    fn from(value: reqwest::Error) -> Self {
        CloudClientError::Http(value)
    }
}

impl fmt::Display for CloudClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudClientError::Http(err) => write!(f, "Network error: {}", err),
            CloudClientError::Failure(msg) => write!(f, "{}", msg),
            CloudClientError::AuthenticationFailed => write!(f, "{}", AUTH_FAILURE),
        }
    }
}

#[derive(Clone)]
pub struct CloudApi {
    client: Client,
    base_url: String,
}

impl CloudApi {
    pub fn new(base_url: impl Into<String>) -> Result<Self, CloudClientError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(CloudClientError::Http)?;
        Ok(Self {
            client,
            base_url: base_url.into(),
        })
    }

    pub fn verify_master(&self, auth_hash: &str) -> Result<(), CloudClientError> {
        let url = self.url("auth");
        let headers = auth_headers(auth_hash)?;
        let res = self
            .client
            .post(url)
            .headers(headers)
            .header(CONTENT_TYPE, "application/json")
            .body("{}")
            .send()
            .map_err(CloudClientError::Http)?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(CloudClientError::AuthenticationFailed)
        }
    }

    pub fn fetch_vault(&self, auth_hash: &str) -> Result<Vec<VaultEntry>, CloudClientError> {
        let headers = auth_headers(auth_hash)?;
        let res = self
            .client
            .get(self.url("vault"))
            .headers(headers)
            .send()
            .map_err(CloudClientError::Http)?;
        let response = Self::parse_response(res)?;
        Ok(response.vault.unwrap_or_default())
    }

    pub fn save_vault(
        &self,
        auth_hash: &str,
        vault: &[VaultEntry],
    ) -> Result<(), CloudClientError> {
        let payload = json!({
            "vault": vault,
        });
        let response = self.post_with_auth(self.url("vault"), auth_hash, payload)?;
        if response.success {
            Ok(())
        } else {
            Err(CloudClientError::Failure(response.message))
        }
    }

    pub fn info(&self, auth_hash: &str) -> Result<CloudInfoResponse, CloudClientError> {
        let headers = auth_headers(auth_hash)?;
        let res = self
            .client
            .get(self.url("info"))
            .headers(headers)
            .send()
            .map_err(CloudClientError::Http)?;

        if res.status().is_success() {
            res.json().map_err(CloudClientError::Http)
        } else if res.status().as_u16() == 401 {
            Err(CloudClientError::AuthenticationFailed)
        } else {
            let status = res.status();
            let message = res
                .text()
                .unwrap_or_else(|_| "Unable to read response body".to_string());
            Err(CloudClientError::Failure(format!(
                "Server returned {}: {}",
                status, message
            )))
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), path)
    }

    fn post_with_auth<T: serde::Serialize>(
        &self,
        url: String,
        auth_hash: &str,
        payload: T,
    ) -> Result<CloudResponse, CloudClientError> {
        let headers = auth_headers(auth_hash)?;
        let res = self
            .client
            .post(url)
            .headers(headers)
            .json(&payload)
            .send()
            .map_err(CloudClientError::Http)?;
        Self::parse_response(res)
    }

    fn parse_response(res: Response) -> Result<CloudResponse, CloudClientError> {
        if res.status().is_success() {
            res.json().map_err(CloudClientError::Http)
        } else if res.status().as_u16() == 401 {
            Err(CloudClientError::AuthenticationFailed)
        } else {
            let status = res.status();
            let message = res
                .text()
                .unwrap_or_else(|_| "Unable to read response body".to_string());
            Err(CloudClientError::Failure(format!(
                "Server returned {}: {}",
                status, message
            )))
        }
    }
}

fn auth_headers(auth_hash: &str) -> Result<HeaderMap, CloudClientError> {
    let mut headers = HeaderMap::new();
    let value = HeaderValue::from_str(auth_hash)
        .map_err(|_| CloudClientError::Failure("Invalid master password hash".into()))?;
    headers.insert(MASTER_HASH_HEADER, value);
    Ok(headers)
}

pub struct RemoteSession {
    api: CloudApi,
    auth_hash: String,
    encryption_key: Key<Aes256Gcm>,
}

impl RemoteSession {
    pub fn new(base_url: String, master_password: String) -> Result<Self, CloudClientError> {
        let hash = blake3::hash(master_password.as_bytes());
        Self::from_hash_bytes(base_url, hash.to_hex().to_string(), hash.as_bytes())
    }

    pub fn from_hash(base_url: String, auth_hash: String) -> Result<Self, CloudClientError> {
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(&auth_hash, &mut bytes).map_err(|_| {
            CloudClientError::Failure("Stored master password hash is invalid".into())
        })?;
        Self::from_hash_bytes(base_url, auth_hash, &bytes)
    }

    fn from_hash_bytes(
        base_url: String,
        auth_hash: String,
        hash_bytes: &[u8],
    ) -> Result<Self, CloudClientError> {
        let api = CloudApi::new(base_url)?;
        api.verify_master(&auth_hash)?;

        let encryption_key = Key::<Aes256Gcm>::from_slice(hash_bytes).clone();

        Ok(Self {
            api,
            auth_hash,
            encryption_key,
        })
    }

    pub fn auth_hash(&self) -> &str {
        &self.auth_hash
    }

    pub fn encryption_key(&self) -> &Key<Aes256Gcm> {
        &self.encryption_key
    }

    pub fn fetch_vault(&self) -> Result<Vec<VaultEntry>, CloudClientError> {
        self.api.fetch_vault(&self.auth_hash)
    }

    pub fn save_vault(&self, vault: &[VaultEntry]) -> Result<(), CloudClientError> {
        self.api.save_vault(&self.auth_hash, vault)
    }
}

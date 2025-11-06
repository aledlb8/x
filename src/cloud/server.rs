use crate::cloud::types::{CloudInfoResponse, CloudResponse, VaultEntry};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use dirs;
use serde::Deserialize;
use serde_json;
use sled::{self, Tree};
use std::fmt;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub bind_address: String,
    pub port: u16,
    pub data_path: Option<PathBuf>,
    pub master_hash: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: 4000,
            data_path: None,
            master_hash: None,
        }
    }
}

impl ServerConfig {
    pub fn storage_path(&self) -> PathBuf {
        if let Some(path) = &self.data_path {
            return path.clone();
        }

        let mut base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        base.push("x_cli");
        base.push("cloud_host.db");
        base
    }

    fn socket_addr(&self) -> Result<SocketAddr, ServerError> {
        let addr = format!("{}:{}", self.bind_address, self.port);
        addr.parse::<SocketAddr>()
            .map_err(|err| ServerError::Address(err.to_string()))
    }
}

#[derive(Debug)]
pub enum ServerError {
    Io(std::io::Error),
    Serde(serde_json::Error),
    Database(sled::Error),
    Http(String),
    Runtime(std::io::Error),
    Address(String),
    Unauthorized,
    MissingMasterPassword,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::Io(err) => write!(f, "I/O error: {}", err),
            ServerError::Serde(err) => write!(f, "Serialization error: {}", err),
            ServerError::Database(err) => write!(f, "Database error: {}", err),
            ServerError::Http(err) => write!(f, "HTTP server error: {}", err),
            ServerError::Runtime(err) => write!(f, "Runtime error: {}", err),
            ServerError::Address(err) => write!(f, "Invalid address: {}", err),
            ServerError::Unauthorized => write!(f, "Unauthorized"),
            ServerError::MissingMasterPassword => {
                write!(f, "Master password is required to start the server")
            }
        }
    }
}

impl std::error::Error for ServerError {}

impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        ServerError::Io(value)
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(value: serde_json::Error) -> Self {
        ServerError::Serde(value)
    }
}

impl From<sled::Error> for ServerError {
    fn from(value: sled::Error) -> Self {
        ServerError::Database(value)
    }
}

pub const MASTER_HASH_KEY: &str = "master_hash";
const MASTER_HASH_HEADER: &str = "x-master-hash";
const VAULT_KEY: &str = "vault";
const AUTH_FAILURE: &str = "Authentication failed";

struct CloudStore {
    meta: Tree,
    vault: Tree,
    lock: Mutex<()>,
}

impl CloudStore {
    async fn new(path: PathBuf) -> Result<Self, ServerError> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let db = sled::open(path)?;
        let meta = db.open_tree("meta")?;
        let vault = db.open_tree("vault")?;
        Ok(Self {
            meta,
            vault,
            lock: Mutex::new(()),
        })
    }

    fn master_hash(&self) -> Result<Option<String>, ServerError> {
        Ok(self
            .meta
            .get(MASTER_HASH_KEY)?
            .map(|bytes| String::from_utf8_lossy(&bytes).to_string()))
    }

    fn set_master_hash(&self, hash: &str) -> Result<(), ServerError> {
        self.meta.insert(MASTER_HASH_KEY, hash.as_bytes())?;
        self.meta.flush()?;
        Ok(())
    }

    fn ensure_master_hash(&self, provided: &str) -> Result<(), ServerError> {
        match self.master_hash()? {
            Some(existing) if existing == provided => Ok(()),
            Some(_) => Err(ServerError::Unauthorized),
            None => self.set_master_hash(provided),
        }
    }

    fn verify_master(&self, provided: &str) -> Result<(), ServerError> {
        match self.master_hash()? {
            Some(existing) if existing == provided => Ok(()),
            _ => Err(ServerError::Unauthorized),
        }
    }

    async fn load_vault(&self) -> Result<Vec<VaultEntry>, ServerError> {
        let _guard = self.lock.lock().await;
        let data = self.vault.get(VAULT_KEY)?;
        if let Some(bytes) = data {
            Ok(serde_json::from_slice(&bytes)?)
        } else {
            Ok(Vec::new())
        }
    }

    async fn save_vault(&self, entries: &[VaultEntry]) -> Result<(), ServerError> {
        let _guard = self.lock.lock().await;
        let data = serde_json::to_vec(entries)?;
        self.vault.insert(VAULT_KEY, data)?;
        self.vault.flush()?;
        Ok(())
    }
}

type SharedStore = Arc<CloudStore>;

#[derive(Deserialize)]
struct VaultUpdateRequest {
    vault: Vec<VaultEntry>,
}

pub fn run_server(config: ServerConfig) -> Result<(), ServerError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(ServerError::Runtime)?;

    runtime.block_on(async move {
        let path = config.storage_path();
        let provided_hash = config
            .master_hash
            .clone()
            .ok_or(ServerError::MissingMasterPassword)?;
        let store = Arc::new(CloudStore::new(path).await?);
        store.ensure_master_hash(&provided_hash)?;
        let app = build_router(store.clone());

        let addr = config.socket_addr()?;
        let listener = TcpListener::bind(addr).await?;
        let local_addr = listener.local_addr()?;

        println!(
            "Cloud API server running on http://{}:{}/api/cloudsync",
            local_addr.ip(),
            local_addr.port()
        );
        println!("Press Ctrl+C to stop the server.");

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|err| ServerError::Http(err.to_string()))
    })
}

fn build_router(store: SharedStore) -> Router {
    Router::new()
        .route("/api/cloudsync/auth", post(auth_handler))
        .route("/api/cloudsync/vault", get(vault_get_handler))
        .route("/api/cloudsync/vault", post(vault_post_handler))
        .route("/api/cloudsync/info", get(info_handler))
        .with_state(store)
}

async fn auth_handler(State(store): State<SharedStore>, headers: HeaderMap) -> StatusCode {
    match authenticate(store.as_ref(), &headers) {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            eprintln!("Auth handler error: {}", err);
            StatusCode::UNAUTHORIZED
        }
    }
}

async fn vault_get_handler(
    State(store): State<SharedStore>,
    headers: HeaderMap,
) -> (StatusCode, Json<CloudResponse>) {
    if let Err(err) = authenticate(store.as_ref(), &headers) {
        return to_error_response(err);
    }

    match store.load_vault().await {
        Ok(vault) => (
            StatusCode::OK,
            Json(CloudResponse {
                success: true,
                message: "Vault retrieved.".to_string(),
                vault: Some(vault),
            }),
        ),
        Err(err) => to_error_response(err),
    }
}

async fn vault_post_handler(
    State(store): State<SharedStore>,
    headers: HeaderMap,
    Json(payload): Json<VaultUpdateRequest>,
) -> (StatusCode, Json<CloudResponse>) {
    if let Err(err) = authenticate(store.as_ref(), &headers) {
        return to_error_response(err);
    }

    if let Err(err) = store.save_vault(&payload.vault).await {
        return to_error_response(err);
    }

    (
        StatusCode::OK,
        Json(CloudResponse {
            success: true,
            message: "Vault updated successfully.".to_string(),
            vault: None,
        }),
    )
}

async fn info_handler(
    State(store): State<SharedStore>,
    headers: HeaderMap,
) -> (StatusCode, Json<CloudInfoResponse>) {
    if let Err(err) = authenticate(store.as_ref(), &headers) {
        eprintln!("Info auth error: {}", err);
        return (
            StatusCode::UNAUTHORIZED,
            Json(CloudInfoResponse {
                success: false,
                message: Some(AUTH_FAILURE.to_string()),
                entry_count: 0,
            }),
        );
    }

    match store.load_vault().await {
        Ok(vault) => (
            StatusCode::OK,
            Json(CloudInfoResponse {
                success: true,
                message: Some("Vault reachable.".to_string()),
                entry_count: vault.len(),
            }),
        ),
        Err(err) => {
            eprintln!("Info error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CloudInfoResponse {
                    success: false,
                    message: Some(err.to_string()),
                    entry_count: 0,
                }),
            )
        }
    }
}

fn to_error_response(err: ServerError) -> (StatusCode, Json<CloudResponse>) {
    let (status, message) = classify_error(&err);
    (
        status,
        Json(CloudResponse {
            success: false,
            message,
            vault: None,
        }),
    )
}

fn classify_error(err: &ServerError) -> (StatusCode, String) {
    match err {
        ServerError::Address(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        ServerError::Unauthorized => (StatusCode::UNAUTHORIZED, AUTH_FAILURE.to_string()),
        ServerError::MissingMasterPassword => (
            StatusCode::BAD_REQUEST,
            "Master password required".to_string(),
        ),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    }
}

async fn shutdown_signal() {
    if tokio::signal::ctrl_c().await.is_ok() {
        println!("Shutdown signal received. Stopping cloud API...");
    }
}

fn authenticate(store: &CloudStore, headers: &HeaderMap) -> Result<(), ServerError> {
    let hash = headers
        .get(MASTER_HASH_HEADER)
        .and_then(|value| value.to_str().ok())
        .ok_or(ServerError::Unauthorized)?;
    store.verify_master(hash)
}

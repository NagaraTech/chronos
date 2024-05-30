use std::path::PathBuf;
use std::path::Path;
use crate::error::{ZchronodConfigError, ZchronodConfigResult};
use serde::Deserialize;
use serde::Serialize;

/// Zchronod Node Config
#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct ZchronodConfig {
    pub db: DbConfig,
    pub net: NetworkConfig,
    pub node: NodeConfig,
    pub api: ApiConfig,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct DbConfig {
    pub storage_root_path: Option<StorageRootPath>,
    pub pg_db_url: String,
    pub pg_db_name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct NetworkConfig {
    pub inner_p2p: String,          // vlc server bind udp socket
    pub outer_p2p: Option<String>,
    pub ws_url: String
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct NodeConfig {
    pub node_id: Option<String>,
    pub cache_msg_maximum: u64,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct ApiConfig {
   pub read_maximum: u64,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct StorageRootPath(PathBuf);

impl StorageRootPath {
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl ZchronodConfig {
    pub fn load_config(path: PathBuf) -> ZchronodConfigResult<ZchronodConfig> {
        let p: &Path = path.as_ref();
        let config_yaml = std::fs::read_to_string(p).map_err(|err| match err {
            e @ std::io::Error { .. } if e.kind() == std::io::ErrorKind::NotFound => {
                ZchronodConfigError::ConfigMissing(path.into())
            }
            _ => err.into(),
        })?;
        serde_yaml::from_str(&config_yaml).map_err(ZchronodConfigError::SerializationError)
    }
}
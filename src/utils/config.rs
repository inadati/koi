use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::utils::error::{KoiError, Result};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct KoiConfig {
    #[serde(default)]
    pub remote: RemoteConfig,
    #[serde(default)]
    pub paths: PathsConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RemoteConfig {
    pub org: Option<String>,
}

// ~/.koi/remotes.toml
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RemotesConfig {
    pub active: Option<String>,
    #[serde(default)]
    pub remotes: BTreeMap<String, RemoteEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteEntry {
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathsConfig {
    pub local: String,
    pub global: String,
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            local: ".claude/skills".to_string(),
            global: "~/.claude/skills".to_string(),
        }
    }
}

pub fn config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| {
        KoiError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory not found",
        ))
    })?;
    Ok(home.join(".koi").join("config.toml"))
}

pub fn load_config() -> Result<KoiConfig> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(KoiConfig::default());
    }
    let content = fs::read_to_string(&path)?;
    let config: KoiConfig = toml::from_str(&content)?;
    Ok(config)
}

pub fn remotes_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| {
        KoiError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory not found",
        ))
    })?;
    Ok(home.join(".koi").join("remotes.toml"))
}

pub fn load_remotes() -> Result<RemotesConfig> {
    let path = remotes_path()?;
    if !path.exists() {
        return Ok(RemotesConfig::default());
    }
    let content = fs::read_to_string(&path)?;
    let config: RemotesConfig = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_remotes(config: &RemotesConfig) -> Result<()> {
    let path = remotes_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(config)?;
    fs::write(&path, content)?;
    Ok(())
}

pub fn get_org() -> Result<String> {
    // remotes.tomlのactiveを優先、なければconfig.tomlにフォールバック
    let remotes = load_remotes()?;
    if let Some(active) = remotes.active {
        return Ok(active);
    }
    let config = load_config()?;
    config.remote.org.ok_or(KoiError::OrgNotSet)
}

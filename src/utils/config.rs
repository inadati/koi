use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::utils::error::{KoiError, Result};

// ~/.koi/config.toml（パス設定のみ）
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct KoiConfig {
    #[serde(default)]
    pub paths: PathsConfig,
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

// ~/.koi/remotes.toml
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RemotesConfig {
    #[serde(default)]
    pub remotes: BTreeMap<String, RemoteEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteEntry {
    pub org: String,
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

/// エイリアス名からGitHub org名を解決する
pub fn resolve_org(alias: &str, remotes: &RemotesConfig) -> Result<String> {
    remotes
        .remotes
        .get(alias)
        .map(|e| e.org.clone())
        .ok_or_else(|| {
            KoiError::RemoteNotFound(format!(
                "リモート '{}' が見つかりません。`koi remote add` で追加してください",
                alias
            ))
        })
}

/// エイリアス名のバリデーション（英数字・ハイフン・アンダースコアのみ）
pub fn validate_alias(alias: &str) -> bool {
    !alias.is_empty()
        && alias
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

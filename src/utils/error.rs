use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KoiError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("gh CLI is not installed. Please install: https://cli.github.com/")]
    GhNotInstalled,

    #[error("gh CLI is not authenticated. Please run: gh auth login")]
    GhNotAuthenticated,

    #[error("gh API error: {0}")]
    GhApi(String),

    #[error("git command failed: {0}")]
    Git(String),

    #[error("Skill not found: {0}")]
    SkillNotFound(String),

    #[error("Skill already installed: {0}")]
    SkillAlreadyInstalled(String),

    #[error("Config not found: org is not set. Please run: koi remote set-org <org>")]
    OrgNotSet,

    #[error("Directory not found: {0}")]
    DirNotFound(PathBuf),

    #[error("User cancelled")]
    Cancelled,
}

pub type Result<T> = std::result::Result<T, KoiError>;

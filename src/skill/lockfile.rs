use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::utils::error::Result;
use crate::utils::fs::expand_tilde;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SkillsFile {
    #[serde(default)]
    pub skills: BTreeMap<String, String>,
}

pub fn local_lockfile_path() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    Ok(cwd.join(".koi.skills"))
}

pub fn global_lockfile_path() -> Result<PathBuf> {
    Ok(expand_tilde("~/.koi/global.skills"))
}

pub fn lockfile_path(global: bool) -> Result<PathBuf> {
    if global {
        global_lockfile_path()
    } else {
        local_lockfile_path()
    }
}

pub fn load_lockfile(global: bool) -> Result<SkillsFile> {
    let path = lockfile_path(global)?;
    if !path.exists() {
        return Ok(SkillsFile::default());
    }
    let content = fs::read_to_string(&path)?;
    let skills_file: SkillsFile = toml::from_str(&content)?;
    Ok(skills_file)
}

pub fn save_lockfile(global: bool, skills_file: &SkillsFile) -> Result<()> {
    let path = lockfile_path(global)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(skills_file)?;
    fs::write(&path, content)?;
    Ok(())
}

pub fn add_skill(global: bool, name: &str, repo: &str) -> Result<()> {
    let mut skills_file = load_lockfile(global)?;
    skills_file
        .skills
        .insert(name.to_string(), repo.to_string());
    save_lockfile(global, &skills_file)
}

pub fn remove_skill(global: bool, name: &str) -> Result<()> {
    let mut skills_file = load_lockfile(global)?;
    skills_file.skills.remove(name);
    save_lockfile(global, &skills_file)
}

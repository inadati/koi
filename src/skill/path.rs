use std::path::PathBuf;

use crate::utils::config::load_config;
use crate::utils::error::Result;
use crate::utils::fs::expand_tilde;

pub fn local_skills_dir() -> Result<PathBuf> {
    let config = load_config()?;
    let cwd = std::env::current_dir()?;
    Ok(cwd.join(&config.paths.local))
}

pub fn global_skills_dir() -> Result<PathBuf> {
    let config = load_config()?;
    Ok(expand_tilde(&config.paths.global))
}

pub fn skills_dir(global: bool) -> Result<PathBuf> {
    if global {
        global_skills_dir()
    } else {
        local_skills_dir()
    }
}

pub fn skill_path(global: bool, skill_name: &str) -> Result<PathBuf> {
    Ok(skills_dir(global)?.join(skill_name))
}

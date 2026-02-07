use std::path::Path;

use crate::git::command::git_in_dir;
use crate::utils::error::Result;

pub fn has_local_changes(dir: &Path) -> Result<bool> {
    let output = git_in_dir(dir, &["status", "--porcelain"])?;
    Ok(!output.trim().is_empty())
}

pub fn stash(dir: &Path) -> Result<()> {
    git_in_dir(dir, &["stash"])?;
    Ok(())
}

pub fn pull(dir: &Path) -> Result<()> {
    git_in_dir(dir, &["pull"])?;
    Ok(())
}

pub fn push_all(dir: &Path) -> Result<()> {
    git_in_dir(dir, &["add", "-A"])?;
    git_in_dir(dir, &["commit", "--no-verify", "-m", "update skill"])?;
    git_in_dir(dir, &["push", "--no-verify"])?;
    Ok(())
}

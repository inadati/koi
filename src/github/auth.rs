use std::process::Command;

use crate::utils::error::{KoiError, Result};

pub fn check_gh_installed() -> Result<()> {
    Command::new("gh")
        .arg("--version")
        .output()
        .map_err(|_| KoiError::GhNotInstalled)?;
    Ok(())
}

pub fn check_gh_auth() -> Result<()> {
    check_gh_installed()?;
    let output = Command::new("gh").args(["auth", "status"]).output()?;
    if !output.status.success() {
        return Err(KoiError::GhNotAuthenticated);
    }
    Ok(())
}

fn setup_git_credential() -> Result<()> {
    Command::new("gh")
        .args(["auth", "setup-git"])
        .output()?;
    Ok(())
}

pub fn ensure_gh_ready() -> Result<()> {
    check_gh_auth()?;
    setup_git_credential()?;
    Ok(())
}

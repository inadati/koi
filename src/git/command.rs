use std::path::Path;
use std::process::Command;

use crate::utils::error::{KoiError, Result};

pub fn git_in_dir(dir: &Path, args: &[&str]) -> Result<String> {
    let dir_str = dir
        .to_str()
        .ok_or_else(|| KoiError::Git(format!("Invalid path: {:?}", dir)))?;

    let output = Command::new("git")
        .arg("-C")
        .arg(dir_str)
        .args(args)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(KoiError::Git(format!(
            "git -C {} {} failed: {}",
            dir_str,
            args.join(" "),
            stderr
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn git_clone(url: &str, dest: &Path) -> Result<()> {
    let dest_str = dest
        .to_str()
        .ok_or_else(|| KoiError::Git(format!("Invalid path: {:?}", dest)))?;

    let output = Command::new("git")
        .args(["clone", url, dest_str])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(KoiError::Git(format!(
            "git clone {} {} failed: {}",
            url, dest_str, stderr
        )));
    }

    Ok(())
}

use std::process::Command;

use crate::utils::error::{KoiError, Result};

pub fn list_org_repo_names(org: &str) -> Result<Vec<String>> {
    let output = Command::new("gh")
        .args([
            "api",
            &format!("orgs/{org}/repos"),
            "--paginate",
            "--jq",
            ".[].name",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(KoiError::GhApi(stderr.to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut names: Vec<String> = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().to_string())
        .collect();
    names.sort();
    Ok(names)
}

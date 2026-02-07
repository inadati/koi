use std::io::Write;
use std::process::{Command, Stdio};

use crate::utils::error::{KoiError, Result};

pub fn gh_api(endpoint: &str) -> Result<String> {
    let output = Command::new("gh").args(["api", endpoint]).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(KoiError::GhApi(stderr.to_string()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn gh_api_post(endpoint: &str, body: &str) -> Result<String> {
    let mut child = Command::new("gh")
        .args(["api", endpoint, "-X", "POST", "--input", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(body.as_bytes())?;
    }
    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(KoiError::GhApi(stderr.to_string()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn gh_api_put(endpoint: &str, body: &str) -> Result<String> {
    let mut child = Command::new("gh")
        .args(["api", endpoint, "-X", "PUT", "--input", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(body.as_bytes())?;
    }
    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(KoiError::GhApi(stderr.to_string()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

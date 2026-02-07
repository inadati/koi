use std::io::{self, BufRead, Write};

use crate::utils::error::{KoiError, Result};

pub fn confirm(message: &str) -> Result<bool> {
    print!("{} (y/n): ", message);
    io::stdout().flush()?;

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap_or(Ok(String::new()))?;
    let trimmed = line.trim().to_lowercase();

    match trimmed.as_str() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        "" => Err(KoiError::Cancelled),
        _ => Ok(false),
    }
}

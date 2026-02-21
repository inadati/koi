use std::io::{self, BufRead, Write};

use crate::utils::error::Result;

pub fn input_text(prompt: &str) -> Result<String> {
    print!("{}: ", prompt);
    io::stdout().flush()?;

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap_or(Ok(String::new()))?;
    Ok(line.trim().to_string())
}

pub fn confirm(message: &str) -> Result<bool> {
    print!("{} (Y/n): ", message);
    io::stdout().flush()?;

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap_or(Ok(String::new()))?;
    let trimmed = line.trim().to_lowercase();

    match trimmed.as_str() {
        "" | "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        _ => Ok(false),
    }
}

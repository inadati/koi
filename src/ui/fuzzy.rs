use std::io::{self, BufRead, Write};

use crate::utils::error::{KoiError, Result};

/// Phase 1: 番号入力による簡易選択（Phase 3でdioxus FuzzySearchに置換）
pub fn select_from_list(items: &[String], prompt_msg: &str) -> Result<String> {
    if items.is_empty() {
        return Err(KoiError::SkillNotFound("no items available".to_string()));
    }

    println!("{}", prompt_msg);
    for (i, item) in items.iter().enumerate() {
        println!("  [{}] {}", i + 1, item);
    }
    print!("番号を入力 (1-{}): ", items.len());
    io::stdout().flush()?;

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap_or(Ok(String::new()))?;
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return Err(KoiError::Cancelled);
    }

    let index: usize = trimmed
        .parse()
        .map_err(|_| KoiError::Cancelled)?;

    if index == 0 || index > items.len() {
        return Err(KoiError::Cancelled);
    }

    Ok(items[index - 1].clone())
}

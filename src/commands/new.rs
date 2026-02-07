use crate::ui::progress;
use crate::utils::error::Result;

pub fn run(_name: &str) -> Result<()> {
    progress::info("koi new は Phase 2 で実装予定です");
    Ok(())
}

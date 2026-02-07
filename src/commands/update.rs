use crate::ui::progress;
use crate::utils::error::Result;

pub fn run(_global: bool) -> Result<()> {
    progress::info("koi update は Phase 2 で実装予定です");
    Ok(())
}

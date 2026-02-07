use crate::ui::progress;
use crate::utils::config::{load_config, save_config};
use crate::utils::error::Result;

pub fn run_set_org(org: &str) -> Result<()> {
    let mut config = load_config()?;
    config.remote.org = Some(org.to_string());
    save_config(&config)?;
    progress::success(&format!("org を '{}' に設定しました", org));
    Ok(())
}

pub fn run_update(_global: bool) -> Result<()> {
    progress::info("koi remote update は Phase 2 で実装予定です");
    Ok(())
}

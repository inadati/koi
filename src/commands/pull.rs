use crate::git::sync::{has_local_changes, pull, stash};
use crate::github::auth::ensure_gh_ready;
use crate::skill::path::skills_dir;
use crate::skill::validator::is_valid_skill;
use crate::ui::progress;
use crate::utils::error::Result;
use crate::utils::fs::list_subdirs;

pub fn run(global: bool) -> Result<()> {
    ensure_gh_ready()?;

    let dir = skills_dir(global)?;
    let names = list_subdirs(&dir)?;

    if names.is_empty() {
        progress::info("同期対象のスキルがありません");
        return Ok(());
    }

    for name in &names {
        let skill_dir = dir.join(name);
        if !is_valid_skill(&skill_dir) {
            continue;
        }

        if has_local_changes(&skill_dir)? {
            stash(&skill_dir)?;
            progress::warn(&format!(
                "スキル \"{}\" にローカル変更がありました → git stashで退避しました",
                name
            ));
        }

        progress::info(&format!("{} を同期中...", name));
        pull(&skill_dir)?;
        progress::success(&format!("{} を同期しました", name));
    }

    Ok(())
}

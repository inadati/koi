use std::fs;

use crate::git::clone::clone_skill;
use crate::github::auth::ensure_gh_ready;
use crate::skill::lockfile::load_lockfile;
use crate::skill::path::{skill_path, skills_dir};
use crate::ui::progress;
use crate::utils::config::{load_remotes, resolve_org};
use crate::utils::error::Result;

pub fn run(global: bool) -> Result<()> {
    ensure_gh_ready()?;

    let remotes = load_remotes()?;
    let skills_file = load_lockfile(global)?;
    if skills_file.skills.is_empty() {
        progress::info("復元するスキルがありません");
        return Ok(());
    }

    let dir = skills_dir(global)?;
    fs::create_dir_all(&dir)?;

    let mut had_error = false;
    for (name, alias) in &skills_file.skills {
        let dest = skill_path(global, name)?;
        if dest.exists() {
            progress::warn(&format!("{} は既に追加済み、スキップ", name));
            continue;
        }

        // エイリアス名からorg名を解決
        let org = match resolve_org(alias, &remotes) {
            Ok(o) => o,
            Err(e) => {
                progress::warn(&format!("{} をスキップ: {}", name, e));
                had_error = true;
                continue;
            }
        };

        progress::info(&format!("{} を追加中...", name));
        match clone_skill(&org, name, &dest) {
            Ok(()) => progress::success(&format!("{} を追加しました", name)),
            Err(e) => {
                progress::warn(&format!("{} の追加に失敗: {}", name, e));
                had_error = true;
            }
        }
    }

    if had_error {
        progress::warn("一部のスキルの復元に失敗しました");
    } else {
        progress::success("復元が完了しました");
    }
    Ok(())
}

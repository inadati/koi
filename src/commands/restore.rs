use std::fs;

use crate::git::clone::clone_skill;
use crate::github::auth::ensure_gh_ready;
use crate::skill::lockfile::load_lockfile;
use crate::skill::path::{skill_path, skills_dir};
use crate::ui::progress;
use crate::utils::config::get_org;
use crate::utils::error::Result;

pub fn run(global: bool) -> Result<()> {
    ensure_gh_ready()?;
    let org = get_org()?;

    let skills_file = load_lockfile(global)?;
    if skills_file.skills.is_empty() {
        progress::info("復元するスキルがありません");
        return Ok(());
    }

    let dir = skills_dir(global)?;
    fs::create_dir_all(&dir)?;

    for (name, _repo_ref) in &skills_file.skills {
        let dest = skill_path(global, name)?;
        if dest.exists() {
            progress::warn(&format!("{} は既に追加済み、スキップ", name));
            continue;
        }
        progress::info(&format!("{} を追加中...", name));
        clone_skill(&org, name, &dest)?;
        progress::success(&format!("{} を追加しました", name));
    }

    progress::success("復元が完了しました");
    Ok(())
}

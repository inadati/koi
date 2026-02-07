use crate::git::sync::{add_and_commit, has_local_changes, push};
use crate::github::auth::ensure_gh_ready;
use crate::skill::path::skills_dir;
use crate::skill::validator::is_valid_skill;
use crate::ui::progress;
use crate::utils::config::{load_config, save_config};
use crate::utils::error::Result;
use crate::utils::fs::list_subdirs;

pub fn run_set_org(org: &str) -> Result<()> {
    let mut config = load_config()?;
    config.remote.org = Some(org.to_string());
    save_config(&config)?;
    progress::success(&format!("org を '{}' に設定しました", org));
    Ok(())
}

pub fn run_update(global: bool) -> Result<()> {
    ensure_gh_ready()?;

    let dir = skills_dir(global)?;
    let names = list_subdirs(&dir)?;

    if names.is_empty() {
        progress::info("push対象のスキルがありません");
        return Ok(());
    }

    let mut had_errors = false;

    for name in &names {
        let skill_dir = dir.join(name);
        if !is_valid_skill(&skill_dir) {
            continue;
        }

        // 変更があればadd + commit
        if has_local_changes(&skill_dir)? {
            progress::info(&format!("{} の変更をコミット中...", name));
            add_and_commit(&skill_dir)?;
        }

        // push
        progress::info(&format!("{} をpush中...", name));
        match push(&skill_dir) {
            Ok(()) => {
                progress::success(&format!("{} をpushしました", name));
            }
            Err(_) => {
                eprintln!(
                    "Error: スキル \"{}\" のリモートが別の環境で更新されています",
                    name
                );
                eprintln!("  先に koi update を実行してください");
                had_errors = true;
            }
        }
    }

    if had_errors {
        std::process::exit(1);
    }

    Ok(())
}

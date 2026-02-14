use crate::git::sync::{add_and_commit, has_local_changes, pull, push, stash, stash_pop};
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

    let mut had_errors = false;

    for name in &names {
        let skill_dir = dir.join(name);
        if !is_valid_skill(&skill_dir) {
            continue;
        }

        let has_changes = has_local_changes(&skill_dir)?;

        if has_changes {
            // ローカル変更がある場合: stash → pull → stash pop → commit → push
            stash(&skill_dir)?;
            progress::info(&format!("{} のローカル変更を退避しました", name));

            progress::info(&format!("{} をpull中...", name));
            pull(&skill_dir)?;

            match stash_pop(&skill_dir) {
                Ok(()) => {
                    // stash popが成功 → commit + push
                    progress::info(&format!("{} の変更をコミット中...", name));
                    add_and_commit(&skill_dir)?;

                    progress::info(&format!("{} をpush中...", name));
                    match push(&skill_dir) {
                        Ok(()) => {
                            progress::success(&format!("{} を同期しました", name));
                        }
                        Err(e) => {
                            eprintln!("Error: スキル \"{}\" のpushに失敗しました: {}", name, e);
                            had_errors = true;
                        }
                    }
                }
                Err(_) => {
                    // stash popでコンフリクト → 警告を表示して次のスキルへ
                    progress::warn(&format!(
                        "スキル \"{}\" でコンフリクトが発生しました",
                        name
                    ));
                    eprintln!("  手動で解決してください: {}", skill_dir.display());
                    had_errors = true;
                }
            }
        } else {
            // ローカル変更がない場合: pullのみ
            progress::info(&format!("{} をpull中...", name));
            pull(&skill_dir)?;
            progress::success(&format!("{} を同期しました", name));
        }
    }

    if had_errors {
        std::process::exit(1);
    }

    Ok(())
}

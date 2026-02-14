use std::fs;

use crate::git::clone::clone_skill;
use crate::github::auth::ensure_gh_ready;
use crate::github::repo::list_org_repo_names;
use crate::skill::lockfile::{add_skill, load_lockfile};
use crate::skill::path::{skill_path, skills_dir};
use crate::ui::fuzzy::select_multiple_from_list;
use crate::ui::progress;
use crate::utils::config::get_org;
use crate::utils::error::{KoiError, Result};

pub fn run(name: Option<String>, global: bool, restore: bool) -> Result<()> {
    ensure_gh_ready()?;
    let org = get_org()?;

    if restore {
        return run_restore(global, &org);
    }

    let repo_names = match name {
        Some(n) => vec![n],
        None => {
            progress::info("リモートリポジトリを取得中...");
            let repos = list_org_repo_names(&org)?;
            let installed = load_lockfile(global)?;
            let repos: Vec<String> = repos
                .into_iter()
                .filter(|r| !installed.skills.contains_key(r))
                .collect();
            if repos.is_empty() {
                return Err(KoiError::SkillNotFound(format!(
                    "org '{}' にクローン可能なスキルがありません",
                    org
                )));
            }
            select_multiple_from_list(&repos, "クローンするスキルを選択:")?
        }
    };

    let dir = skills_dir(global)?;
    fs::create_dir_all(&dir)?;

    let mut has_error = false;
    for repo_name in &repo_names {
        let dest = skill_path(global, repo_name)?;
        if dest.exists() {
            progress::warn(&format!("{} は既にクローン済み、スキップ", repo_name));
            continue;
        }

        progress::info(&format!("{} をクローン中...", repo_name));
        match clone_skill(&org, repo_name, &dest) {
            Ok(()) => {
                let repo_ref = format!("{}/{}", org, repo_name);
                add_skill(global, repo_name, &repo_ref)?;
                progress::success(&format!("{} をクローンしました", repo_name));
            }
            Err(e) => {
                progress::warn(&format!("{} のクローンに失敗: {}", repo_name, e));
                has_error = true;
            }
        }
    }

    if has_error {
        progress::warn("一部のスキルのクローンに失敗しました");
    }
    Ok(())
}

fn run_restore(global: bool, org: &str) -> Result<()> {
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
            progress::warn(&format!("{} は既にクローン済み、スキップ", name));
            continue;
        }
        progress::info(&format!("{} をクローン中...", name));
        clone_skill(org, name, &dest)?;
        progress::success(&format!("{} をクローンしました", name));
    }

    progress::success("復元が完了しました");
    Ok(())
}

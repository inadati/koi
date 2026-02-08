use std::fs;

use crate::git::clone::clone_skill;
use crate::github::auth::ensure_gh_ready;
use crate::github::repo::list_org_repo_names;
use crate::skill::lockfile::{add_skill, load_lockfile};
use crate::skill::path::{skill_path, skills_dir};
use crate::ui::fuzzy::select_from_list;
use crate::ui::progress;
use crate::utils::config::get_org;
use crate::utils::error::{KoiError, Result};

pub fn run(name: Option<String>, global: bool, restore: bool) -> Result<()> {
    ensure_gh_ready()?;
    let org = get_org()?;

    if restore {
        return run_restore(global, &org);
    }

    let repo_name = match name {
        Some(n) => n,
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
                    "org '{}' にインストール可能なスキルがありません",
                    org
                )));
            }
            select_from_list(&repos, "インストールするスキルを選択:")?
        }
    };

    let dest = skill_path(global, &repo_name)?;
    if dest.exists() {
        return Err(KoiError::SkillAlreadyInstalled(repo_name));
    }

    let dir = skills_dir(global)?;
    fs::create_dir_all(&dir)?;

    progress::info(&format!("{} をインストール中...", repo_name));
    clone_skill(&org, &repo_name, &dest)?;

    let repo_ref = format!("{}/{}", org, repo_name);
    add_skill(global, &repo_name, &repo_ref)?;

    progress::success(&format!("{} をインストールしました", repo_name));
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
            progress::warn(&format!("{} は既にインストール済み、スキップ", name));
            continue;
        }
        progress::info(&format!("{} をインストール中...", name));
        clone_skill(org, name, &dest)?;
        progress::success(&format!("{} をインストールしました", name));
    }

    progress::success("復元が完了しました");
    Ok(())
}

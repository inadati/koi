use std::fs;

use crate::git::clone::clone_skill;
use crate::github::auth::ensure_gh_ready;
use crate::github::repo::list_org_repo_names;
use crate::skill::lockfile::{add_skill, load_lockfile};
use crate::skill::path::{skill_path, skills_dir};
use crate::ui::fuzzy::select_multiple_from_list;
use crate::ui::progress;
use crate::utils::config::{load_remotes, resolve_org};
use crate::utils::error::{KoiError, Result};

pub fn run(name: Option<String>, global: bool) -> Result<()> {
    ensure_gh_ready()?;

    let remotes = load_remotes()?;
    if remotes.remotes.is_empty() {
        return Err(KoiError::RemoteNotFound(
            "リモートが登録されていません。`koi remote add` で追加してください".to_string(),
        ));
    }

    // 暫定: 最初のリモートを使用（#21で全リモート横断検索に変更予定）
    let (alias, _) = remotes.remotes.iter().next().unwrap();
    let alias = alias.clone();
    let org = resolve_org(&alias, &remotes)?;

    let repo_names = match name {
        Some(n) => vec![n],
        None => {
            progress::info("リモートリポジトリを取得中...");
            let repos = list_org_repo_names(&org)?;
            let installed_local = load_lockfile(false).unwrap_or_default();
            let installed_global = load_lockfile(true).unwrap_or_default();
            let repos: Vec<String> = repos
                .into_iter()
                .filter(|r| {
                    !installed_local.skills.contains_key(r)
                        && !installed_global.skills.contains_key(r)
                })
                .collect();
            if repos.is_empty() {
                return Err(KoiError::SkillNotFound(format!(
                    "remote '{}' に追加可能なスキルがありません",
                    alias
                )));
            }
            select_multiple_from_list(&repos, "追加するスキルを選択:")?
        }
    };

    let dir = skills_dir(global)?;
    fs::create_dir_all(&dir)?;

    let mut has_error = false;
    for repo_name in &repo_names {
        let dest = skill_path(global, repo_name)?;
        if dest.exists() {
            progress::warn(&format!("{} は既に追加済み、スキップ", repo_name));
            continue;
        }

        progress::info(&format!("{} を追加中...", repo_name));
        match clone_skill(&org, repo_name, &dest) {
            Ok(()) => {
                add_skill(global, repo_name, &alias)?;
                progress::success(&format!("{} を追加しました", repo_name));
            }
            Err(e) => {
                progress::warn(&format!("{} の追加に失敗: {}", repo_name, e));
                has_error = true;
            }
        }
    }

    if has_error {
        progress::warn("一部のスキルの追加に失敗しました");
    }
    Ok(())
}

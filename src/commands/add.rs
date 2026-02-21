use std::collections::HashMap;
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

    let installed_local = load_lockfile(false).unwrap_or_default();
    let installed_global = load_lockfile(true).unwrap_or_default();

    // 選択された (repo_name, alias) のリスト
    let selected: Vec<(String, String)> = match name {
        Some(n) => {
            // 全リモートを横断して <n> に一致するリポジトリを探す（AI用直接指定）
            let mut found = None;
            for (alias, entry) in &remotes.remotes {
                match list_org_repo_names(&entry.org) {
                    Ok(repos) => {
                        if repos.iter().any(|r| r == &n) {
                            found = Some((n.clone(), alias.clone()));
                            break;
                        }
                    }
                    Err(e) => {
                        progress::warn(&format!("remote '{}' の取得に失敗: {}", alias, e));
                    }
                }
            }
            match found {
                Some(pair) => vec![pair],
                None => {
                    return Err(KoiError::SkillNotFound(format!(
                        "スキル '{}' が見つかりませんでした",
                        n
                    )));
                }
            }
        }
        None => {
            // 全リモートを横断してリポジトリ一覧を収集
            progress::info("リモートリポジトリを取得中...");

            // display_string → (repo_name, alias) のマッピング
            let mut display_map: HashMap<String, (String, String)> = HashMap::new();
            let mut display_items: Vec<String> = Vec::new();

            for (alias, entry) in &remotes.remotes {
                match list_org_repo_names(&entry.org) {
                    Ok(repos) => {
                        for repo in repos {
                            // インストール済みをスキル名（キー）でフィルタリング
                            if installed_local.skills.contains_key(&repo)
                                || installed_global.skills.contains_key(&repo)
                            {
                                continue;
                            }
                            let display = format!("{:<30} @{}", repo, alias);
                            display_map.insert(display.clone(), (repo, alias.clone()));
                            display_items.push(display);
                        }
                    }
                    Err(e) => {
                        progress::warn(&format!("remote '{}' の取得に失敗: {}", alias, e));
                    }
                }
            }

            display_items.sort();

            if display_items.is_empty() {
                return Err(KoiError::SkillNotFound(
                    "追加可能なスキルがありません".to_string(),
                ));
            }

            let selected_displays =
                select_multiple_from_list(&display_items, "追加するスキルを選択:")?;

            selected_displays
                .into_iter()
                .filter_map(|d| display_map.remove(&d))
                .collect()
        }
    };

    let dir = skills_dir(global)?;
    fs::create_dir_all(&dir)?;

    let mut has_error = false;
    for (repo_name, alias) in &selected {
        let org = match resolve_org(alias, &remotes) {
            Ok(o) => o,
            Err(e) => {
                progress::warn(&format!("{} の追加に失敗: {}", repo_name, e));
                has_error = true;
                continue;
            }
        };

        let dest = skill_path(global, repo_name)?;
        if dest.exists() {
            progress::warn(&format!("{} は既に追加済み、スキップ", repo_name));
            continue;
        }

        progress::info(&format!("{} を追加中...", repo_name));
        match clone_skill(&org, repo_name, &dest) {
            Ok(()) => {
                add_skill(global, repo_name, alias)?;
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

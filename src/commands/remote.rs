use crate::skill::lockfile::load_lockfile;
use crate::ui::fuzzy::select_from_list;
use crate::ui::progress;
use crate::ui::prompt::input_text;
use crate::utils::config::{load_remotes, save_remotes, validate_alias, RemoteEntry};
use crate::utils::error::{KoiError, Result};

pub fn run_add(org: &str, name: Option<String>) -> Result<()> {
    let mut remotes = load_remotes()?;

    // エイリアス名を決定
    let alias = match name {
        Some(n) => {
            if !validate_alias(&n) {
                return Err(KoiError::InvalidAlias(n));
            }
            n
        }
        None => loop {
            let input = input_text(
                "エイリアス名を入力してください (英数字・ハイフン・アンダースコアのみ)",
            )?;
            if validate_alias(&input) {
                break input;
            }
            progress::warn("無効なエイリアス名です。英数字・ハイフン・アンダースコアのみ使用できます");
        },
    };

    // エイリアス重複チェック
    if remotes.remotes.contains_key(&alias) {
        return Err(KoiError::RemoteNotFound(format!(
            "エイリアス '{}' は既に使用されています",
            alias
        )));
    }

    // 同じorgの二重登録チェック（警告のみ）
    if remotes.remotes.values().any(|e| e.org == org) {
        progress::warn(&format!(
            "org '{}' は既に別のエイリアスで登録されています",
            org
        ));
    }

    remotes
        .remotes
        .insert(alias.clone(), RemoteEntry { org: org.to_string() });
    save_remotes(&remotes)?;
    progress::success(&format!("remote '{}' ({}) を追加しました", alias, org));
    Ok(())
}

pub fn run_remove(alias: Option<String>) -> Result<()> {
    let mut remotes = load_remotes()?;
    let alias_name = match alias {
        Some(n) => n,
        None => {
            let remote_names: Vec<String> = remotes.remotes.keys().cloned().collect();
            if remote_names.is_empty() {
                return Err(KoiError::SkillNotFound(
                    "削除可能なremoteがありません".to_string(),
                ));
            }
            select_from_list(&remote_names, "削除するremoteを選択:")?
        }
    };

    if !remotes.remotes.contains_key(&alias_name) {
        return Err(KoiError::RemoteNotFound(format!(
            "remote '{}' は登録されていません",
            alias_name
        )));
    }

    // .koi.skills の参照チェック（警告のみ）
    let local_lockfile = load_lockfile(false).unwrap_or_default();
    let global_lockfile = load_lockfile(true).unwrap_or_default();
    let referencing: Vec<String> = local_lockfile
        .skills
        .iter()
        .chain(global_lockfile.skills.iter())
        .filter(|(_, v)| v.as_str() == alias_name)
        .map(|(k, _)| k.clone())
        .collect();
    if !referencing.is_empty() {
        progress::warn(&format!(
            "remote '{}' は以下のスキルから参照されています: {}",
            alias_name,
            referencing.join(", ")
        ));
        progress::warn("削除後、これらのスキルの restore / add が失敗します");
    }

    remotes.remotes.remove(&alias_name);
    save_remotes(&remotes)?;
    progress::success(&format!("remote '{}' を削除しました", alias_name));
    Ok(())
}

pub fn run_list() -> Result<()> {
    let remotes = load_remotes()?;
    if remotes.remotes.is_empty() {
        progress::info("登録されているremoteがありません");
        return Ok(());
    }
    for (alias, entry) in &remotes.remotes {
        println!("  {} ({})", alias, entry.org);
    }
    Ok(())
}

pub fn run_set_url(alias: &str, new_org: &str) -> Result<()> {
    let mut remotes = load_remotes()?;
    match remotes.remotes.get_mut(alias) {
        Some(entry) => {
            entry.org = new_org.to_string();
            save_remotes(&remotes)?;
            progress::success(&format!(
                "remote '{}' のorg名を '{}' に更新しました",
                alias, new_org
            ));
            Ok(())
        }
        None => Err(KoiError::RemoteNotFound(format!(
            "remote '{}' は登録されていません",
            alias
        ))),
    }
}

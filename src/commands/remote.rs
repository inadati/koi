use crate::ui::fuzzy::select_from_list;
use crate::ui::progress;
use crate::utils::config::{load_remotes, save_remotes, RemoteEntry};
use crate::utils::error::{KoiError, Result};

pub fn run_add(org: &str) -> Result<()> {
    let mut remotes = load_remotes()?;
    if remotes.remotes.contains_key(org) {
        progress::info(&format!("remote '{}' は既に登録されています", org));
        return Ok(());
    }
    remotes.remotes.insert(
        org.to_string(),
        RemoteEntry {
            description: String::new(),
        },
    );
    // 初めてのremote追加時はactiveに設定
    if remotes.active.is_none() {
        remotes.active = Some(org.to_string());
    }
    save_remotes(&remotes)?;
    progress::success(&format!("remote '{}' を追加しました", org));
    Ok(())
}

pub fn run_remove(org: Option<String>) -> Result<()> {
    let mut remotes = load_remotes()?;
    let org_name = match org {
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

    if !remotes.remotes.contains_key(&org_name) {
        return Err(KoiError::SkillNotFound(format!(
            "remote '{}' は登録されていません",
            org_name
        )));
    }

    remotes.remotes.remove(&org_name);
    // 削除したのがactiveだった場合、activeをクリアまたは別のremoteに切り替え
    if remotes.active.as_deref() == Some(&org_name) {
        remotes.active = remotes.remotes.keys().next().cloned();
    }
    save_remotes(&remotes)?;
    progress::success(&format!("remote '{}' を削除しました", org_name));
    Ok(())
}

pub fn run_list() -> Result<()> {
    let remotes = load_remotes()?;
    if remotes.remotes.is_empty() {
        progress::info("登録されているremoteがありません");
        return Ok(());
    }
    for (name, entry) in &remotes.remotes {
        let marker = if remotes.active.as_deref() == Some(name) {
            "* "
        } else {
            "  "
        };
        if entry.description.is_empty() {
            println!("{}{}", marker, name);
        } else {
            println!("{}{} ({})", marker, name, entry.description);
        }
    }
    Ok(())
}

pub fn run_switch(org: Option<String>) -> Result<()> {
    let mut remotes = load_remotes()?;
    let org_name = match org {
        Some(n) => n,
        None => {
            let remote_names: Vec<String> = remotes.remotes.keys().cloned().collect();
            if remote_names.is_empty() {
                return Err(KoiError::SkillNotFound(
                    "切り替え可能なremoteがありません".to_string(),
                ));
            }
            select_from_list(&remote_names, "切り替え先のremoteを選択:")?
        }
    };

    if !remotes.remotes.contains_key(&org_name) {
        return Err(KoiError::SkillNotFound(format!(
            "remote '{}' は登録されていません",
            org_name
        )));
    }

    remotes.active = Some(org_name.clone());
    save_remotes(&remotes)?;
    progress::success(&format!("remote を '{}' に切り替えました", org_name));
    Ok(())
}

use crate::ui::fuzzy::select_from_list;
use crate::ui::progress;
use crate::utils::config::{load_config, save_config};
use crate::utils::error::{KoiError, Result};

pub fn run_add(org: &str) -> Result<()> {
    let mut config = load_config()?;
    config.remote.org = Some(org.to_string());
    save_config(&config)?;
    progress::success(&format!("remote '{}' を追加しました", org));
    Ok(())
}

pub fn run_remove(org: Option<String>) -> Result<()> {
    let org_name = match org {
        Some(n) => n,
        None => {
            // TODO: ~/.koi/remotes.tomlから登録済みremote一覧を取得
            let remotes = vec!["example-org".to_string()]; // スタブ
            if remotes.is_empty() {
                return Err(KoiError::SkillNotFound(
                    "削除可能なremoteがありません".to_string(),
                ));
            }
            select_from_list(&remotes, "削除するremoteを選択:")?
        }
    };

    // TODO: ~/.koi/remotes.tomlから削除する実装
    progress::success(&format!("remote '{}' を削除しました", org_name));
    Ok(())
}

pub fn run_list() -> Result<()> {
    // TODO: ~/.koi/remotes.tomlから一覧を取得する実装
    let config = load_config()?;
    if let Some(org) = config.remote.org {
        println!("* {}", org);
    } else {
        progress::info("登録されているremoteがありません");
    }
    Ok(())
}

pub fn run_switch(org: Option<String>) -> Result<()> {
    let org_name = match org {
        Some(n) => n,
        None => {
            // TODO: ~/.koi/remotes.tomlから登録済みremote一覧を取得
            let remotes = vec!["example-org".to_string()]; // スタブ
            if remotes.is_empty() {
                return Err(KoiError::SkillNotFound(
                    "切り替え可能なremoteがありません".to_string(),
                ));
            }
            select_from_list(&remotes, "切り替え先のremoteを選択:")?
        }
    };

    let mut config = load_config()?;
    config.remote.org = Some(org_name.clone());
    save_config(&config)?;
    progress::success(&format!("remote を '{}' に切り替えました", org_name));
    Ok(())
}

use std::fs;

use crate::git::clone::clone_skill;
use crate::github::api::{gh_api_post, gh_api_put};
use crate::github::auth::ensure_gh_ready;
use crate::skill::lockfile::add_skill;
use crate::skill::path::{skill_path, skills_dir};
use crate::ui::fuzzy::select_from_list;
use crate::ui::progress;
use crate::utils::config::{load_remotes, resolve_org};
use crate::utils::error::{KoiError, Result};

pub fn run(name: &str, remote_alias: Option<String>) -> Result<()> {
    ensure_gh_ready()?;

    let remotes = load_remotes()?;
    if remotes.remotes.is_empty() {
        return Err(KoiError::RemoteNotFound(
            "リモートが登録されていません。`koi remote add` で追加してください".to_string(),
        ));
    }

    // リモートエイリアスを決定
    let alias = match remote_alias {
        Some(a) => a,
        None => {
            let remote_names: Vec<String> = remotes.remotes.keys().cloned().collect();
            if remote_names.len() == 1 {
                remote_names.into_iter().next().unwrap()
            } else {
                select_from_list(&remote_names, "作成先のremoteを選択:")?
            }
        }
    };

    let org = resolve_org(&alias, &remotes)?;

    let dest = skill_path(false, name)?;
    if dest.exists() {
        return Err(KoiError::SkillAlreadyInstalled(name.to_string()));
    }

    // 1. orgに新規プライベートリポジトリを作成
    progress::info(&format!("リポジトリ {}/{} を作成中...", org, name));
    let repo_body = serde_json::json!({
        "name": name,
        "private": true,
        "auto_init": true,
    });
    gh_api_post(&format!("orgs/{}/repos", org), &repo_body.to_string())?;

    // 2. SKILL.mdテンプレートを配置
    let skill_md_content = format!("# {}\n\nスキルの説明をここに記載してください。\n", name);
    let encoded = base64_encode(&skill_md_content);
    let file_body = serde_json::json!({
        "message": "add SKILL.md",
        "content": encoded,
    });
    gh_api_put(
        &format!("repos/{}/{}/contents/SKILL.md", org, name),
        &file_body.to_string(),
    )?;

    // 3. git clone
    let dir = skills_dir(false)?;
    fs::create_dir_all(&dir)?;

    progress::info(&format!("{} をクローン中...", name));
    clone_skill(&org, name, &dest)?;

    // 4. .koi.skillsに追記（エイリアス名を記録）
    add_skill(false, name, &alias)?;

    progress::success(&format!("{} を作成しました", name));
    Ok(())
}

fn base64_encode(input: &str) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

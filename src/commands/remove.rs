use crate::skill::lockfile::remove_skill;
use crate::skill::path::{skill_path, skills_dir};
use crate::skill::validator::is_valid_skill;
use crate::ui::fuzzy::select_multiple_from_list;
use crate::ui::prompt::confirm;
use crate::ui::progress;
use crate::utils::error::{KoiError, Result};
use crate::utils::fs::list_subdirs;

pub fn run(name: Option<String>, global: bool) -> Result<()> {
    let skill_names = match name {
        Some(n) => vec![n],
        None => {
            let dir = skills_dir(global)?;
            let names: Vec<String> = list_subdirs(&dir)?
                .into_iter()
                .filter(|n| is_valid_skill(&dir.join(n)))
                .collect();
            if names.is_empty() {
                return Err(KoiError::SkillNotFound(
                    "削除可能なスキルがありません".to_string(),
                ));
            }
            select_multiple_from_list(&names, "削除するスキルを選択:")?
        }
    };

    let names_display = skill_names.join(", ");
    if !confirm(&format!("{} を削除しますか？", names_display))? {
        progress::info("キャンセルしました");
        return Ok(());
    }

    for skill_name in &skill_names {
        let dest = skill_path(global, skill_name)?;
        if !dest.exists() {
            progress::warn(&format!("{} が見つかりません、スキップ", skill_name));
            continue;
        }

        std::fs::remove_dir_all(&dest)?;
        remove_skill(global, skill_name)?;
        progress::success(&format!("{} を削除しました", skill_name));
    }

    Ok(())
}

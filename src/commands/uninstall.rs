use crate::skill::lockfile::remove_skill;
use crate::skill::path::{skill_path, skills_dir};
use crate::skill::validator::is_valid_skill;
use crate::ui::fuzzy::select_from_list;
use crate::ui::prompt::confirm;
use crate::ui::progress;
use crate::utils::error::{KoiError, Result};
use crate::utils::fs::list_subdirs;

pub fn run(name: Option<String>, global: bool) -> Result<()> {
    let skill_name = match name {
        Some(n) => n,
        None => {
            let dir = skills_dir(global)?;
            let names: Vec<String> = list_subdirs(&dir)?
                .into_iter()
                .filter(|n| is_valid_skill(&dir.join(n)))
                .collect();
            if names.is_empty() {
                return Err(KoiError::SkillNotFound(
                    "アンインストール可能なスキルがありません".to_string(),
                ));
            }
            select_from_list(&names, "アンインストールするスキルを選択:")?
        }
    };

    let dest = skill_path(global, &skill_name)?;
    if !dest.exists() {
        return Err(KoiError::SkillNotFound(skill_name));
    }

    if !confirm(&format!("{} をアンインストールしますか？", skill_name))? {
        progress::info("キャンセルしました");
        return Ok(());
    }

    std::fs::remove_dir_all(&dest)?;
    remove_skill(global, &skill_name)?;

    progress::success(&format!("{} をアンインストールしました", skill_name));
    Ok(())
}

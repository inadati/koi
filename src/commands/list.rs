use crate::skill::metadata::read_skill_info;
use crate::skill::path::skills_dir;
use crate::skill::validator::is_valid_skill;
use crate::ui::progress;
use crate::utils::error::Result;
use crate::utils::fs::list_subdirs;

pub fn run(global: bool) -> Result<()> {
    let dir = skills_dir(global)?;
    let names = list_subdirs(&dir)?;

    if names.is_empty() {
        progress::info("インストール済みのスキルはありません");
        return Ok(());
    }

    for name in &names {
        let skill_dir = dir.join(name);
        if !is_valid_skill(&skill_dir) {
            continue;
        }
        let info = read_skill_info(&skill_dir);
        match info.description {
            Some(desc) => println!("  {} - {}", info.name, desc),
            None => println!("  {}", info.name),
        }
    }

    Ok(())
}

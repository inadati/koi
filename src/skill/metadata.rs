use std::fs;
use std::path::Path;

pub struct SkillInfo {
    pub name: String,
    pub description: Option<String>,
}

pub fn read_skill_info(skill_dir: &Path) -> SkillInfo {
    let name = skill_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let skill_md = skill_dir.join("SKILL.md");
    let description = if skill_md.exists() {
        fs::read_to_string(&skill_md).ok().and_then(|content| {
            content
                .lines()
                .find(|line| !line.trim().is_empty() && !line.starts_with('#'))
                .map(|line| line.trim().to_string())
        })
    } else {
        None
    };

    SkillInfo { name, description }
}

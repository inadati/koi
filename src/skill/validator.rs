use std::path::Path;

pub fn is_valid_skill(skill_dir: &Path) -> bool {
    skill_dir.is_dir() && skill_dir.join(".git").exists()
}

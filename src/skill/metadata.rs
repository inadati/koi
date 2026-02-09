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
        fs::read_to_string(&skill_md)
            .ok()
            .and_then(|content| parse_description(&content))
    } else {
        None
    };

    SkillInfo { name, description }
}

fn parse_description(content: &str) -> Option<String> {
    let trimmed = content.trim_start();

    // YAML Front Matter がある場合: --- で始まり、次の --- までがメタデータ
    if trimmed.starts_with("---") {
        let after_first = &trimmed[3..].trim_start_matches('\n');
        if let Some(end) = after_first.find("\n---") {
            let front_matter = &after_first[..end];
            // Front Matterから description を探す
            for line in front_matter.lines() {
                if let Some(rest) = line.strip_prefix("description:") {
                    let desc = rest.trim().to_string();
                    if !desc.is_empty() {
                        return Some(desc);
                    }
                }
            }
        }
    }

    // Front Matterがない、またはdescriptionフィールドがない場合:
    // 本文から最初の意味のある行を取得
    let body = skip_front_matter(content);
    body.lines()
        .find(|line| {
            let t = line.trim();
            !t.is_empty() && !t.starts_with('#')
        })
        .map(|line| line.trim().to_string())
}

fn skip_front_matter(content: &str) -> &str {
    let trimmed = content.trim_start();
    if trimmed.starts_with("---") {
        let after_first = &trimmed[3..].trim_start_matches('\n');
        if let Some(end) = after_first.find("\n---") {
            let rest = &after_first[end + 4..];
            return rest;
        }
    }
    content
}

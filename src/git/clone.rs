use std::path::Path;

use crate::git::command::git_clone;
use crate::utils::error::Result;

pub fn clone_skill(org: &str, repo_name: &str, dest: &Path) -> Result<()> {
    let url = format!("https://github.com/{}/{}.git", org, repo_name);
    git_clone(&url, dest)
}

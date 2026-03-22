use serde::{Deserialize, Serialize};

/// A source of agent skills: a local directory or a git repo.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SkillSource {
    /// Local directory containing skill subdirectories with SKILL.md files.
    Path { path: String },
    /// Git repository containing skill subdirectories with SKILL.md files.
    Git { git: String },
}

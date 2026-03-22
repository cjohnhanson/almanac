use std::path::Path;

use crate::error::Error;
use crate::source::SkillSource;

/// A single skill entry: name, description, and where to find the full content.
#[derive(Debug, Clone, PartialEq)]
pub struct SkillEntry {
    pub name: String,
    pub description: String,
    pub source: SkillLocation,
}

/// Where the full SKILL.md content lives.
#[derive(Debug, Clone, PartialEq)]
pub enum SkillLocation {
    /// On-disk path to the SKILL.md file.
    File(String),
    /// Built into the binary; use `almanac show <name>`.
    #[allow(dead_code)] // Populated when built-in skill content is authored.
    BuiltIn,
}

/// YAML frontmatter parsed from a SKILL.md file.
#[derive(Debug, serde::Deserialize)]
struct SkillFrontmatter {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
}

/// A built-in skill: name, description, and full content compiled into the binary.
struct BuiltInSkill {
    name: &'static str,
    description: &'static str,
    content: &'static str,
}

/// All built-in skills. Content will be added as skills are authored.
const BUILTIN_SKILLS: &[BuiltInSkill] = &[];

/// Scan all configured skill sources and return an index of available skills.
pub fn index(project_dir: &Path, sources: &[SkillSource]) -> Vec<SkillEntry> {
    let mut entries = Vec::new();

    entries.extend(builtin_skills());

    for source in sources {
        match source {
            SkillSource::Path { path } => {
                let resolved = resolve_path(project_dir, path);
                if let Ok(found) = scan_directory(&resolved) {
                    entries.extend(found);
                }
            }
            SkillSource::Git { git: _ } => {
                // Git sources require a local clone. Not yet implemented.
            }
        }
    }

    entries
}

/// Print the full content of a named skill. Returns Ok(true) if found, Ok(false) if not.
pub fn show(name: &str, project_dir: &Path, sources: &[SkillSource]) -> Result<bool, Error> {
    // Check built-in skills first.
    for skill in BUILTIN_SKILLS {
        if skill.name == name {
            print!("{}", skill.content);
            return Ok(true);
        }
    }

    // Check file-based skills.
    let entries = index(project_dir, sources);
    for entry in &entries {
        if entry.name == name {
            if let SkillLocation::File(path) = &entry.source {
                let content = std::fs::read_to_string(path)
                    .map_err(|e| Error::General(format!("failed to read {path}: {e}")))?;
                print!("{content}");
                return Ok(true);
            }
        }
    }

    Ok(false)
}

/// Format the skill index for injection into agent context.
pub fn format_index(entries: &[SkillEntry]) -> String {
    if entries.is_empty() {
        return String::new();
    }

    let mut out = String::from(
        "## Skills (almanac)\n\nAvailable skills — read the full SKILL.md when needed:\n\n",
    );
    for entry in entries {
        let retrieval = match &entry.source {
            SkillLocation::File(path) => format!("file: {path}"),
            SkillLocation::BuiltIn => format!("`almanac show {}`", entry.name),
        };
        out.push_str(&format!(
            "- **{}**: {} ({})\n",
            entry.name, entry.description, retrieval
        ));
    }
    out.push('\n');
    out
}

/// Format the skill index as JSON for machine consumption.
pub fn format_index_json(entries: &[SkillEntry]) -> String {
    let items: Vec<serde_json::Value> = entries
        .iter()
        .map(|e| {
            serde_json::json!({
                "name": e.name,
                "description": e.description,
                "source": match &e.source {
                    SkillLocation::File(path) => serde_json::json!({"file": path}),
                    SkillLocation::BuiltIn => serde_json::json!({"builtin": true}),
                },
            })
        })
        .collect();
    serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string())
}

// --- internal helpers ---

fn scan_directory(dir: &Path) -> Result<Vec<SkillEntry>, Error> {
    let mut entries = Vec::new();

    let read_dir = std::fs::read_dir(dir)
        .map_err(|e| Error::General(format!("failed to read skill dir {}: {e}", dir.display())))?;

    for entry in read_dir.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let skill_md = path.join("SKILL.md");
        if !skill_md.exists() {
            continue;
        }
        if let Ok(entry) = parse_skill_md(&skill_md, &path) {
            entries.push(entry);
        }
    }

    Ok(entries)
}

fn parse_skill_md(skill_md: &Path, skill_dir: &Path) -> Result<SkillEntry, Error> {
    let content = std::fs::read_to_string(skill_md)
        .map_err(|e| Error::General(format!("failed to read {}: {e}", skill_md.display())))?;

    let frontmatter = extract_frontmatter(&content)
        .ok_or_else(|| Error::General(format!("no frontmatter in {}", skill_md.display())))?;

    let parsed: SkillFrontmatter = serde_yml::from_str(frontmatter).map_err(|e| {
        Error::General(format!(
            "invalid frontmatter in {}: {e}",
            skill_md.display()
        ))
    })?;

    let dir_name = skill_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    Ok(SkillEntry {
        name: parsed.name.unwrap_or_else(|| dir_name.to_string()),
        description: parsed.description.unwrap_or_default(),
        source: SkillLocation::File(skill_md.to_string_lossy().into_owned()),
    })
}

fn extract_frontmatter(content: &str) -> Option<&str> {
    let trimmed = content.trim_start();
    let rest = trimmed.strip_prefix("---")?;
    let end = rest.find("\n---")?;
    Some(rest[..end].trim())
}

fn resolve_path(project_dir: &Path, path: &str) -> std::path::PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }
    if Path::new(path).is_absolute() {
        Path::new(path).to_path_buf()
    } else {
        project_dir.join(path)
    }
}

fn builtin_skills() -> Vec<SkillEntry> {
    BUILTIN_SKILLS
        .iter()
        .map(|s| SkillEntry {
            name: s.name.to_string(),
            description: s.description.to_string(),
            source: SkillLocation::BuiltIn,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_frontmatter_basic() {
        let content = "---\nname: my-skill\ndescription: Does a thing\n---\n\n# My Skill\n";
        let fm = extract_frontmatter(content).unwrap();
        assert!(fm.contains("name: my-skill"));
        assert!(fm.contains("description: Does a thing"));
    }

    #[test]
    fn extract_frontmatter_missing_returns_none() {
        assert!(extract_frontmatter("# No frontmatter\n").is_none());
    }

    #[test]
    fn extract_frontmatter_incomplete_returns_none() {
        assert!(extract_frontmatter("---\nname: incomplete\n").is_none());
    }

    #[test]
    fn parse_skill_md_extracts_name_and_description() {
        let dir = tempfile::tempdir().unwrap();
        let skill_dir = dir.path().join("my-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: my-skill\ndescription: Does a useful thing\n---\n\n# Instructions\n",
        )
        .unwrap();

        let entry = parse_skill_md(&skill_dir.join("SKILL.md"), &skill_dir).unwrap();
        assert_eq!(entry.name, "my-skill");
        assert_eq!(entry.description, "Does a useful thing");
    }

    #[test]
    fn parse_skill_md_falls_back_to_dir_name() {
        let dir = tempfile::tempdir().unwrap();
        let skill_dir = dir.path().join("fallback-name");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\ndescription: Just a description\n---\nContent\n",
        )
        .unwrap();

        let entry = parse_skill_md(&skill_dir.join("SKILL.md"), &skill_dir).unwrap();
        assert_eq!(entry.name, "fallback-name");
    }

    #[test]
    fn scan_directory_finds_skills() {
        let dir = tempfile::tempdir().unwrap();

        for name in &["skill-a", "skill-b"] {
            let skill = dir.path().join(name);
            std::fs::create_dir_all(&skill).unwrap();
            std::fs::write(
                skill.join("SKILL.md"),
                format!("---\nname: {name}\ndescription: A skill\n---\nContent\n"),
            )
            .unwrap();
        }

        // Non-skill directory
        let not_a_skill = dir.path().join("not-a-skill");
        std::fs::create_dir_all(&not_a_skill).unwrap();
        std::fs::write(not_a_skill.join("README.md"), "just a readme").unwrap();

        let entries = scan_directory(dir.path()).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn scan_directory_skips_root_skill_md() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("SKILL.md"), "---\nname: root\n---\n").unwrap();
        let entries = scan_directory(dir.path()).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn scan_directory_nonexistent_returns_error() {
        assert!(scan_directory(Path::new("/nonexistent/path")).is_err());
    }

    #[test]
    fn index_with_local_path_source() {
        let dir = tempfile::tempdir().unwrap();
        let skill = dir.path().join("skills").join("my-skill");
        std::fs::create_dir_all(&skill).unwrap();
        std::fs::write(
            skill.join("SKILL.md"),
            "---\nname: my-skill\ndescription: A test skill\n---\nContent\n",
        )
        .unwrap();

        let sources = vec![SkillSource::Path {
            path: dir.path().join("skills").to_string_lossy().into_owned(),
        }];

        let entries = index(dir.path(), &sources);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "my-skill");
    }

    #[test]
    fn index_with_relative_path_source() {
        let dir = tempfile::tempdir().unwrap();
        let skill = dir.path().join("skills").join("local-skill");
        std::fs::create_dir_all(&skill).unwrap();
        std::fs::write(
            skill.join("SKILL.md"),
            "---\nname: local-skill\ndescription: A local skill\n---\nContent\n",
        )
        .unwrap();

        let sources = vec![SkillSource::Path {
            path: "./skills/".into(),
        }];

        let entries = index(dir.path(), &sources);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "local-skill");
    }

    #[test]
    fn index_empty_sources_returns_only_builtins() {
        let dir = tempfile::tempdir().unwrap();
        let entries = index(dir.path(), &[]);
        assert!(entries.is_empty());
    }

    #[test]
    fn index_skips_missing_sources() {
        let dir = tempfile::tempdir().unwrap();
        let sources = vec![SkillSource::Path {
            path: "/nonexistent/skills/".into(),
        }];
        let entries = index(dir.path(), &sources);
        assert!(entries.is_empty());
    }

    #[test]
    fn show_returns_false_for_unknown() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!show("nonexistent", dir.path(), &[]).unwrap());
    }

    #[test]
    fn show_returns_true_for_file_skill() {
        let dir = tempfile::tempdir().unwrap();
        let skill = dir.path().join("skills").join("my-skill");
        std::fs::create_dir_all(&skill).unwrap();
        std::fs::write(
            skill.join("SKILL.md"),
            "---\nname: my-skill\ndescription: test\n---\n\n# My Skill\n",
        )
        .unwrap();

        let sources = vec![SkillSource::Path {
            path: dir.path().join("skills").to_string_lossy().into_owned(),
        }];

        assert!(show("my-skill", dir.path(), &sources).unwrap());
    }

    #[test]
    fn show_no_partial_match() {
        let dir = tempfile::tempdir().unwrap();
        let skill = dir.path().join("skills").join("my-skill");
        std::fs::create_dir_all(&skill).unwrap();
        std::fs::write(
            skill.join("SKILL.md"),
            "---\nname: my-skill\ndescription: test\n---\nContent\n",
        )
        .unwrap();

        let sources = vec![SkillSource::Path {
            path: dir.path().join("skills").to_string_lossy().into_owned(),
        }];

        assert!(!show("my", dir.path(), &sources).unwrap());
    }

    #[test]
    fn format_index_empty() {
        assert_eq!(format_index(&[]), "");
    }

    #[test]
    fn format_index_includes_entries() {
        let entries = vec![SkillEntry {
            name: "test-skill".into(),
            description: "A test".into(),
            source: SkillLocation::File("/path/to/SKILL.md".into()),
        }];
        let output = format_index(&entries);
        assert!(output.contains("test-skill"));
        assert!(output.contains("A test"));
        assert!(output.contains("file: /path/to/SKILL.md"));
    }

    #[test]
    fn format_index_json_produces_valid_json() {
        let entries = vec![
            SkillEntry {
                name: "a".into(),
                description: "first".into(),
                source: SkillLocation::File("/a/SKILL.md".into()),
            },
            SkillEntry {
                name: "b".into(),
                description: "second".into(),
                source: SkillLocation::BuiltIn,
            },
        ];
        let json = format_index_json(&entries);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.as_array().unwrap().len(), 2);
    }

    #[test]
    fn resolve_path_relative() {
        let project = Path::new("/home/user/project");
        let resolved = resolve_path(project, "./skills/");
        assert_eq!(resolved, Path::new("/home/user/project/./skills/"));
    }

    #[test]
    fn resolve_path_absolute() {
        let project = Path::new("/home/user/project");
        let resolved = resolve_path(project, "/opt/skills/");
        assert_eq!(resolved, Path::new("/opt/skills/"));
    }
}

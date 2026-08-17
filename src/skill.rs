use std::path::Path;

use crate::error::Error;
use crate::source::SkillSource;

/// A single skill entry: name, description, and where to find the full content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillEntry {
    pub name: String,
    pub description: String,
    pub source: SkillLocation,
}

/// Where the full SKILL.md content lives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillLocation {
    /// Path to the SKILL.md file on disk.
    File(String),
}

/// YAML frontmatter parsed from a SKILL.md file.
///
/// A key that almanac does not model is kept, so a tool that writes its
/// own key into a SKILL.md never loses it.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct SkillFrontmatter {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(flatten)]
    extra: yaml_serde::Mapping,
}

/// Scan the configured sources and return the skills a command can
/// reach.
///
/// The sources are in precedence order: the nearer source wins a name.
/// A name that a later source also holds is dropped here, so the list
/// shows what a command would actually load. `almanac check` reports
/// the drop, because a skill that silently replaced another is the
/// worst way to find out.
#[must_use]
pub fn index(project_dir: &Path, sources: &[SkillSource]) -> Vec<SkillEntry> {
    let mut entries: Vec<SkillEntry> = Vec::new();

    for source in sources {
        match source {
            SkillSource::Path { path } => {
                let resolved = resolve_path(project_dir, path);
                if let Ok(found) = scan_directory(&resolved) {
                    let fresh: Vec<SkillEntry> = found
                        .into_iter()
                        .filter(|f| !entries.iter().any(|e| e.name == f.name))
                        .collect();
                    entries.extend(fresh);
                }
            }
            SkillSource::Git { git: _ } => {
                // Git sources require a local clone. Not yet implemented.
            }
        }
    }

    entries
}

/// Print the full content of one skill. Returns Ok(true) when the
/// skill is found, and Ok(false) when it is not.
///
/// A `name/references/<file>` path fetches one reference file. When the
/// name is a skill, and the skill directory has a `references/`
/// directory, the output ends with a listing of those files.
pub fn show(name: &str, project_dir: &Path, sources: &[SkillSource]) -> Result<bool, Error> {
    let Some(content) = show_captured(name, project_dir, sources)? else {
        return Ok(false);
    };
    print!("{content}");
    Ok(true)
}

/// Return the content of one skill, or of one reference file, as a
/// string. Returns Ok(None) when it is not found.
pub fn show_captured(
    name: &str,
    project_dir: &Path,
    sources: &[SkillSource],
) -> Result<Option<String>, Error> {
    // Check for reference path: "skill-name/references/file.md"
    if let Some((skill_name, ref_path)) = parse_reference_path(name) {
        return show_reference(skill_name, ref_path, project_dir, sources);
    }

    let entries = index(project_dir, sources);
    for entry in &entries {
        if entry.name == name {
            let SkillLocation::File(path) = &entry.source;
            let mut content = std::fs::read_to_string(path)
                .map_err(|e| Error::General(format!("failed to read {path}: {e}")))?;
            let skill_dir = Path::new(path).parent().unwrap_or_else(|| Path::new("."));
            append_file_references(name, skill_dir, &mut content);
            return Ok(Some(content));
        }
    }

    Ok(None)
}

/// Parse "skill-name/references/file.md" into ("skill-name", "file.md").
fn parse_reference_path(name: &str) -> Option<(&str, &str)> {
    let rest = name.split_once("/references/")?;
    if rest.0.is_empty() || rest.1.is_empty() {
        return None;
    }
    Some(rest)
}

/// Fetch a reference file for a file-based skill.
fn show_reference(
    skill_name: &str,
    ref_file: &str,
    project_dir: &Path,
    sources: &[SkillSource],
) -> Result<Option<String>, Error> {
    // A reference names one file inside the skill's references
    // directory. A '..' check alone let an absolute path through, and
    // join replaces on an absolute component, so the name decided
    // which file was read.
    if !mdstore::is_plain_stem(ref_file) {
        return Ok(None);
    }

    let entries = index(project_dir, sources);
    for entry in &entries {
        if entry.name == skill_name {
            let SkillLocation::File(path) = &entry.source;
            let skill_dir = Path::new(path).parent().unwrap_or_else(|| Path::new("."));
            let refs_dir = skill_dir.join("references");

            // A library may be content somebody else controls, and the
            // reference name arrives from caller text. The handle
            // confines it to the references directory, so a name that
            // climbs out is refused by the operating system.
            //
            // This was a canonicalize and a starts_with. Between the
            // check and the open, the path could change; the handle
            // has no such gap, because there is no second lookup.
            let Ok(refs) = mdstore::confined::StoreDir::open(&refs_dir) else {
                return Ok(None);
            };
            if !refs.is_document(ref_file) {
                return Ok(None);
            }
            let content = refs.read(ref_file).map_err(|e| {
                Error::General(format!("failed to read {ref_file} in references: {e}"))
            })?;
            return Ok(Some(content));
        }
    }

    Ok(None)
}

/// Append a references listing when the skill directory holds a
/// references/ directory.
fn append_file_references(skill_name: &str, skill_dir: &Path, content: &mut String) {
    let refs_dir = skill_dir.join("references");
    if !refs_dir.is_dir() {
        return;
    }
    let mut files: Vec<String> = std::fs::read_dir(&refs_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_file())
        .filter_map(|e| e.file_name().to_str().map(String::from))
        .collect();
    if files.is_empty() {
        return;
    }
    files.sort();
    content.push_str("\n\n## References\n\nUse `almanac show <skill>/references/<file>` to load a reference.\n\n");
    for file in &files {
        use std::fmt::Write as _;
        let _ = writeln!(content, "  `almanac show {skill_name}/references/{file}`");
    }
}

/// Format the skill index for injection into agent context.
#[must_use]
pub fn format_index(entries: &[SkillEntry]) -> String {
    if entries.is_empty() {
        return String::new();
    }

    let mut out = String::from(
        "## Skills (almanac)\n\nAvailable skills — read the full SKILL.md when needed:\n\n",
    );
    out.push_str(&format_index_list(entries));
    out
}

/// Format the skill list without a header. Use it when the caller
/// supplies its own framing.
#[must_use]
pub fn format_index_list(entries: &[SkillEntry]) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    for entry in entries {
        let SkillLocation::File(path) = &entry.source;
        let retrieval = format!("file: {path}");
        let _ = writeln!(
            out,
            "- **{}**: {} ({})",
            entry.name, entry.description, retrieval
        );
    }
    out.push('\n');
    out
}

/// Format the skill index as JSON for a machine to read.
#[must_use]
pub fn format_index_json(entries: &[SkillEntry]) -> String {
    let items: Vec<serde_json::Value> = entries
        .iter()
        .map(|e| {
            let SkillLocation::File(path) = &e.source;
            serde_json::json!({
                "name": e.name,
                "description": e.description,
                "source": serde_json::json!({"file": path}),
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
        // A symlinked skill directory would otherwise be walked, and
        // the library is content that somebody else may control.
        if !std::fs::symlink_metadata(&path).is_ok_and(|m| m.is_dir()) {
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
    // A skill directory can hold a symlinked SKILL.md pointing at a
    // file outside the library. The guard refuses anything that is not
    // a regular file.
    let name = skill_md
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| Error::General(format!("{} has no file name", skill_md.display())))?;
    let content = mdstore::confined::StoreDir::open(skill_dir)
        .and_then(|dir| dir.read(name))
        .map_err(|e| Error::General(format!("failed to read {}: {e}", skill_md.display())))?;

    // mdstore parses the frontmatter, so almanac, zettel, and tisket
    // all read a frontmattered markdown file the same way. A
    // hand-rolled reader drifts from the writer, and the drift shows up
    // as a file that one tool writes and another cannot read.
    let doc = mdstore::document::parse::<SkillFrontmatter>(&content).map_err(|e| match e {
        mdstore::Error::MissingFrontmatter | mdstore::Error::UnclosedFrontmatter => {
            Error::General(format!("no frontmatter in {}", skill_md.display()))
        }
        other => Error::General(format!(
            "invalid frontmatter in {}: {other}",
            skill_md.display()
        )),
    })?;
    let parsed = doc.frontmatter;

    let dir_name = skill_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let name = parsed.name.unwrap_or_else(|| dir_name.to_string());

    // The agentskills.io spec requires the name to match the directory
    // name. Warn when it does not.
    if name != dir_name {
        eprintln!(
            "warning: skill name '{name}' does not match directory '{dir_name}' in {}",
            skill_md.display()
        );
    }

    Ok(SkillEntry {
        name,
        description: parsed.description.unwrap_or_default(),
        source: SkillLocation::File(skill_md.to_string_lossy().into_owned()),
    })
}

fn resolve_path(project_dir: &Path, path: &str) -> std::path::PathBuf {
    if let Some(rest) = path.strip_prefix("~/")
        && let Some(home) = dirs::home_dir()
    {
        return home.join(rest);
    }
    if Path::new(path).is_absolute() {
        Path::new(path).to_path_buf()
    } else {
        project_dir.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Write a SKILL.md and parse it, as the scan does.
    fn parse_content(content: &str) -> Result<SkillEntry, Error> {
        let dir = tempfile::tempdir().unwrap();
        let skill_dir = dir.path().join("probe-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        let path = skill_dir.join("SKILL.md");
        std::fs::write(&path, content).unwrap();
        parse_skill_md(&path, &skill_dir)
    }

    #[test]
    fn a_file_without_frontmatter_is_rejected() {
        let err = parse_content("# No frontmatter\n").unwrap_err().to_string();
        assert!(err.contains("no frontmatter"), "{err}");
    }

    #[test]
    fn an_unclosed_frontmatter_block_is_rejected() {
        let err = parse_content("---\nname: probe-skill\n")
            .unwrap_err()
            .to_string();
        assert!(err.contains("no frontmatter"), "{err}");
    }

    #[test]
    fn invalid_yaml_is_rejected_with_the_file_named() {
        let err = parse_content("---\nname: [unclosed\n---\nbody\n")
            .unwrap_err()
            .to_string();
        assert!(err.contains("invalid frontmatter"), "{err}");
        assert!(err.contains("SKILL.md"), "{err}");
    }

    #[test]
    fn a_quoted_description_with_metacharacters_round_trips() {
        // A hand-rolled reader splits on the first "\n---", which a
        // description containing that text would break. mdstore parses
        // the YAML, so the value survives.
        let entry = parse_content(
            "---\nname: probe-skill\ndescription: \"colons: yes, brackets [x], and a quote\\\"\"\n---\nbody\n",
        )
        .unwrap();
        assert_eq!(
            entry.description,
            "colons: yes, brackets [x], and a quote\""
        );
    }

    #[test]
    fn frontmatter_keys_almanac_does_not_model_are_kept() {
        let entry = parse_content(
            "---\nname: probe-skill\ndescription: d\nlicense: MIT\nversion: 2\n---\nbody\n",
        )
        .unwrap();
        assert_eq!(entry.name, "probe-skill");
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
    fn index_empty_sources_returns_nothing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(index(dir.path(), &[]).is_empty());
    }

    #[test]
    fn index_skips_missing_sources() {
        let dir = tempfile::tempdir().unwrap();
        let sources = vec![SkillSource::Path {
            path: "/nonexistent/skills/".into(),
        }];
        assert!(index(dir.path(), &sources).is_empty());
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
                source: SkillLocation::File("/b/SKILL.md".into()),
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

    // --- references/ support ---

    fn make_skill_with_refs(dir: &Path) -> Vec<SkillSource> {
        let skill = dir.join("skills").join("my-skill");
        let refs = skill.join("references");
        std::fs::create_dir_all(&refs).unwrap();
        std::fs::write(
            skill.join("SKILL.md"),
            "---\nname: my-skill\ndescription: A skill with refs\n---\n\n# My Skill\n",
        )
        .unwrap();
        std::fs::write(
            refs.join("deep-dive.md"),
            "# Deep Dive\n\nDetailed content.\n",
        )
        .unwrap();
        std::fs::write(refs.join("examples.md"), "# Examples\n\nSome examples.\n").unwrap();
        vec![SkillSource::Path {
            path: dir.join("skills").to_string_lossy().into_owned(),
        }]
    }

    #[test]
    fn show_reference_file_returns_content() {
        let dir = tempfile::tempdir().unwrap();
        let sources = make_skill_with_refs(dir.path());
        let output = show_to_string("my-skill/references/deep-dive.md", dir.path(), &sources);
        assert!(output.contains("# Deep Dive"));
        assert!(output.contains("Detailed content."));
    }

    /// A reference name is caller text and reaches this from a
    /// published library. The handle refuses a name that leaves the
    /// references directory, and refuses a link that points out of it.
    #[test]
    fn a_reference_cannot_leave_the_references_directory() {
        let dir = tempfile::tempdir().unwrap();
        let sources = make_skill_with_refs(dir.path());
        std::fs::write(dir.path().join("secret.md"), "SECRET").unwrap();

        for name in [
            "../../secret.md",
            "../SKILL.md",
            "/etc/hosts",
            "..",
            ".",
            "a/b.md",
        ] {
            let out = show_reference("my-skill", name, dir.path(), &sources).unwrap();
            assert!(out.is_none(), "{name} was read");
        }

        // A link planted inside the references directory is refused by
        // type, not followed. A predicate that canonicalized first
        // accepted this, because the resolved path was checked and the
        // open happened afterwards.
        let refs = dir.path().join("skills/my-skill/references");
        std::os::unix::fs::symlink(dir.path().join("secret.md"), refs.join("planted.md")).unwrap();
        let out = show_reference("my-skill", "planted.md", dir.path(), &sources).unwrap();
        assert!(out.is_none(), "a planted link was followed");

        // A link whose target sits inside the references directory is
        // still refused. This is what separates the handle from the
        // canonicalize-and-compare it replaced: that predicate asked
        // where the link pointed, and accepted it whenever the answer
        // was 'inside'. A library the reader does not control decides
        // what its files are, and a link is not a file.
        std::os::unix::fs::symlink("examples.md", refs.join("inside.md")).unwrap();
        let out = show_reference("my-skill", "inside.md", dir.path(), &sources).unwrap();
        assert!(
            out.is_none(),
            "a link inside the references directory was followed"
        );

        // What genuinely sits there still reads.
        let good = show_reference("my-skill", "examples.md", dir.path(), &sources).unwrap();
        assert!(good.is_some_and(|c| c.contains("Some examples")));
    }

    #[test]
    fn show_reference_nonexistent_returns_false() {
        let dir = tempfile::tempdir().unwrap();
        let sources = make_skill_with_refs(dir.path());
        assert!(!show("my-skill/references/nonexistent.md", dir.path(), &sources).unwrap());
    }

    #[test]
    fn show_skill_with_refs_appends_listing() {
        let dir = tempfile::tempdir().unwrap();
        let sources = make_skill_with_refs(dir.path());
        let output = show_to_string("my-skill", dir.path(), &sources);
        assert!(output.contains("# My Skill"));
        assert!(output.contains("## References"));
        assert!(output.contains("almanac show my-skill/references/deep-dive.md"));
        assert!(output.contains("almanac show my-skill/references/examples.md"));
    }

    #[test]
    fn show_skill_without_refs_no_listing() {
        let dir = tempfile::tempdir().unwrap();
        let skill = dir.path().join("skills").join("plain-skill");
        std::fs::create_dir_all(&skill).unwrap();
        std::fs::write(
            skill.join("SKILL.md"),
            "---\nname: plain-skill\ndescription: No refs\n---\n\n# Plain\n",
        )
        .unwrap();
        let sources = vec![SkillSource::Path {
            path: dir.path().join("skills").to_string_lossy().into_owned(),
        }];
        let output = show_to_string("plain-skill", dir.path(), &sources);
        assert!(output.contains("# Plain"));
        assert!(!output.contains("## References"));
    }

    /// Helper: capture `show()` output as a string instead of printing to stdout.
    fn show_to_string(name: &str, project_dir: &Path, sources: &[SkillSource]) -> String {
        show_captured(name, project_dir, sources)
            .unwrap()
            .unwrap_or_default()
    }
}

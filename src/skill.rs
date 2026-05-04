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

/// All built-in skills, baked into the binary at compile time.
const BUILTIN_SKILLS: &[BuiltInSkill] = &[
    BuiltInSkill {
        name: "anti-slop",
        description: "Pre-generation constraints for avoiding AI-written prose patterns. Applies to all writing: docs, READMEs, commit messages, PR descriptions, comments, essays.",
        content: include_str!("../skills/anti-slop/SKILL.md"),
    },
    BuiltInSkill {
        name: "api-design-eval",
        description: "API design evaluation using Richardson REST maturity model, Google/Zalando/Microsoft guidelines, JSON:API patterns, and developer experience heuristics.",
        content: include_str!("../skills/api-design-eval/SKILL.md"),
    },
    BuiltInSkill {
        name: "architecture-eval",
        description: "Architecture evaluation using ATAM quality attribute scenarios, ISO 25010 quality characteristics, coupling/cohesion analysis, SOLID at the system level, Conway's Law alignment, C4 model, ADR review, and technical debt assessment.",
        content: include_str!("../skills/architecture-eval/SKILL.md"),
    },
    BuiltInSkill {
        name: "code-review-eval",
        description: "Structured code review using Fagan inspection passes, Google's priority hierarchy, SOLID principles, Fowler's code smells, and cognitive complexity.",
        content: include_str!("../skills/code-review-eval/SKILL.md"),
    },
    BuiltInSkill {
        name: "continuous-improvement",
        description: "Driver-tick loop for long-running project sessions. A scheduled prompt fires on a regular cadence, dispatches fresh-eyes reviewers across review dimensions, and applies mechanical catches inline.",
        content: include_str!("../skills/continuous-improvement/SKILL.md"),
    },
    BuiltInSkill {
        name: "debugging",
        description: "Structured debugging using scientific method, 5 Whys, Ishikawa fishbone diagrams, fault tree analysis, delta debugging, wolf fence bisection, Kepner-Tregoe IS/IS NOT analysis, and blameless incident investigation.",
        content: include_str!("../skills/debugging/SKILL.md"),
    },
    BuiltInSkill {
        name: "design-review",
        description: "Systematic design evaluation for web UIs using Nielsen's heuristics, Gestalt principles, and cognitive walkthrough.",
        content: include_str!("../skills/design-review/SKILL.md"),
    },
    BuiltInSkill {
        name: "doc-coauthoring",
        description: "Structured co-authoring workflow for substantial writing (design docs, RFCs, specs, proposals, long-form prose). Three stages: context gathering, section-by-section drafting, reader testing.",
        content: include_str!("../skills/doc-coauthoring/SKILL.md"),
    },
    BuiltInSkill {
        name: "doc-editing",
        description: "Collaborative writing of project documentation with section-by-section drafting where the user directs and curates rather than receiving a finished document. Applies Di\u{00e1}taxis type discipline and source-verified technical claims.",
        content: include_str!("../skills/doc-editing/SKILL.md"),
    },
    BuiltInSkill {
        name: "full-review",
        description: "Dispatch all review skills as independent fresh-context subagents against the current diff or working tree. Each subagent reads its skill file and the changes cold, reports severity-rated findings, and merges results after all complete.",
        content: include_str!("../skills/full-review/SKILL.md"),
    },
    BuiltInSkill {
        name: "library-first-eval",
        description: "Audit for homegrown implementations of solved problems. Walks every module looking for custom code that reimplements what an existing dependency or a well-maintained open-source library already provides.",
        content: include_str!("../skills/library-first-eval/SKILL.md"),
    },
    BuiltInSkill {
        name: "performance-eval",
        description: "Web performance evaluation using RAIL model, Core Web Vitals (LCP, INP, CLS), performance budgets, critical rendering path analysis, Lighthouse scoring, and anti-pattern detection.",
        content: include_str!("../skills/performance-eval/SKILL.md"),
    },
    BuiltInSkill {
        name: "playwright-missouri",
        description: "Writing missouri tests that need browser interaction \u{2014} testing web UIs, verifying rendered state, or automating browser workflows as part of state graph transitions.",
        content: include_str!("../skills/playwright-missouri/SKILL.md"),
    },
    BuiltInSkill {
        name: "product-eval",
        description: "Structured product evaluation using Jobs-to-be-Done analysis, falsifiability testing, scope discipline, primary-deliverable critique, guardrail-metric audit, and risk identification. For PRDs, product briefs, feature specs, or phase plans.",
        content: include_str!("../skills/product-eval/SKILL.md"),
    },
    BuiltInSkill {
        name: "qa-cli",
        description: "Exploratory QA for CLI tools. Enumerates testable areas via SFDPOT heuristics adapted for command-line interfaces, dispatches sub-agents as independent checkers, evaluates results against consistency oracles.",
        content: include_str!("../skills/qa-cli/SKILL.md"),
    },
    BuiltInSkill {
        name: "qa-web",
        description: "Exploratory QA for web UIs using agent-browser. Enumerates testable areas via SFDPOT heuristics, dispatches sub-agents as independent checkers, evaluates results against consistency oracles (FEW HICCUPPS), runs fresh-eyes sessions.",
        content: include_str!("../skills/qa-web/SKILL.md"),
    },
    BuiltInSkill {
        name: "research",
        description: "Structured research using Cynefin classification, PRISMA-style search accountability, Zettelkasten note processing, grounded theory coding, ACH for competing hypotheses, and source triangulation.",
        content: include_str!("../skills/research/SKILL.md"),
    },
    BuiltInSkill {
        name: "security-review",
        description: "Security evaluation using OWASP Top 10, STRIDE threat modeling, DREAD risk rating, and a structured review checklist covering input validation, auth, session management, cryptography, error handling, logging, data protection, and dependencies.",
        content: include_str!("../skills/security-review/SKILL.md"),
    },
    BuiltInSkill {
        name: "structured-thinking",
        description: "Structured thinking and communication using Minto Pyramid (SCQA), BLUF, MECE issue trees, Six Thinking Hats, first principles, steel manning, OODA loop, and Socratic questioning.",
        content: include_str!("../skills/structured-thinking/SKILL.md"),
    },
    BuiltInSkill {
        name: "testing-strategy",
        description: "Test strategy design using shape models (pyramid, trophy, diamond), Marick's testing quadrants, risk-based prioritization, RCRCRC regression selection, Beck's 12 test desiderata, Google's test size classification, and specialized techniques (contract, property-based, mutation testing).",
        content: include_str!("../skills/testing-strategy/SKILL.md"),
    },
    BuiltInSkill {
        name: "tisket-writing",
        description: "Write well-scoped tisket issues using INVEST criteria, problem-first framing, testable acceptance criteria, and vertical slicing for decomposition.",
        content: include_str!("../skills/tisket-writing/SKILL.md"),
    },
    BuiltInSkill {
        name: "writing-docs-eval",
        description: "Documentation-specific evaluation using IBM DQTI nine characteristics, Di\u{00e1}taxis per-type quality criteria, and Baker's Every Page is Page One principles.",
        content: include_str!("../skills/writing-docs-eval/SKILL.md"),
    },
    BuiltInSkill {
        name: "writing-review",
        description: "Orchestrates writing evaluation by classifying content and dispatching the appropriate framework skills. Runs sentence-level checks, docs evaluation, readability metrics, voice consistency, and fresh-eyes review.",
        content: include_str!("../skills/writing-review/SKILL.md"),
    },
    BuiltInSkill {
        name: "writing-sentence-level",
        description: "Sentence-level writing evaluation using Orwell's 6 rules, Williams' clarity diagnostics, readability metrics, and persuasion frameworks (AIDA/PAS).",
        content: include_str!("../skills/writing-sentence-level/SKILL.md"),
    },
    BuiltInSkill {
        name: "zettel",
        description: "Working with the zettel knowledge base \u{2014} searching for context, creating notes on request, doing research, or exploring the note graph.",
        content: include_str!("../skills/zettel/SKILL.md"),
    },
];

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
///
/// Supports `name/references/<file>` paths for fetching individual reference files.
/// When showing a skill (not a reference), appends a reference listing if the skill
/// has a `references/` directory.
pub fn show(name: &str, project_dir: &Path, sources: &[SkillSource]) -> Result<bool, Error> {
    match show_captured(name, project_dir, sources)? {
        Some(content) => {
            print!("{content}");
            Ok(true)
        }
        None => Ok(false),
    }
}

/// Return the content of a named skill (or reference) as a string.
/// Returns Ok(None) if not found.
pub fn show_captured(
    name: &str,
    project_dir: &Path,
    sources: &[SkillSource],
) -> Result<Option<String>, Error> {
    // Check for reference path: "skill-name/references/file.md"
    if let Some((skill_name, ref_path)) = parse_reference_path(name) {
        return show_reference(skill_name, ref_path, project_dir, sources);
    }

    // Check built-in skills first.
    for skill in BUILTIN_SKILLS {
        if skill.name == name {
            let mut content = skill.content.to_string();
            append_builtin_references(name, &mut content);
            return Ok(Some(content));
        }
    }

    // Check file-based skills.
    let entries = index(project_dir, sources);
    for entry in &entries {
        if entry.name == name {
            if let SkillLocation::File(path) = &entry.source {
                let mut content = std::fs::read_to_string(path)
                    .map_err(|e| Error::General(format!("failed to read {path}: {e}")))?;
                let skill_dir = Path::new(path).parent().unwrap_or(Path::new("."));
                append_file_references(name, skill_dir, &mut content);
                return Ok(Some(content));
            }
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

/// Fetch a reference file for a file-based or built-in skill.
fn show_reference(
    skill_name: &str,
    ref_file: &str,
    project_dir: &Path,
    sources: &[SkillSource],
) -> Result<Option<String>, Error> {
    // Reject path traversal.
    if ref_file.contains("..") {
        return Ok(None);
    }

    // Check built-in references first.
    for skill in BUILTIN_SKILLS {
        if skill.name == skill_name {
            for r in builtin_references(skill_name) {
                if r.name == ref_file {
                    return Ok(Some(r.content.to_string()));
                }
            }
            return Ok(None);
        }
    }

    // Check file-based skills.
    let entries = index(project_dir, sources);
    for entry in &entries {
        if entry.name == skill_name {
            if let SkillLocation::File(path) = &entry.source {
                let skill_dir = Path::new(path).parent().unwrap_or(Path::new("."));
                let ref_path = skill_dir.join("references").join(ref_file);
                if ref_path.is_file() {
                    let content = std::fs::read_to_string(&ref_path).map_err(|e| {
                        Error::General(format!("failed to read {}: {e}", ref_path.display()))
                    })?;
                    return Ok(Some(content));
                }
            }
            return Ok(None);
        }
    }

    Ok(None)
}

/// Append a references listing to content if the file-based skill has a references/ dir.
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
        content.push_str(&format!(
            "  `almanac show {skill_name}/references/{file}`\n"
        ));
    }
}

/// Append a references listing for a built-in skill.
fn append_builtin_references(skill_name: &str, content: &mut String) {
    let refs = builtin_references(skill_name);
    if refs.is_empty() {
        return;
    }
    content.push_str("\n\n## References\n\nUse `almanac show <skill>/references/<file>` to load a reference.\n\n");
    for r in &refs {
        content.push_str(&format!(
            "  `almanac show {skill_name}/references/{}`\n",
            r.name
        ));
    }
}

/// A built-in reference file.
struct BuiltInReference {
    name: &'static str,
    content: &'static str,
}

/// Return built-in references for a skill. Currently empty — populated as
/// skills gain reference files.
fn builtin_references(_skill_name: &str) -> Vec<BuiltInReference> {
    // TODO: match on skill_name and return include_str!() references
    // when skills have references/ directories.
    vec![]
}

/// Format the skill index for injection into agent context.
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

/// Format just the skill list (no header). For callers that provide their own framing.
pub fn format_index_list(entries: &[SkillEntry]) -> String {
    let mut out = String::new();
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

    let name = parsed.name.unwrap_or_else(|| dir_name.to_string());

    // Warn if name doesn't match directory name per agentskills.io spec.
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
        let file_entries: Vec<_> = entries.iter().filter(|e| e.source != SkillLocation::BuiltIn).collect();
        assert_eq!(file_entries.len(), 1);
        assert_eq!(file_entries[0].name, "my-skill");
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
        let file_entries: Vec<_> = entries.iter().filter(|e| e.source != SkillLocation::BuiltIn).collect();
        assert_eq!(file_entries.len(), 1);
        assert_eq!(file_entries[0].name, "local-skill");
    }

    #[test]
    fn index_empty_sources_returns_only_builtins() {
        let dir = tempfile::tempdir().unwrap();
        let entries = index(dir.path(), &[]);
        assert_eq!(entries.len(), BUILTIN_SKILLS.len());
        assert!(entries.iter().all(|e| e.source == SkillLocation::BuiltIn));
    }

    #[test]
    fn index_skips_missing_sources() {
        let dir = tempfile::tempdir().unwrap();
        let sources = vec![SkillSource::Path {
            path: "/nonexistent/skills/".into(),
        }];
        let entries = index(dir.path(), &sources);
        assert_eq!(entries.len(), BUILTIN_SKILLS.len());
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
        std::fs::write(refs.join("deep-dive.md"), "# Deep Dive\n\nDetailed content.\n").unwrap();
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

    /// Helper: capture show() output as a string instead of printing to stdout.
    fn show_to_string(name: &str, project_dir: &Path, sources: &[SkillSource]) -> String {
        show_captured(name, project_dir, sources).unwrap().unwrap_or_default()
    }
}

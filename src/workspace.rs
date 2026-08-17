//! The library a command reads from: this one plus the libraries it
//! declares.
//!
//! A library declares the other libraries it draws on, in `stores.yml`,
//! under local aliases. Two things follow that a single directory
//! cannot give:
//!
//! **Precedence.** A skill name is the identity, and two libraries can
//! hold the same name. The nearer library wins: this library first,
//! then each declared library in declaration order. The loser is not
//! discarded quietly; `check` reports every shadowed name, because a
//! skill that silently replaced another is the worst way to find out.
//!
//! **Linking.** A skill declares the skills it needs, in a `requires:`
//! frontmatter key. An entry can name another library: `alias:name`.
//! `check` reports one that names no skill.

use std::path::Path;

use mdstore::snapshot::{DocId, DocumentSource, Entry, Snapshot};
use mdstore::store::{FetchingLocator, LocalPaths, StoreContent, StoreGraph, StoreRef};

use crate::error::Error;

/// A skill as it was loaded from a library.
#[derive(Debug, Clone)]
pub struct SkillDoc {
    pub name: String,
    pub description: String,
    /// The skills this one needs, as written.
    pub requires: Vec<String>,
    /// The path of the SKILL.md inside its library.
    pub rel_path: String,
}

/// The frontmatter almanac reads. Keys it does not model are kept.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Frontmatter {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    requires: Vec<String>,
    #[serde(flatten)]
    extra: yaml_serde::Mapping,
}

/// Reads the skills of one library.
pub struct SkillSource;

impl DocumentSource for SkillSource {
    type Doc = SkillDoc;

    fn load(
        &self,
        content: &StoreContent,
        skipped: &mut Vec<String>,
    ) -> mdstore::Result<Vec<Entry<SkillDoc>>> {
        // A library holds one directory for each skill, and the SKILL.md
        // sits inside it.
        let dir_name = library_dir_of(content);
        let mut skills = Vec::new();
        for skill_dir in subdirectories(content, &dir_name) {
            let rel = format!("{dir_name}/{skill_dir}/SKILL.md");
            if !content.exists(&rel) {
                continue;
            }
            // One unreadable skill never takes down a library. A bare
            // `?` here is caught one level up, where the whole member
            // becomes an empty document list, so a served library
            // answered `[]` for one stray byte.
            let text = match content.read(&rel) {
                Ok(t) => t,
                Err(e) => {
                    skipped.push(format!("{rel} ({e})"));
                    continue;
                }
            };
            match mdstore::document::parse::<Frontmatter>(&text) {
                Ok(doc) => {
                    let fm = doc.frontmatter;
                    let name = fm.name.unwrap_or_else(|| skill_dir.clone());
                    // The name identifies the skill everywhere: it is
                    // the id a tool asks for, the authority of a
                    // skill:// URI, and the directory a vendored copy
                    // lives in. A published library supplies it, so a
                    // name holding a separator would collide with
                    // another skill's file URIs.
                    if !mdstore::is_plain_stem(&name) {
                        skipped.push(format!("{rel} (the name `{name}` is not a plain name)"));
                        continue;
                    }
                    skills.push(Entry {
                        id: name.clone(),
                        doc: SkillDoc {
                            name,
                            description: fm.description.unwrap_or_default(),
                            requires: fm.requires,
                            rel_path: rel,
                        },
                    });
                }
                Err(e) => skipped.push(format!("{rel} ({e})")),
            }
        }
        skills.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(skills)
    }

    /// The skills this one needs. Each is parsed against the aliases
    /// that the library holding this skill declares.
    fn references(&self, doc: &SkillDoc, member: usize, graph: &StoreGraph) -> Vec<StoreRef> {
        let aliases = graph.config(member).aliases();
        doc.requires
            .iter()
            .map(|r| StoreRef::parse(r, &aliases))
            .collect()
    }

    fn resolve_local(&self, id: &str, entries: &[Entry<SkillDoc>]) -> Option<usize> {
        entries.iter().position(|e| e.id == id)
    }
}

/// The library directory a store's manifest names, or the default.
///
/// A dependency's manifest is content somebody else controls, so the
/// value it names must stay inside that store. Without the guard, a
/// dependency saying `library: /somewhere` has its skills listed, read,
/// and served from a directory that has nothing to do with it.
fn library_dir_of(content: &StoreContent) -> String {
    let configured = if content.exists("almanac.yml")
        && let Ok(text) = content.read("almanac.yml")
        && let Ok(value) = yaml_serde::from_str::<yaml_serde::Value>(&text)
        && let Some(dir) = value.get("library").and_then(|d| d.as_str())
    {
        dir.to_string()
    } else {
        return "skills".to_string();
    };

    match content.dir() {
        // A directory on this machine: resolve it and check that it
        // stays inside the store.
        Some(root) => match mdstore::store::document_dir(root, &configured) {
            Ok(_) => configured,
            Err(_) => "skills".to_string(),
        },
        // A git tree holds no symlink to follow, so the text alone
        // decides. A value that climbs out names nothing in the tree.
        None => {
            if configured.starts_with('/') || configured.contains("..") {
                "skills".to_string()
            } else {
                configured
            }
        }
    }
}

/// The subdirectories of a library directory.
///
/// One implementation answers for a local directory and for a git
/// tree. This held a third copy of it, and a copy is a copy that can
/// miss the link test.
fn subdirectories(content: &StoreContent, dir_name: &str) -> Vec<String> {
    content.subdirectories(dir_name)
}

/// A skill as seen from this library.
pub struct View<'a> {
    pub id: DocId,
    pub skill: &'a SkillDoc,
    /// The library this skill came from: empty for this one.
    pub store: String,
    /// True when a nearer library holds the same name.
    pub shadowed: bool,
}

/// One problem that `check` reports.
#[derive(Debug, serde::Serialize)]
pub struct Finding {
    /// What holds the problem: a skill name, an alias, or a library.
    pub subject: String,
    /// What is wrong with it.
    pub detail: String,
    /// The class of problem, for a caller that groups them.
    pub kind: String,
}

/// One library in the closure.
#[derive(Debug, serde::Serialize)]
pub struct StoreRow {
    pub alias: String,
    pub source: String,
    pub skills: usize,
    pub unavailable: Option<String>,
    pub age: Option<String>,
}

/// A loaded set of libraries.
pub struct Workspace {
    snapshot: Snapshot<SkillSource>,
}

impl Workspace {
    pub fn open(root: &Path) -> Result<Self, Error> {
        Self::open_with(root, false)
    }

    pub fn open_fetching(root: &Path) -> Result<Self, Error> {
        Self::open_with(root, true)
    }

    fn open_with(root: &Path, fetching: bool) -> Result<Self, Error> {
        let registry = mdstore::registry::Registry::load().unwrap_or_default();
        let graph = if fetching {
            StoreGraph::open(
                root,
                &mdstore::registry::RegistryLocator::new(registry, FetchingLocator),
            )
        } else {
            StoreGraph::open(
                root,
                &mdstore::registry::RegistryLocator::new(registry, LocalPaths),
            )
        }
        .map_err(|e| Error::General(e.to_string()))?;
        let snapshot =
            Snapshot::load(graph, &SkillSource).map_err(|e| Error::General(e.to_string()))?;
        Ok(Self { snapshot })
    }

    /// True when this library declares no other library.
    #[must_use]
    pub const fn is_single_store(&self) -> bool {
        self.snapshot.graph.members.len() == 1
    }

    /// Every skill that a command can reach, in precedence order, with
    /// the shadowed duplicates marked.
    ///
    /// The nearer library wins a name. Members are ordered breadth-first
    /// in declaration order, so the winner is the first one seen.
    #[must_use]
    pub fn skills(&self) -> Vec<View<'_>> {
        let mut seen: Vec<&str> = Vec::new();
        let mut views = Vec::new();
        for (id, entry) in self.snapshot.documents() {
            let shadowed = seen.contains(&entry.id.as_str());
            if !shadowed {
                seen.push(&entry.id);
            }
            views.push(View {
                id,
                skill: &entry.doc,
                store: self.snapshot.graph.members[id.member].alias_path.join("/"),
                shadowed,
            });
        }
        views
    }

    /// The skill a name resolves to, after precedence.
    #[must_use]
    pub fn resolve(&self, name: &str) -> Option<View<'_>> {
        self.skills()
            .into_iter()
            .find(|v| !v.shadowed && v.skill.name == name)
    }

    /// Names that more than one library holds, with the library that
    /// wins and the ones that lose.
    #[must_use]
    pub fn shadowed(&self) -> Vec<(String, String, Vec<String>)> {
        let views = self.skills();
        let mut out: Vec<(String, String, Vec<String>)> = Vec::new();
        for view in views.iter().filter(|v| v.shadowed) {
            let winner = views
                .iter()
                .find(|v| !v.shadowed && v.skill.name == view.skill.name)
                .map(|v| label(&v.store))
                .unwrap_or_default();
            match out.iter_mut().find(|(n, _, _)| n == &view.skill.name) {
                Some((_, _, losers)) => losers.push(label(&view.store)),
                None => out.push((view.skill.name.clone(), winner, vec![label(&view.store)])),
            }
        }
        out
    }

    /// `requires:` entries that name no skill, with the skill that
    /// declared them.
    #[must_use]
    pub fn dangling_requires(&self) -> Vec<(String, String)> {
        self.snapshot
            .dangling
            .iter()
            .filter_map(|(from, r)| {
                self.snapshot
                    .get(*from)
                    .map(|_| (self.snapshot.qualify(*from), r.to_string()))
            })
            .collect()
    }

    /// Libraries that could not be read.
    #[must_use]
    pub fn missing(&self) -> Vec<String> {
        self.snapshot.missing()
    }

    /// Files the load skipped.
    #[must_use]
    pub fn skipped(&self) -> &[String] {
        &self.snapshot.skipped
    }

    /// Declarations that other clones could not follow.
    #[must_use]
    pub fn unshareable(&self, root: &Path) -> Vec<(String, String)> {
        self.snapshot.graph.config(0).unshareable(root)
    }

    /// The directory each declared library keeps its skills in, in
    /// precedence order, for a caller that scans directories.
    ///
    /// The value comes from the guarded resolution, so a dependency
    /// that names a directory outside itself is not returned.
    #[must_use]
    pub fn declared_library_dirs(&self) -> Vec<String> {
        self.snapshot
            .graph
            .members
            .iter()
            .skip(1)
            .filter_map(|m| {
                let content = m.content.as_ref()?;
                let root = content.dir()?;
                let dir_name = library_dir_of(content);
                let joined = mdstore::store::document_dir(root, &dir_name).ok()?;
                joined
                    .is_dir()
                    .then(|| joined.to_string_lossy().into_owned())
            })
            .collect()
    }

    /// One row for each library in the closure.
    #[must_use]
    pub fn store_members(&self) -> Vec<StoreRow> {
        self.snapshot
            .graph
            .members
            .iter()
            .enumerate()
            .map(|(i, m)| StoreRow {
                alias: m.alias_path.join("/"),
                source: match &m.source {
                    mdstore::StoreSource::Path(p) => p.display().to_string(),
                    mdstore::StoreSource::Git { url, rev } => rev
                        .as_ref()
                        .map_or_else(|| url.clone(), |r| format!("{url}@{r}")),
                    mdstore::StoreSource::Blob { url } => url.clone(),
                },
                skills: self.snapshot.member_documents(i).count(),
                unavailable: m.unavailable.clone(),
                age: m
                    .content
                    .as_ref()
                    .and_then(mdstore::StoreContent::fetch_age),
            })
            .collect()
    }

    /// Fetch each declared remote library.
    #[must_use]
    pub fn sync_all(&self) -> Vec<(String, Result<(), Error>)> {
        let mut results = Vec::new();
        for (i, member) in self.snapshot.graph.members.iter().enumerate() {
            if i == 0 || matches!(member.source, mdstore::StoreSource::Path(_)) {
                continue;
            }
            results.push((
                member.alias_path.join("/"),
                mdstore::store::sync_source(&member.source)
                    .map_err(|e| Error::General(e.to_string())),
            ));
        }
        results
    }

    /// Every problem in the closure, as one list.
    ///
    /// This is the whole of `check`. Each interface presents the same
    /// findings, so the CLI and a server cannot drift apart.
    #[must_use]
    pub fn check(&self, root: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();

        // A requirement that names no skill.
        for (skill, target) in self.dangling_requires() {
            findings.push(Finding {
                subject: skill,
                detail: format!("requires {target}, which names no skill"),
                kind: "requirement".to_string(),
            });
        }
        // A name that two libraries hold. The nearer one wins, and the
        // loser is invisible unless this says so.
        for (name, winner, losers) in self.shadowed() {
            findings.push(Finding {
                subject: name,
                detail: format!("comes from {winner}; also in {}", losers.join(", ")),
                kind: "shadowed".to_string(),
            });
        }
        for alias in self.missing() {
            findings.push(Finding {
                subject: alias,
                detail: "is not available".to_string(),
                kind: "unreachable library".to_string(),
            });
        }
        for (alias, why) in self.unshareable(root) {
            findings.push(Finding {
                subject: alias,
                detail: why,
                kind: "declaration".to_string(),
            });
        }
        for skipped in self.skipped() {
            findings.push(Finding {
                subject: "library".to_string(),
                detail: format!("skipped {skipped}"),
                kind: "scan".to_string(),
            });
        }
        // A declaration the walk could not follow at all. Without this
        // the closure was silently shorter than the config asked for.
        for finding in &self.snapshot.graph.findings {
            findings.push(Finding {
                subject: "stores.yml".to_string(),
                detail: finding.clone(),
                kind: "declaration".to_string(),
            });
        }
        // A file beside a SKILL.md that could not be read. It is absent
        // from the published digests, and a host that verifies would
        // otherwise never learn it was dropped.
        for view in self.skills().iter().filter(|v| !v.shadowed) {
            for problem in self.skill_file_problems(view) {
                findings.push(Finding {
                    subject: view.skill.name.clone(),
                    detail: problem,
                    kind: "unreadable file".to_string(),
                });
            }
        }
        findings
    }

    /// Files beside a skill's SKILL.md that could not be read.
    #[must_use]
    pub fn skill_file_problems(&self, view: &View<'_>) -> Vec<String> {
        let member = &self.snapshot.graph.members[view.id.member];
        let Some(mdstore::StoreContent::Dir(root)) = member.content.as_ref() else {
            return Vec::new();
        };
        let Some(parent) = std::path::Path::new(&view.skill.rel_path).parent() else {
            return Vec::new();
        };
        let skill_dir = root.root().join(parent);
        let mut problems = Vec::new();
        collect_problems(&skill_dir, &skill_dir, &mut problems);
        problems.sort();
        problems
    }

    /// The files beside a skill's SKILL.md, each with its bytes.
    ///
    /// They are resolved against the library that holds the skill, not
    /// against this one. Resolving against this library read a
    /// different library's files, or none, and the digest then covered
    /// bytes that the skill does not contain.
    #[must_use]
    pub fn skill_files(&self, view: &View<'_>) -> Vec<(String, Vec<u8>)> {
        let member = &self.snapshot.graph.members[view.id.member];
        let Some(content) = member.content.as_ref() else {
            return Vec::new();
        };
        let Some(parent) = std::path::Path::new(&view.skill.rel_path).parent() else {
            return Vec::new();
        };
        let mut out = Vec::new();
        match content {
            StoreContent::Dir(root) => {
                let skill_dir = root.root().join(parent);
                collect_files(&skill_dir, &skill_dir, &mut out);
            }
            StoreContent::GitTree { cache, rev, .. } => {
                let prefix = parent.to_string_lossy().to_string();
                if let Ok(paths) = mdstore::git::list_tree(cache, rev, &prefix) {
                    for name in paths {
                        if name == "SKILL.md" || name.starts_with('.') {
                            continue;
                        }
                        let full = format!("{prefix}/{name}");
                        if let Ok(text) = mdstore::git::show(cache, rev, &full) {
                            out.push((name, text.into_bytes()));
                        }
                    }
                }
            }
        }
        out.sort_by(|a, b| a.0.cmp(&b.0));
        out
    }

    /// The full text of a skill, from whichever library won its name.
    pub fn read_skill(&self, view: &View<'_>) -> Result<String, Error> {
        let member = &self.snapshot.graph.members[view.id.member];
        let content = member
            .content
            .as_ref()
            .ok_or_else(|| Error::General(format!("library '{}' is not available", view.store)))?;
        content
            .read(&view.skill.rel_path)
            .map_err(|e| Error::General(e.to_string()))
    }
}

/// Walk a skill directory, skipping links and dot entries.
fn collect_files(base: &std::path::Path, dir: &std::path::Path, out: &mut Vec<(String, Vec<u8>)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        // A library is content somebody else may control, so a link is
        // skipped rather than followed.
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            collect_files(base, &path, out);
            continue;
        }
        let Ok(rel) = path.strip_prefix(base) else {
            continue;
        };
        let rel = rel.to_string_lossy().to_string();
        if rel == "SKILL.md" || rel.starts_with('.') {
            continue;
        }
        // A regular file, or nothing. A library is content somebody
        // else may control, and opening a named pipe to read it blocks
        // until a writer arrives. None is coming, so the walk never
        // returns, and this walk backs the served library: an agent's
        // tool call hangs forever.
        //
        // The dirent type answers without opening anything, so the
        // pipe is never touched.
        if !file_type.is_file() {
            continue;
        }
        if let Ok(bytes) = std::fs::read(&path) {
            out.push((rel, bytes));
        }
    }
}

/// Walk a skill directory and report what could not be read.
fn collect_problems(base: &std::path::Path, dir: &std::path::Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        out.push(format!("{} is not readable", dir.display()));
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_symlink() {
            let rel = path.strip_prefix(base).unwrap_or(&path);
            out.push(format!(
                "{} is a symlink and is not published",
                rel.display()
            ));
            continue;
        }
        if file_type.is_dir() {
            collect_problems(base, &path, out);
            continue;
        }
        let rel = path.strip_prefix(base).unwrap_or(&path);
        let rel = rel.to_string_lossy().to_string();
        if rel == "SKILL.md" || rel.starts_with('.') {
            continue;
        }
        // Named by type rather than opened. This is the diagnostic
        // command, so it must survive what it reports: reading a named
        // pipe here blocked forever, and check is exactly where a
        // person looks when a library behaves strangely.
        if !file_type.is_file() {
            out.push(format!("{rel} is not a regular file and is not published"));
            continue;
        }
        if std::fs::read(&path).is_err() {
            out.push(format!("{rel} is not readable and is not published"));
        }
    }
}

fn label(store: &str) -> String {
    if store.is_empty() {
        "(this library)".to_string()
    } else {
        store.to_string()
    }
}

#[cfg(test)]
mod walker_tests {
    use super::*;

    /// A named pipe must not stop the walk.
    ///
    /// Opening a pipe to read blocks until a writer arrives, and none
    /// is coming. Both walkers skipped a link by type and then read
    /// whatever was left, so a pipe in a skill directory hung the CLI
    /// and the served library: an agent's tool call never returned.
    ///
    /// The walk runs on its own thread, because a regression here
    /// would otherwise hang the suite rather than fail it.
    #[test]
    fn a_named_pipe_does_not_stop_either_walk() {
        let base = std::env::temp_dir().join(format!("almanac-pipe-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(base.join("references")).unwrap();
        std::fs::write(base.join("references/real.md"), "ref").unwrap();
        let made = std::process::Command::new("mkfifo")
            .arg(base.join("references/pipe.md"))
            .status()
            .is_ok_and(|s| s.success());
        assert!(made, "mkfifo is unavailable, so this test asserts nothing");

        let dir = base.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut files = Vec::new();
            collect_files(&dir, &dir, &mut files);
            let mut problems = Vec::new();
            collect_problems(&dir, &dir, &mut problems);
            let _ = tx.send((files, problems));
        });

        let Ok((files, problems)) = rx.recv_timeout(std::time::Duration::from_secs(5)) else {
            panic!("a walk blocked on a named pipe");
        };

        // Every other reader that walks the same directory. The first
        // fix closed two doors of six, and the four below hung on this
        // exact fixture one command later: status and sync through the
        // hash, the red-flag scan, the vendor copy, and a dev source's
        // own SKILL.md.
        let (tx2, rx2) = std::sync::mpsc::channel();
        let dir2 = base.clone();
        // The destination must sit OUTSIDE the tree being copied. A
        // destination inside it made copy_tree recurse into its own
        // output and fail on that, before it reached the pipe, so the
        // copy assertion passed with the guard removed.
        let dst = std::env::temp_dir().join(format!("almanac-pipe-copy-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dst);
        std::thread::spawn(move || {
            let hashed = crate::hash::hash_tree(&dir2).is_err();
            let flags = crate::flags::scan(&dir2);
            let copied = crate::vendor::copy_tree(&dir2, &dst).is_err();
            let _ = tx2.send((hashed, flags, copied));
        });
        let Ok((hash_refused, flags, copy_refused)) =
            rx2.recv_timeout(std::time::Duration::from_secs(5))
        else {
            panic!("the hash, the red-flag scan or the vendor copy blocked on a named pipe");
        };
        assert!(hash_refused, "the hash accepted a pipe as content");
        assert!(copy_refused, "the vendor copy accepted a pipe as content");
        // The scanner names the pipe, and still runs its path-only
        // checks on it. A guard at the walk dropped both, so a pipe
        // named payload.sh at a skill root scanned as clean.
        assert!(
            flags
                .iter()
                .any(|f| f.path.contains("pipe.md") && f.what.contains("not a regular file")),
            "the scanner did not name the pipe: {flags:?}"
        );

        let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
        assert!(
            names.contains(&"references/real.md"),
            "the real file was dropped"
        );
        assert!(
            !names.iter().any(|n| n.contains("pipe")),
            "a pipe was published"
        );
        assert!(
            problems
                .iter()
                .any(|p| p.contains("pipe.md") && p.contains("not a regular file")),
            "check did not name the pipe: {problems:?}"
        );

        let _ = std::fs::remove_dir_all(&base);
    }
}

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
    extra: serde_yml::Mapping,
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
            let text = content.read(&rel)?;
            match mdstore::document::parse::<Frontmatter>(&text) {
                Ok(doc) => {
                    let fm = doc.frontmatter;
                    let name = fm.name.unwrap_or_else(|| skill_dir.clone());
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
fn library_dir_of(content: &StoreContent) -> String {
    if content.exists("almanac.yml")
        && let Ok(text) = content.read("almanac.yml")
        && let Ok(value) = serde_yml::from_str::<serde_yml::Value>(&text)
        && let Some(dir) = value.get("library").and_then(|d| d.as_str())
    {
        return dir.to_string();
    }
    "skills".to_string()
}

/// The subdirectories of a library directory.
fn subdirectories(content: &StoreContent, dir_name: &str) -> Vec<String> {
    if let Some(root) = content.dir() {
        let Ok(entries) = std::fs::read_dir(root.join(dir_name)) else {
            return Vec::new();
        };
        let mut names: Vec<String> = entries
            .flatten()
            .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
            .filter_map(|e| e.file_name().to_str().map(std::string::ToString::to_string))
            .filter(|n| !n.starts_with('.'))
            .collect();
        names.sort();
        return names;
    }
    let mut names: Vec<String> = Vec::new();
    if let StoreContent::GitTree { cache, rev, .. } = content
        && let Ok(paths) = mdstore::git::list_tree(cache, rev, dir_name)
    {
        for path in paths {
            if let Some((skill, _)) = path.split_once('/')
                && !skill.starts_with('.')
                && !names.contains(&skill.to_string())
            {
                names.push(skill.to_string());
            }
        }
    }
    names.sort();
    names
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
                    mdstore::StoreSource::Git { url, rev } => {
                        rev.as_ref().map_or_else(|| url.clone(), |r| format!("{url}@{r}"))
                    }
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

fn label(store: &str) -> String {
    if store.is_empty() {
        "(this library)".to_string()
    } else {
        store.to_string()
    }
}

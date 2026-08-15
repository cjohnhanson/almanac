//! The almanac manifest: `almanac.yml`, next to the library directory
//! it governs.
//!
//! The manifest and the vendored content together are the reproducible
//! record. Both live in the same repo, so the manifest binds the
//! content against upstream drift and accident. It does not protect
//! against a compromised library repo. The docs state this limit
//! plainly.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const MANIFEST_NAME: &str = "almanac.yml";
/// Stamp file that almanac writes inside every vendored directory. It
/// marks the directory as managed. The hash excludes it. `remove`
/// refuses a directory without it.
pub const ORIGIN_STAMP: &str = ".almanac-origin";

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    /// Library directory, relative to the manifest's own directory.
    pub library: String,
    #[serde(default)]
    pub skills: Vec<Entry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// Names the vendored directory, and addresses the entry on the CLI.
    /// It must match the SKILL.md frontmatter name. `add` refuses a
    /// mismatch.
    pub name: String,
    /// `github:owner/repo`, `git:<url>`, or `dev:<path>`.
    pub source: String,
    /// Skill directory within the source (git sources).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Branch or tag that the pin came from (git sources). `update`
    /// resolves it again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    /// Full commit sha (git sources). Absent for a dev: snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    /// `sha256-v1:` content hash of the vendored tree.
    pub sha256: String,
}

impl Entry {
    /// A `dev:` entry is a snapshot outside the reproducibility
    /// contract. `sync --check` skips it, and it reports drift as
    /// information, not as a failure.
    #[must_use]
    pub fn is_dev(&self) -> bool {
        self.source.starts_with("dev:")
    }
}

#[derive(Debug)]
pub enum ManifestError {
    NotFound(PathBuf),
    Invalid(String),
    Io(String),
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(p) => write!(f, "no {} at {}", MANIFEST_NAME, p.display()),
            Self::Invalid(e) => write!(f, "invalid manifest: {e}"),
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

impl Manifest {
    #[must_use]
    pub fn new(library: &str) -> Self {
        Self {
            library: library.to_string(),
            skills: Vec::new(),
        }
    }

    pub fn load(dir: &Path) -> Result<Self, ManifestError> {
        let path = dir.join(MANIFEST_NAME);
        let bytes = std::fs::read_to_string(&path).map_err(|_| ManifestError::NotFound(path))?;
        yaml_serde::from_str(&bytes).map_err(|e| ManifestError::Invalid(e.to_string()))
    }

    pub fn save(&self, dir: &Path) -> Result<(), ManifestError> {
        let rendered =
            yaml_serde::to_string(self).map_err(|e| ManifestError::Invalid(e.to_string()))?;
        let path = dir.join(MANIFEST_NAME);
        let tmp = dir.join(format!("{MANIFEST_NAME}.tmp"));
        std::fs::write(&tmp, rendered).map_err(|e| ManifestError::Io(e.to_string()))?;
        std::fs::rename(&tmp, &path).map_err(|e| ManifestError::Io(e.to_string()))
    }

    #[must_use]
    pub fn library_dir(&self, manifest_dir: &Path) -> PathBuf {
        manifest_dir.join(&self.library)
    }

    #[must_use]
    pub fn entry(&self, name: &str) -> Option<&Entry> {
        self.skills.iter().find(|e| e.name == name)
    }

    pub fn upsert(&mut self, entry: Entry) {
        if let Some(existing) = self.skills.iter_mut().find(|e| e.name == entry.name) {
            *existing = entry;
        } else {
            self.skills.push(entry);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_and_omits_absent_fields() {
        let dir = std::env::temp_dir().join(format!("almanac-manifest-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        let mut m = Manifest::new("skills");
        m.upsert(Entry {
            name: "gaff".into(),
            source: "github:cjohnhanson/gaff".into(),
            path: Some("skills/gaff".into()),
            r#ref: Some("main".into()),
            rev: Some("abc".into()),
            sha256: "sha256-v1:00".into(),
        });
        m.upsert(Entry {
            name: "local".into(),
            source: "dev:../x".into(),
            path: None,
            r#ref: None,
            rev: None,
            sha256: "sha256-v1:11".into(),
        });
        m.save(&dir).unwrap();
        let text = std::fs::read_to_string(dir.join(MANIFEST_NAME)).unwrap();
        assert!(
            !text.contains("rev: null"),
            "absent fields omitted:\n{text}"
        );
        let loaded = Manifest::load(&dir).unwrap();
        assert_eq!(loaded.skills.len(), 2);
        assert!(loaded.entry("local").unwrap().is_dev());
        assert!(!loaded.entry("gaff").unwrap().is_dev());
    }

    #[test]
    fn upsert_replaces_by_name() {
        let mut m = Manifest::new("skills");
        for sha in ["sha256-v1:aa", "sha256-v1:bb"] {
            m.upsert(Entry {
                name: "x".into(),
                source: "dev:.".into(),
                path: None,
                r#ref: None,
                rev: None,
                sha256: sha.into(),
            });
        }
        assert_eq!(m.skills.len(), 1);
        assert_eq!(m.entry("x").unwrap().sha256, "sha256-v1:bb");
    }
}

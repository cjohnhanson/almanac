//! Content hashing for vendored skill directories.
//!
//! Versioned, framed, filesystem-independent:
//!
//! - Entries sorted by raw path bytes; paths are `/`-separated relative
//!   paths, NFC-normalized (macOS git precomposes; Linux does not —
//!   without normalization the same tree hashes differently by OS).
//! - Each entry framed as `u64_le(len(path)) ‖ path ‖ kind ‖
//!   u64_le(len(payload)) ‖ payload`, where kind is `f` (regular),
//!   `x` (executable), or `l` (symlink, payload = link target). Framing
//!   kills concatenation ambiguity (`ab`+`c` vs `a`+`bc`).
//! - Deny-listed names are excluded everywhere: `.DS_Store`, `.git`,
//!   `__pycache__`, and almanac's own `.almanac-origin` stamp.
//! - Empty directories contribute nothing and are not carried.
//! - Stored form is prefixed `sha256-v1:` so the scheme can evolve.
//!
//! Hard errors, not silent lossiness: paths that collide
//! case-insensitively (this machine's APFS is case-insensitive — such a
//! tree cannot round-trip here) and symlinks escaping the root.

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;

pub const HASH_PREFIX: &str = "sha256-v1:";
pub const DENY_LIST: [&str; 4] = [".DS_Store", ".git", "__pycache__", ".almanac-origin"];

#[derive(Debug, PartialEq, Eq)]
pub enum HashError {
    CaseCollision(String, String),
    EscapingSymlink(String),
    Unreadable(String),
}

impl std::fmt::Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CaseCollision(a, b) => write!(
                f,
                "paths `{a}` and `{b}` collide case-insensitively; this tree cannot round-trip on case-insensitive filesystems"
            ),
            Self::EscapingSymlink(p) => write!(f, "symlink `{p}` escapes the skill root"),
            Self::Unreadable(p) => write!(f, "cannot read `{p}`"),
        }
    }
}

#[must_use]
pub fn denied(name: &str) -> bool {
    DENY_LIST.contains(&name)
}

enum Kind {
    Regular,
    Executable,
    Symlink(String),
}

/// Hash a directory tree under the v1 scheme.
pub fn hash_tree(root: &Path) -> Result<String, HashError> {
    let mut entries: Vec<(String, Kind, PathBuf)> = Vec::new();
    collect(root, root, &mut entries)?;
    entries.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));

    let mut lower_seen: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    for (path, _, _) in &entries {
        if let Some(prior) = lower_seen.insert(path.to_lowercase(), path.clone())
            && &prior != path {
                return Err(HashError::CaseCollision(prior, path.clone()));
            }
    }

    let mut hasher = Sha256::new();
    for (path, kind, abs) in entries {
        hasher.update((path.len() as u64).to_le_bytes());
        hasher.update(path.as_bytes());
        match kind {
            Kind::Regular => {
                hasher.update(b"f");
                let bytes = std::fs::read(&abs).map_err(|_| HashError::Unreadable(path.clone()))?;
                hasher.update((bytes.len() as u64).to_le_bytes());
                hasher.update(&bytes);
            }
            Kind::Executable => {
                hasher.update(b"x");
                let bytes = std::fs::read(&abs).map_err(|_| HashError::Unreadable(path.clone()))?;
                hasher.update((bytes.len() as u64).to_le_bytes());
                hasher.update(&bytes);
            }
            Kind::Symlink(target) => {
                hasher.update(b"l");
                hasher.update((target.len() as u64).to_le_bytes());
                hasher.update(target.as_bytes());
            }
        }
    }
    let digest = hasher.finalize();
    let hex = digest.iter().fold(
        String::with_capacity(digest.len() * 2),
        |mut acc, b| {
            use std::fmt::Write as _;
            let _ = write!(acc, "{b:02x}");
            acc
        },
    );
    Ok(format!("{HASH_PREFIX}{hex}"))
}

fn collect(
    root: &Path,
    dir: &Path,
    out: &mut Vec<(String, Kind, PathBuf)>,
) -> Result<(), HashError> {
    let entries =
        std::fs::read_dir(dir).map_err(|_| HashError::Unreadable(dir.display().to_string()))?;
    for entry in entries.filter_map(Result::ok) {
        let p = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if denied(&name) {
            continue;
        }
        let meta = std::fs::symlink_metadata(&p)
            .map_err(|_| HashError::Unreadable(p.display().to_string()))?;
        let rel: String = p
            .strip_prefix(root)
            .expect("children live under root")
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/")
            .nfc()
            .collect();
        if meta.file_type().is_symlink() {
            let target = std::fs::read_link(&p).map_err(|_| HashError::Unreadable(rel.clone()))?;
            let resolved = if target.is_absolute() {
                target.clone()
            } else {
                p.parent().expect("has parent").join(&target)
            };
            let canonical_root = root
                .canonicalize()
                .map_err(|_| HashError::Unreadable(root.display().to_string()))?;
            let escaped = resolved
                .canonicalize()
                .map_or(true, |r| !r.starts_with(&canonical_root));
            if escaped {
                return Err(HashError::EscapingSymlink(rel));
            }
            out.push((rel, Kind::Symlink(target.to_string_lossy().into_owned()), p));
        } else if meta.is_dir() {
            collect(root, &p, out)?;
        } else {
            #[cfg(unix)]
            let exec = {
                use std::os::unix::fs::PermissionsExt;
                meta.permissions().mode() & 0o111 != 0
            };
            #[cfg(not(unix))]
            let exec = false;
            out.push((
                rel,
                if exec {
                    Kind::Executable
                } else {
                    Kind::Regular
                },
                p,
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("almanac-hash-{tag}-{}", std::process::id()));
        std::fs::remove_dir_all(&d).ok();
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn framing_distinguishes_path_content_splits() {
        let a = tree("frame-a");
        std::fs::write(a.join("ab"), "c").unwrap();
        let b = tree("frame-b");
        std::fs::write(b.join("a"), "bc").unwrap();
        assert_ne!(hash_tree(&a).unwrap(), hash_tree(&b).unwrap());
    }

    #[test]
    fn deny_list_and_stamp_do_not_affect_hash() {
        let a = tree("deny-a");
        std::fs::write(a.join("SKILL.md"), "x").unwrap();
        let h1 = hash_tree(&a).unwrap();
        std::fs::write(a.join(".DS_Store"), "junk").unwrap();
        std::fs::write(a.join(".almanac-origin"), "stamp").unwrap();
        assert_eq!(hash_tree(&a).unwrap(), h1);
        assert!(h1.starts_with(HASH_PREFIX));
    }

    #[test]
    fn executable_bit_changes_hash() {
        let a = tree("exec");
        std::fs::write(a.join("run.sh"), "#!/bin/sh\n").unwrap();
        let h1 = hash_tree(&a).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(a.join("run.sh"), std::fs::Permissions::from_mode(0o755))
                .unwrap();
            assert_ne!(hash_tree(&a).unwrap(), h1);
        }
    }

    #[test]
    fn escaping_symlink_is_refused_and_internal_allowed() {
        let a = tree("links");
        std::fs::write(a.join("SKILL.md"), "x").unwrap();
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("SKILL.md", a.join("inner")).unwrap();
            assert!(hash_tree(&a).is_ok());
            std::os::unix::fs::symlink("/etc/passwd", a.join("outer")).unwrap();
            assert!(matches!(hash_tree(&a), Err(HashError::EscapingSymlink(_))));
        }
    }

    #[test]
    fn stable_across_creation_order() {
        let a = tree("order-a");
        std::fs::write(a.join("z.md"), "1").unwrap();
        std::fs::write(a.join("a.md"), "2").unwrap();
        let b = tree("order-b");
        std::fs::write(b.join("a.md"), "2").unwrap();
        std::fs::write(b.join("z.md"), "1").unwrap();
        assert_eq!(hash_tree(&a).unwrap(), hash_tree(&b).unwrap());
    }
}

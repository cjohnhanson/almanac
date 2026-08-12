//! Fetching sources and vendoring skill trees into the library.
//!
//! Git operations shell out to the git CLI. Fetch strategy for a pinned
//! rev: depth-1 SHA fetch (GitHub allows it), then the recorded ref with
//! a full fetch, then a full clone — the path taken is reported. Copies
//! honor the hash deny-list, refuse escaping symlinks, and stamp the
//! vendored dir with `.almanac-origin` so the managed set is explicit.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::hash::{denied, hash_tree, HashError};
use crate::manifest::{Entry, ORIGIN_STAMP};

#[derive(Debug)]
pub enum VendorError {
    Git(String),
    BadSource(String),
    Hash(HashError),
    Io(String),
}

impl std::fmt::Display for VendorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Git(e) => write!(f, "git: {e}"),
            Self::BadSource(s) => write!(
                f,
                "unrecognized source `{s}` (expected github:owner/repo, git:<url>, or dev:<path>)"
            ),
            Self::Hash(e) => write!(f, "{e}"),
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

/// A source's git URL, or the local dir for dev sources.
pub enum Located {
    Git { url: String },
    Dev { dir: PathBuf },
}

pub fn locate(source: &str) -> Result<Located, VendorError> {
    if let Some(repo) = source.strip_prefix("github:") {
        return Ok(Located::Git {
            url: format!("https://github.com/{repo}"),
        });
    }
    if let Some(url) = source.strip_prefix("git:") {
        return Ok(Located::Git {
            url: url.to_string(),
        });
    }
    if let Some(dir) = source.strip_prefix("dev:") {
        return Ok(Located::Dev {
            dir: PathBuf::from(dir),
        });
    }
    if source.split('/').count() == 2 && !source.contains(':') {
        // npx-skills-style shorthand: owner/repo
        return Ok(Located::Git {
            url: format!("https://github.com/{source}"),
        });
    }
    Err(VendorError::BadSource(source.to_string()))
}

fn git(dir: &Path, args: &[&str]) -> Result<String, VendorError> {
    let out = Command::new("git")
        .current_dir(dir)
        .env("GIT_TERMINAL_PROMPT", "0")
        .args(args)
        .output()
        .map_err(|e| VendorError::Git(e.to_string()))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(VendorError::Git(format!(
            "`git {}`: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        )))
    }
}

/// Fetch `rev` from `url` into a temp checkout. Returns (checkout dir,
/// fetch path taken). The temp dir must outlive use of the path.
pub fn fetch_rev(
    url: &str,
    rev: &str,
    r#ref: Option<&str>,
) -> Result<(tempdir::TempDirHandle, &'static str), VendorError> {
    let tmp = tempdir::create("almanac-fetch")?;
    git(&tmp.path, &["init", "-q"])?;
    git(&tmp.path, &["remote", "add", "origin", url])?;

    if git(&tmp.path, &["fetch", "-q", "--depth", "1", "origin", rev]).is_ok() {
        git(&tmp.path, &["checkout", "-q", "FETCH_HEAD"])?;
        return Ok((tmp, "sha-fetch"));
    }
    if let Some(r) = r#ref
        && git(&tmp.path, &["fetch", "-q", "origin", r]).is_ok()
            && git(&tmp.path, &["checkout", "-q", rev]).is_ok()
        {
            return Ok((tmp, "ref-fetch"));
        }
    git(&tmp.path, &["fetch", "-q", "origin"])?;
    git(&tmp.path, &["checkout", "-q", rev])?;
    Ok((tmp, "full-fetch"))
}

/// Resolve a ref (or the remote HEAD) to a commit sha. Hard error when
/// nothing resolves — never proceed against an unknown base.
pub fn resolve_remote(url: &str, r#ref: Option<&str>) -> Result<(String, String), VendorError> {
    let tmp = tempdir::create("almanac-resolve")?;
    if let Some(r) = r#ref {
        let out = git(&tmp.path, &["ls-remote", url, r])?;
        if let Some(sha) = out.split_whitespace().next().filter(|s| !s.is_empty()) {
            return Ok((sha.to_string(), r.to_string()));
        }
        return Err(VendorError::Git(format!("ref `{r}` not found on {url}")));
    }
    let out = git(&tmp.path, &["ls-remote", "--symref", url, "HEAD"])?;
    let branch = out
        .lines()
        .find_map(|l| l.strip_prefix("ref: refs/heads/"))
        .and_then(|l| l.split_whitespace().next())
        .ok_or_else(|| VendorError::Git(format!("cannot resolve default branch of {url}")))?
        .to_string();
    let sha = out
        .lines()
        .find(|l| l.ends_with("HEAD") && !l.starts_with("ref:"))
        .and_then(|l| l.split_whitespace().next())
        .ok_or_else(|| VendorError::Git(format!("cannot resolve HEAD of {url}")))?
        .to_string();
    Ok((sha, branch))
}

/// Copy a skill tree into the library, deny-list honored, escaping
/// symlinks refused (`hash_tree` already validates; copy re-checks
/// nothing and simply skips symlinks whose targets it copied as links).
pub fn copy_tree(src: &Path, dst: &Path) -> Result<(), VendorError> {
    std::fs::create_dir_all(dst).map_err(|e| VendorError::Io(e.to_string()))?;
    let entries = std::fs::read_dir(src).map_err(|e| VendorError::Io(e.to_string()))?;
    for entry in entries.filter_map(Result::ok) {
        let name = entry.file_name();
        if denied(&name.to_string_lossy()) {
            continue;
        }
        let from = entry.path();
        let to = dst.join(&name);
        let meta = std::fs::symlink_metadata(&from).map_err(|e| VendorError::Io(e.to_string()))?;
        if meta.file_type().is_symlink() {
            let target = std::fs::read_link(&from).map_err(|e| VendorError::Io(e.to_string()))?;
            #[cfg(unix)]
            std::os::unix::fs::symlink(&target, &to).map_err(|e| VendorError::Io(e.to_string()))?;
        } else if meta.is_dir() {
            copy_tree(&from, &to)?;
        } else {
            std::fs::copy(&from, &to).map_err(|e| VendorError::Io(e.to_string()))?;
        }
    }
    Ok(())
}

/// Vendor a validated skill tree into `library/<name>`: hash, copy,
/// stamp. Returns the content hash.
pub fn vendor(skill_src: &Path, library: &Path, entry: &Entry) -> Result<String, VendorError> {
    let hash = hash_tree(skill_src).map_err(VendorError::Hash)?;
    let dst = library.join(&entry.name);
    if dst.exists() {
        std::fs::remove_dir_all(&dst).map_err(|e| VendorError::Io(e.to_string()))?;
    }
    copy_tree(skill_src, &dst)?;
    let stamp = format!(
        "managed-by: almanac\nname: {}\nsource: {}\n{}sha256: {hash}\n",
        entry.name,
        entry.source,
        entry
            .rev
            .as_deref()
            .map(|r| format!("rev: {r}\n"))
            .unwrap_or_default(),
    );
    std::fs::write(dst.join(ORIGIN_STAMP), stamp).map_err(|e| VendorError::Io(e.to_string()))?;
    Ok(hash)
}

#[must_use]
pub fn is_managed(dir: &Path) -> bool {
    dir.join(ORIGIN_STAMP).is_file()
}

/// Minimal temp-dir handling (no external dep): unique dir under the
/// system temp root, removed on drop.
pub mod tempdir {
    use super::VendorError;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    pub struct TempDirHandle {
        pub path: PathBuf,
    }

    impl Drop for TempDirHandle {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.path).ok();
        }
    }

    pub fn create(tag: &str) -> Result<TempDirHandle, VendorError> {
        let path = std::env::temp_dir().join(format!(
            "{tag}-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&path).map_err(|e| VendorError::Io(e.to_string()))?;
        Ok(TempDirHandle { path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locate_parses_all_source_forms() {
        assert!(
            matches!(locate("github:o/r"), Ok(Located::Git { url }) if url == "https://github.com/o/r")
        );
        assert!(
            matches!(locate("o/r"), Ok(Located::Git { url }) if url == "https://github.com/o/r")
        );
        assert!(matches!(
            locate("git:ssh://x/y.git"),
            Ok(Located::Git { .. })
        ));
        assert!(matches!(
            locate("dev:../gaff/skills/gaff"),
            Ok(Located::Dev { .. })
        ));
        assert!(locate("ftp://nope").is_err());
        assert!(locate("just-a-word").is_err());
    }

    #[test]
    fn vendor_stamps_and_hash_survives_round_trip() {
        let base = std::env::temp_dir().join(format!("almanac-vendor-{}", std::process::id()));
        std::fs::remove_dir_all(&base).ok();
        let src = base.join("src");
        let lib = base.join("lib");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("SKILL.md"), "---\nname: x\n---\nbody\n").unwrap();
        let entry = Entry {
            name: "x".into(),
            source: "dev:src".into(),
            path: None,
            r#ref: None,
            rev: None,
            sha256: String::new(),
        };
        let hash = vendor(&src, &lib, &entry).unwrap();
        assert!(is_managed(&lib.join("x")));
        assert_eq!(
            hash_tree(&lib.join("x")).unwrap(),
            hash,
            "stamp excluded from hash"
        );
    }
}

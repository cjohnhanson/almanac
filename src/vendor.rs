//! Fetches sources and vendors skill trees into the library.
//!
//! Every git operation runs in-process on gix; no git program is
//! spawned. To fetch a pinned commit from a network source it tries a
//! depth-1 fetch of the sha, which GitHub permits. It then tries a fetch
//! of the recorded ref. It then tries a full fetch. It reports the path
//! that worked. A local repository is read in place, at the rev, with no
//! fetch at all. A copy honors the hash deny-list
//! and refuses a symlink that points outside the tree. It writes an
//! `.almanac-origin` stamp into the vendored directory, so the managed
//! set is explicit.

use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use gix::bstr::ByteSlice as _;

use crate::hash::{HashError, denied, hash_tree};
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

/// A source's git URL, or the local directory of a dev source.
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

fn gix_err(context: &str, e: impl std::fmt::Display) -> VendorError {
    VendorError::Git(format!("{context}: {e}"))
}

/// Where a URL leads. gix's file transport would spawn git-upload-pack,
/// so a local repository is opened in place. ssh would spawn ssh, so it
/// is refused with the fix in the message.
enum Reach {
    Local(PathBuf),
    Network(gix::Url),
}

fn reach(url: &str) -> Result<Reach, VendorError> {
    let parsed = gix::url::parse(gix::bstr::BStr::new(url))
        .map_err(|e| VendorError::Git(format!("{url}: {e}")))?;
    match parsed.scheme {
        gix::url::Scheme::File => Ok(Reach::Local(
            gix::path::from_bstr(parsed.path.as_bstr()).into_owned(),
        )),
        gix::url::Scheme::Http | gix::url::Scheme::Https | gix::url::Scheme::Git => {
            Ok(Reach::Network(parsed))
        }
        gix::url::Scheme::Ssh => Err(VendorError::Git(format!(
            "{url}: an ssh transport needs an ssh process, and almanac spawns none; use https"
        ))),
        gix::url::Scheme::Ext(s) => Err(VendorError::Git(format!("{url}: unsupported scheme {s}"))),
    }
}

fn open_local(path: &Path) -> Result<gix::Repository, VendorError> {
    gix::open_opts(path, gix::open::Options::isolated())
        .map_err(|e| gix_err(&format!("open {}", path.display()), e))
}

/// Fetch into `bare` what `spec` names, from `url`. `depth` limits the
/// history for a sha fetch. Returns whether the fetch succeeded; a
/// failure is a normal branch of the try order, not an error.
fn fetch_into(
    bare: &gix::Repository,
    url: &gix::Url,
    spec: &str,
    depth: Option<u32>,
) -> Result<(), VendorError> {
    let remote = bare
        .remote_at(url.clone())
        .map_err(|e| gix_err("remote", e))?
        .with_refspecs([spec], gix::remote::Direction::Fetch)
        .map_err(|e| gix_err("refspec", e))?
        .with_fetch_tags(gix::remote::fetch::Tags::None);
    let mut prepare = remote
        .connect(gix::remote::Direction::Fetch)
        .map_err(|e| gix_err("connect", e))?
        .with_credentials(mdstore::git::credential_fn())
        .prepare_fetch(
            gix::progress::Discard,
            gix::remote::ref_map::Options::default(),
        )
        .map_err(|e| gix_err("fetch", e))?;
    if let Some(d) = depth.and_then(std::num::NonZeroU32::new) {
        prepare = prepare.with_shallow(gix::remote::fetch::Shallow::DepthAtRemote(d));
    }
    prepare
        .receive(gix::progress::Discard, &AtomicBool::new(false))
        .map_err(|e| gix_err("fetch", e))?;
    Ok(())
}

/// Write the tree of `commit` under `into`: files with their mode,
/// symlinks as symlinks. Submodule entries are skipped.
fn write_tree(
    repo: &gix::Repository,
    commit: gix::ObjectId,
    into: &Path,
) -> Result<(), VendorError> {
    let commit = repo.find_commit(commit).map_err(|e| gix_err("commit", e))?;
    let tree = commit.tree().map_err(|e| gix_err("tree", e))?;
    write_tree_at(repo, &tree, into)
}

fn write_tree_at(
    repo: &gix::Repository,
    tree: &gix::Tree<'_>,
    into: &Path,
) -> Result<(), VendorError> {
    let io = |e: std::io::Error| VendorError::Io(e.to_string());
    std::fs::create_dir_all(into).map_err(io)?;
    for entry in tree.iter() {
        let entry = entry.map_err(|e| gix_err("tree entry", e))?;
        let name = gix::path::from_bstr(entry.filename()).into_owned();
        let target = into.join(&name);
        let mode = entry.mode();
        if mode.is_tree() {
            let sub = repo
                .find_tree(entry.oid().to_owned())
                .map_err(|e| gix_err("subtree", e))?;
            write_tree_at(repo, &sub, &target)?;
        } else if mode.is_link() {
            let blob = repo
                .find_blob(entry.oid().to_owned())
                .map_err(|e| gix_err("symlink", e))?;
            let dest = gix::path::from_bstr(blob.data.as_bstr()).into_owned();
            #[cfg(unix)]
            std::os::unix::fs::symlink(&dest, &target).map_err(io)?;
            #[cfg(not(unix))]
            std::fs::write(&target, &blob.data).map_err(io)?;
        } else if mode.is_blob() {
            let blob = repo
                .find_blob(entry.oid().to_owned())
                .map_err(|e| gix_err("blob", e))?;
            std::fs::write(&target, &blob.data).map_err(io)?;
            #[cfg(unix)]
            if mode.is_executable() {
                use std::os::unix::fs::PermissionsExt as _;
                std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o755))
                    .map_err(io)?;
            }
        }
    }
    Ok(())
}

/// Fetch `rev` from `url` into a temporary checkout. Returns the
/// checkout directory and the name of the fetch path that worked. The
/// temporary directory must outlive every use of that path.
pub fn fetch_rev(
    url: &str,
    rev: &str,
    r#ref: Option<&str>,
) -> Result<(tempdir::TempDirHandle, &'static str), VendorError> {
    let tmp = tempdir::create("almanac-fetch")?;
    match reach(url)? {
        Reach::Local(path) => {
            let repo = open_local(&path)?;
            let id = repo
                .rev_parse_single(format!("{rev}^{{commit}}").as_str())
                .map_err(|e| gix_err(&format!("{rev} in {}", path.display()), e))?
                .detach();
            write_tree(&repo, id, &tmp.path)?;
            Ok((tmp, "local-read"))
        }
        Reach::Network(remote) => {
            // The bare object store lives beside the checkout, in its own
            // temp dir, and goes with it.
            let bare_dir = tempdir::create("almanac-fetch-objects")?;
            gix::init_bare(&bare_dir.path).map_err(|e| gix_err("init", e))?;
            // Reopened isolated: init_bare opens with the user's gitconfig,
            // and a url.insteadOf there could turn https into ssh.
            let bare = open_local(&bare_dir.path)?;
            let want = format!("+{rev}:refs/almanac/want");
            let how = if fetch_into(&bare, &remote, &want, Some(1)).is_ok() {
                "sha-fetch"
            } else if r#ref.is_some_and(|r| {
                fetch_into(&bare, &remote, &format!("+{r}:refs/almanac/ref"), None).is_ok()
            }) {
                "ref-fetch"
            } else {
                fetch_into(&bare, &remote, "+refs/heads/*:refs/heads/*", None)?;
                "full-fetch"
            };
            let id = bare
                .rev_parse_single(format!("{rev}^{{commit}}").as_str())
                .map_err(|e| gix_err(&format!("{rev} on {url}"), e))?
                .detach();
            write_tree(&bare, id, &tmp.path)?;
            drop(bare);
            drop(bare_dir);
            Ok((tmp, how))
        }
    }
}

/// Resolve a ref, or the remote HEAD, to a commit sha. Returns an error
/// when nothing resolves. Almanac never works from an unknown base.
pub fn resolve_remote(url: &str, r#ref: Option<&str>) -> Result<(String, String), VendorError> {
    match reach(url)? {
        Reach::Local(path) => {
            let repo = open_local(&path)?;
            if let Some(r) = r#ref {
                let id = repo
                    .rev_parse_single(format!("{r}^{{commit}}").as_str())
                    .map_err(|_| VendorError::Git(format!("ref `{r}` not found on {url}")))?;
                return Ok((id.to_string(), r.to_string()));
            }
            let branch = repo
                .head_name()
                .ok()
                .flatten()
                .map(|n| n.shorten().to_string())
                .ok_or_else(|| {
                    VendorError::Git(format!("cannot resolve default branch of {url}"))
                })?;
            let sha = repo
                .head_id()
                .map_err(|_| VendorError::Git(format!("cannot resolve HEAD of {url}")))?
                .to_string();
            Ok((sha, branch))
        }
        Reach::Network(remote) => {
            let bare_dir = tempdir::create("almanac-resolve")?;
            gix::init_bare(&bare_dir.path).map_err(|e| gix_err("init", e))?;
            // Reopened isolated: init_bare opens with the user's gitconfig,
            // and a url.insteadOf there could turn https into ssh.
            let bare = open_local(&bare_dir.path)?;
            // A remote with no fetch refspec advertises only what its
            // tag setting names, so HEAD and the heads would be missing.
            // The heads refspec and no prefix filter make this a full
            // `ls-remote --symref`.
            let remote = bare
                .remote_at(remote)
                .map_err(|e| gix_err("remote", e))?
                .with_refspecs(
                    ["+refs/heads/*:refs/heads/*"],
                    gix::remote::Direction::Fetch,
                )
                .map_err(|e| gix_err("refspec", e))?
                .with_fetch_tags(gix::remote::fetch::Tags::All);
            let options = gix::remote::ref_map::Options {
                prefix_from_spec_as_filter_on_remote: false,
                ..Default::default()
            };
            let map = remote
                .connect(gix::remote::Direction::Fetch)
                .map_err(|e| gix_err("connect", e))?
                .with_credentials(mdstore::git::credential_fn())
                .ref_map(gix::progress::Discard, options)
                .map_err(|e| gix_err("ls-remote", e))?;
            // The advertised refs, as `git ls-remote --symref` lists them.
            let (map, _handshake) = map;
            let refs = &map.remote_refs;
            let find = |name: &str| -> Option<String> {
                refs.iter().find_map(|r| match r {
                    gix::protocol::handshake::Ref::Direct {
                        full_ref_name,
                        object,
                    } if full_ref_name == name => Some(object.to_string()),
                    // An annotated tag pins the commit it peels to, as
                    // the local path and `{r}^{commit}` do.
                    gix::protocol::handshake::Ref::Peeled {
                        full_ref_name,
                        object,
                        ..
                    } if full_ref_name == name => Some(object.to_string()),
                    gix::protocol::handshake::Ref::Symbolic {
                        full_ref_name,
                        object,
                        ..
                    } if full_ref_name == name => Some(object.to_string()),
                    _ => None,
                })
            };
            if let Some(r) = r#ref {
                let sha = find(r)
                    .or_else(|| find(&format!("refs/heads/{r}")))
                    .or_else(|| find(&format!("refs/tags/{r}")))
                    .ok_or_else(|| VendorError::Git(format!("ref `{r}` not found on {url}")))?;
                return Ok((sha, r.to_string()));
            }
            let (sha, branch) = refs
                .iter()
                .find_map(|r| match r {
                    gix::protocol::handshake::Ref::Symbolic {
                        full_ref_name,
                        target,
                        object,
                        ..
                    } if full_ref_name == "HEAD" => Some((object.to_string(), target.to_string())),
                    _ => None,
                })
                .ok_or_else(|| {
                    VendorError::Git(format!("cannot resolve default branch of {url}"))
                })?;
            let branch = branch
                .strip_prefix("refs/heads/")
                .unwrap_or(&branch)
                .to_string();
            Ok((sha, branch))
        }
    }
}

/// Copy a skill tree into the library.
///
/// The copy honors the deny-list and recreates a symlink as a symlink.
/// It does not re-check the symlink targets, because `hash_tree` runs
/// first and refuses a symlink that points outside the tree.
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

/// Vendor a validated skill tree into `library/<name>`. Hashes the
/// tree, copies it, then stamps it. Returns the content hash.
pub fn vendor(skill_src: &Path, library: &Path, entry: &Entry) -> Result<String, VendorError> {
    // This function removes a directory and writes in its place, and
    // the name it uses came from a published SKILL.md or from an
    // almanac.yml that a poisoned add already wrote. Check it here as
    // well as at the point of entry: a guard on one route is a guard
    // the other route does not have.
    if !mdstore::is_plain_stem(&entry.name) {
        return Err(VendorError::Io(format!(
            "`{}` is not a plain skill name; a name may not hold a path separator",
            entry.name
        )));
    }
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

/// Minimal temporary directories, with no external dependency. Each one
/// is a unique directory under the system temp root. Drop removes it.
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
        let base = std::env::temp_dir().join(format!("almanac-vendor-copy-{}", std::process::id()));
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

    fn sig() -> gix::actor::Signature {
        gix::actor::Signature {
            name: "t".into(),
            email: "t@e".into(),
            time: gix::date::Time::new(1_600_000_000, 0),
        }
    }

    /// One commit with a nested tree and an executable, all through gix.
    fn commit(repo: &gix::Repository, files: &[(&str, &str, bool)], msg: &str) -> gix::ObjectId {
        let mut top = Vec::new();
        let mut sub = Vec::new();
        for (path, text, exec) in files {
            let oid = repo.write_blob(text.as_bytes()).unwrap().detach();
            let kind = if *exec {
                gix::objs::tree::EntryKind::BlobExecutable
            } else {
                gix::objs::tree::EntryKind::Blob
            };
            match path.split_once('/') {
                Some((_dir, name)) => sub.push(gix::objs::tree::Entry {
                    mode: kind.into(),
                    filename: name.into(),
                    oid,
                }),
                None => top.push(gix::objs::tree::Entry {
                    mode: kind.into(),
                    filename: (*path).into(),
                    oid,
                }),
            }
        }
        if !sub.is_empty() {
            let mut t = gix::objs::Tree { entries: sub };
            t.entries.sort();
            let oid = repo.write_object(&t).unwrap().detach();
            top.push(gix::objs::tree::Entry {
                mode: gix::objs::tree::EntryKind::Tree.into(),
                filename: "skill".into(),
                oid,
            });
        }
        let mut tree = gix::objs::Tree { entries: top };
        tree.entries.sort();
        let tree = repo.write_object(&tree).unwrap().detach();
        let parents: Vec<_> = repo
            .head_id()
            .ok()
            .map(gix::Id::detach)
            .into_iter()
            .collect();
        let s = sig();
        repo.commit_as(
            s.to_ref(&mut gix::date::parse::TimeBuf::default()),
            s.to_ref(&mut gix::date::parse::TimeBuf::default()),
            "HEAD",
            msg,
            tree,
            parents,
        )
        .unwrap()
        .detach()
    }

    #[test]
    fn a_local_source_is_read_in_place_at_the_pinned_rev_and_resolved_at_head() {
        let base = std::env::temp_dir().join(format!("almanac-vendor-gix-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let repo = gix::init(&base).unwrap();
        let first = commit(
            &repo,
            &[
                ("skill/SKILL.md", "---\nname: x\n---\none\n", false),
                ("skill/run.sh", "#!/bin/sh\n", true),
            ],
            "one",
        );
        let second = commit(
            &repo,
            &[("skill/SKILL.md", "---\nname: x\n---\ntwo\n", false)],
            "two",
        );
        let url = format!("file://{}", base.display());

        let (tmp, how) = fetch_rev(&url, &first.to_string(), None).unwrap();
        assert_eq!(how, "local-read");
        assert_eq!(
            std::fs::read_to_string(tmp.path.join("skill/SKILL.md")).unwrap(),
            "---\nname: x\n---\none\n"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let mode = std::fs::metadata(tmp.path.join("skill/run.sh"))
                .unwrap()
                .permissions()
                .mode();
            assert_ne!(mode & 0o111, 0, "the executable bit is kept");
        }
        drop(tmp);

        let (sha, branch) = resolve_remote(&url, None).unwrap();
        assert_eq!(sha, second.to_string());
        assert_eq!(branch, "main");
        let (sha, r) = resolve_remote(&url, Some("main")).unwrap();
        assert_eq!(
            (sha.as_str(), r.as_str()),
            (second.to_string().as_str(), "main")
        );
        assert!(resolve_remote(&url, Some("no-such-ref")).is_err());
        let Err(e) = fetch_rev("git@example.com:o/r.git", "main", None) else {
            panic!("ssh must be refused")
        };
        assert!(e.to_string().contains("https"), "{e}");
        let Err(e) = fetch_rev(&url, "0000000000000000000000000000000000000000", None) else {
            panic!("an unknown rev must fail")
        };
        assert!(e.to_string().contains("0000000"), "{e}");
    }
}

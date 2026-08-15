//! Manifest operations: init, add, sync, update, list, remove, index-md.
//!
//! The trust model, stated plainly: `add` trusts a source on first use.
//! It shows the red-flag report and the staged tree, and nothing lands
//! without `--accept`. Almanac then pins the content, and every later
//! change arrives through an `update` that shows a diff first. No
//! change lands without a report.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::Error;
use crate::flags;
use crate::hash::hash_tree;
use crate::manifest::{Entry, Manifest, ManifestError, ORIGIN_STAMP};
use crate::skill;
use crate::source::SkillSource;
use crate::vendor::{self, Located, VendorError};

impl From<ManifestError> for Error {
    fn from(e: ManifestError) -> Self {
        Self::General(e.to_string())
    }
}
impl From<VendorError> for Error {
    fn from(e: VendorError) -> Self {
        Self::General(e.to_string())
    }
}

pub fn init(dir: &Path, library: &str) -> Result<(), Error> {
    if dir.join(crate::manifest::MANIFEST_NAME).exists() {
        return Err(Error::General("almanac.yml already exists".into()));
    }
    std::fs::create_dir_all(dir.join(library)).map_err(|e| Error::General(e.to_string()))?;
    Manifest::new(library).save(dir)?;
    println!("initialized almanac.yml (library: {library})");
    Ok(())
}

pub struct AddOpts {
    pub name: Option<String>,
    pub path: Option<String>,
    pub r#ref: Option<String>,
    pub rev: Option<String>,
    pub accept: bool,
}

/// Say where the unaccepted content is, and what to do with it.
///
/// A git source was copied into a staging directory almanac owns, so
/// removing it is the way to discard it. A dev source was read in
/// place from a directory the user owns, and nothing was staged.
/// Telling them to remove that path would tell them to delete the
/// skill they are adding.
fn report_not_accepted(shown: &Path, staged: bool) {
    if staged {
        println!(
            "staged only; inspect {} and run again with --accept",
            shown.display()
        );
        println!("remove {} to discard it", shown.display());
    } else {
        println!(
            "read in place from {}; nothing was staged. Run again with --accept.",
            shown.display()
        );
    }
}

/// The path a user should inspect for a staged-but-unaccepted skill.
///
/// A git source lives in a temp directory that drops when add returns,
/// so the tree is copied to a persistent `.almanac-staged/<name>/`
/// directory and that path is returned. A dev source is already a stable
/// directory the user owns, so its own path is returned.
fn staged_inspect_path(
    dir: &Path,
    name: &str,
    skill_src: &Path,
    is_temp: bool,
) -> Result<PathBuf, Error> {
    if !is_temp {
        return Ok(skill_src.to_path_buf());
    }
    let staged = dir.join(".almanac-staged").join(name);
    std::fs::remove_dir_all(&staged).ok();
    if let Some(parent) = staged.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    vendor::copy_tree(skill_src, &staged)?;
    Ok(staged)
}

pub fn add(dir: &Path, source: &str, opts: &AddOpts) -> Result<(), Error> {
    let mut manifest = Manifest::load(dir)?;

    // Stage the skill tree.
    let (tmp_dir, skill_src, resolved_source, rev, r#ref): (
        Option<vendor::tempdir::TempDirHandle>,
        PathBuf,
        String,
        Option<String>,
        Option<String>,
    ) = match vendor::locate(source)? {
        Located::Dev { dir: src_dir } => {
            let abs = if src_dir.is_absolute() {
                src_dir.clone()
            } else {
                dir.join(&src_dir)
            };
            if !abs.is_dir() {
                return Err(Error::General(format!(
                    "dev source {} not found",
                    abs.display()
                )));
            }
            (None, abs, format!("dev:{}", src_dir.display()), None, None)
        }
        Located::Git { url } => {
            let (rev, branch) = match &opts.rev {
                Some(r) => (r.clone(), opts.r#ref.clone().unwrap_or_default()),
                None => vendor::resolve_remote(&url, opts.r#ref.as_deref())?,
            };
            let (tmp, how) = vendor::fetch_rev(&url, &rev, opts.r#ref.as_deref())?;
            eprintln!("fetched {rev} via {how}");
            let sub = opts.path.clone().unwrap_or_default();
            let skill_src = if sub.is_empty() {
                tmp.path.clone()
            } else {
                tmp.path.join(&sub)
            };
            if !skill_src.join("SKILL.md").is_file() {
                return Err(Error::General(format!(
                    "no SKILL.md at {} (use --path to name the skill directory)",
                    skill_src.display()
                )));
            }
            let canonical = if source.contains(':') {
                source.to_string()
            } else {
                format!("github:{source}")
            };
            let r#ref = if branch.is_empty() {
                opts.r#ref.clone()
            } else {
                Some(branch)
            };
            (Some(tmp), skill_src, canonical, Some(rev), r#ref)
        }
    };

    // Agents read the frontmatter name, so it is authoritative. The
    // manifest name must match it. Almanac refuses a mismatch here
    // instead of warning about it forever.
    let fm_name = skill_md_name(&skill_src)?;
    let name = opts.name.clone().unwrap_or_else(|| fm_name.clone());
    if name != fm_name {
        return Err(Error::General(format!(
            "manifest name `{name}` does not match SKILL.md name `{fm_name}`"
        )));
    }
    if manifest.entry(&name).is_some() {
        return Err(Error::General(format!(
            "`{name}` already in the manifest (use `almanac update {name}`)"
        )));
    }

    // Hash the tree before anything is copied. hash_tree refuses a
    // symlink that leaves the skill, so an escaping link stops the add
    // rather than being staged first and refused afterwards.
    crate::hash::hash_tree(&skill_src)
        .map_err(|e| Error::General(format!("cannot hash {name}: {e}")))?;

    // Red-flag report before anything lands.
    let flags = flags::scan(&skill_src);
    print_flags(&name, &flags);

    if !opts.accept {
        let shown = staged_inspect_path(dir, &name, &skill_src, tmp_dir.is_some())?;
        report_not_accepted(&shown, tmp_dir.is_some());
        return Err(Error::General("not accepted".into()));
    }

    let entry_proto = Entry {
        name: name.clone(),
        source: resolved_source,
        path: opts.path.clone(),
        r#ref,
        rev,
        sha256: String::new(),
    };
    let library = manifest.library_dir(dir);
    let hash = vendor::vendor(&skill_src, &library, &entry_proto)?;
    manifest.upsert(Entry {
        sha256: hash,
        ..entry_proto
    });
    manifest.save(dir)?;
    println!("added {name}");
    Ok(())
}

pub fn sync(dir: &Path, check: bool) -> Result<(), Error> {
    let manifest = Manifest::load(dir)?;
    let library = manifest.library_dir(dir);
    let mut failures = 0;

    for entry in &manifest.skills {
        let vendored = library.join(&entry.name);
        let state = if vendored.is_dir() {
            match hash_tree(&vendored) {
                Ok(h) if h == entry.sha256 => "clean",
                Ok(_) => "drifted",
                Err(_) => "unreadable",
            }
        } else {
            "missing"
        };

        if check {
            if entry.is_dev() {
                println!(
                    "skip  {} (dev snapshot; outside the check contract)",
                    entry.name
                );
            } else if state == "clean" {
                println!("ok    {}", entry.name);
            } else {
                println!("FAIL  {} ({state})", entry.name);
                failures += 1;
            }
            continue;
        }

        // Write the entry to disk.
        if state == "clean" {
            println!("ok    {}", entry.name);
        } else if entry.is_dev() {
            println!(
                "skip  {} (dev snapshot {state}; add it again from its source to refresh)",
                entry.name
            );
        } else {
            let url = source_url(&entry.source)?;
            let rev = entry
                .rev
                .as_deref()
                .ok_or_else(|| Error::General(format!("{}: git source without rev", entry.name)))?;
            let (tmp, how) = vendor::fetch_rev(&url, rev, entry.r#ref.as_deref())?;
            let sub = entry.path.clone().unwrap_or_default();
            let src = if sub.is_empty() {
                tmp.path.clone()
            } else {
                tmp.path.join(&sub)
            };
            // Verify the fetched content against the pin BEFORE it
            // touches the library. vendor removes the existing directory,
            // so writing first would destroy the good content when a pin
            // no longer matches upstream.
            let fetched = hash_tree(&src).map_err(|e| Error::General(e.to_string()))?;
            if fetched == entry.sha256 {
                vendor::vendor(&src, &library, entry)?;
                println!("synced {} (via {how})", entry.name);
            } else {
                println!(
                    "FAIL  {} (fetched content does not match the pinned hash; library left as is)",
                    entry.name
                );
                failures += 1;
            }
        }
    }

    if failures > 0 {
        Err(Error::General(format!(
            "{failures} entr{} failed",
            if failures == 1 { "y" } else { "ies" }
        )))
    } else {
        Ok(())
    }
}

pub fn update(dir: &Path, names: &[String], yes: bool) -> Result<(), Error> {
    let mut manifest = Manifest::load(dir)?;
    let library = manifest.library_dir(dir);
    let selected: Vec<Entry> = manifest
        .skills
        .iter()
        .filter(|e| names.is_empty() || names.contains(&e.name))
        .cloned()
        .collect();
    if selected.is_empty() {
        return Err(Error::General("no matching entries".into()));
    }

    for entry in selected {
        if entry.is_dev() {
            println!(
                "skip  {} (dev snapshot; add it again to refresh)",
                entry.name
            );
            continue;
        }
        let url = source_url(&entry.source)?;
        let (new_rev, r#ref) = vendor::resolve_remote(&url, entry.r#ref.as_deref())?;
        if Some(&new_rev) == entry.rev.as_ref() {
            println!("up-to-date {}", entry.name);
            continue;
        }
        let (tmp, _how) = vendor::fetch_rev(&url, &new_rev, Some(&r#ref))?;
        let sub = entry.path.clone().unwrap_or_default();
        let new_src = if sub.is_empty() {
            tmp.path.clone()
        } else {
            tmp.path.join(&sub)
        };

        println!(
            "== {} {} -> {new_rev}",
            entry.name,
            entry.rev.as_deref().unwrap_or("?")
        );
        let flags = flags::scan(&new_src);
        print_flags(&entry.name, &flags);
        show_diff(&library.join(&entry.name), &new_src);

        if yes {
            let updated = Entry {
                rev: Some(new_rev),
                r#ref: Some(r#ref),
                ..entry.clone()
            };
            let hash = vendor::vendor(&new_src, &library, &updated)?;
            manifest.upsert(Entry {
                sha256: hash,
                ..updated
            });
            manifest.save(dir)?;
            println!("updated {}", entry.name);
        } else {
            println!("not applied; run again with --yes to accept");
        }
    }
    Ok(())
}

pub fn remove(dir: &Path, name: &str) -> Result<(), Error> {
    let mut manifest = Manifest::load(dir)?;
    if manifest.entry(name).is_none() {
        return Err(Error::General(format!("`{name}` is not in the manifest")));
    }
    let vendored = manifest.library_dir(dir).join(name);
    if vendored.exists() {
        if !vendor::is_managed(&vendored) {
            return Err(Error::General(format!(
                "{} has no {ORIGIN_STAMP} stamp; almanac does not delete an unmanaged directory",
                vendored.display()
            )));
        }
        std::fs::remove_dir_all(&vendored).map_err(|e| Error::General(e.to_string()))?;
    }
    manifest.skills.retain(|e| e.name != name);
    manifest.save(dir)?;
    println!("removed {name}");
    Ok(())
}

pub fn list(dir: &Path) -> Result<(), Error> {
    let manifest = Manifest::load(dir)?;
    let library = manifest.library_dir(dir);
    for entry in &manifest.skills {
        let vendored = library.join(&entry.name);
        let state = if vendored.is_dir() {
            match hash_tree(&vendored) {
                Ok(h) if h == entry.sha256 => "clean",
                _ => "drifted",
            }
        } else {
            "missing"
        };
        let rev = entry
            .rev
            .as_deref()
            .map_or_else(|| "-".to_string(), |r| r.chars().take(9).collect());
        println!(
            "{:<28} {:<9} {:<8} {}",
            entry.name, rev, state, entry.source
        );
    }
    // Unmanaged neighbors: almanac lists them and never touches them.
    if let Ok(entries) = std::fs::read_dir(&library) {
        let mut unmanaged: Vec<String> = entries
            .filter_map(Result::ok)
            .filter(|e| e.path().is_dir())
            .filter(|e| !vendor::is_managed(&e.path()))
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| manifest.entry(n).is_none())
            .collect();
        unmanaged.sort();
        for name in unmanaged {
            println!("{name:<28} {:<9} {:<8} unmanaged", "-", "-");
        }
    }
    Ok(())
}

/// Markdown skills index for context injection, as a gaff prime section.
///
/// The output degrades in steps under the byte budget: full lines while
/// they fit, then name-only lines, then a truncation note. Almanac
/// never cuts the list without a note.
pub fn index_md(dir: &Path, max_bytes: usize) -> Result<String, Error> {
    let manifest = Manifest::load(dir)?;
    let library = manifest.library_dir(dir);
    let entries = skill::index(
        dir,
        &[SkillSource::Path {
            path: library.to_string_lossy().into_owned(),
        }],
    );
    Ok(format_index_md(&entries, max_bytes))
}

/// The markdown index over explicit path sources. This needs no
/// manifest, so a plain skill directory can feed a gaff section.
#[must_use]
pub fn index_md_sources(dir: &Path, sources: &[SkillSource], max_bytes: usize) -> String {
    format_index_md(&skill::index(dir, sources), max_bytes)
}

fn format_index_md(entries: &[skill::SkillEntry], max_bytes: usize) -> String {
    let mut out = String::from("Available skills (show with `almanac show <name>`):\n");
    let mut omitted = 0usize;
    for e in entries {
        let full = format!("- **{}** — {}\n", e.name, e.description);
        let short = format!("- {}\n", e.name);
        let note_room = 40; // room kept for the truncation note
        if out.len() + full.len() + note_room <= max_bytes {
            out.push_str(&full);
        } else if out.len() + short.len() + note_room <= max_bytes {
            out.push_str(&short);
        } else {
            omitted += 1;
        }
    }
    if omitted > 0 {
        use std::fmt::Write as _;
        let _ = writeln!(out, "…and {omitted} more (truncated)");
    }
    out
}

fn source_url(source: &str) -> Result<String, Error> {
    match vendor::locate(source)? {
        Located::Git { url } => Ok(url),
        Located::Dev { .. } => Err(Error::General("dev source has no URL".into())),
    }
}

fn skill_md_name(skill_src: &Path) -> Result<String, Error> {
    let md = std::fs::read_to_string(skill_src.join("SKILL.md"))
        .map_err(|_| Error::General(format!("no SKILL.md in {}", skill_src.display())))?;
    md.lines()
        .skip(1)
        .take_while(|l| l.trim() != "---")
        .find_map(|l| l.strip_prefix("name:"))
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .ok_or_else(|| Error::General("SKILL.md frontmatter has no name".into()))
}

fn print_flags(name: &str, flags: &[flags::Flag]) {
    if flags.is_empty() {
        println!("red-flag scan: {name}: clean");
    } else {
        println!("red-flag scan: {name}: {} finding(s)", flags.len());
        for f in flags {
            println!("  ! {}: {}", f.path, f.what);
        }
    }
}

/// `git diff --no-index` exits 1 when the trees differ. That is the
/// normal case here, not an error.
fn show_diff(old: &Path, new: &Path) {
    let out = Command::new("git")
        .args(["diff", "--no-index", "--stat", "--"])
        .arg(old)
        .arg(new)
        .output();
    if let Ok(o) = out {
        print!("{}", String::from_utf8_lossy(&o.stdout));
    }
    let out = Command::new("git")
        .args(["diff", "--no-index", "--"])
        .arg(old)
        .arg(new)
        .output();
    if let Ok(o) = out {
        print!("{}", String::from_utf8_lossy(&o.stdout));
    }
}

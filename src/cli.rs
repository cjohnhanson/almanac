use std::path::Path;

use clap::Parser;

use crate::docs;
use crate::error::Error;
use crate::skill;
use crate::source::SkillSource;

#[derive(Parser)]
#[command(
    name = "almanac",
    version,
    about = "Almanac curates agent skills and indexes them for agents to read",
    max_term_width = 98
)]
pub struct Args {
    /// Project root directory. Defaults to the current directory.
    #[arg(long, global = true, default_value = ".")]
    pub root: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser)]
pub enum Command {
    /// Create an almanac.yml manifest that governs a library directory.
    Init {
        /// Library directory, relative to the manifest.
        #[arg(long, default_value = "skills")]
        library: String,
    },
    /// Fetch a skill, print its red-flag report, and vendor it with --accept.
    Add {
        /// github:owner/repo, owner/repo, git:<url>, or dev:<path>.
        source: String,
        /// Manifest name. Must match the SKILL.md frontmatter name.
        #[arg(long)]
        name: Option<String>,
        /// Skill directory within the source.
        #[arg(long)]
        path: Option<String>,
        /// Branch or tag to resolve.
        #[arg(long = "ref")]
        r#ref: Option<String>,
        /// Exact commit to pin.
        #[arg(long)]
        rev: Option<String>,
        /// Accept the staged content into the library. Trust on first use.
        #[arg(long)]
        accept: bool,
    },
    /// Write every pinned entry to disk. --check verifies instead, and exits 1 on drift.
    Sync {
        #[arg(long)]
        check: bool,
    },
    /// Fetch upstream, print the red flags and the diff, then re-pin with --yes.
    Update {
        /// Entry names. Defaults to every git-sourced entry.
        names: Vec<String>,
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Remove an entry and its vendored directory. Refuses an unmanaged directory.
    Remove { name: String },
    /// Print every manifest entry with its pin and drift state, then the unmanaged neighbors.
    Status,
    /// List the available skills with name, description, and source.
    List {
        /// Skill source directories (repeatable).
        #[arg(long = "source", short = 's')]
        sources: Vec<String>,
    },
    /// Print the full SKILL.md content of one skill.
    Show {
        /// The skill name to print.
        name: String,

        /// Skill source directories (repeatable).
        #[arg(long = "source", short = 's')]
        sources: Vec<String>,
    },
    /// Print a JSON index of the available skills.
    Index {
        /// Skill source directories (repeatable).
        #[arg(long = "source", short = 's')]
        sources: Vec<String>,
        /// Print a markdown skills index, as a gaff prime-section payload.
        #[arg(long)]
        md: bool,
        /// Byte budget for --md. The output degrades to name-only
        /// lines, then to a truncation note. Defaults to 4096, which
        /// matches gaff's inject cap.
        #[arg(long, default_value_t = 4096)]
        max_bytes: usize,
    },
    /// Browse bundled documentation.
    Docs {
        /// Topic slug to print, or "search" to search.
        topic: Option<String>,
        /// Search query. Used when the topic is "search".
        query: Option<String>,
    },
}

/// Run one CLI command. clc calls this when it mounts almanac as a
/// subcommand.
pub fn run_command(root: &Path, sources: &[SkillSource], command: Command) -> Result<(), Error> {
    match command {
        Command::List {
            sources: extra_sources,
        } => {
            let all_sources = merge_sources(sources, &extra_sources);
            cmd_list(root, &all_sources);
            Ok(())
        }
        Command::Show {
            name,
            sources: extra_sources,
        } => {
            let all_sources = merge_sources(sources, &extra_sources);
            if skill::show(&name, root, &all_sources)? {
                Ok(())
            } else {
                Err(Error::General(format!("skill '{name}' not found")))
            }
        }
        Command::Index {
            sources: extra_sources,
            md,
            max_bytes,
        } => {
            if md {
                if extra_sources.is_empty() {
                    print!("{}", crate::ops::index_md(root, max_bytes)?);
                } else {
                    let all_sources = merge_sources(sources, &extra_sources);
                    print!("{}", crate::ops::index_md_sources(root, &all_sources, max_bytes));
                }
            } else {
                let all_sources = merge_sources(sources, &extra_sources);
                cmd_index(root, &all_sources);
            }
            Ok(())
        }
        Command::Docs { topic, query } => cmd_docs(topic.as_deref(), query.as_deref()),
        Command::Init { library } => crate::ops::init(root, &library),
        Command::Add {
            source,
            name,
            path,
            r#ref,
            rev,
            accept,
        } => crate::ops::add(
            root,
            &source,
            &crate::ops::AddOpts {
                name,
                path,
                r#ref,
                rev,
                accept,
            },
        ),
        Command::Sync { check } => crate::ops::sync(root, check),
        Command::Update { names, yes } => crate::ops::update(root, &names, yes),
        Command::Remove { name } => crate::ops::remove(root, &name),
        Command::Status => crate::ops::list(root),
    }
}

/// Run the standalone binary. It uses the CLI arguments only.
pub fn run(args: Args) -> Result<(), Error> {
    let root = Path::new(&args.root);
    // Standalone mode: no config sources, only CLI --source flags.
    run_command(root, &[], args.command)
}

fn cmd_list(root: &Path, sources: &[SkillSource]) {
    let entries = skill::index(root, sources);
    if entries.is_empty() {
        println!("No skills configured.");
        return;
    }
    for entry in &entries {
        println!("{:<30} {} [file]", entry.name, entry.description);
    }
}

fn cmd_index(root: &Path, sources: &[SkillSource]) {
    let entries = skill::index(root, sources);
    println!("{}", skill::format_index_json(&entries));
}

fn cmd_docs(topic: Option<&str>, query: Option<&str>) -> Result<(), Error> {
    match topic {
        None | Some("list") => {
            print!("{}", docs::format_list(docs::PAGES));
            Ok(())
        }
        Some("search") => {
            let q = query.unwrap_or("");
            if q.is_empty() {
                return Err(Error::General(
                    "usage: almanac docs search <query>".to_string(),
                ));
            }
            let matches = docs::find_matching(docs::PAGES, q);
            if matches.is_empty() {
                eprintln!("no docs matching '{q}'");
            } else {
                print!("{}", docs::format_list_from_refs(&matches));
            }
            Ok(())
        }
        Some(identifier) => {
            if let Some(page) = docs::find(identifier) {
                print!("{}", page.content());
                return Ok(());
            }
            eprintln!("unknown doc: {identifier}");
            eprintln!();
            print!("{}", docs::format_list(docs::PAGES));
            Err(Error::General(format!("doc '{identifier}' not found")))
        }
    }
}

/// Merge the sources from the config with the CLI --source flags.
fn merge_sources(config_sources: &[SkillSource], cli_sources: &[String]) -> Vec<SkillSource> {
    let mut all: Vec<SkillSource> = config_sources.to_vec();
    for s in cli_sources {
        all.push(SkillSource::Path { path: s.clone() });
    }
    all
}

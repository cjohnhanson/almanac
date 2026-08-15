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
    /// Write the man pages into a directory (the package build uses this)
    #[command(hide = true)]
    GenMan {
        /// Output directory for the section-1 pages
        dir: std::path::PathBuf,
    },

    /// Print a shell completion script (the package build uses this)
    #[command(hide = true)]
    GenCompletions {
        /// Target shell
        shell: clap_complete::Shell,
    },

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
    /// Manage the declared libraries.
    #[command(subcommand)]
    Store(StoreCommand),
    /// Check the libraries for dangling requirements and store problems.
    Check,
    /// Serve this library over MCP.
    Serve {
        /// The surfaces to offer, separated by commas: skills,
        /// resources, tools. Tools are the surface every client can
        /// call, so a default keeps them on.
        #[arg(long, default_value = "skills,tools")]
        surfaces: String,
        /// Where to listen. Omitted, the server speaks on stdin and
        /// stdout, for a client that starts it as a child process.
        /// Given, it serves over HTTP for a client that connects to it.
        #[arg(long)]
        bind: Option<String>,
    },
    /// Browse bundled documentation.
    Docs {
        /// Topic slug to print, or "search" to search.
        topic: Option<String>,
        /// Search query. Used when the topic is "search".
        query: Option<String>,
    },
}

/// Run one CLI command. A host that mounts almanac as a subcommand
/// calls this with its own sources.
pub fn run_command(root: &Path, sources: &[SkillSource], command: Command) -> Result<(), Error> {
    match command {
        Command::GenMan { dir } => {
            use clap::CommandFactory as _;
            std::fs::create_dir_all(&dir).map_err(|e| Error::General(e.to_string()))?;
            crate::mangen::write_man_pages(&Args::command(), &dir)
                .map_err(|e| Error::General(e.to_string()))?;
            Ok(())
        }

        Command::GenCompletions { shell } => {
            use clap::CommandFactory as _;
            clap_complete::generate(
                shell,
                &mut Args::command(),
                "almanac",
                &mut std::io::stdout(),
            );
            Ok(())
        }

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
            cmd_show(root, &name, &all_sources)
        }
        Command::Index {
            sources: extra_sources,
            md,
            max_bytes,
        } => {
            // This payload is what an agent reads before it asks for
            // anything. It once covered only the manifest's own
            // library, so every declared library was missing from the
            // one listing the agent sees.
            let all_sources = merge_sources(sources, &extra_sources);
            if md {
                let entries = reachable_skills(root, &all_sources);
                print!("{}", crate::ops::format_index_md(&entries, max_bytes));
            } else {
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
        Command::Store(cmd) => cmd_store(root, cmd),
        Command::Check => cmd_check(root),
        Command::Serve { surfaces, bind } => cmd_serve(root, &surfaces, bind.as_deref()),
        Command::Sync { check } => crate::ops::sync(root, check),
        Command::Update { names, yes } => crate::ops::update(root, &names, yes),
        Command::Remove { name } => crate::ops::remove(root, &name),
        Command::Status => crate::ops::list(root),
    }
}

/// The `store` subcommands.
fn cmd_store(root: &Path, cmd: StoreCommand) -> Result<(), Error> {
    match cmd {
        StoreCommand::List => {
            let ws = crate::workspace::Workspace::open(root)?;
            for m in ws.store_members() {
                let label = if m.alias.is_empty() {
                    "(this library)".to_string()
                } else {
                    m.alias.clone()
                };
                let state = m.unavailable.as_ref().map_or_else(
                    || format!("{} skill(s)", m.skills),
                    |why| format!("unavailable: {why}"),
                );
                let age = m
                    .age
                    .as_ref()
                    .map_or_else(String::new, |age| format!("  synced {age}"));
                println!("{label}  {}  {state}{age}", m.source);
            }
            Ok(())
        }
        StoreCommand::Sync => {
            let ws = crate::workspace::Workspace::open_fetching(root)?;
            let results = ws.sync_all();
            if results.is_empty() {
                println!("no remote libraries declared");
            }
            let mut failed = false;
            for (alias, outcome) in results {
                match outcome {
                    Ok(()) => println!("{alias}  synced"),
                    Err(e) => {
                        failed = true;
                        eprintln!("{alias}  failed: {e}");
                    }
                }
            }
            if failed {
                std::process::exit(1);
            }
            Ok(())
        }
    }
}

/// Report the problems that the declarations create.
fn cmd_check(root: &Path) -> Result<(), Error> {
    let ws = crate::workspace::Workspace::open(root)?;
    let findings = ws.check(root);
    if findings.is_empty() {
        println!("no problems found");
        return Ok(());
    }
    println!("{} problem(s):", findings.len());
    for f in &findings {
        match f.kind.as_str() {
            "requirement" => println!("  {} {}", f.subject, f.detail),
            "shadowed" => println!("  '{}' {}", f.subject, f.detail),
            "unreachable library" => println!("  library '{}' {}", f.subject, f.detail),
            _ => println!("  '{}': {}", f.subject, f.detail),
        }
    }
    std::process::exit(1);
}

/// Serve this library over MCP.
///
/// A served library is always read-only. Almanac's writes are curation:
/// `add` and `accept` decide what the library vouches for, and that
/// decision is a person's. No access mode makes them callable.
fn cmd_serve(root: &Path, surfaces: &str, bind: Option<&str>) -> Result<(), Error> {
    let surfaces =
        mdstore::mcp::Surfaces::parse_list(surfaces).map_err(|e| Error::General(e.to_string()))?;
    let config = mdstore::mcp::ServeConfig::read_only(root, surfaces, "almanac");
    crate::serve::run(config, bind)
}

/// The library subcommands.
#[derive(clap::Subcommand, Clone, Copy)]
pub enum StoreCommand {
    /// List the libraries this library reads, in precedence order.
    List,
    /// Fetch the declared remote libraries into the local cache.
    Sync,
}

/// Run the standalone binary.
pub fn run(args: Args) -> Result<(), Error> {
    let root = Path::new(&args.root);
    // The curated library is a source. Without this, `almanac init`
    // and `almanac add` build a library that `almanac list` cannot
    // see, and the tool's main purpose fails on its own output.
    run_command(root, &manifest_sources(root), args.command)
}

/// The library directory that `almanac.yml` governs, as a source.
/// A directory with no manifest contributes nothing.
fn manifest_sources(root: &Path) -> Vec<SkillSource> {
    let mut sources = Vec::new();
    if let Ok(manifest) = crate::manifest::Manifest::load(root)
        && let Ok(library) = manifest.library_dir(root)
        && library.is_dir()
    {
        sources.push(SkillSource::Path {
            path: library.to_string_lossy().into_owned(),
        });
    }
    // Each declared library follows this one, in declaration order, so
    // a name that two libraries hold resolves to the nearer library.
    //
    // The workspace supplies the directory, because it applies the
    // containment guard to a dependency's own `library:` value. A
    // second copy of this resolution here would not, and a dependency
    // could then name any directory on the machine.
    if let Ok(ws) = crate::workspace::Workspace::open(root) {
        for dir in ws.declared_library_dirs() {
            sources.push(SkillSource::Path { path: dir });
        }
    }
    sources
}

/// Print one skill, or one file beside it.
///
/// A skill with a directory on this machine goes through the directory
/// path, which appends the references listing and reads one reference
/// file by name. A library that lives in git has no directory to scan,
/// and was invisible here while the MCP server served it in full, so
/// the workspace reads it the way the server does.
fn cmd_show(root: &Path, name: &str, sources: &[SkillSource]) -> Result<(), Error> {
    if skill::show(name, root, sources)? {
        return Ok(());
    }
    if let Ok(ws) = crate::workspace::Workspace::open(root) {
        let (skill_name, file) = match name.split_once('/') {
            Some((n, rest)) => (n, Some(rest.trim_start_matches("references/"))),
            None => (name, None),
        };
        if let Some(view) = ws.resolve(skill_name) {
            match file {
                None => {
                    print!("{}", ws.read_skill(&view)?);
                    return Ok(());
                }
                Some(wanted) => {
                    for (rel, bytes) in ws.skill_files(&view) {
                        if rel == wanted || rel.ends_with(&format!("/{wanted}")) {
                            print!("{}", String::from_utf8_lossy(&bytes));
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
    Err(Error::General(format!("skill '{name}' not found")))
}

fn cmd_list(root: &Path, sources: &[SkillSource]) {
    let entries = reachable_skills(root, sources);
    if entries.is_empty() {
        println!("No skills configured.");
        return;
    }
    for entry in &entries {
        println!("{:<30} {} [file]", entry.name, entry.description);
    }
}

fn cmd_index(root: &Path, sources: &[SkillSource]) {
    let entries = reachable_skills(root, sources);
    println!("{}", skill::format_index_json(&entries));
}

/// Every skill a command can reach, in precedence order.
///
/// The workspace answers first, because it reads a library wherever it
/// lives: a directory on this machine, or a git tree in the cache. A
/// directory scan sees only the first kind, and a git-declared library
/// was then invisible to `list`, `show` and `index` while the MCP
/// server served it in full. The curator must see what the agent sees.
///
/// Extra `--source` directories a host passes follow, and lose a name
/// the workspace already holds.
fn reachable_skills(root: &Path, sources: &[SkillSource]) -> Vec<skill::SkillEntry> {
    let mut entries = Vec::new();
    if let Ok(ws) = crate::workspace::Workspace::open(root) {
        for view in ws.skills().iter().filter(|v| !v.shadowed) {
            entries.push(skill::SkillEntry {
                name: view.skill.name.clone(),
                description: view.skill.description.clone(),
                source: skill::SkillLocation::File(view.skill.rel_path.clone()),
            });
        }
    }
    for entry in skill::index(root, sources) {
        if !entries.iter().any(|e| e.name == entry.name) {
            entries.push(entry);
        }
    }
    entries
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

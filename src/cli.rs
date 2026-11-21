use std::path::Path;

use clap::Parser;

use crate::error::Error;
use crate::skill;
use crate::source::SkillSource;

/// The one-line description. `--help` prints it and `prime` prints it, from
/// this one place, so the two cannot drift.
pub const ABOUT: &str = "Almanac curates agent skills and indexes them for agents to read";

#[derive(Parser)]
#[command(name = "almanac", version, about = ABOUT, max_term_width = 98)]
pub struct Args {
    // Handler: run (src/cli.rs), and mdstore::resolve::resolve_root.
    /// Directory that holds almanac.yml, taken literally
    ///
    /// No walk, and no fallback. Without --root, the nearest almanac.yml
    /// at or above the working directory is used. With none, a read uses
    /// the configured root library and a write needs --home. init
    /// creates almanac.yml in this directory, and the skills directory
    /// (--library) inside it.
    #[arg(long, global = true)]
    pub root: Option<std::path::PathBuf>,

    // Handler: run (src/cli.rs), and mdstore::resolve::resolve_root.
    /// Act on the configured root library (`almanac store root`),
    /// wherever the command runs
    ///
    /// A command that resolves no library refuses it: init, prime, docs,
    /// store root, and list, show, or index with --source.
    #[arg(long, global = true, conflicts_with = "root")]
    pub home: bool,

    /// Read the user config from this file instead of its fixed path.
    /// A test seam; a flag is visible where an env var is not. The last
    /// occurrence wins, so a wrapper can pin a default that a specific
    /// call still overrides.
    #[arg(long, global = true, hide = true, overrides_with = "user_config")]
    pub user_config: Option<std::path::PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

/// Whether a command needs a store at all, and whether it writes.
/// Exhaustive on the leaf action, so a new subcommand does not compile
/// until it is classified.
const fn intent(command: &Command) -> Option<mdstore::resolve::Intent> {
    use mdstore::resolve::Intent::{Read, Write};
    Some(match command {
        // Rootless: these never resolve a store. Init acts on its
        // literal root.
        Command::GenMan { .. }
        | Command::GenCompletions { .. }
        | Command::Prime
        | Command::Docs(_)
        | Command::Init { .. }
        | Command::Store(StoreCommand::Root(_)) => return None,
        // With explicit --source directories, listing needs no library
        // at all: the sources are the answer. Only a sourceless call
        // resolves a library.
        Command::List { sources }
        | Command::Show { sources, .. }
        | Command::Index { sources, .. }
            if !sources.is_empty() =>
        {
            return None;
        }
        Command::List { .. }
        | Command::Show { .. }
        | Command::Index { .. }
        | Command::Status
        | Command::Check
        | Command::Store(StoreCommand::List) => Read,
        // Curation writes the library; sync writes the cache and the
        // vendored trees; a served library is a standing surface that
        // must never rest on a fallback its clients cannot see.
        Command::Add { .. }
        | Command::Sync { .. }
        | Command::Update { .. }
        | Command::Remove { .. }
        | Command::Serve { .. }
        | Command::Store(StoreCommand::Sync) => Write,
    })
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

    // Handler: run_command, Command::Init (src/cli.rs), and init
    // (src/ops.rs).
    /// Create an almanac.yml manifest that governs a library directory
    ///
    /// Writes almanac.yml in the working directory, or in the --root
    /// directory, and creates the library directory beside it. Fails
    /// when almanac.yml already exists.
    Init {
        /// Library directory, relative to the manifest
        #[arg(long, default_value = "skills")]
        library: String,
    },

    // Handler: run_command, Command::Add (src/cli.rs), and add
    // (src/ops.rs).
    /// Fetch a skill, print its red-flag report, and vendor it with --accept
    ///
    /// Without --accept, nothing lands in the library: a git source is
    /// copied to .almanac-staged/<name>/ for inspection, a dev source is
    /// read in place, and the command exits 1. With --accept, the skill
    /// is copied into the library with an .almanac-origin stamp, and its
    /// entry is pinned in almanac.yml to a commit (git sources) and a
    /// content hash.
    ///
    /// A git source resolves --ref, or the default branch, to a commit
    /// unless --rev names one. A local repository is read in place; a
    /// remote one is fetched over http, https, or git://. ssh is
    /// refused.
    Add {
        /// github:owner/repo, owner/repo, git:<url>, or dev:<path>
        source: String,
        /// Manifest name. Must match the SKILL.md frontmatter name
        #[arg(long)]
        name: Option<String>,
        /// Skill directory within a git source. A dev: source names the
        /// skill directory itself
        #[arg(long)]
        path: Option<String>,
        /// Branch or tag to resolve
        #[arg(long = "ref")]
        r#ref: Option<String>,
        /// Exact commit to pin
        #[arg(long)]
        rev: Option<String>,
        /// Accept the staged content into the library. Trust on first use
        #[arg(long)]
        accept: bool,
    },

    // Handler: run_command, Command::Sync (src/cli.rs), and sync
    // (src/ops.rs).
    /// Write every pinned entry to disk
    ///
    /// Prints one line for each entry. A clean entry prints `ok`. An
    /// entry fetched again at its pin prints `synced`. A dev: snapshot
    /// that is not clean prints `skip` and is left as it is. An entry
    /// whose fetched content does not match the pinned hash prints
    /// `FAIL`; the library keeps its copy, and the command exits 1.
    Sync {
        /// Verify instead of writing: `FAIL` for an entry that is drifted,
        /// missing, or unreadable, and exit 1. A dev: snapshot prints
        /// `skip` whatever its state
        #[arg(long)]
        check: bool,
    },

    // Handler: run_command, Command::Update (src/cli.rs), and update
    // (src/ops.rs).
    /// Fetch upstream, print the red flags and the diff, then re-pin with --yes
    ///
    /// Resolves each entry's recorded ref again. An entry already at that
    /// commit prints `up-to-date`, and a dev: entry prints `skip`.
    /// Without --yes, nothing changes.
    Update {
        /// Entry names. Defaults to every entry
        ///
        /// A name that matches no entry is ignored while another name
        /// matches one; when none match, the command fails.
        names: Vec<String>,
        /// Apply the update and re-pin
        #[arg(long, short = 'y')]
        yes: bool,
    },

    // Handler: run_command, Command::Remove (src/cli.rs), and remove
    // (src/ops.rs).
    /// Remove an entry and its vendored directory. Refuses an unmanaged directory
    ///
    /// A directory without an .almanac-origin stamp is unmanaged, and
    /// almanac does not delete it.
    Remove {
        /// The entry name
        name: String,
    },

    // Handler: run_command, Command::Status (src/cli.rs), and list
    // (src/ops.rs).
    /// Print every manifest entry with its pin and drift state, then the unmanaged neighbors
    ///
    /// Each entry prints its name, the first nine characters of its
    /// commit (`-` for a dev: snapshot), its state, and its source. The
    /// state is `clean`, `drifted`, `missing`, or `unreadable`. A
    /// directory in the library that has no .almanac-origin stamp and no
    /// entry follows, marked `unmanaged`.
    Status,

    // Handler: run_command, Command::List (src/cli.rs), cmd_list and
    // reachable_skills (src/cli.rs).
    /// List the available skills with name, description, and source
    ///
    /// Covers this library and the libraries it declares in stores.yml,
    /// then each --source directory. A name that two libraries hold
    /// prints once, from the library that wins it.
    List {
        /// Skill source directory. Repeatable
        #[arg(long = "source", short = 's')]
        sources: Vec<String>,
    },

    // Handler: run_command, Command::Show (src/cli.rs), cmd_show
    // (src/cli.rs), and show_captured (src/skill.rs).
    /// Print the full SKILL.md content of one skill, after a line that
    /// names the skill's directory
    ///
    /// The first line names the absolute directory that holds the
    /// SKILL.md, so a relative path in the skill body resolves against
    /// it. A skill from a library that lives in git has no directory on
    /// this machine, and prints without that line. A skill directory
    /// with a references/ directory ends with a `## References` section,
    /// one `almanac show` command for each file. `almanac show
    /// <name>/references/<file>` prints one reference file. Exits 1 when
    /// the skill is not found.
    Show {
        /// The skill name to print
        name: String,

        /// Skill source directory. Repeatable
        #[arg(long = "source", short = 's')]
        sources: Vec<String>,
    },

    // Handler: run_command, Command::Index (src/cli.rs), cmd_index
    // (src/cli.rs), and format_index_md (src/ops.rs).
    /// Print a JSON index of the available skills
    ///
    /// The JSON is one array, with a `name`, `description`, and `source`
    /// for each skill. The skills are the ones `almanac list` prints.
    Index {
        /// Skill source directory. Repeatable
        #[arg(long = "source", short = 's')]
        sources: Vec<String>,
        /// Print a markdown skills index for an agent's context
        ///
        /// Its header names the command that loads a skill, with --home
        /// when --home chose the library.
        #[arg(long)]
        md: bool,
        /// Byte budget for --md
        ///
        /// The output degrades to name-only lines, then to a truncation
        /// note.
        #[arg(long, default_value_t = 4096)]
        max_bytes: usize,
    },

    // Handler: cmd_store and run_store_root (src/cli.rs).
    /// Manage the declared libraries
    #[command(subcommand)]
    Store(StoreCommand),

    // Handler: cmd_check (src/cli.rs), and Workspace::check
    // (src/workspace.rs).
    /// Check the libraries for dangling requirements and store problems
    ///
    /// Reports each of these:
    ///
    /// - a requires: entry that names no skill
    ///
    /// - a skill name that two libraries hold, with the library that
    ///   wins and each that loses
    ///
    /// - a declared library that is not available, or a stores.yml
    ///   declaration that cannot be followed
    ///
    /// - a file the scan skipped, such as a SKILL.md that cannot be
    ///   parsed, or a library that could not be loaded
    ///
    /// - a file beside a SKILL.md that cannot be read
    ///
    /// - in a library with `shared: true`, a declaration that another
    ///   clone cannot follow
    ///
    /// With no problem, prints `no problems found` and exits 0. Otherwise
    /// prints the count and one line for each problem, and exits 1.
    Check,

    // Handler: cmd_serve (src/cli.rs), and run (src/serve.rs).
    /// Serve this library over MCP
    ///
    /// The server is read-only, and no flag changes that. It has no
    /// authentication: bind it to 127.0.0.1, or put a proxy that
    /// authenticates in front of it. The library is chosen as for a
    /// write, so serve never falls back to the root library, and it
    /// names a library it found above the working directory. Tools: `almanac_list_skills`,
    /// `almanac_get_skill`, and `almanac_check`. Resources:
    /// `skill://<name>/SKILL.md` and `skill://<name>/references/<file>`.
    Serve {
        /// The surfaces to offer, separated by commas: skills,
        /// resources, tools
        ///
        /// Tools are the surface every client can call, so the default
        /// keeps them on.
        #[arg(long, default_value = "skills,tools")]
        surfaces: String,
        /// Where to listen, as host:port
        ///
        /// Omitted, the server speaks on stdin and stdout, for a client
        /// that starts it as a child process. Given, it serves over HTTP
        /// at /mcp, for a client that connects to it.
        #[arg(long)]
        bind: Option<String>,
    },

    // Handler: run_command, Command::Prime, and prime (src/cli.rs).
    /// Print what almanac is and how to use it, for an agent's context
    ///
    /// The output depends only on the binary version: no argument,
    /// config, or library changes it. When an agent reads it is the
    /// caller's choice.
    Prime,

    // Handler: cmd_docs (src/cli.rs), and diataxis::DocSet::render.
    /// Browse bundled documentation
    ///
    /// Without a topic, lists the pages and their slugs. A topic is a
    /// slug, a title, or a unique prefix of one.
    Docs(diataxis::DocsArgs),
}

/// This tool's own documentation, compiled in.
///
/// The build script embedded every page in `docs/`, so nothing here
/// lists them and `almanac docs` works from any directory.
static DOCS: &[(&str, &str)] = diataxis::embedded_docs!();

/// The prime: what almanac is, for an agent's context.
///
/// A pure function of the binary. It states the model and the commands
/// an agent reaches for. It names no other tool, no host, and no
/// location, and it directs nothing: when to run almanac is the caller's
/// policy. Under 700 bytes, checked by a test.
#[must_use]
pub fn prime() -> String {
    format!(
        "# almanac\n\
         {ABOUT}\n\
         A skill is a directory with a SKILL.md: frontmatter with a name and a one-line \
         description, then a body of instructions. A library is a directory with \
         almanac.yml, found upward from the cwd or named by --root. With none, reads use \
         the root library, writes need --home. Each entry is pinned to a commit and a \
         hash. stores.yml may declare other libraries; the nearer wins a name collision.\n\
         Commands:\n\
         \x20 almanac list\n\
         \x20 almanac show <name>\n\
         \x20 almanac index [--md] [--max-bytes <n>]\n\
         \x20 almanac status\n\
         \x20 almanac store list\n\
         More: almanac --help; almanac docs\n"
    )
}

/// Run one CLI command. A host that mounts almanac as a subcommand
/// calls this with its own sources.
pub fn run_command(root: &Path, sources: &[SkillSource], command: Command) -> Result<(), Error> {
    run_command_at(root, sources, command, false)
}

/// Run one CLI command. `home` is true when `--home` chose the root,
/// so text that tells an agent what to run next can carry it.
fn run_command_at(
    root: &Path,
    sources: &[SkillSource],
    command: Command,
    home: bool,
) -> Result<(), Error> {
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
            // anything, so it covers every reachable library and not
            // only the one the manifest names.
            let all_sources = merge_sources(sources, &extra_sources);
            if md {
                let entries = reachable_skills(root, &all_sources);
                print!("{}", crate::ops::format_index_md(&entries, max_bytes, home));
            } else {
                cmd_index(root, &all_sources);
            }
            Ok(())
        }
        Command::Prime => {
            print!("{}", prime());
            Ok(())
        }
        Command::Docs(args) => cmd_docs(&args),
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
        Command::Store(cmd) => cmd_store(root, &cmd),
        Command::Check => cmd_check(root),
        Command::Serve { surfaces, bind } => cmd_serve(root, &surfaces, bind.as_deref()),
        Command::Sync { check } => crate::ops::sync(root, check),
        Command::Update { names, yes } => crate::ops::update(root, &names, yes),
        Command::Remove { name } => crate::ops::remove(root, &name),
        Command::Status => crate::ops::list(root),
    }
}

/// The `store` subcommands.
fn cmd_store(root: &Path, cmd: &StoreCommand) -> Result<(), Error> {
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
        StoreCommand::Root(_) => {
            unreachable!("store root is rootless and handled in run()")
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
/// A served library is always read-only. The commands that write are
/// the curation commands: `add` and its `--accept` flag decide what the
/// library vouches for, and a person makes that decision at the command
/// line. No access mode makes them callable over MCP.
fn cmd_serve(root: &Path, surfaces: &str, bind: Option<&str>) -> Result<(), Error> {
    let surfaces =
        mdstore::mcp::Surfaces::parse_list(surfaces).map_err(|e| Error::General(e.to_string()))?;
    let config = mdstore::mcp::ServeConfig::read_only(root, surfaces, "almanac");
    crate::serve::run(config, bind)
}

/// The library subcommands.
#[derive(clap::Subcommand, Clone)]
pub enum StoreCommand {
    // Handler: cmd_store, StoreCommand::List (src/cli.rs), and
    // Workspace::store_members (src/workspace.rs).
    /// List the libraries this library reads, in precedence order
    ///
    /// The first row is this library. Each other row is a declared
    /// library: its alias, its source, and its skill count, or why it is
    /// unavailable. A library that a declared library declares follows
    /// it, under an alias path such as `shared/deep`. A library that
    /// lives in git also shows the age of its cache.
    List,
    // Handler: cmd_store, StoreCommand::Sync (src/cli.rs), and
    // Workspace::sync_all (src/workspace.rs).
    /// Fetch the declared remote libraries into the local cache
    ///
    /// Prints `synced` or `failed` for each library that lives in git or
    /// at a blob URL, and exits 1 when any fails. Along with add, update,
    /// and sync, this is one of four commands that reach the network.
    Sync,
    // Handler: run_store_root (src/cli.rs).
    /// Show or set the root library that reads fall back to
    ///
    /// The setting lives in ~/.config/almanac/config.yml, and the home
    /// directory comes from the passwd database, not $HOME. The path must
    /// hold almanac.yml. Each tool keeps its own setting, so a repository
    /// that also holds tisket.yml and zettel.yml can be the root for
    /// each of the three. A note on stderr names either file when it is
    /// missing.
    Root(StoreRootArgs),
}

#[derive(clap::Args, Clone)]
pub struct StoreRootArgs {
    /// The directory of the root library. Without it, print the
    /// current setting
    pub path: Option<std::path::PathBuf>,
    /// Replace an already-set root with a different one
    #[arg(long)]
    pub force: bool,
}

/// Run the standalone binary.
pub fn run(args: Args) -> Result<(), Error> {
    // Rootless commands never resolve a store, so `almanac prime` and
    // `almanac store root` work from any cwd with no config at all.
    let Some(intent) = intent(&args.command) else {
        if args.home {
            return Err(Error::General(
                "--home does not apply to this command; it names the root library for reads and writes".to_string(),
            ));
        }
        if let Command::Store(StoreCommand::Root(a)) = args.command {
            return run_store_root(&a, args.user_config.as_deref());
        }
        // A rootless command acts on the literal --root when one is
        // given, and on the working directory when none is.
        let cwd = std::env::current_dir().map_err(|e| Error::General(e.to_string()))?;
        let root = match &args.root {
            Some(r) if r.is_relative() => cwd.join(r),
            Some(r) => r.clone(),
            None => cwd,
        };
        return run_command(&root, &manifest_sources(&root), args.command);
    };
    let cwd = std::env::current_dir().map_err(|e| Error::General(e.to_string()))?;
    let config = load_user_config(args.user_config.as_deref())?;
    let resolved = mdstore::resolve::resolve_root(
        &cwd,
        args.root.as_deref(),
        args.home,
        intent,
        &config,
        VOCAB,
    )
    .map_err(|e| Error::General(e.to_string()))?;
    announce(&resolved.root, &cwd, resolved.via, intent);
    // The curated library is a source, and it comes from the resolved
    // root. Taken from the literal argument instead, a walked or
    // fallback run would list nothing and say nothing.
    let root = resolved.root;
    run_command_at(&root, &manifest_sources(&root), args.command, args.home)
}

const VOCAB: mdstore::resolve::Vocabulary<'static> = mdstore::resolve::Vocabulary {
    marker: "almanac.yml",
    noun: "library",
    tool: crate::TOOL,
};

fn load_user_config(path: Option<&Path>) -> Result<mdstore::userconfig::UserConfig, Error> {
    path.map_or_else(
        || mdstore::userconfig::UserConfig::load(VOCAB.tool),
        mdstore::userconfig::UserConfig::load_from,
    )
    .map_err(|e| Error::General(e.to_string()))
}

/// One stderr line whenever the answer to "where did that act" is not
/// visible in the command line itself. A read found by walking or by
/// fallback names its source; every write names its target.
fn announce(root: &Path, cwd: &Path, via: mdstore::resolve::Via, intent: mdstore::resolve::Intent) {
    use mdstore::resolve::{Intent, Via};
    match (intent, via) {
        (_, Via::Flag) => {}
        (Intent::Write, Via::Home) => {
            eprintln!("almanac: writing to root library {}", root.display());
        }
        // A write into the cwd's own library is what the command line
        // already says; only a target elsewhere needs naming.
        (Intent::Write, _) if root != cwd => {
            eprintln!("almanac: writing to library {}", root.display());
        }
        (Intent::Read, Via::Walk) if root != cwd => {
            eprintln!(
                "almanac: using library at {} (almanac.yml above {})",
                root.display(),
                cwd.display()
            );
        }
        (Intent::Read, Via::Config) => {
            eprintln!(
                "almanac: no almanac.yml at or above {}; reading root library {}",
                cwd.display(),
                root.display()
            );
        }
        (Intent::Read | Intent::Write, _) => {}
    }
}

/// `almanac store root [<path>] [--force]`: show or set the root
/// library in ~/.config/almanac/config.yml. Rootless, and acts on its
/// literal argument; resolution here would be circular.
fn run_store_root(args: &StoreRootArgs, user_config: Option<&Path>) -> Result<(), Error> {
    let config = load_user_config(user_config)?;
    let Some(path) = &args.path else {
        // The file that was read, never a literal: --user-config makes
        // any fixed path a possible lie.
        let read_from = user_config
            .map(std::path::Path::to_path_buf)
            .or_else(|| mdstore::userconfig::config_path(crate::TOOL));
        match (&config.root_store, &read_from) {
            (Some(r), Some(f)) => println!("root_store: {} ({})", r.display(), f.display()),
            (Some(r), None) => println!("root_store: {}", r.display()),
            (None, _) => println!("root_store: unset"),
        }
        return Ok(());
    };
    let cwd = std::env::current_dir().map_err(|e| Error::General(e.to_string()))?;
    let abs = if path.is_relative() {
        cwd.join(path)
    } else {
        path.clone()
    };
    if !abs.join("almanac.yml").is_file() {
        return Err(Error::General(format!(
            "no almanac.yml in {}; run `almanac init` there first",
            abs.display()
        )));
    }
    if let Some(old) = &config.root_store
        && *old != abs
        && !args.force
    {
        return Err(Error::General(format!(
            "root_store is {}; pass --force to change it to {}",
            old.display(),
            abs.display()
        )));
    }
    for sibling in ["tisket.yml", "zettel.yml"] {
        if !abs.join(sibling).is_file() {
            eprintln!(
                "almanac: note: {} has no {sibling}; the other tools will not treat it as their root",
                abs.display()
            );
        }
    }
    if config.root_store.as_deref() == Some(abs.as_path()) {
        println!("root_store: {} (unchanged)", abs.display());
        return Ok(());
    }
    let old = config.root_store;
    let target = user_config
        .map(std::path::Path::to_path_buf)
        .or_else(|| mdstore::userconfig::config_path(VOCAB.tool))
        .ok_or_else(|| Error::General("no home directory resolves".to_string()))?;
    mdstore::userconfig::UserConfig::save_root_to(&target, &abs)
        .map_err(|e| Error::General(e.to_string()))?;
    match old {
        Some(o) => println!(
            "root_store: {} -> {} ({})",
            o.display(),
            abs.display(),
            target.display()
        ),
        None => println!("root_store: {} ({})", abs.display(), target.display()),
    }
    Ok(())
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
/// so the workspace reads it the way the MCP server does.
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
                    print!("{}", ws.show_skill(&view)?);
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
/// directory scan sees only the first kind, so `list`, `show` and
/// `index` would miss a git-declared library that the MCP server
/// serves in full. The curator must see what the agent sees.
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

fn cmd_docs(args: &diataxis::DocsArgs) -> Result<(), Error> {
    let set = diataxis::DocSet::from_embedded(DOCS).map_err(|e| Error::General(e.to_string()))?;
    let request = args.request().map_err(|e| Error::General(e.to_string()))?;
    match set.render(request) {
        Ok(text) => {
            print!("{text}");
            Ok(())
        }
        Err(e) => {
            eprintln!("{e}");
            eprintln!();
            print!("{}", set.listing());
            Err(Error::General(e.to_string()))
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

#[cfg(test)]
mod prime_tests {
    use super::*;
    use clap::CommandFactory as _;

    /// The prime is a pure function of the binary and its shape is a
    /// contract that assemblers rely on. This test is the whole of that
    /// contract, in code.
    #[test]
    fn prime_has_the_contract_shape() {
        let p = prime();
        let lines: Vec<&str> = p.lines().collect();
        assert!(p.len() <= 700, "prime is {} bytes; the cap is 700", p.len());
        assert_eq!(lines[0], "# almanac");
        assert_eq!(lines[1], ABOUT, "line 2 is the --help about string");
        assert!(
            p.ends_with('\n') && !p.ends_with("\n\n"),
            "one trailing newline"
        );
        assert!(!p.contains('\t'), "no tabs");
        assert!(!p.contains("[gaff:"), "no spoofable prefix");
        assert!(
            !p.chars().any(|c| c.is_control() && c != '\n'),
            "no control chars"
        );
        assert!(
            lines.iter().skip(1).all(|l| !l.starts_with('#')),
            "no headings below line 1"
        );
        assert!(
            lines
                .last()
                .unwrap()
                .starts_with("More: almanac --help; almanac docs")
        );
        for word in [
            "gaff",
            "tisket",
            "zettel",
            "mdstore",
            "skills.sh",
            "Claude",
            "always",
            "never",
            "session start",
        ] {
            assert!(!p.contains(word), "prime must not say {word:?}");
        }
    }

    /// Every `Commands:` line resolves against the real command table,
    /// and every flag it names exists on that subcommand. This is the
    /// drift the prime exists to prevent, caught where it lives.
    #[test]
    fn every_prime_command_exists() {
        let p = prime();
        let cmd = Args::command();
        let start = p.find("Commands:\n").expect("a Commands: block") + "Commands:\n".len();
        let end = p.find("More:").expect("a More: line");
        for line in p[start..end].lines() {
            let mut words = line.split_whitespace();
            assert_eq!(
                words.next(),
                Some("almanac"),
                "{line:?} starts with the tool"
            );
            let mut node = &cmd;
            let mut rest: Vec<&str> = Vec::new();
            for w in words {
                if let Some(sub) = node.get_subcommands().find(|s| s.get_name() == w) {
                    node = sub;
                } else {
                    rest.push(w);
                }
            }
            assert!(node.get_name() != "almanac", "{line:?} names no subcommand");
            for w in rest {
                let flag = w.trim_start_matches('[').trim_end_matches(']');
                if let Some(long) = flag.strip_prefix("--") {
                    let long = long.split(' ').next().unwrap();
                    assert!(
                        node.get_arguments().any(|a| a.get_long() == Some(long)),
                        "{line:?}: `--{long}` is not a flag of `{}`",
                        node.get_name()
                    );
                }
            }
        }
        assert!(p[start..end].lines().count() <= 7, "at most seven commands");
    }
}

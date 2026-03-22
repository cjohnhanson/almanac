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
    about = "Agent skill aggregator — index and retrieve skills from pluggable sources",
    max_term_width = 98
)]
pub struct Args {
    /// Project root directory (default: current directory).
    #[arg(long, global = true, default_value = ".")]
    pub root: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser)]
pub enum Command {
    /// List all available skills (name + description + source).
    List {
        /// Skill source directories (repeatable).
        #[arg(long = "source", short = 's')]
        sources: Vec<String>,
    },
    /// Print the full SKILL.md content of a named skill.
    Show {
        /// The skill name to display.
        name: String,

        /// Skill source directories (repeatable).
        #[arg(long = "source", short = 's')]
        sources: Vec<String>,
    },
    /// Print a machine-readable JSON index of all available skills.
    Index {
        /// Skill source directories (repeatable).
        #[arg(long = "source", short = 's')]
        sources: Vec<String>,
    },
    /// Browse bundled documentation.
    Docs {
        /// Topic slug to display, or "search" to search.
        topic: Option<String>,
        /// Search query (when topic is "search").
        query: Option<String>,
    },
}

/// Run a CLI command (used when almanac is mounted as a subcommand by clc).
pub fn run_command(
    root: &Path,
    sources: &[SkillSource],
    command: Command,
) -> Result<(), Error> {
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
        } => {
            let all_sources = merge_sources(sources, &extra_sources);
            cmd_index(root, &all_sources);
            Ok(())
        }
        Command::Docs { topic, query } => cmd_docs(topic.as_deref(), query.as_deref()),
    }
}

/// Run from standalone binary (reads config or uses CLI args only).
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
        let source_label = match &entry.source {
            skill::SkillLocation::File(_) => "file",
            skill::SkillLocation::BuiltIn => "built-in",
        };
        println!("{:<30} {} [{}]", entry.name, entry.description, source_label);
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
                Ok(())
            } else {
                eprintln!("unknown doc: {identifier}");
                eprintln!();
                print!("{}", docs::format_list(docs::PAGES));
                Err(Error::General(format!("doc '{identifier}' not found")))
            }
        }
    }
}

/// Merge config-provided sources with CLI --source flags.
fn merge_sources(config_sources: &[SkillSource], cli_sources: &[String]) -> Vec<SkillSource> {
    let mut all: Vec<SkillSource> = config_sources.to_vec();
    for s in cli_sources {
        all.push(SkillSource::Path { path: s.clone() });
    }
    all
}

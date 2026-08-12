<!-- metadata
title: "What is Almanac?"
description: "Agent skill aggregation from pluggable sources"
type: explanation
-->

# What is Almanac?

Almanac indexes agent skills so agents can find them and load them. A
skill is a directory with a SKILL.md file. The YAML frontmatter
declares a name and a description. The body holds the instructions.

Almanac implements the agentskills.io specification for SKILL.md
parsing and frontmatter. An agent reads the list of names and
descriptions at session start. It loads the full content later with
`almanac show <name>`.

## Where skills come from

Almanac reads skills from directories on disk. The main directory is
the curated library that `almanac.yml` governs. Read `almanac docs
curation` for the manifest workflow.

Add more directories with `--source`. Each `--source` flag adds one
directory. Nothing is compiled into the binary.

Almanac vendors skills into the library from three kinds of source:

- `github:owner/repo` — a GitHub repository, pinned to a commit and a
  content hash
- `git:<url>` — any git server, pinned the same way
- `dev:<path>` — a local snapshot of a skill you develop alongside

## How agents use it

Print a markdown index and inject it into agent context at session
start:

    almanac index --md --max-bytes 4096

The agent then loads one skill with `almanac show <name>`, or it reads
the SKILL.md file directly. When clc mounts almanac, clc injects the
index and passes the source directories from `clc.yml`.

## SKILL.md format

Each skill lives in its own directory with a `SKILL.md` entry point:

```
my-skill/
├── SKILL.md           # Required. Instructions with YAML frontmatter.
└── ...                # Supporting files. The agent reads them on demand.
```

The agentskills.io spec requires `name` and `description` in the
frontmatter. The `name` must use lowercase letters, digits, and
hyphens. It must match the directory name. It must be 64 characters or
shorter. Almanac uses the directory name when `name` is absent.

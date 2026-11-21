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

`list`, `show`, and `index` read more directories with `--source`.
Each `--source` flag adds one directory. With `--source`, almanac does
not walk up to find a library: it reads the `--root` directory, or the
working directory, as it is. Nothing is compiled into the binary.

A library can also declare other libraries in `stores.yml`, under local
aliases. The nearer library wins a skill name: this library first, then
each declared library in declaration order. A skill declares the skills
it needs in a `requires:` frontmatter key, and an entry can name another
library with `alias:name`. Read `almanac docs composition` for the
model.

Almanac vendors skills into the library from three kinds of source:

- `github:owner/repo`: a GitHub repository, pinned to a commit and a
  content hash.
- `git:<url>`: any git server over http, https, or git://, or a local
  repository, pinned the same way.
- `dev:<path>`: a local snapshot of a skill you develop alongside.

## How agents use it

Print a markdown index and inject it into agent context at session
start. In a library with two skills:

```console
$ almanac index --md --max-bytes 4096
Available skills (show with `almanac show <name>`):
- **reviewing** — How to review a pull request.
- **writing** — How to write a commit message.

```

The agent then loads one skill with `almanac show <name>`, or it reads
the SKILL.md file directly. A context-lifecycle tool such as
[gaff](https://github.com/cjohnhanson/gaff) can inject the index into a
session on a cadence.

## SKILL.md format

Each skill lives in its own directory with a `SKILL.md` entry point:

```text
my-skill/
├── SKILL.md           # Required. Instructions with YAML frontmatter.
└── ...                # Supporting files. The agent reads them on demand.
```

The agentskills.io spec requires `name` and `description` in the
frontmatter. The `name` uses lowercase letters, digits, and hyphens, it
matches the directory name, and it is 64 characters or shorter. `list`,
`show`, and `index` use the directory name when `name` is absent.
`almanac add` requires `name`, and refuses a SKILL.md without it.

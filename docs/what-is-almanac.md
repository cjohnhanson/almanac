<!-- metadata
title: "What is Almanac?"
description: "Agent skill aggregation from pluggable sources"
type: explanation
-->

# What is Almanac?

Almanac indexes agent skills from multiple sources so agents can search and load them.
A skill is a directory containing a SKILL.md file — markdown with YAML frontmatter
that gives an agent procedural knowledge for a specific domain or task.

Almanac implements the core of the agentskills.io specification: SKILL.md parsing,
name/description frontmatter, and progressive disclosure (metadata at startup,
full content on demand). It does not yet support the optional directory conventions
(`scripts/`, `references/`, `assets/`) from the spec.

## Skill sources

Almanac reads skills from three kinds of sources:

- **Local paths** — directories on disk containing skill subdirectories
- **Git repos** — remote repositories cloned and cached locally (planned)
- **Built-in** — skills compiled into the almanac binary

Sources are configured in `clc.yml` under the `skills:` key, or passed via
`--source` flags on the command line.

## How agents use it

At session start, clc injects a skill index into the agent's context —
a flat list of skill names and descriptions. The agent decides when to load
a skill based on the description. Full content is retrieved via `almanac show <name>`
or by reading the SKILL.md file directly.

## SKILL.md format

Each skill lives in its own directory with a `SKILL.md` entry point:

```
my-skill/
├── SKILL.md           # Required — instructions with YAML frontmatter
└── ...                # Supporting files (read by the agent on demand)
```

Frontmatter requires `name` and `description` per the agentskills.io spec.
The `name` field must be lowercase alphanumeric with hyphens, match the
directory name, and be at most 64 characters. If `name` is omitted, almanac
falls back to the directory name.

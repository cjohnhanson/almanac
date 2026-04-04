<!-- metadata
title: "What is Almanac?"
description: "Agent skill aggregation from pluggable sources"
type: explanation
-->

# What is Almanac?

Almanac indexes agent skills from multiple sources so agents can search
and load them. A skill is a directory with a SKILL.md file: YAML
frontmatter declares name and description, the body holds instructions.

Almanac implements the agentskills.io specification for SKILL.md
parsing and frontmatter. At session start, agents see a list of skill
names and descriptions. Full content is loaded on demand via `almanac
show <name>`.

## Skill sources

Almanac reads skills from three kinds of sources:

- **Local paths** — directories on disk containing skill subdirectories
- **Git repos** — remote repositories cloned and cached locally (planned)
- **Built-in** — skills compiled into the almanac binary

Sources are configured in `clc.yml` under the `skills:` key, or passed via
`--source` flags on the command line.

## How agents use it

clc injects the skill index into agent context at session start.
Agents load full skill content via `almanac show <name>` or by reading
the SKILL.md file directly.

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

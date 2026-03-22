<!-- metadata
title: "CLI Reference"
description: "Complete command reference for the almanac skill aggregator"
type: reference
-->

# Almanac CLI Reference

## almanac list

List all available skills with name, description, and source type.

    almanac list [--source <path>]

## almanac show \<name\>

Print the full SKILL.md content for a named skill.

    almanac show <name> [--source <path>]

## almanac index

Print a machine-readable JSON index of all available skills.

    almanac index [--source <path>]

## almanac docs

Browse bundled almanac documentation.

    almanac docs                    List available docs (shows slugs)
    almanac docs list               Same as bare `almanac docs`
    almanac docs <identifier>       Print a doc by slug, title, or unique prefix
    almanac docs search <query>     Search across all docs

## Options

- `--source <path>`, `-s <path>` — Add a skill source directory. Repeatable.
- `--root <path>` — Project root directory (default: current directory).

## Mounted via clc

When used through clc, sources come from `clc.yml`:

```yaml
skills:
  - path: ./skills/
  - path: ~/Projects/co.d/skills/
```

`clc almanac list`, `clc almanac show <name>`, etc.

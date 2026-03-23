<!-- metadata
title: "Almanac CLI Reference"
description: "Complete command reference for the almanac skill aggregator"
type: reference
-->

# Almanac CLI Reference

```
almanac <command>
```

Agent skill aggregator — index and retrieve skills from pluggable sources.

## Global Options

`--root <path>` — Project root directory. Defaults to `.` (current directory). Applies to all subcommands.

`--version` — Print version and exit.

`--help` — Print help and exit.

## Commands

### `almanac list`

List all available skills with name, description, and source type.

```
almanac list [--source <path>]
```

| Option | Short | Description |
|--------|-------|-------------|
| `--source <path>` | `-s` | Skill source directory. Repeatable — each adds another source |

Output columns: `NAME`, `DESCRIPTION`, `[SOURCE_TYPE]`. Source types are `file` (local directory) or `built-in` (compiled into the binary).

### `almanac show <name>`

Print the full SKILL.md content for a named skill.

```
almanac show <name> [--source <path>]
```

| Argument/Option | Short | Description |
|-----------------|-------|-------------|
| `<name>` | | Skill name to display |
| `--source <path>` | `-s` | Skill source directory. Repeatable |

Prints the raw SKILL.md content to stdout. Exits with an error if the skill is not found.

### `almanac index`

Print a machine-readable JSON index of all available skills.

```
almanac index [--source <path>]
```

| Option | Short | Description |
|--------|-------|-------------|
| `--source <path>` | `-s` | Skill source directory. Repeatable |

Output is a JSON array of skill objects, each containing `name`, `description`, and source metadata.

### `almanac docs`

Browse bundled almanac documentation.

```
almanac docs                    List available docs (shows slugs)
almanac docs list               Same as bare `almanac docs`
almanac docs <identifier>       Print a doc by slug, title, or unique prefix
almanac docs search <query>     Search across all docs
```

## Mounted via clc

When used through clc, sources come from `clc.yml` under the `skills:` key:

```yaml
skills:
  - path: ./skills/
  - path: ~/Projects/co.d/skills/
```

Subcommands are available as `clc almanac list`, `clc almanac show <name>`, etc. Sources from `clc.yml` are merged with any `--source` flags passed on the command line.

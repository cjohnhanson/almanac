<!-- metadata
title: "Almanac CLI Reference"
description: "Complete command reference for the almanac skill aggregator"
type: reference
-->

# Almanac CLI Reference

```
almanac <command>
```

Almanac curates agent skills and indexes them for agents to read.

## Global Options

`--root <dir>` — Library directory, literal: the directory must hold
`almanac.yml`; no walk, no fallback.

`--home` — Act on the configured root library, wherever the command
runs.

`--version` — Print the version and exit.

`--help` — Print the help and exit.

## Root resolution

Without `--root`, a command finds its library by one rule, identical in
tisket and zettel: the nearest `almanac.yml` at or above the working
directory wins. The walk requires a regular file and stops at the first
directory the invoking user does not own, so a marker planted in a
shared ancestor captures nothing. With no library found, a read falls
back to the root library set in `~/.config/mdstore/config.yml` and says
so on stderr; a write never falls back — it fails and names `--home`.
No environment variable participates. The config path is fixed, and the
home directory comes from the passwd database, not `$HOME`: both are
repo-settable channels.

Every write prints its resolved target on stderr unless `--root` was
passed. A read resolved by walking or by fallback prints its source.

## Library commands

These commands read and write `almanac.yml`. Read `almanac docs
curation` for the manifest workflow and the trust model.

### `almanac init`

Create an `almanac.yml` manifest that governs a library directory.

```
almanac init [--library <dir>]
```

| Option | Description |
|--------|-------------|
| `--library <dir>` | Library directory, relative to the manifest. Defaults to `skills` |

### `almanac add <source>`

Fetch a skill, print its red-flag report, and vendor it with
`--accept`.

```
almanac add <source> [--name <name>] [--path <dir>] [--ref <ref>] [--rev <sha>] [--accept]
```

| Argument/Option | Description |
|-----------------|-------------|
| `<source>` | `github:owner/repo`, `owner/repo`, `git:<url>`, or `dev:<path>` |
| `--name <name>` | Manifest name. Must match the SKILL.md frontmatter name |
| `--path <dir>` | Skill directory within the source |
| `--ref <ref>` | Branch or tag to resolve |
| `--rev <sha>` | Exact commit to pin |
| `--accept` | Accept the staged content into the library |

Without `--accept`, almanac stages the skill, prints the report, and
exits 1. Nothing lands in the library.

### `almanac sync`

Write every pinned entry to disk.

```
almanac sync [--check]
```

| Option | Description |
|--------|-------------|
| `--check` | Verify instead of writing. Exits 1 when a pinned entry drifts |

Each entry prints one line: `ok`, `skip` for a `dev:` snapshot, or
`FAIL`.

### `almanac update [names...]`

Fetch upstream, print the red flags and the diff, and re-pin with
`--yes`.

```
almanac update [names...] [--yes]
```

| Argument/Option | Short | Description |
|-----------------|-------|-------------|
| `[names...]` | | Entry names. Defaults to every git-sourced entry |
| `--yes` | `-y` | Apply the update and re-pin |

Without `--yes`, almanac prints the report and changes nothing.

### `almanac remove <name>`

Remove an entry and its vendored directory.

```
almanac remove <name>
```

Almanac refuses to delete a directory without an `.almanac-origin`
stamp.

### `almanac status`

List every manifest entry with its pin, its drift state, and its
source. The drift state is `clean`, `drifted`, or `missing`. Unmanaged
neighbor directories follow, marked `unmanaged`.

```
almanac status
```

## Index commands

### `almanac list`

List the available skills with name, description, and source type.

```
almanac list [--source <path>]
```

| Option | Short | Description |
|--------|-------|-------------|
| `--source <path>` | `-s` | Skill source directory. Repeatable. Each flag adds one source |

Output columns: `NAME`, `DESCRIPTION`, `[SOURCE_TYPE]`. Skills come
from path sources (`file`). Nothing is compiled into the binary. The
curated library is the main source. Read `almanac docs curation`.

### `almanac show <name>`

Print the full SKILL.md content of one skill.

```
almanac show <name> [--source <path>]
```

| Argument/Option | Short | Description |
|-----------------|-------|-------------|
| `<name>` | | Skill name to print |
| `--source <path>` | `-s` | Skill source directory. Repeatable |

Almanac prints the raw SKILL.md content to stdout. It exits with an
error when the skill is not found.

### `almanac index`

Print a JSON index of the available skills.

```
almanac index [--source <path>] [--md] [--max-bytes <n>]
```

| Option | Short | Description |
|--------|-------|-------------|
| `--source <path>` | `-s` | Skill source directory. Repeatable |
| `--md` | | Print a markdown index of the library instead. Reads `almanac.yml` |
| `--max-bytes <n>` | | Byte budget for `--md`. Defaults to 4096 |

The JSON output is an array of skill objects. Each object holds `name`,
`description`, and source metadata.

Under the `--md` budget the output degrades in steps: full lines first,
then names only, then a truncation note.

### `almanac prime`

Print what almanac is and how to use it, for an agent's context. The
output depends only on the binary version: no arguments, config, or
store changes it. Put it into an agent's context; policy about when to
use almanac belongs to the caller.

### `almanac docs`

Read the bundled almanac documentation.

```
almanac docs                    List the available docs and their slugs
almanac docs list               Same as bare `almanac docs`
almanac docs <identifier>       Print one doc by slug, title, or unique prefix
almanac docs search <query>     Search every doc
```


## `almanac store list`

List the libraries that this library reads, in precedence order. The
first row is this library. Each other row shows a declared library, its
source, and its skill count. A remote library also shows the age of its
cache.

## `almanac store root`

Show or set the root library that reads fall back to. `almanac store
root` prints the current setting; `almanac store root <path>` writes it
to `~/.config/mdstore/config.yml` (the path must hold `almanac.yml`;
changing an existing setting needs `--force`). The file is shared with
tisket and zettel, so one private repo serves all three tools.

## `almanac store sync`

Fetch each declared remote library into the local cache. This is the
only command that reaches the network.

## `almanac check`

Report the problems that the declarations create:

- A `requires:` entry names no skill.
- Two libraries hold one skill name. The report gives the library that
  wins and each library that loses.
- A declared library is not available.
- A file could not be read.
- In a `shared: true` library, a clone cannot reach a declared library.

The command exits non-zero when it finds any. Read `almanac docs
composition` for the model.

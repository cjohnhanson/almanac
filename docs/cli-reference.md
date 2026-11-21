<!-- metadata
title: "Almanac CLI reference"
description: "Complete command reference for the almanac skill aggregator"
type: reference
-->

# Almanac CLI reference

Every visible command, with its `--help` output, followed by the rule
that picks the library a command acts on.

<!-- Begin generated CLI help: cargo test --test docs -->

## almanac

```text
Almanac curates agent skills and indexes them for agents to read

Usage: almanac [OPTIONS] <COMMAND>

Commands:
  init    Create an almanac.yml manifest that governs a library directory
  add     Fetch a skill, print its red-flag report, and vendor it with --accept
  sync    Write every pinned entry to disk
  update  Fetch upstream, print the red flags and the diff, then re-pin with --yes
  remove  Remove an entry and its vendored directory. Refuses an unmanaged directory
  status  Print every manifest entry with its pin and drift state, then the unmanaged neighbors
  list    List the available skills with name and description
  show    Print the full SKILL.md content of one skill, after a line that names the skill's
          directory
  index   Print a JSON index of the available skills
  store   Manage the declared libraries
  check   Check the libraries for dangling requirements and store problems
  serve   Serve this library over MCP
  prime   Print what almanac is and how to use it, for an agent's context
  docs    Browse bundled documentation
  help    Print this message or the help of the given subcommand(s)

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## almanac init

```text
Create an almanac.yml manifest that governs a library directory

Writes almanac.yml in the working directory, or in the --root directory, and creates the library
directory beside it. Fails when almanac.yml already exists.

Usage: almanac init [OPTIONS]

Options:
      --library <LIBRARY>
          Library directory, relative to the manifest
          
          [default: skills]

      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac add

```text
Fetch a skill, print its red-flag report, and vendor it with --accept

Without --accept, nothing lands in the library: a git source is copied to .almanac-staged/<name>/
for inspection, a dev source is read in place, and the command exits 1. With --accept, the skill
is copied into the library with an .almanac-origin stamp, and its entry is pinned in almanac.yml
to a commit (git sources) and a content hash.

A git source resolves --ref, or the default branch, to a commit unless --rev names one. A local
repository is read in place; a remote one is fetched over http, https, or git://. ssh is refused.

Usage: almanac add [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>
          github:owner/repo, owner/repo, git:<url>, or dev:<path>

Options:
      --name <NAME>
          Manifest name. Must match the SKILL.md frontmatter name

      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

      --path <PATH>
          Skill directory within a git source. A dev: source names the skill directory itself

      --ref <REF>
          Branch or tag to resolve

      --rev <REV>
          Exact commit to pin

      --accept
          Accept the staged content into the library. Trust on first use

  -h, --help
          Print help (see a summary with '-h')
```

## almanac sync

```text
Write every pinned entry to disk

Prints one line for each entry. A clean entry prints `ok`. An entry fetched again at its pin
prints `synced`. A dev: snapshot that is not clean prints `skip` and is left as it is. An entry
whose fetched content does not match the pinned hash prints `FAIL`; the library keeps its copy,
and the command exits 1.

Usage: almanac sync [OPTIONS]

Options:
      --check
          Verify instead of writing: `FAIL` for an entry that is drifted, missing, or unreadable,
          and exit 1. A dev: snapshot prints `skip` whatever its state

      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac update

```text
Fetch upstream, print the red flags and the diff, then re-pin with --yes

Resolves each entry's recorded ref again. An entry already at that commit prints `up-to-date`, and
a dev: entry prints `skip`. Without --yes, nothing changes.

Usage: almanac update [OPTIONS] [NAMES]...

Arguments:
  [NAMES]...
          Entry names. Defaults to every entry
          
          A name that matches no entry is ignored while another name matches one; when none match,
          the command fails.

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

  -y, --yes
          Apply the update and re-pin

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac remove

```text
Remove an entry and its vendored directory. Refuses an unmanaged directory

A directory without an .almanac-origin stamp is unmanaged, and almanac does not delete it.

Usage: almanac remove [OPTIONS] <NAME>

Arguments:
  <NAME>
          The entry name

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac status

```text
Print every manifest entry with its pin and drift state, then the unmanaged neighbors

Each entry prints its name, the first nine characters of its commit (`-` for a dev: snapshot), its
state, and its source. The state is `clean`, `drifted`, `missing`, or `unreadable`. A directory in
the library that has no .almanac-origin stamp and no entry follows, marked `unmanaged`.

Usage: almanac status [OPTIONS]

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac list

```text
List the available skills with name and description

Covers this library and the libraries it declares in stores.yml, then each --source directory. A
name that two libraries hold prints once, from the library that wins it. Every row ends in
`[file]`, whichever library holds the skill.

Usage: almanac list [OPTIONS]

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

  -s, --source <SOURCES>
          Skill source directory. Repeatable
          
          With --source, almanac does not walk up to find a library: it reads the --root
          directory, or the working directory, as it is. A relative --source path is read from
          that directory.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac show

```text
Print the full SKILL.md content of one skill, after a line that names the skill's directory

The first line names the absolute directory that holds the SKILL.md, so a relative path in the
skill body resolves against it. A skill from a library that lives in git has no directory on this
machine, and prints without that line. A skill directory with a references/ directory ends with a
`## References` section, one `almanac show` command for each file. `almanac show
<name>/references/<file>` prints one reference file. Exits 1 when the skill is not found.

show reads the library directories on this machine first (this library, each path: library, and
each --source directory) and a library declared with git: after them. When a git: library wins a
name that one of those directories also holds, show prints the directory's copy. list and index
give the git: library's copy.

Usage: almanac show [OPTIONS] <NAME>

Arguments:
  <NAME>
          The skill name to print

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

  -s, --source <SOURCES>
          Skill source directory. Repeatable
          
          With --source, almanac does not walk up to find a library: it reads the --root
          directory, or the working directory, as it is. A relative --source path is read from
          that directory.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac index

```text
Print a JSON index of the available skills

The JSON is one array, with a `name`, `description`, and `source` for each skill. The skills are
the ones `almanac list` prints.

Usage: almanac index [OPTIONS]

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

  -s, --source <SOURCES>
          Skill source directory. Repeatable
          
          With --source, almanac does not walk up to find a library: it reads the --root
          directory, or the working directory, as it is. A relative --source path is read from
          that directory.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

      --md
          Print a markdown skills index for an agent's context
          
          Its header names the command that loads a skill, with --home when --home chose the
          library.

      --max-bytes <MAX_BYTES>
          Byte budget for --md
          
          The output degrades to name-only lines, then to a truncation note.
          
          [default: 4096]

  -h, --help
          Print help (see a summary with '-h')
```

## almanac store

```text
Manage the declared libraries

Usage: almanac store [OPTIONS] <COMMAND>

Commands:
  list  List the libraries this library reads, in precedence order
  sync  Fetch the declared remote libraries into the local cache
  root  Show or set the root library that reads fall back to
  help  Print this message or the help of the given subcommand(s)

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac store list

```text
List the libraries this library reads, in precedence order

The first row is this library. Each other row is a declared library: its alias, its source, and
its skill count, or why it is unavailable. A library that a declared library declares follows it,
under an alias path such as `shared/deep`. A library that lives in git also shows the age of its
cache.

Usage: almanac store list [OPTIONS]

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac store sync

```text
Fetch the declared remote libraries into the local cache

Prints `synced` or `failed` for each library that lives in git or at a blob URL, and exits 1 when
any fails. Along with add, update, and sync, this is one of four commands that reach the network.

Usage: almanac store sync [OPTIONS]

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac store root

```text
Show or set the root library that reads fall back to

The setting lives in ~/.config/almanac/config.yml, and the home directory comes from the passwd
database, not $HOME. The path must hold almanac.yml. Each tool keeps its own setting, so a
repository that also holds tisket.yml and zettel.yml can be the root for each of the three. A note
on stderr names either file when it is missing.

Usage: almanac store root [OPTIONS] [PATH]

Arguments:
  [PATH]
          The directory of the root library. Without it, print the current setting

Options:
      --force
          Replace an already-set root with a different one

      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac check

```text
Check the libraries for dangling requirements and store problems

Reports each of these:

- a requires: entry that names no skill

- a skill name that two libraries hold, with the library that wins and each that loses

- a declared library that is not available, or a stores.yml declaration that cannot be followed

- a file the scan skipped, such as a SKILL.md that cannot be parsed, or a library that could not
be loaded

- a file beside a SKILL.md that cannot be read

- in a library with `shared: true`, a declaration that another clone cannot follow

With no problem, prints `no problems found` and exits 0. Otherwise prints the count and one line
for each problem, and exits 1.

Usage: almanac check [OPTIONS]

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac serve

```text
Serve this library over MCP

The server is read-only, and no flag changes that. It has no authentication: bind it to 127.0.0.1,
or put a proxy that authenticates in front of it. serve picks its library as a write does: it
never falls back to the root library. It names a library it found above the working directory on
stderr.

Tools: `almanac_list_skills`, `almanac_get_skill`, and `almanac_check`. Resources:
`skill://<name>/SKILL.md` and `skill://<name>/references/<file>`.

Usage: almanac serve [OPTIONS]

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --surfaces <SURFACES>
          The surfaces to offer, separated by commas: skills, resources, tools
          
          Tools are the surface every client can call, so the default keeps them on.
          
          [default: skills,tools]

      --bind <BIND>
          Where to listen, as host:port
          
          Omitted, the server speaks on stdin and stdout, for a client that starts it as a child
          process. Given, it serves over HTTP at /mcp, for a client that connects to it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac prime

```text
Print what almanac is and how to use it, for an agent's context

The output depends only on the binary version: no argument, config, or library changes it. When an
agent reads it is the caller's choice.

Usage: almanac prime [OPTIONS]

Options:
      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

## almanac docs

```text
Browse bundled documentation

Without a topic, lists the pages and their slugs. A topic is a slug, a title, or a unique prefix
of one.

Usage: almanac docs [OPTIONS] [TOPIC] [QUERY]

Arguments:
  [TOPIC]
          Topic slug to show, or "search" to search the docs

  [QUERY]
          Search query (when topic is "search")

Options:
  -a, --all
          Print every page whole, in one stream

      --root <ROOT>
          Directory that holds almanac.yml, taken literally
          
          No walk, and no fallback. Without --root, the nearest almanac.yml at or above the
          working directory is used. With none, a read uses the configured root library and a
          write needs --home. init creates almanac.yml in this directory, and the skills directory
          (--library) inside it.

      --home
          Act on the configured root library (`almanac store root`), wherever the command runs
          
          A command that resolves no library refuses it: init, prime, docs, store root, and list,
          show, or index with --source.

  -h, --help
          Print help (see a summary with '-h')
```

<!-- End generated CLI help -->

## Root resolution

Without `--root`, a command finds its library by one rule: the nearest
`almanac.yml` at or above the working directory wins. The walk requires
a regular file. It stops at the first directory the invoking user does
not own, so a marker planted in a shared ancestor captures nothing.

With no library found, a read falls back to the root library set in
`~/.config/almanac/config.yml`, and it says so on stderr. A write never
falls back. It fails and names `--home`.

The writes are `add`, `sync` (with or without `--check`), `update`, `remove`, `store sync`, and
`serve`. `serve` changes nothing, but a served library must never rest
on a fallback its clients cannot see, so it resolves as a write. The
reads are `list`, `show`, `index`, `status`, `check`, and `store list`.

No environment variable takes part in the resolution. The config path
is fixed, and the home directory comes from the passwd database rather
than from `$HOME`. A repository can set either channel, and neither one
decides where a command writes.

A write prints its resolved target on stderr unless the target is the
working directory's own library or `--root` named it. A read prints its
source when the walk found a library above the working directory, or
when it fell back to the root library.

`init`, `prime`, `docs`, and `store root` resolve no library. Neither
do `list`, `show`, and `index` when `--source` names a directory: they
read the `--root` directory, or the working directory, as it is.

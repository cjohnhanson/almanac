<!-- metadata
title: Composed libraries
description: Declaring other libraries, name precedence, and skill requirements.
type: explanation
-->

# Composed libraries

One library rarely holds every skill. Your own library holds what you
wrote, a team library holds what the team shares, and an upstream
library holds what somebody else maintains.

A library declares the other libraries that it draws on. The
declarations live in `stores.yml`, beside `almanac.yml`. The examples
on this page start in a library that holds a `writing` skill, beside a
team library in `../team-skills` and a git repository in `../upstream`:

```yaml {filename="stores.yml"}
format: 2
stores:
  - alias: shared
    path: ../team-skills
  - alias: upstream
    git: ../upstream
```

Each declaration takes a `path:`, a `git:` URL, or a `blob:` prefix. A
`git:` URL can also be an https URL, such as
`https://example.com/org/skills`.

A library that lives in git is read from a local cache, and
`almanac store sync` fills it. `almanac store list` then shows each
library in precedence order:

```console
$ almanac store sync
upstream  synced

$ almanac store list
(this library)  [CWD]  1 skill(s)
shared  ../team-skills  2 skill(s)
upstream  [PARENT]/upstream  1 skill(s)  synced just now

```

## Precedence

A skill name is the identity. Two libraries can hold the same name, so
one of them has to win.

The nearer library wins. This library comes first. Each declared
library follows, in the order that `stores.yml` lists them. A library
that a declared library declares comes after that one.

`list`, `show`, and `index` all give the winner, so the list shows what
an agent loads. Both this library and `shared` hold `writing`:

```console
$ almanac list
writing                        How to write a commit message. [file]
testing                        How the team tests. [file]
linting                        How to configure the linters. [file]

$ almanac show writing
Base directory for this skill: [CWD]/skills/writing

---
name: writing
description: How to write a commit message.
---

Write the subject line in the imperative mood.

```

## A shadowed name is reported

A skill that loses a name is still on disk. It is invisible to `show`,
which is a surprise if you do not know it happened.

`almanac check` reports every shadowed name, the library that won it,
and each library that also holds it. Run the command when a skill does
not behave the way its file reads.

## Requirements

A skill declares the skills it needs:

```markdown {filename="skills/reviewing/SKILL.md"}
---
name: reviewing
description: My review process.
requires:
  - shared:testing
  - writing
  - shared:linting
---

Run the tests, then read the diff.
```

An entry with no alias names a skill in the same library. An entry with
an alias names a skill in the library that the alias declares.

The text before the first colon is an alias only when the library
declares that alias. An entry that holds a colon for another reason
therefore keeps its meaning.

`almanac check` reports an entry that names no skill. An entry whose
prefix is not a declared alias is read as a plain name, so it is
reported the same way. Here `shared` holds no `linting`, and `writing`
is shadowed:

```console
$ almanac check
? 1
2 problem(s):
  reviewing requires shared:linting, which names no skill
  'writing' comes from (this library); also in shared

```

## Remote libraries

`almanac store sync` fetches each declared remote library into a local
cache. Four commands reach the network. `add` fetches one skill at the
commit it pins. `update` resolves each entry's ref again and fetches
the new commit. `sync` refetches an entry that is missing, drifted, or
unreadable, and `store sync` fills the cache for a declared library.
Every other command reads what is already vendored or cached, so an
answer never changes because of a fetch nobody asked for.

That matters where egress is controlled. Those four commands are the
ones to allow or to refuse.

A git library keeps one bare clone for each URL, and its skills are
read at the revision that the declaration names. Two libraries that
name different revisions of one URL share that clone.

`almanac store list` shows each library, its skill count, and, for a
remote library, the age of its cache.

## Direction

A library reads the libraries that it declares. It does not read the
libraries that declare it.

A private library can declare a shared one. A shared library must not
declare a private one, because the private library does not exist for
the other people who clone the shared one. `almanac check` reports a
declaration in a `shared: true` library that other clones could not
follow.

## Serving a library

`almanac serve` offers this library over MCP, so a client reads it
without a copy on disk.

- `almanac serve` speaks MCP on stdin and stdout, for a client that
  starts the process.
- `almanac serve --bind 127.0.0.1:8931` serves over HTTP at
  `http://127.0.0.1:8931/mcp`, for a client that connects.
- `almanac serve --surfaces skills,resources,tools` offers all three
  surfaces.

### Surfaces

A client cannot be asked which interface it understands. The protocol
makes `resources` and `tools` server capabilities: the server offers
them, and a client that cannot use one never calls it. So the choice is
configuration.

- `skills`: the skills extension, `skills/list` and `skills/get`. The
  server returns each skill's frontmatter as written and a digest for
  each of its files. That digest is what the extension asks a host to
  verify.
  Almanac already pins content by SHA-256, so the digests are the same
  ones the manifest uses.
- `resources`: one readable resource for each file, at
  `skill://<name>/SKILL.md` and `skill://<name>/references/<file>`.
- `tools`: `almanac_list_skills`, `almanac_get_skill`, and
  `almanac_check`. Every client can call a tool, so this surface is the
  floor that a served library can rely on.

The default is `skills,tools`: the extension for a client that
implements it, and tools for one that does not. Trim the set when you
know the client, because each tool definition costs context on every
request.

### Authentication

A served library has none. The server answers whoever opens the
connection.

Authentication belongs in front of the server, in a program built for
it. A reverse proxy terminates TLS and checks a token or an identity
provider.

Bind to `127.0.0.1` for a client on this machine. To serve anybody
else, put the server behind a proxy that authenticates, and let the
proxy decide who reaches it.

### What a served library allows

A served library is read-only, and no flag changes that. The commands
that write are the curation commands. `add` fetches a skill and prints
its red-flag report, and `--accept` vendors it and pins the content.
That decision belongs to a person at the command line. A remote caller
reads what the library already holds.

`serve` picks its library as a write does, although it changes nothing.
A served library must never rest on a fallback its clients cannot see,
so `serve` needs a library at or above the working directory, `--root`,
or `--home`, and it names a library it found above the working
directory on stderr.

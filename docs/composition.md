<!-- metadata
title: Composed libraries
description: Declaring other libraries, name precedence, and skill requirements.
-->

# Composed libraries

One library rarely holds every skill. A personal library holds the
skills you wrote. A team library holds the skills the team shares. An
upstream library holds the skills that somebody else maintains.

A library declares the other libraries that it draws on. The
declarations live in `stores.yml`, beside `almanac.yml`:

```yaml
format: 2
stores:
  - alias: shared
    path: ../team-skills
  - alias: upstream
    git: https://example.com/org/skills
```

Each declaration takes a `path:`, a `git:` URL, or a `blob:` prefix.

## Precedence

A skill name is the identity. Two libraries can hold the same name, so
one of them has to win.

The nearer library wins. This library comes first. Each declared
library follows, in the order that `stores.yml` lists them. A library
that a declared library declares comes after that one.

`list`, `show`, and `index` all give the winner, so the list shows what
an agent loads.

## A shadowed name is reported

A skill that loses a name is still on disk. It is invisible to `show`,
which is a surprise if you do not know it happened.

`almanac check` reports every shadowed name, the library that won it,
and each library that also holds it. Run the command when a skill does
not behave the way its file reads.

## Requirements

A skill declares the skills it needs:

```yaml
---
name: reviewing
description: My review process.
requires:
  - shared:testing
  - writing
---
```

An entry with no alias names a skill in the same library. An entry with
an alias names a skill in the library that the alias declares.

The text before the first colon is an alias only when the library
declares that alias. An entry that holds a colon for another reason
therefore keeps its meaning.

`almanac check` reports an entry that names no skill, and an entry that
uses an alias that the library does not declare.

## Remote libraries

`almanac store sync` fetches each declared remote library into a local
cache. It is the only command that reaches the network. Every other
command reads what the cache holds, so an answer never changes because
of a fetch that nobody asked for.

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

```sh
almanac serve                                   # stdin and stdout
almanac serve --bind 127.0.0.1:8931             # for clients that connect
almanac serve --surfaces skills,resources,tools
```

### Surfaces

A client cannot be asked which interface it understands. The protocol
makes `resources` and `tools` server capabilities: the server offers
them, and a client that cannot use one never calls it. So the choice is
configuration.

- `skills` — the skills extension, `skills/list` and `skills/get`. The
  server returns each skill's frontmatter as written and a digest for
  each of its files, which is what the extension asks a host to verify.
  Almanac already pins content by SHA-256, so the digests are the same
  ones the manifest uses.
- `resources` — one readable resource for each file, at
  `skill://<name>/SKILL.md` and `skill://<name>/references/<file>`.
- `tools` — `almanac_list_skills`, `almanac_get_skill`, and
  `almanac_check`. Every client can call a tool, so this surface is the
  floor that a served library can rely on.

The default is `skills,tools`: the extension for a client that
implements it, and tools for one that does not. Trim the set when you
know the client, because each tool definition costs context on every
request.

### Authentication

A served library has none. The server answers whoever opens the
connection.

This is deliberate. Authentication belongs in front of the server, in
something built for it: a reverse proxy that terminates TLS and checks
a token or an identity provider. Three tools each carrying their own
half-implementation would be three places to get it wrong.

Bind to `127.0.0.1` for a client on this machine. To serve anybody
else, put the server behind a proxy that authenticates, and let the
proxy decide who reaches it.

### What a served library allows

A served library is read-only, and there is no flag that changes it.
Almanac's writes are curation. `add` fetches a skill and prints its
red-flag report; `accept` vendors it and pins the content. Both decide
what the library vouches for, and that decision is a person's at the
command line. A remote caller reads what the library already holds.

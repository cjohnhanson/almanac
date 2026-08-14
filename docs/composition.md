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

# almanac

almanac curates agent skills. A skill is a directory that contains a
`SKILL.md` file. The file holds YAML frontmatter with a name and a
one-line description, then a body of instructions. `almanac list` shows
the available skills. `almanac show <name>` prints the full content of
one skill. An agent sees what it can do without reading every SKILL.md
first.

## Install

The package is `lmnc`, because `almanac` was taken on every registry. On
npm it is `@cjohnhanson/lmnc`, because the registry refuses `lmnc` as too
close to names it already holds. The command is `almanac`, and both
names install together.

```sh
cargo install --locked lmnc
brew install cjohnhanson/tap/almanac
uv tool install lmnc
npm install -g @cjohnhanson/lmnc
```

`cargo install` builds from source. It needs Rust 1.88 and a C
compiler. The other three carry a prebuilt binary for macOS and Linux,
x86-64 and arm64, published by a tagged release.

To build the unreleased `main` branch:

```sh
cargo install --locked --git https://github.com/cjohnhanson/almanac
```

Or run it without installing:

```sh
uvx lmnc list
npx @cjohnhanson/lmnc list
```

A release also carries prebuilt archives and a `.deb`, on the [releases
page](https://github.com/cjohnhanson/almanac/releases). Each archive
holds the binary and the man page. Install a `.deb` with `dpkg -i`: it
is a file, not a repository, so `apt-get install` does not reach it.

Check the install with `almanac --version`.

## Usage

```sh
almanac list                  # every skill, with descriptions
almanac show <name>           # the full SKILL.md content
almanac index                 # a JSON index for machines
almanac docs [topic]          # the bundled documentation
```

## How it works

`almanac.yml` governs one library directory. Almanac pins every entry
to a commit and a content hash, and stamps every vendored directory as
managed. `add` trusts a source on first use: it runs a mechanical
red-flag scan, and it needs an explicit `--accept`. `update` shows the
upstream diff and a fresh scan before it re-pins, and `sync --check`
fails when a pinned entry drifts.

```yaml
library: skills
skills:
- name: gaff
  source: github:cjohnhanson/gaff
  path: skills/gaff
  ref: main
  rev: 849b76e2…full sha…
  sha256: sha256-v1:…
```

The index is small enough to inject into agent context. `list` prints
one line per skill. `show` loads the instructions for one skill on
demand. `index --md --max-bytes 4096` prints a markdown skills index
that fits a byte budget. `prime` prints what almanac is and how to
use it, for an agent's context; it depends only on the binary version.

## Sources

- `github:owner/repo` (or bare `owner/repo`): pinned by commit and hash.
- `git:<url>`: any git server over https, git://, or a local path.
  Almanac tries a sha fetch, then the recorded ref, then a full fetch,
  all in-process on gix. No git program runs. An ssh URL is refused,
  because gix would spawn ssh for it.
- `dev:<path>`: a local snapshot of a skill you develop alongside.
  `sync --check` skips it and reports drift as information.

## Composed libraries

A library can draw on other libraries. Declare them in `stores.yml`,
each under a local alias:

```yaml
format: 2
stores:
  - alias: shared
    path: ../team-skills
  - alias: upstream
    git: https://example.com/org/skills
```

A skill name is the identity, and two libraries can hold the same name.
The nearer library wins: this library first, then each declared library
in declaration order. `list`, `show`, and `index` all give the winner,
so the list shows what an agent loads.

A skill that loses a name stays on disk. `almanac check` reports every
shadowed name and the library it came from.

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

An entry with no alias names a skill in the same library. `almanac
check` reports an entry that names no skill, and an entry that uses an
alias the library does not declare.

`almanac store list` shows the libraries and their skill counts.
`almanac store sync` fetches the remote ones into a local cache. Four
commands reach the network: `add`, `update`, `sync`, and `store sync`.
Every other command reads what is already vendored or cached, so an
answer never changes because of a fetch nobody asked for.

## Serving a library

```sh
almanac serve                          # stdin and stdout, for a child process
almanac serve --bind 127.0.0.1:8931    # for clients that connect
```

The server offers the skills extension, readable resources, and tools.
A client cannot be asked which it understands, so the surfaces are
configuration: `--surfaces skills,resources,tools`. The default is
`skills,tools`, because every client can call a tool. A served library
is read-only, and no flag changes that: curating a skill is a decision
a person makes at the command line. It also has no authentication, so
bind it to `127.0.0.1` or put an authenticating proxy in front.

Read `almanac docs composition` for the model.

## Documentation

- [What is Almanac?](docs/what-is-almanac.md): skill format, sources, design
- [Curation and pinning](docs/curation.md): the manifest workflow and the trust model
- [CLI reference](docs/cli-reference.md): every command and flag

## Related

Tools that keep their data as plaintext in the repository, where an
agent can read it:

- [tisket](https://github.com/cjohnhanson/tisket): an issue tracker. Markdown issues with YAML frontmatter, in the repository
- [zettel](https://github.com/cjohnhanson/zettel): zettelkasten notes for a repository
- [gaff](https://github.com/cjohnhanson/gaff): a context-lifecycle handler for coding agents
- [missouri](https://github.com/cjohnhanson/missouri): end-to-end tests as directed graphs of filesystem states
- [mdstore](https://github.com/cjohnhanson/mdstore): the frontmattered markdown library almanac indexes skills with

## License

MIT.

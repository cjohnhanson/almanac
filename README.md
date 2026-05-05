# almanac

A skill aggregator for coding agents. Skills are directories containing
a `SKILL.md` file with YAML frontmatter (name, one-line description) and
an instructional body. `almanac list` shows what's available; `almanac
show <name>` prints the full content. Agents discover what they know how
to do without reading every SKILL.md upfront.

## Install

```sh
cargo install --git https://github.com/cjohnhanson/almanac
```

## Usage

```sh
almanac list                  # every available skill, with descriptions
almanac show <name>           # full SKILL.md content
almanac index                 # machine-readable JSON index
almanac docs [topic]          # bundled documentation
```

## How it works

Skills come from two kinds of sources today: built-in (26 general-purpose
skills compiled into the binary) and local directories listed in a
project's `.almanac.yml`. Remote git repos are on the roadmap.

The point is to keep the index cheap. `list` returns a one-line
description per skill and fits comfortably in context; `show` loads
a single skill's full instructions only when the agent decides one is
relevant.

## Built-in skills

- **Review and evaluation** — `code-review-eval`, `architecture-eval`,
  `api-design-eval`, `security-review`, `performance-eval`,
  `design-review`, `library-first-eval`, `testing-strategy`,
  `product-eval`, `full-review`
- **Writing** — `writing-review`, `writing-docs-eval`,
  `writing-sentence-level`, `anti-slop`, `doc-coauthoring`,
  `doc-editing`, `readme-authoring`
- **Process and thinking** — `structured-thinking`, `research`,
  `debugging`, `tisket-writing`, `continuous-improvement`
- **QA** — `qa-cli`, `qa-web`
- **Tool integration** — `playwright-missouri`, `zettel`

## Configuration

Drop an `almanac.yml` at the project root to add local skill
directories on top of the built-ins:

```yaml
sources:
  - path: ./.skills
  - path: ~/skills
```

## Documentation

- [What is Almanac?](docs/what-is-almanac.md) — skill format, sources, design
- [CLI Reference](docs/cli-reference.md) — complete command documentation

## Related

Plaintext, git-tracked, agent-readable tooling:

- [tisket](https://github.com/cjohnhanson/tisket) — file-based issue tracker
- [zettel](https://github.com/cjohnhanson/zettel) — zettelkasten knowledge base
- [belmont](https://github.com/cjohnhanson/belmont) — secrets manager for LLM agents
- [mdstore](https://github.com/cjohnhanson/mdstore) — frontmattered markdown library
- [codelikecody](https://github.com/cjohnhanson/codelikecody) — workflow engine that bundles these

## License

MIT.

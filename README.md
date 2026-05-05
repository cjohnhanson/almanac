# 📖 almanac

> An almanac (historically spelled almanack) is a regularly published
> listing of a set of current information about one or multiple subjects.
> —Wikipedia

Almanac aggregates agent skills from multiple sources and makes them
discoverable. A skill is a directory containing a `SKILL.md` file with
YAML frontmatter that declares a name and one-line description. The
body holds the actual instructions.

## Install

```sh
cargo install --git https://github.com/cjohnhanson/almanac
```

## How it works

Skills come from three kinds of sources: local directories you point at,
built-in skills compiled into the binary, and (planned) remote git
repositories.

`almanac list` prints every available skill with its description.
`almanac show <name>` prints the full SKILL.md content. Coding agents
call these commands to discover what they know how to do, loading full
instructions only for skills relevant to the current task.

## Built-in skills

Almanac ships with 25 general-purpose skills compiled into the binary:

- **Review & evaluation** — `code-review-eval`, `architecture-eval`,
  `api-design-eval`, `security-review`, `performance-eval`,
  `design-review`, `library-first-eval`, `testing-strategy`,
  `product-eval`, `full-review`
- **Writing** — `writing-review`, `writing-docs-eval`,
  `writing-sentence-level`, `anti-slop`, `doc-coauthoring`, `doc-editing`
- **Process & thinking** — `structured-thinking`, `research`,
  `debugging`, `tisket-writing`, `continuous-improvement`
- **QA** — `qa-cli`, `qa-web`
- **Tool integration** — `playwright-missouri`, `zettel`

`almanac list` shows them all with descriptions. `almanac show <name>`
prints the full content of any one.

## Usage

```sh
almanac list                  # all available skills
almanac show <name>           # full SKILL.md content
almanac index                 # machine-readable JSON index
almanac docs [topic]          # bundled documentation
```

## Configuring extra sources

Drop a `.almanac.yml` (or `almanac.yml`) in your project to add local
skill directories on top of the built-ins:

```yaml
sources:
  - path: ./.skills
  - path: ~/skills
```

## Documentation

- [What is Almanac?](docs/what-is-almanac.md) — skill format, sources, progressive disclosure
- [CLI Reference](docs/cli-reference.md) — complete command documentation

## Related

Part of a loose ecosystem of plaintext, git-tracked, agent-readable
tooling.

- [tisket](https://github.com/cjohnhanson/tisket) — file-based issue tracker
- [zettel](https://github.com/cjohnhanson/zettel) — zettelkasten knowledge base
- [belmont](https://github.com/cjohnhanson/belmont) — secrets manager
- [mdstore](https://github.com/cjohnhanson/mdstore) — frontmattered markdown library
- [codelikecody](https://github.com/cjohnhanson/codelikecody) — workflow engine that bundles these

## License

MIT.

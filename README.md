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

almanac is the curation and reproducibility layer for agent skills.
Discovery and multi-agent installation belong to `npx skills` and the
registries; almanac is what they lack: a manifest with pins,
review-gated updates, and drift visibility for the skills you actually
run — in an ecosystem where audited public skills carry prompt
injection at a measured ~36%.

`almanac.yml` governs a library directory. Every entry is pinned
(commit + content hash) and every vendored directory is stamped as
managed. `add` is trust-on-first-use with a mechanical red-flag scan
and an explicit `--accept`; `update` shows the upstream diff and a
fresh scan before re-pinning; `sync --check` fails loudly on drift.
Nothing changes silently.

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

The index stays cheap for context injection: `list` is one line per
skill, `show` loads one skill's full instructions on demand, and
`index --md --max-bytes 4096` emits a budget-shaped skills index to
wire into [gaff](https://github.com/cjohnhanson/gaff) as a cadence-
refreshed prime section.

## Sources

- `github:owner/repo` (or bare `owner/repo`) — pinned by rev + hash
- `git:<url>` — any git server, with a sha → ref → full-clone fetch fallback
- `dev:<path>` — local snapshot for skills developed alongside; outside
  the `sync --check` contract, drift reported as info

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

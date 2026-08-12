<!-- metadata
title: Curation and pinning
description: The manifest workflow — add, sync, update, drift, and the trust model.
-->

# Curation and pinning

almanac is the reproducibility layer for agent skills. Discovery and
multi-agent installation belong to `npx skills` and the registries;
almanac is what they lack: a manifest with pins, review-gated updates,
and drift visibility, for the skills you actually run.

## The manifest

`almanac.yml` lives next to the library directory it governs:

    library: skills
    skills:
    - name: gaff
      source: github:cjohnhanson/gaff
      path: skills/gaff
      ref: main
      rev: <full commit sha>
      sha256: sha256-v1:<content hash>

Every vendored directory carries an `.almanac-origin` stamp — the
managed-set marker. `remove` refuses directories without it, and
`status` lists unmanaged neighbors without touching them.

## Trust model, stated plainly

`add` is trust on first use. The staged tree gets a mechanical red-flag
scan (tool-granting frontmatter, executable bits, non-markdown
payloads, invisible unicode, base64 blobs, pipe-to-shell commands, NUL
bytes) and nothing lands without `--accept`. After that, content is
pinned: `update` shows the upstream diff and a fresh scan, and re-pins
only with `--yes`. Nothing changes silently.

The manifest binds content against upstream drift and accident — not
against a compromised library repo, since both live together.

## Sources

- `github:owner/repo` (or bare `owner/repo`) — pinned by rev + hash
- `git:<url>` — any git URL, same pinning
- `dev:<path>` — a local snapshot for skills you develop alongside;
  outside the `sync --check` contract, drift shown as info

Fetch strategy for pins: direct sha fetch, then the recorded ref, then
a full clone — whichever the server permits.

## Integrity

    almanac sync            # materialize every pin
    almanac sync --check    # verify; exit 1 on pinned-entry drift
    almanac status          # pins, drift, unmanaged neighbors

## Context injection (gaff)

    almanac index --md --max-bytes 4096 > .gaff/sections/skills.md

emits a skills index sized for gaff's injection cap; wire it as a gaff
section with a refresh cadence and re-run after sync. Degradation under
the budget is explicit: full lines, then names, then a truncation note.

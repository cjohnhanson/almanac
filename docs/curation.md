<!-- metadata
title: Curation and pinning
description: The manifest workflow: add, sync, update, drift, and the trust model.
-->

# Curation and pinning

Almanac makes a library of agent skills reproducible. A manifest pins
every skill to a commit and a content hash. Updates pass through a diff
and a scan. Drift is visible. This page describes that workflow.

## The manifest

`almanac.yml` sits next to the library directory it governs:

    library: skills
    skills:
    - name: gaff
      source: github:cjohnhanson/gaff
      path: skills/gaff
      ref: main
      rev: <full commit sha>
      sha256: sha256-v1:<content hash>

Almanac writes an `.almanac-origin` stamp into every vendored
directory. The stamp marks the directory as managed. `remove` refuses a
directory without a stamp. `status` lists unmanaged neighbor
directories and does not touch them.

## The trust model

`add` trusts a source on first use. Almanac scans the staged tree for
red flags: tool-granting frontmatter, executable bits, non-markdown
payloads, invisible unicode, base64 blobs, pipe-to-shell commands, and
NUL bytes. Nothing lands without `--accept`.

Almanac then pins the content. `update` shows the upstream diff and a
fresh scan, and it re-pins only with `--yes`. No change lands without a
report.

The manifest binds the content against upstream drift and accident. It
does not protect against a compromised library repo, because the
manifest and the content live in the same repo.

## Sources

- `github:owner/repo` (or bare `owner/repo`) — pinned by commit and hash
- `git:<url>` — any git URL, pinned the same way
- `dev:<path>` — a local snapshot of a skill you develop alongside.
  `sync --check` skips it and reports drift as information.

To fetch a pinned commit, almanac tries a direct sha fetch, then the
recorded ref, then a full clone. It uses the first one the server
permits.

## Integrity

    almanac sync            # write every pinned entry to disk
    almanac sync --check    # verify; exit 1 when a pinned entry drifts
    almanac status          # pins, drift, and unmanaged neighbors

## Context injection (gaff)

    almanac index --md --max-bytes 4096 > .gaff/sections/skills.md

This prints a skills index that fits gaff's injection cap. Wire it in
as a gaff section, give it a refresh cadence, and run it again after
each sync. The output degrades in steps under the budget: full lines
first, then names only, then a truncation note.

---
title: 'no shell-outs: vendor.rs fetches with gix; ops.rs diffs with a library'
status: done
priority: null
assignee: null
due_date: null
labels: []
depends_on: []
created: 2026-08-16T19:54:34Z
updated: 2026-08-16T21:26:07Z
---

## Goal

`vendor::fetch_rev` runs git init/remote/fetch/checkout; it uses gix to fetch a rev from a URL and write the tree to a directory. `ops::show_diff` runs `git diff --no-index`; it uses a diff library. No `Command::new` remains in almanac.

## Why

Single-binary rule; see mdstore's issue of the same title. Introduced 2026-08-12 with the curation reshape.

## Scratch Notes

2026-08-16: done. vendor.rs on gix (local read in place; network fetch sha→ref→full; tree written by hand; ssh refused), ops.rs diff on similar. QA fixes: ref_map lists heads+HEAD (was tags only), symlink not followed in diff, denied names skipped, annotated tag pins its commit. Network add/update proven against github:cjohnhanson/gaff with a git shim on PATH: nothing spawned. Filed: --rev verbatim / .almanac-staged leftover.

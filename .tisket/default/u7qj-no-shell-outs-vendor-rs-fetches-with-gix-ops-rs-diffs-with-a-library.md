---
title: 'no shell-outs: vendor.rs fetches with gix; ops.rs diffs with a library'
status: in_progress
priority: null
assignee: null
due_date: null
labels: []
depends_on: []
created: 2026-08-16T19:54:34Z
updated: 2026-08-16T20:12:37Z
---

## Goal

`vendor::fetch_rev` runs git init/remote/fetch/checkout; it uses gix to fetch a rev from a URL and write the tree to a directory. `ops::show_diff` runs `git diff --no-index`; it uses a diff library. No `Command::new` remains in almanac.

## Why

Single-binary rule; see mdstore's issue of the same title. Introduced 2026-08-12 with the curation reshape.

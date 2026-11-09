---
title: 'prime: build it to the contract, and stop naming gaff in --help and the README'
status: done
priority: null
assignee: null
due_date: null
labels:
- prime
depends_on: []
created: 2026-08-16T19:08:38Z
updated: 2026-11-09T15:41:26Z
---

See mdstore's prime-contract issue. almanac has no prime today. Build one under 700 bytes with the shape test. Separately: 'almanac index --md' and '--max-bytes' help strings and the README sentence 'wire it into gaff as a prime section and refresh it on a cadence' all name a sibling tool, and the README advice is wrong on its own terms (a gaff section is file-only). Remove them; the relation between tools is the assembler's policy.

## Scratch Notes

> closed in triage 2026-11-09: almanac prime shipped and --help and docs stop naming a sibling tool, commit d600a5e (https://github.com/cjohnhanson/almanac/commit/d600a5e)

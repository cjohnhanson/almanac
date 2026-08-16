---
title: add --rev stores the string verbatim; a short sha or branch name is not a pin
status: todo
priority: null
assignee: null
due_date: null
labels: []
depends_on: []
created: 2026-08-16T21:02:31Z
updated: 2026-08-16T21:02:31Z
---

## Problem

`almanac add --rev ad14371` writes `rev: ad14371` (the user's string) to almanac.yml and the stamp; `update` then reports a phantom update against the full sha. `fetch_rev` resolves `{rev}^{commit}` but returns only the tree. Also `.almanac-staged/<name>` from a non-accepted add survives a later `--accept`. Found by QA on 2026-08-16.

## Fix

`fetch_rev` returns the resolved commit id; `add` stores that. `add --accept` removes the staged copy.

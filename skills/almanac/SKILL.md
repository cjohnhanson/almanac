---
name: almanac
description: Curate a pinned, review-gated library of agent skills with almanac — vendor skills from git repos or local paths into a manifest-governed directory, detect drift, gate updates behind diffs and red-flag scans, and emit a markdown skills index for context injection. Use when adding a skill to a repo's curated library, checking library integrity, or updating pinned skills.
---

# almanac

The library is governed by almanac.yml next to it. Everything vendored
is pinned (rev + content hash) and stamped; nothing changes silently.

## Rules

- Never edit files inside a vendored skill directory — fix upstream and
  `almanac update <name>`, or use a dev: source for local iteration.
- `almanac add <source>` stages and reports red flags; nothing lands
  without --accept. Treat the report as signals to read, not noise.
- Run `almanac sync --check` to verify library integrity (exit 1 on
  drift of any pinned entry; dev: snapshots are exempt).
- `almanac status` shows every entry's pin and drift state, plus
  unmanaged neighbor directories almanac will never touch.

## Common calls

    almanac add github:owner/repo --path skills/name --accept
    almanac add dev:../myrepo/skills/name --accept
    almanac update [name] --yes
    almanac index --md --max-bytes 4096   # gaff prime-section payload
    almanac show <name>

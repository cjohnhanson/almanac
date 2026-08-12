---
name: almanac
description: Curate a pinned, review-gated library of agent skills with almanac. Vendor skills from git repos or local paths into a manifest-governed directory, detect drift, gate updates behind diffs and red-flag scans, and print a markdown skills index for context injection. Use when adding a skill to a repo's curated library, checking library integrity, or updating pinned skills.
---

# almanac

`almanac.yml` governs the library directory next to it. Almanac pins
every vendored skill to a commit and a content hash, and it stamps
every vendored directory. No change lands without a report.

## Rules

- Never edit a file inside a vendored skill directory. Fix it upstream
  and run `almanac update <name>`, or use a `dev:` source to iterate
  locally.
- Run `almanac add <source>` to stage a skill and print its red-flag
  report. Nothing lands without `--accept`. Read the report; the flags
  are signals, not verdicts.
- Run `almanac sync --check` to verify the library. It exits 1 when a
  pinned entry drifts. It skips `dev:` snapshots.
- Run `almanac status` to see the pin and drift state of every entry.
  It also lists unmanaged neighbor directories, which almanac never
  touches.

## Common calls

    almanac add github:owner/repo --path skills/name --accept
    almanac add dev:../myrepo/skills/name --accept
    almanac update [name] --yes
    almanac index --md --max-bytes 4096   # gaff prime-section payload
    almanac show <name>

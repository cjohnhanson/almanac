---
name: almanac
description: Curate a pinned library of agent skills with almanac. Vendor skills from git or local paths, detect drift, gate updates behind diffs, and print a skills index. Use when adding, checking, or updating a repository's skills.
---

# almanac

`almanac.yml` governs the library directory next to it. Almanac pins
every vendored skill to a commit and a content hash, and stamps every
vendored directory as managed. `add` and `update` both print a report
before anything changes.

## Rules

- Never edit a file inside a vendored skill directory. Fix it upstream
  and run `almanac update <name>`, or use a `dev:` source to iterate
  locally.
- Run `almanac add <source>` to stage a skill and print its red-flag
  report. Nothing lands without `--accept`. Read the report before you
  accept: a flag marks content to inspect.
- Run `almanac sync --check` to verify the library. It exits 1 when a
  pinned entry drifts. It skips `dev:` snapshots.
- Run `almanac status` to see the pin and drift state of every entry.
  It also lists unmanaged neighbor directories, which almanac never
  touches.

## Common calls

    almanac add github:owner/repo --path skills/name --accept
    almanac add dev:../myrepo/skills/name --accept
    almanac update [name] --yes
    almanac index --md --max-bytes 4096   # the index, for an agent's context
    almanac show <name>

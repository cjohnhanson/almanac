<!-- metadata
title: Curation and pinning
description: "The manifest workflow: add, sync, update, drift, and the trust model."
type: explanation
-->

# Curation and pinning

A manifest pins every skill in the library to a commit and a content
hash, so the library rebuilds the same way on another machine. An
update passes through a diff and a scan before it re-pins, and
`sync --check` reports an entry that no longer matches its pin.

## The manifest

`almanac.yml` sits next to the library directory it governs. The
examples on this page start in a library made with `almanac init`, and
add a skill from a git repository beside it, `../upstream`, which holds
the skill in `skills/writing`.

`add` without `--accept` stages the skill, prints its red-flag report,
and exits 1. Nothing lands in the library:

```console
$ almanac add git:../upstream --path skills/writing
? 1
fetched 8c2a240f7c38782fe67fee883fded73966c1211d via local-read
red-flag scan: writing: clean
staged only; inspect [CWD]/.almanac-staged/writing and run again with --accept
remove [CWD]/.almanac-staged/writing to discard it
almanac: not accepted

```

With `--accept`, almanac copies the skill into the library and pins it
in the manifest:

```console
$ almanac add git:../upstream --path skills/writing --accept
fetched 8c2a240f7c38782fe67fee883fded73966c1211d via local-read
red-flag scan: writing: clean
added writing

$ cat almanac.yml
library: skills
skills:
- name: writing
  source: git:../upstream
  path: skills/writing
  ref: main
  rev: 8c2a240f7c38782fe67fee883fded73966c1211d
  sha256: sha256-v1:1b768d649363e68bcc4b337a2d047e5a2bcc1217297a14e5dcf2b5062254b68b

$ ls -A skills/writing
.almanac-origin
SKILL.md

```

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
fresh scan, and it re-pins only with `--yes`. Every command that
changes the library prints a report first.

When upstream commits a change to the skill:

```markdown {filename="SKILL.md"}
---
name: writing
description: How to write a commit message.
---

Write the subject line in the imperative mood. Keep it under 50
characters.
```

```console
$ mv SKILL.md ../upstream/skills/writing/SKILL.md

$ git -C ../upstream commit -q -am "Limit the subject line"

$ almanac update writing
== writing 8c2a240f7c38782fe67fee883fded73966c1211d -> bb5d4e7e34c75882d882256a7937bea45a84bf40
red-flag scan: writing: clean
 SKILL.md | +2 -1
 1 file(s) changed
--- a/SKILL.md
+++ b/SKILL.md
@@ -3,4 +3,5 @@
 description: How to write a commit message.
 ---
 
-Write the subject line in the imperative mood.
+Write the subject line in the imperative mood. Keep it under 50
+characters.
not applied; run again with --yes to accept

$ almanac update writing --yes
== writing 8c2a240f7c38782fe67fee883fded73966c1211d -> bb5d4e7e34c75882d882256a7937bea45a84bf40
red-flag scan: writing: clean
 SKILL.md | +2 -1
 1 file(s) changed
--- a/SKILL.md
+++ b/SKILL.md
@@ -3,4 +3,5 @@
 description: How to write a commit message.
 ---
 
-Write the subject line in the imperative mood.
+Write the subject line in the imperative mood. Keep it under 50
+characters.
updated writing

```

The manifest binds the content against upstream drift and accident. It
does not protect against a compromised library repo, because the
manifest and the content live in the same repo.

## Sources

- `github:owner/repo` (or bare `owner/repo`): pinned by commit and hash.
- `git:<url>`: any http, https, git://, or local git URL, pinned the
  same way. ssh is refused, because almanac spawns no ssh process.
- `dev:<path>`: a local snapshot of a skill you develop alongside.
  `sync --check` skips it and reports drift as information.

Almanac reads a local repository in place. To fetch a pinned commit
from a server, almanac tries a direct sha fetch, then the recorded ref,
then a full clone. It uses the first one the server permits.

## Integrity

`sync --check` verifies every pinned entry, and `status` prints the
pins, the drift state, and the unmanaged neighbors:

```console
$ almanac sync --check
ok    writing

$ mkdir skills/notes

$ almanac status
writing                      bb5d4e7e3 clean    git:../upstream
notes                        -         -        unmanaged

```

An edit to a vendored file is drift:

```markdown {filename="skills/writing/SKILL.md"}
---
name: writing
description: How to write a commit message.
---

Write anything.
```

```console
$ almanac status
writing                      bb5d4e7e3 drifted  git:../upstream
notes                        -         -        unmanaged

$ almanac sync --check
? 1
FAIL  writing (drifted)
almanac: 1 entry failed

```

`sync` writes every pinned entry back to disk at its pin:

```console
$ almanac sync
synced writing (via local-read)

$ almanac status
writing                      bb5d4e7e3 clean    git:../upstream
notes                        -         -        unmanaged

```

## Context for an agent

`almanac prime` and `almanac index --md` both emit text meant for an
agent's context. Neither one decides when it is read. That is the
caller's policy.

```console
$ almanac prime
# almanac
Almanac curates agent skills and indexes them for agents to read
A skill is a directory with a SKILL.md: frontmatter with a name and a one-line description, then a body of instructions. A library is a directory with almanac.yml, found upward from the cwd or named by --root. With none, reads use the root library, writes need --home. Each entry is pinned to a commit and a hash. stores.yml may declare other libraries; the nearer wins a name collision.
Commands:
  almanac list
  almanac show <name>
  almanac index [--md] [--max-bytes <n>]
  almanac status
  almanac store list
More: almanac --help; almanac docs

```

What almanac is, its model, and the commands an agent reaches for. The
output depends only on the binary version and stays under 700 bytes.
Put it wherever your host primes a session, and it stays exact until
almanac is upgraded.

```console
$ almanac index --md --max-bytes 4096
Available skills (show with `almanac show <name>`):
- **writing** — How to write a commit message.

```

The skills index, as markdown, under a byte budget. The output degrades
in steps under the budget: full lines first, then names only, then a
truncation note. Regenerate it after each sync.

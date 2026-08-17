---
title: 'almanac: a named pipe hangs the CLI and the served library'
status: todo
priority: null
assignee: null
due_date: null
labels:
- review-followup
depends_on: []
created: 2026-08-17T02:45:49Z
updated: 2026-08-17T02:45:49Z
---

The round-three fresh-eyes review returned LAND and left four items.
The last one is the most serious and predates all of this work.

1. A named pipe in a skill directory hangs almanac forever, and hangs
   the MCP server with it.

       mkfifo skills/s/references/pipe.md
       almanac show s/references/.env   -> hangs
       almanac check                    -> hangs

   Both are unguarded ambient reads in src/workspace.rs: std::fs::read
   in collect_files and in collect_problems. Each walker skips a
   symlink by type and never checks for a regular file, so opening the
   pipe blocks until a writer arrives and none is coming. src/serve.rs
   calls skill_files from two places, so a served library hangs an
   agent's tool call and it never returns.

   mdstore added its own regular-file guard for exactly this, and runs
   that test on a separate thread so a hang cannot take down the suite.
   The consumer kept the unguarded read.

2. A check-then-open race between is_real_directory and
   StoreDir::open, two syscalls wide. Measured: 0 wins in 40000
   attempts unmodified; 45 wins in 2000 with a 200 microsecond sleep
   inserted between the two calls, which is what proves the window is
   real rather than merely narrow.

   The race-free shape needs no mdstore change: hold the handle on the
   skill directory, which scan_directory already type-checked, and read
   the relative path references/<file> through it. One resolution, no
   pair. It differs on one row: a link that stays inside the skill
   directory would start being accepted, and almanac check currently
   reports such a link as a problem. That is a decision about what a
   library may ship, not something to fold into a security fix.

   Winning the race needs a live adversarial process with write access
   to the library, concurrent with the read. Inert content cannot do
   it, and an attacker with that access can usually put the target in
   references/ directly.

3. The comment above the listing filter claims the listing and
   show_reference apply the same test. They do not. A dot-named file is
   listed by is_document, refused by show_reference because
   is_plain_stem rejects a leading dot, and then served anyway by the
   workspace fallback in cmd_show. The halves agree because a third
   path catches it. Correct today, and the comment claims a guarantee
   the code does not make.

4. A hard link out of the references directory still reads. It has no
   target to inspect and its metadata says regular file. Noted on the
   guard already. git does not carry one; tar does, and so does a
   hand-copied library.

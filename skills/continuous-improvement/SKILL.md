---
name: continuous-improvement
description: >
  Set up and run the driver-tick loop for a long-running project session.
  A scheduled prompt fires on a regular cadence with project-specific
  safety rails, dispatches fresh-eyes reviewers across whatever review
  dimensions the project cares about, and applies mechanical catches
  inline. The driver does not farm out implementation or tisket pickup;
  those stay in the main loop. Use when the user wants continuous review
  coverage on a long-running project, or when the user invokes
  /continuous-improvement. Not for ad-hoc questions or single-pass work.
user-invocable: true
---

# Continuous Improvement

The driver loop keeps a session productive across a multi-hour work
window. Without the loop, the session coasts: count updates, stack
checks, tick reports. With the loop, a scheduled prompt fires on a
regular cadence and tells the session to run a fresh-eyes pass against
code that has landed since the last tick, so review dimensions don't go
stale and new features don't ship unreviewed.

Implementation work and tisket pickup stay in the main loop. The user
is actively collaborating on those and catching issues mid-keystroke;
handing them to a subagent loses that feedback loop and arrives as a
black-box diff after the fact. Subagents in this skill are read-only:
reviews, research, lookups.

## Step 1: Set the cadence

Schedule a recurring prompt (via `CronCreate`, `/loop`, or the project's
preferred mechanism) at whatever interval matches how fast new code
lands. The prompt should contain the full safety rails and the tick
instructions, so each firing is self-contained and the session doesn't
need to remember state.

The prompt MUST include:

1. **Hard safety rails.** Project-specific rules that are
   irreversible-mistake territory if violated. The list comes from the
   project's CLAUDE.md and the user's standing instructions.
2. **Development loop rules.** Who owns the dev environment. What
   operations are allowed (read-only inspection) vs forbidden (direct
   mutation of shared state, bypassing the project's standard build
   path). How to recover from broken states.
3. **Firing checklist.** What to do on every tick: stack check, read
   scratch for notes since last tick, check in-flight subagents,
   dispatch work if idle, apply mechanical catches inline, codify user
   catches into skills.
4. **Dispatch menu.** Round-robin options: fresh-eyes review per
   dimension that applies to the project (code quality, testing, API
   contracts, docs, UI QA if there's a UI), harness polish, targeted
   research or lookup.
5. **Iteration cadence.** "The cron prevents idling, not paces work.
   Never wait for the next tick to do something. If a task is in flight,
   finish it. If a slice is ready to land, land it now. Ticks re-anchor
   the loop when the session would otherwise coast; they are not the
   unit of work. 'Defer to next tick' is a failure mode." Reviews
   rotate so no single dimension goes stale for long, but rotation is
   about review freshness, not about holding back work.
6. **Report format.** "Keep reports brief: what was landed + what's
   next. Never write 'next tick will do X' when X can be done now —
   just do it before reporting."

The prompt is project-specific. A good template lives in the project's
`.scratch/driver-tick-prompt.md` so the user can review and iterate.

## Step 2: What each tick does

1. **Stack check.** Verify the dev environment is up (build green, tests passing, services responding, dependent processes running). Fix forwards if stale. Use the project's standard restart path.

2. **Read scratch.** List `.scratch/*-notes.md`, `*-review.md`, `*-report.md` sorted by mtime. Three-sentence summary of what moved since last tick.

3. **Check in-flight subagents.** If agents are busy, let them finish. Never duplicate scope. An agent running code review must not be shadowed by another agent reviewing the same code.

4. **If idle, dispatch MULTIPLE subagents in parallel (not one).** A
   tick is a launch window, not a single-task slot. A single subagent
   leaves the main session idle for most of the interval. Send several
   independent subagents in ONE message (parallel tool-use block) so
   they run concurrently. Typical parallel set per tick: 2-4 subagents
   drawn from different dimensions so findings don't collide. A
   workable default fan-out:
   - one fresh-eyes review (rotated: code / API / testing / coverage /
     docs / UI QA)
   - one targeted research or lookup (library docs, prior art, spec
     reading) that would otherwise burn main-loop context
   - one harness / docs / workflow polish task if open findings warrant

   Rules:
   - **Every subagent runs on the project's review-grade model.**
     Pass the `model` parameter explicitly on every Agent call, even
     when it feels redundant. Defaults may be smaller than policy
     specifies; omitting the parameter silently downgrades. Review
     dimensions require judgment; a quietly smaller model is the
     single fastest way to produce false "all green" summaries.
   - **Non-overlapping scope.** Two agents may not touch the same files
     or review the same surface. If scopes would overlap, pick one and
     queue the other for the next tick.
   - **All background.** Use `run_in_background: true` so the tick can
     apply mechanical catches inline while the subagents run.
   - **In-flight agents stay in-flight.** Before dispatching new ones,
     list running agents and drop any scope that would duplicate them.

   Dimensions to rotate across (pick 2-4 per tick):
   - Fresh-eyes code review (type smells, anti-slop, architecture drift)
   - Fresh-eyes API review (contract vs code drift, error shape, Richardson)
   - Fresh-eyes testing review (tests-test-what-matters)
   - Coverage fresh-eyes review (per-module behavior vs assertions)
   - Fresh-eyes docs review (factual drift, audience fit, slop)
   - Fresh-eyes UI QA via agent-browser (qa-web skill) for projects with a UI
   - Harness / docs / workflow polish if open findings
   - Targeted research or library-doc reading that would otherwise
     crowd the main-loop context

5. **Apply mechanical catches inline.** Em dashes, banned phrases, stale doc claims, version refs in prose, type-check / lint warnings the project considers must-fix. No subagent needed for these. Grep, fix, verify.

6. **Every new user catch gets codified.** Update `skills/<name>/SKILL.md` with the specific rule and motivating example. Not scratch. Not memory. The skill file is the codification.

7. **Report briefly.** A few sentences — what was landed, what's next, any blocker.

## Step 3: Dispatch patterns

### Fresh-eyes review subagent brief

```
Fresh-eyes [DIMENSION] review of [PROJECT].

Worktree: [PATH]

Zero prior context. Read cold. Read CLAUDE.md first. Then skills/[relevant-skill]/SKILL.md.

[PRIOR CONTEXT: what was fixed last iteration, what the reviewer should re-verify.]

Check:
1-N. [dimension-specific heuristics from the skill]

For each finding: file:line, severity (blocker/major/minor), what's wrong, what the fix is.

If zero new findings: "CONVERGED". Keep it brief.
```

Key constraints:
- Specify the skill to load. Subagents don't know which skill applies without direction.
- Require severity ratings. Without severity, every finding looks equal and the fix pass can't prioritize.
- Force a convergence signal ("CONVERGED" or findings list). Without it, the loop can't exit.

### Research / lookup subagent brief

```
Research [TOPIC] for [PROJECT].

Worktree: [PATH]

Scope: answer [SPECIFIC QUESTION], returning the minimal set of facts
the main loop needs to proceed. Read docs, scan prior art, quote
authoritative sources. Do not edit project files; this is a
read-only pass.

Return: a short summary with citations (URLs, file:line). Flag any
place the answer contradicts assumptions in the plan.
```

## Step 4: Don't coast

The most common failure mode: after convergence on a few dimensions, the loop declares "steady state" and starts doing count updates every tick. This is the failure mode the driver exists to prevent.

Indicators of coasting:
- "All dimensions converged" when new features shipped since last review
- "Session complete" when tiskets are still `todo`
- "Stack healthy, no work to do" when the user-facing surface has never been QA'd
- Multiple consecutive ticks doing test-count updates in README/CLAUDE.md

The rule: if the tick would be a count update or a report rewrite, it is not a tick. Find real work: an untested surface, an unreviewed feature, an open tisket, a stale review dimension.

## Step 5: Stop conditions

The driver stops when:
- User explicitly says stop.
- The safety rails are at risk (ambiguous instruction that could push to remote, touch real data).
- The main deliverable is complete AND every tisket is closed AND every dimension is converged AND the user has not indicated further work.

Otherwise: keep going. The cron is the instruction to do the job.

## Anti-patterns

- **One subagent does everything.** The review subagent should review, not fix. The fix subagent should fix, not review. Separate them so the next review is fresh.
- **Dispatching a single subagent per tick.** A single subagent leaves the main session mostly idle. Default fan-out is 2-4 parallel subagents spanning different dimensions, sent in ONE message so they run concurrently. The single-agent pattern treats the cron like a one-task queue; it is meant to be a launch window.
- **Dispatching before checking in-flight agents.** Duplicate scope wastes time and produces conflicting changes.
- **Codifying catches to scratch instead of `skills/`.** Scratch is ephemeral. Skills are the durable record. A catch in scratch will not prevent the same miss next session.
- **"Let me know if you want me to do X."** Never. Just do it. The cron is the direction.
- **Count updates as tick work.** The test count in CLAUDE.md is a derived value. Update it when it changes, but the update is not the tick. The tick is what made the count change.
- **"Tick report" as the point of the tick.** The user does not want a report. The user wants the job done. If a tick produces a report and zero changed behavior, it is coasting. Stack check + scratch scan + "deferred to next tick" is ceremony, not work. The report is a footer on real progress, not a substitute for it.

- **Waiting for a cron fire to continue work.** The cron is a safety net against an idle session, not a pacing mechanism. If a slice is ready to wire, wire it now. If verify is green and the next slice's shape is clear, advance now. "Next tick will wire X" is the exact failure mode this skill exists to prevent. The tick is meant to prevent idling, not to license it. Continue driving the slice to completion; use cron-fires to catch up on review dispatch and report cadence, never to gate implementation.

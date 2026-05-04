---
name: full-review
description: >
  Dispatch all review skills as independent fresh-context subagents
  against the current diff or working tree. Each subagent reads its skill
  file and the changes cold, reports severity-rated findings, and merges
  results after all complete. Use when the user invokes /full-review or
  asks for a comprehensive review pass.
user-invocable: true
---

# Full Review

Dispatch one fresh subagent per applicable review skill, in parallel,
with no shared context between subagents. After all complete, merge
their findings into a single severity-ranked list.

## Skills to dispatch

Pick the subset that applies based on what changed.

Always-relevant for code changes:

- `code-review-eval` — code quality, SOLID, smells, complexity
- `architecture-eval` — system structure, coupling, cohesion, ATAM
- `library-first-eval` — homegrown vs available library audit
- `testing-strategy` — test shape, quadrants, coverage portfolio
- `security-review` — OWASP, STRIDE, input validation, auth, secrets

Context-specific (apply only if the diff includes the relevant surface):

- `api-design-eval` — REST/RPC contracts (API additions or changes)
- `qa-cli` — exploratory CLI testing (CLI tools, command additions)
- `qa-web` — exploratory web QA via agent-browser (web UI changes)
- `design-review` — Nielsen heuristics, Gestalt (visual UI changes)
- `performance-eval` — Core Web Vitals, RAIL, perf budgets (web perf)
- `writing-review` — orchestrates prose evaluation (any docs/README changes)
- `writing-docs-eval` — Diátaxis/DQTI documentation review
- `writing-sentence-level` — Orwell/Williams sentence-level prose review
- `tisket-writing` — INVEST, problem-first framing (issue scoping)
- `product-eval` — JTBD, falsifiability, scope (PRDs/specs/briefs)
- `debugging` — when the change is a bugfix and root cause is in scope
- `research` — when the change cites prior art or external docs

For a broad diff, run all that apply. For a narrow change, only the
relevant subset.

## Dispatch rules

- One fresh subagent per skill. If the project specifies a
  particular model for reviews, pass that `model` parameter
  explicitly on every Agent call rather than relying on defaults.
- No shared conversation context — each subagent reads the skill file
  and the diff cold.
- Send all subagents in a single parallel tool-use block so they run
  concurrently, not sequentially.
- Each subagent reports severity-rated findings: blocker / major /
  minor, with file:line and the specific fix.
- Subagents do not edit; they only report.

## Merge

After all subagents return:

1. Collect findings across all reports.
2. Deduplicate (the same line flagged by two skills counts once,
   labeled with both perspectives).
3. Order by severity: blockers first, then majors, then minors.
4. Present as a single list. Each item: file:line, severity, what's
   wrong, the fix, which skill(s) flagged it.

---
name: tisket-writing
description: >
  Write well-scoped tisket issues using INVEST criteria, problem-first
  framing, testable acceptance criteria, and vertical slicing for
  decomposition. Adapts to issue type: features get job stories and
  acceptance criteria, bugs get reproduction steps, spikes get time-boxed
  questions, discovery issues get problem statements, epics track child
  issues via depends_on. Use when creating tisket issues, scoping work,
  decomposing epics, or when the user invokes /tisket-writing. Not for
  project management or sprint planning.
user-invocable: true
---

# Tisket Issue Writing

Write issues that pass the pickup test: someone unfamiliar with the
context can read the issue and start working without asking questions.
Every issue is a markdown file with YAML frontmatter and a free-form
body. The frontmatter carries structured metadata; the body carries
the human reasoning.

## Frontmatter Fields

```yaml
---
title: "tisket issue list --assignee flag is parsed but silently ignored"
status: todo
priority: 2
labels: [cli, filtering]
depends_on: []
assignee: ""
due_date: ""
---
```

- **title** — concise, specific, identifies the problem or outcome
- **status** — one of: `discovery`, `todo`, `in_progress`, `done`
- **priority** — 1 (urgent/blocking), 2 (important/next), 3 (normal), 4 (low/nice-to-have)
- **labels** — freeform tags for categorization (tool name, concern area, issue type)
- **depends_on** — list of issue filenames this issue is blocked by
- **assignee** — who's working on it (empty until claimed)
- **due_date** — optional deadline

## Issue Types and Status Progression

### discovery — captured idea, not yet scoped

A discovery issue is a parking lot for something that might become
work. It captures enough context to evaluate later without losing the
thread. The bar is low: a clear problem statement and enough context
to remember why it matters.

Discovery issues use the **problem statement format** (see below).
They don't *require* acceptance criteria, detailed reproduction steps,
or implementation guidance — but if you have that information, include
it. The point is to capture everything known, not to artificially
limit detail. A discovery issue with a proposed design and acceptance
criteria is better than one without, as long as the status reflects
that it hasn't been formally scoped.

A discovery issue should answer: "If someone reads this in two weeks,
will they understand why it was filed and whether it's still relevant?"
The more context captured at filing time, the better that answer.

### todo — scoped, ready to work

A todo issue has been deliberately scoped. It has:

1. A clear, specific title
2. A body that defines what done looks like
3. Enough detail for someone unfamiliar to pick it up and work

Promotion from `discovery` to `todo` is a deliberate scoping decision,
not a status change you make because you're about to start working.
Scoping means: the problem is understood, the boundaries are drawn,
the acceptance criteria are written, and the issue passes the pickup
test.

### in_progress — claimed and being worked

Someone is actively working on this. The assignee field should be set.
Scratch notes at the bottom of the issue serve as working memory —
what's been tried, what's blocked, what's next.

### done — completed and verified

The acceptance criteria have been met. The "done when" conditions are
satisfied. The issue stays in the repo as a record.

## Problem-First Framing

Every issue starts with the problem, never the solution. Embedding
the solution in the problem statement is the most common mistake in
issue writing — it forecloses options and makes the issue harder to
evaluate.

### The 5 W's

Before writing the body, answer these internally:

- **Who** is affected? (user, developer, CI system, downstream consumer)
- **What** is broken, missing, or suboptimal?
- **Where** does the problem manifest? (specific command, workflow step, file)
- **When** does it happen? (always, under specific conditions, after a change)
- **Why** does it matter? (what's the consequence of not fixing it)

Not all five need to appear literally in the issue, but if you can't
answer them, the issue isn't ready to write.

### Three-Statement Structure

For the problem statement itself, use this structure:

1. **Ideal** — how things should work (the expectation)
2. **Reality** — how things actually work (the observed behavior)
3. **Consequences** — what happens because of the gap (why it matters)

**Good example:**

> `tisket issue list --assignee cody` should filter issues to only
> those assigned to "cody". Currently, the flag is parsed without
> error but has no effect on output — all issues are returned
> regardless. Users can't find their own issues without piping
> through grep, which breaks on multi-line titles.

**Bad example:**

> We need to fix the assignee filter in the list command.

The bad example embeds a solution ("fix the filter") and doesn't
explain the problem. Maybe there is no filter and one needs to be
built. Maybe the flag should be removed. Maybe the filtering happens
but the display is wrong. The issue should leave those options open.

### Anti-pattern: Solution-as-problem

Watch for these phrasings — they're solutions disguised as problems:

- "Add a --verbose flag to the export command" — what problem does
  verbosity solve? Is the user missing information? Which information?
- "Refactor the parser to use nom" — why? What's wrong with the
  current parser? What breaks?
- "Upgrade to version 3.x" — what does 3.x fix that matters?

Rewrite each as: what is the user (or developer, or system)
experiencing that's painful, and what are the consequences?

## INVEST Criteria

Every `todo` issue should satisfy INVEST. These aren't abstract
principles — they're practical checks that predict whether the issue
will actually get completed without drama.

### Independent

The issue can be worked without waiting on other issues to finish
first. If there's a hard dependency, it should be explicit in the
`depends_on` field, and the dependent issue should be workable on
its own once the dependency is resolved.

**Violation:** "Implement the CLI output formatter" that can only be
tested after "Build the data retrieval layer" is done, but neither
issue mentions the dependency.

**Fix:** Either combine them into one issue that delivers a thin
vertical slice, or make the dependency explicit and ensure the
blocked issue has everything it needs to start immediately once
unblocked.

### Negotiable

The issue describes the outcome, not the implementation. The person
working it should have room to choose how to solve the problem.
Acceptance criteria define what "done" looks like; they don't
prescribe the code structure.

**Violation:** "Use a HashMap<String, Vec<Issue>> to index issues by
label, then iterate with a filter closure that matches the --label
flag value."

**Fix:** "When --label is provided, only issues with that label
appear in output. Order is preserved. No label match returns empty
output, not an error."

### Valuable

The issue delivers something meaningful when completed. A user,
developer, or system is measurably better off. If you can't explain
the value in one sentence without referencing other issues, the
issue might be a horizontal slice that doesn't stand alone.

**Violation:** "Create the Issue struct and implement Display" —
useful only as a building block, delivers no observable value alone.

**Fix:** "tisket issue show <id> prints the issue title, status,
and body to stdout in a readable format." Now it delivers something
someone can use.

### Estimable

Someone familiar with the codebase can look at this issue and have
a rough sense of how long it'll take. If the issue is too vague to
estimate, it needs more detail or should be a spike first.

**Violation:** "Improve the performance of issue listing" — improve
how? By how much? What's slow? Is it disk I/O, parsing, rendering?

**Fix:** "Issue listing takes 800ms for 50 issues. Profile and
reduce to under 200ms. Likely bottleneck is re-parsing all files on
every list invocation; a cache or index may help."

### Small

The issue can be completed in a single focused session (roughly a
few hours of work, not days). If it's bigger, decompose it using
vertical slicing. The test: can you hold the entire problem in your
head while working on it?

**Violation:** "Implement the full CRUD lifecycle for tisket issues"
— that's create, read, update, delete, each with its own edge cases.

**Fix:** Split into: "tisket issue create writes a new markdown file
with valid frontmatter," "tisket issue show reads and displays an
existing issue," etc. Each is independently completable and testable.

### Testable

There is a concrete way to verify the issue is done. The acceptance
criteria or "done when" section describe observable outcomes that can
be checked — by running a command, inspecting output, or observing
behavior. If you can't describe how to verify it, you don't
understand the issue well enough.

**Violation:** "Make the error messages more helpful" — helpful to
whom? In what way? How would you know if they're helpful enough?

**Fix:** "When `tisket issue show` is called with a nonexistent ID,
the error message includes the ID that was searched for, suggests
`tisket issue list` to see available issues, and exits with code 1."

## Title Writing

The title is often the only thing someone reads before deciding
whether to look deeper. It needs to carry enough information to
identify the problem or outcome without opening the issue.

### Qualities of a good title

- **Specific** — identifies the exact thing, not a vague area
- **Action-oriented** — implies what needs to happen or what's wrong
- **Scannable** — someone scrolling a list can understand it in a glance
- **Unique** — distinguishable from other issues about the same component

### Semantic prefixes

Use these when the issue type is clear:

- `feat:` — new capability that doesn't exist yet
- `fix:` — something is broken and needs to be corrected
- `chore:` — maintenance, cleanup, dependency updates
- `docs:` — documentation changes
- `refactor:` — restructuring without behavior change
- `test:` — adding or fixing tests

The prefix is optional for discovery issues (the type often isn't
clear yet) but should be present on todo issues.

### Examples

**Good titles:**

- `fix: tisket issue list --assignee flag is parsed but silently ignored`
- `feat: tisket issue create prompts for required fields when run interactively`
- `chore: remove deprecated --format=legacy flag from issue export`
- `fix: issue body truncated at 4096 bytes when reading files with multibyte UTF-8`
- `test: issue list sorting is untested for priority + date combinations`

**Bad titles:**

- `fix list command` — which list command? What's wrong with it?
- `assignee bug` — what about assignee? What's the bug?
- `improvements` — to what?
- `refactor stuff` — what stuff? Why?
- `TODO` — this is not a title

## Body Structure: Features

Feature issues describe new capability. The body should make clear
what the user (or system) will be able to do that they can't do now.

### Template

```markdown
## Problem

[Three-statement structure: Ideal → Reality → Consequences]

## Acceptance Criteria

- [ ] Given [context], when [action], then [observable outcome]
- [ ] Given [context], when [action], then [observable outcome]
- [ ] [Rule-oriented criterion if Given/When/Then is awkward]

## Out of Scope

- [Thing that might seem related but is explicitly NOT part of this issue]
- [Another thing]

## Done When

- [Concrete, testable statement of what done looks like]
- [Another one]

## Notes

[Any additional context: prior art, links, constraints, open questions]
```

### Example

```markdown
## Problem

Users need to find issues assigned to them across multiple tisket
projects. Currently, `tisket issue list` shows all issues with no
way to filter by assignee. Users with more than ~20 issues resort
to grepping output, which breaks on multi-line content and doesn't
respect frontmatter field boundaries.

## Acceptance Criteria

- [ ] Given issues exist with various assignees, when
      `tisket issue list --assignee cody` is run, then only issues
      where the assignee field matches "cody" are displayed
- [ ] Given no issues match the assignee, when the command is run,
      then output is empty (no error, exit code 0)
- [ ] Given --assignee is combined with other filters (--label,
      --status), then filters compose with AND logic

## Out of Scope

- Partial/fuzzy matching on assignee names
- Listing unique assignees across a project
- Changing assignee via the list command

## Done When

- `tisket issue list --assignee <name>` returns only matching issues
- Flag appears in `tisket issue list --help`
- Existing tests still pass; new tests cover the filter behavior
```

## Body Structure: Bugs

Bug issues describe something that's broken. The bar is
reproducibility — if someone can't reproduce it, they can't fix it.

### Template

```markdown
## Steps to Reproduce

1. [First step — be specific about state, commands, inputs]
2. [Second step]
3. [Step where the bug manifests]

## Expected Behavior

[What should happen at step 3]

## Actual Behavior

[What actually happens — include error messages, wrong output, etc.]

## Environment

- tisket version: [version or commit]
- OS: [if relevant]
- Other context: [shell, terminal, locale, etc. — only if relevant]

## Done When

- [The expected behavior occurs instead of the actual behavior]
- [Regression test exists]
```

### Example

```markdown
## Steps to Reproduce

1. Create an issue with a title containing a colon:
   `tisket issue create "fix: broken parser"`
2. Run `tisket issue show` on the created issue
3. Observe the title field in output

## Expected Behavior

Title displays as `fix: broken parser`.

## Actual Behavior

Title displays as `fix` — everything after the first colon is
silently dropped. The YAML parser treats the unquoted colon as a
key-value separator. The file on disk is also corrupted; the title
field in frontmatter reads `title: fix: broken parser` which is
invalid YAML.

## Environment

- tisket version: 0.4.2
- OS: macOS 14.3 (also confirmed on Linux)

## Done When

- Titles with colons are correctly quoted in frontmatter YAML
- Round-trip (create → show) preserves the full title
- Existing issues with corrupted titles are not made worse
  (migration is a separate issue)
- Regression test covers colons, quotes, and other YAML-special
  characters in titles
```

## Body Structure: Spikes and Discovery

Spikes are time-boxed investigations. They produce knowledge, not
code. The issue should make clear what question needs answering,
how long to spend, and what the deliverable is.

Discovery issues are lighter — they capture a problem that needs
scoping before it can become a todo. They might graduate into a
spike, a feature, a bug, or get closed as not-worth-doing.

### Spike Template

```markdown
## Question

[Specific, bounded question to answer. Not "investigate X" but
"Can X do Y under constraint Z?"]

## Timebox

[How long to spend before stopping and writing up findings,
even if the answer isn't complete. Usually 1-4 hours.]

## Deliverable

[What the spike produces. Options:]
- Decision document (we will / will not do X because Y)
- Proof of concept (working prototype that demonstrates feasibility)
- Recommendation with trade-offs (option A vs B vs C)
- Scoped follow-up issues (the spike produces the todo issues)

## Context

[What depends on the outcome of this spike. Why does this question
need answering now. What decisions are blocked.]

## Done When

- The question has a written answer (even if the answer is "we
  don't know yet and here's why")
- Follow-up issues are filed if work is warranted
- Time spent is within the timebox (or a conscious decision was
  made to extend it)
```

### Discovery Template

```markdown
## Problem

[Three-statement structure, but it's okay to be less precise here.
The point is to capture enough to evaluate later.]

## Open Questions

- [What do we not know yet?]
- [What would we need to learn before scoping this?]

## Why It Matters

[Brief statement of consequences if this is ignored. Helps with
future prioritization.]
```

## Acceptance Criteria

Acceptance criteria are the contract between the issue writer and
the person doing the work. They define what "done" means without
prescribing how to get there.

### Given/When/Then Format

Best for behavior that depends on context or conditions:

```
Given [some precondition or state],
when [an action is performed],
then [an observable outcome occurs].
```

Each criterion should be independently verifiable. If you need to
chain three Given/When/Then statements to describe one behavior,
that's one criterion — but consider whether the issue is too big.

### Rule-Oriented Checklist Format

Best for constraints or invariants that don't fit a scenario:

```
- [ ] Output is valid UTF-8 regardless of input encoding
- [ ] Exit code is 0 on success, 1 on user error, 2 on system error
- [ ] No output is written to stdout on error (errors go to stderr)
```

### Criteria vs. Implementation Details

Criteria describe **observable outcomes**. Implementation details
describe **how the code works internally**. The issue should contain
criteria; implementation details belong in the code or in scratch
notes.

**Criterion:** "When `--format json` is passed, output is valid JSON
that parses without error."

**Implementation detail:** "Use serde_json::to_string_pretty() to
serialize the output struct."

The person working the issue might use serde_json or might hand-build
the JSON or might use a different library entirely. The criterion
doesn't care — it only cares that the output is valid JSON.

### How many criteria?

- **1-3 criteria per issue** is the sweet spot.
- **4+ criteria** usually means the issue is doing too much. Look for
  a natural split point.
- **0 criteria** means the issue isn't scoped. It should be in
  `discovery` status, not `todo`.

## Vertical Slicing

When work is too big for a single issue, decompose it into thin
end-to-end slices rather than horizontal layers.

### The Stakeholder Test

If you can't demonstrate the result of a completed issue to a
stakeholder (user, product owner, fellow developer), it's probably a
horizontal slice. Horizontal slices build infrastructure that only
becomes useful when combined with other slices. Vertical slices
deliver observable value on their own.

**Horizontal slices** (avoid):

1. "Build the data model for issues"
2. "Implement the file parser"
3. "Build the CLI interface"
4. "Add output formatting"

None of these delivers anything usable alone. You can't show
someone "here's the data model" and have them use it.

**Vertical slices** (prefer):

1. "tisket issue create writes a new issue file with title and status"
2. "tisket issue show displays an existing issue's title, status, and body"
3. "tisket issue list prints all issue titles with their status"
4. "tisket issue edit opens an issue in $EDITOR and saves changes"

Each slice goes from user input to observable output. Each can be
shipped and used independently.

### Splitting Techniques

**By workflow step:** If a feature involves multiple steps (create →
validate → transform → output), each step that produces observable
output can be its own issue.

**By business rule:** If a feature has multiple rules or conditions
(filtering by assignee, filtering by label, filtering by status),
each rule can be its own issue.

**By data variation:** If a feature handles different types of input
(YAML frontmatter, JSON frontmatter, no frontmatter), each variation
can be its own issue.

**By operation (CRUD):** Create, read, update, delete are natural
split points. Each operation is independently valuable.

### When NOT to Split

Don't split if:

- The pieces are so small they cost more to track than to do
- The pieces aren't independently testable
- The split creates artificial dependencies (issue B can't start
  until issue A is merged, and A is trivial)

Use judgment. The goal is issues that are small enough to complete
in one session but large enough to deliver real value.

## Epics

An epic is a regular issue that depends on every child issue. Tisket
has no special epic type. The convention:

- Title prefix: `epic: <name>`
- Label: `epic`
- Status: `discovery` (until all children are scoped)
- `depends_on`: lists every child issue

The epic's body has a problem statement, scope description, and a
manually maintained list of child issues with brief descriptions.
Because `depends_on` blocks pickup, the epic is naturally the last
thing closed.

When decomposing work that's too large for a single issue, create
the epic first, then create child issues and add them as
dependencies. The epic tracks the overall goal; children track
individual deliverables.

## The "Done When" Section

Every `todo` issue needs a "Done When" section. This is the
operational definition of complete. It's not aspirational ("the
feature works well") — it's concrete and testable.

### Qualities of good "done when" statements

- **Observable** — someone can look at the system and verify it
- **Specific** — no ambiguity about what "done" means
- **Minimal** — only what's necessary for this issue, not future work
- **Independent** — each statement can be verified on its own

### Good examples

```markdown
## Done When

- `tisket issue list --assignee cody` returns only issues assigned to "cody"
- `tisket issue list --help` documents the --assignee flag
- Existing test suite passes without modification
- At least one test covers the assignee filter with matching issues
- At least one test covers the assignee filter with no matches
```

### Bad examples

```markdown
## Done When

- The feature is implemented ← what does "implemented" mean?
- Tests pass ← which tests? New ones? Existing ones? Both?
- Code is clean ← subjective, unverifiable
- Performance is acceptable ← what's acceptable?
```

## Labels and Priority

### Priority

- **Priority 1 (urgent/blocking):** Something is broken in a way that
  blocks other work or affects users right now. Drop what you're doing.
  Examples: data corruption bug, CI pipeline broken, published tool
  crashes on startup.

- **Priority 2 (important/next):** Significant issue that should be
  addressed soon but isn't blocking anyone right now. Next thing to
  pick up. Examples: feature needed for an upcoming milestone,
  performance regression that's annoying but not catastrophic.

- **Priority 3 (normal):** Standard work. Important enough to track,
  not urgent enough to interrupt current work. Most issues live here.
  Examples: new features, minor bugs, improvements.

- **Priority 4 (low/nice-to-have):** Would be nice but nobody's
  suffering without it. Might get done during slack time, might not.
  Examples: cosmetic improvements, optimizations for uncommon cases,
  "someday" ideas that survived discovery triage.

### Labels

Labels are freeform tags for filtering and grouping. Use them for:

- **Tool or component name:** `cli`, `parser`, `config`, `formatter`
- **Concern area:** `performance`, `ux`, `correctness`, `security`
- **Issue type:** `bug`, `feature`, `spike`, `chore`, `debt`
- **Workflow:** `needs-review`, `blocked`, `quick-win`

Don't over-label. 1-3 labels per issue is typical. If you need more,
the issue might be too broad.

## Dependencies

### When to use depends_on

Use `depends_on` when an issue literally cannot be started until
another issue is completed. Not "it would be nice to do A before B"
but "B requires the output/artifact/change from A to exist."

### Structuring dependency chains

Keep chains short. If A → B → C → D, ask whether B and C can be
restructured to remove the chain. Long dependency chains are a sign
that the work hasn't been sliced vertically — the pieces should be
more independent.

### Anti-patterns

**Circular dependencies:** A depends on B, B depends on A. This
means the issues aren't properly scoped. They're really one issue,
or the dependency is artificial.

**Long chains:** A → B → C → D → E. The person working E has to
wait for four other issues to complete. If any one stalls, everything
downstream stalls. Restructure to flatten the graph.

**Phantom dependencies:** "B depends on A" but actually B could be
worked with a stub or mock and only needs A's output at integration
time. Make the dependency explicit: "B can be worked independently
but needs A merged before integration testing."

**Undeclared dependencies:** Two issues actually depend on each other
but neither says so. The person working one discovers the dependency
mid-work and is blocked. When in doubt, declare the dependency.

## The Pickup Test

This is the ultimate quality gate. Before marking an issue as `todo`,
apply this test:

> Can someone unfamiliar with the context — a new team member, a
> future version of yourself, an agent — read this issue and start
> working without asking clarifying questions?

If the answer is no, the issue needs more detail. Common failures:

- **Missing context:** The issue assumes knowledge that isn't written
  down. "Fix the serialization bug" — which one? Where? What are the
  symptoms?

- **Ambiguous scope:** "Improve error handling" — in which commands?
  What counts as "improved"? What's the current state?

- **No acceptance criteria:** "Add label filtering" — what should the
  behavior be? What are the edge cases? What does the command look
  like?

- **Implicit assumptions:** "This should be straightforward" — says
  who? If it's straightforward, it should be easy to write clear
  acceptance criteria.

- **Missing reproduction steps:** For bugs — if someone can't
  reproduce the problem, they can't verify the fix.

The pickup test isn't about writing a novel. It's about writing
enough that the work can start without a synchronous conversation.
Some issues need two sentences. Some need two pages. Match the
detail to the complexity.

## Scratch Notes

Issues have an optional scratch notes section at the bottom, below a
horizontal rule. This is working memory — used during `in_progress`
to track what's been tried, what's blocked, decisions made during
implementation, and what's next.

```markdown
---

## Scratch Notes

- Tried approach X, ran into Y limitation
- The actual root cause is Z, not what was assumed in the problem statement
- Next: try approach W
- Blocked on: waiting for upstream release 1.2.3
```

Scratch notes are informal. They don't need to be polished. Their
audience is the person doing the work (which might be future-you or
an agent picking up mid-stream). They should be honest about what
didn't work, not just what did.

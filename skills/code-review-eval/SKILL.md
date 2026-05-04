---
name: code-review-eval
description: >
  Structured code review using Fagan inspection passes, Google's priority
  hierarchy (design → functionality → complexity → tests → naming),
  SOLID principles, Fowler's code smells, and cognitive complexity.
  Reviews by concern: correctness, security, performance, maintainability,
  style. Use when reviewing code changes, PRs, or when the user invokes
  /code-review-eval. Not for design review or writing review.
user-invocable: true
---

# Code Review Eval

Structured code review using practitioner frameworks. Each framework
targets a different dimension of code quality. The skill runs multiple
passes over the same code, each pass looking for a specific category
of problem. Findings are synthesized into a single prioritized report.


## Framework 1 — Google's Review Priority Hierarchy

Google's engineering practices define an explicit priority ordering for
review comments. Higher-priority issues block approval; lower-priority
issues are suggestions. Work through these in order and stop investing
time in lower levels if higher levels have serious problems.

### 1. Design

The most important dimension. Ask:

- Does this change belong in this codebase, or should it be in a
  library, a different service, or a different layer?
- Do the abstractions make sense? Are new types/interfaces justified,
  or is this over-engineering a simple problem?
- Does the change integrate well with the rest of the system? Will it
  create coupling that makes future changes harder?
- Is the level of abstraction appropriate? A helper function wrapping
  a single standard-library call adds indirection without value. A
  God class doing everything adds none either.

Design problems are the most expensive to fix later. A change with
correct behavior but wrong design should be sent back.

### 2. Functionality

Does the code actually do what the author intended?

- Trace through the primary success path manually.
- Trace through each error/edge path. What happens on empty input?
  Null? Concurrent access? Network failure?
- Check boundary conditions: off-by-one, integer overflow, empty
  collections, single-element collections.
- For UI changes: consider accessibility, responsiveness, and what
  happens when data is missing or takes a long time to load.
- For data changes: what happens to existing data? Is migration
  needed? Is it backward compatible?

### 3. Complexity

Can another engineer understand and modify this code without
significant effort?

- Is any single function doing too much? Could it be split into
  named steps that each do one thing?
- Is any single class doing too much? Does it have a clear single
  responsibility?
- Is there over-engineering? Abstractions for hypothetical future
  requirements that don't exist yet are complexity, not foresight.
  "You aren't gonna need it" (YAGNI) applies.
- Are there clever tricks that require comments to explain? Clever
  code is a liability. Clear code that a tired engineer at 2am can
  follow is the goal.

### 4. Tests

Tests are not optional and are not second-class code.

- Does every new code path have a corresponding test?
- Are the tests actually testing the behavior, or are they just
  asserting implementation details that will break on refactor?
- Do the tests cover edge cases and error paths, not just the
  happy path?
- Are the tests readable? A test that requires reading the
  implementation to understand what it checks is a bad test.
- Will the tests fail for the right reasons? A test that passes
  when the code is broken is worse than no test.

### 5. Naming

Names are the primary documentation. Bad names create ongoing costs.

- Do function names describe what they do, not how they do it?
  `processData` tells you nothing. `validateUserPermissions` tells
  you what to expect.
- Do variable names describe what they hold? `temp`, `data`, `val`,
  `result` are almost always wrong. `userAge`, `pendingRequests`,
  `retryCount` tell you something.
- Are names consistent with existing codebase conventions? If the
  codebase calls them "repositories," don't introduce "stores."
- Are abbreviations justified? `ctx` for context is fine in Go where
  it's idiomatic. `usrPrflMgr` is never fine.

### 6. Comments

Comments should explain WHY, not WHAT. The code explains what.

- Are there comments that just restate the code? Delete them.
  `// increment counter` above `counter += 1` is noise.
- Are there missing comments where non-obvious decisions were made?
  A regex, a magic number, a workaround for a platform bug — these
  need explanation.
- Are there TODO/FIXME/HACK comments? These should reference a
  tracking issue. Untracked TODOs are where good intentions go to die.

### 7. Style

Style should be enforced by automated tools, not reviewers.

- Does the change follow the project's established formatting and
  linting rules?
- If the project has a style guide, are there violations?
- Style comments in review are low-value if the project has
  auto-formatting. Don't waste review time on what `rustfmt` or
  `prettier` can handle.

### 8. Documentation

For changes that affect the public interface or user-visible behavior:

- Are API docs updated?
- Are README or user-facing docs updated?
- For breaking changes, is there a migration guide?

### 9. Every Line

Read every line of the diff. Not skimming — actually reading. If a
section is hard to understand, that itself is a finding (complexity).

### 10. Context

Look at the file beyond the diff. The surrounding code provides
context that the diff alone hides.

- Does the change duplicate logic that already exists nearby?
- Does the change break assumptions that the surrounding code makes?
- Does the file need refactoring that this change should include
  rather than making the existing mess worse?


## Framework 2 — Review by Concern

Five explicit passes, each looking for a specific category of problem.
This framework complements Google's hierarchy by organizing the search
differently — by what could go wrong rather than by code element.

### Pass 1: Functional Correctness

The code must do what it claims to do.

- Trace the primary success path end-to-end.
- Identify all branch points and trace each branch.
- Check state transitions: can the system reach an invalid state?
- Check error handling: are errors caught, logged, and handled
  appropriately? Are they swallowed silently? Are they propagated
  with enough context to debug?
- Check data flow: is data validated at trust boundaries? Is it
  possible for invalid data to reach processing logic?
- Check concurrency: are there race conditions? Is shared mutable
  state properly guarded? Are operations that should be atomic
  actually atomic?

### Pass 2: Security

Assume the input is hostile.

- Injection: Is user input ever interpolated into SQL, shell
  commands, HTML, or log messages without sanitization?
- Authentication/Authorization: Does every endpoint verify identity
  and permissions? Are there paths that skip auth checks?
- Secrets: Are API keys, passwords, or tokens hardcoded? Are they
  logged? Are they in the diff?
- Data exposure: Does error output or logging reveal internal
  structure, stack traces, or user data to unauthorized parties?
- Deserialization: Is untrusted data deserialized without validation?
  This is a common vector for remote code execution.
- Dependency risk: Do new dependencies have known vulnerabilities?
  Are they actively maintained?

### Pass 3: Performance

Performance problems compound and are expensive to fix after deployment.

- Algorithmic complexity: Is there an O(n²) operation hiding inside
  a loop? Are there unnecessary repeated traversals?
- Database queries: Are there N+1 query patterns? Are queries using
  indexes? Are large result sets fetched when only a count is needed?
- Memory: Are large objects held in memory longer than necessary? Are
  there unbounded collections that grow with input size?
- I/O: Are network calls or disk reads inside loops when they could
  be batched? Are responses cached when appropriate?
- Startup cost: Does the change add work to the hot path or
  initialization that could be deferred?

### Pass 4: Maintainability

Code is read far more often than it is written.

- Can a new team member understand this code without asking the
  author? If not, something needs to change — better names, better
  structure, or better comments.
- Are there implicit dependencies or ordering requirements that
  aren't enforced by the type system or API?
- Is the change testable in isolation, or does it require complex
  setup that couples it to the rest of the system?
- Does the change make future changes harder? Does it close off
  options that might be needed?
- Does the change follow the existing patterns in the codebase, or
  introduce a new pattern? New patterns need strong justification.

### Pass 5: Style and Conventions

The least important pass. Automated tools should handle most of this.

- Formatting consistency with the rest of the codebase.
- Naming conventions (casing, prefixes, suffixes).
- Import organization.
- File and directory structure conventions.
- Language idioms: is the code idiomatic for the language, or does
  it look like it was translated from a different language?


## Framework 3 — SOLID Principles as Review Checks

Each SOLID principle translates to a specific question during review.
These apply most directly to object-oriented code, but the underlying
ideas apply broadly.

### Single Responsibility Principle (SRP)

**Review question:** Does this class/module/function have exactly one
reason to change?

A violation looks like: a `UserService` that handles authentication,
profile updates, email notifications, and audit logging. If the email
provider changes, the `UserService` changes. If the audit format
changes, the `UserService` changes. That's multiple reasons to change.

The fix is not always more classes — sometimes it's clearer function
boundaries within a module. The principle is about cohesion, not about
having tiny classes.

### Open/Closed Principle (OCP)

**Review question:** Can new behavior be added without modifying
existing code?

A violation looks like: a function with a growing `match`/`switch`
statement that must be edited every time a new variant is added. If
adding a new payment method requires editing `processPayment()`,
the design is closed to extension.

Common solutions: strategy pattern, plugin architectures, trait/
interface implementations. But don't over-engineer — a three-case
match statement doesn't need a plugin system.

### Liskov Substitution Principle (LSP)

**Review question:** Can every subtype be used wherever its parent
type is expected without breaking correctness?

A violation looks like: a `ReadOnlyRepository` that extends
`Repository` and throws `UnsupportedOperationException` on `save()`.
Code that accepts a `Repository` and calls `save()` will break at
runtime. The type system promised something the implementation
doesn't deliver.

This extends beyond inheritance to any interface implementation or
protocol conformance. If a function says it accepts a `Writer`, every
`Writer` implementation must actually write, not silently discard.

### Interface Segregation Principle (ISP)

**Review question:** Are callers forced to depend on methods they
don't use?

A violation looks like: a `DatabaseConnection` interface with 40
methods, used by a service that only calls `query()`. Changes to any
of the 39 unused methods can still break the consumer's build or
require recompilation.

Prefer small, focused interfaces. A `Queryable` interface with just
`query()` is better than a `DatabaseConnection` interface that also
has `migrate()`, `backup()`, `monitor()`, etc.

### Dependency Inversion Principle (DIP)

**Review question:** Do high-level modules depend on abstractions, or
on concrete implementations?

A violation looks like: a business logic module that directly imports
and instantiates a PostgreSQL client. Switching databases requires
changing business logic. Testing requires a real database or complex
mocking.

The fix: the business logic depends on a `Repository` trait/interface.
The concrete `PostgresRepository` implements it. The concrete type is
injected, not constructed inside the business logic.


## Framework 4 — Fowler's Code Smells

Code smells are surface indicators of deeper design problems. They
don't always mean something is wrong, but they warrant investigation.
Organized by where they tend to appear.

### Smells Within Methods

- **Long Method:** A function that does too many things. If you need
  to scroll to read it, or if it has sections separated by blank
  lines and comments, each section is probably a separate function.

- **Long Parameter List:** More than three or four parameters suggest
  the function is doing too much, or related parameters should be
  grouped into a structure.

- **Duplicated Code:** The same logic in two places. When the logic
  needs to change, both places must be found and updated. They won't
  be.

- **Dead Code:** Code that is never executed — unreachable branches,
  unused functions, commented-out blocks. It creates confusion about
  what's intentional and what's vestigial.

- **Speculative Generality:** Abstractions, parameters, or hooks
  added for future use cases that don't exist. They add complexity
  now and may never be needed. "You aren't gonna need it."

### Smells Between Classes

- **Feature Envy:** A method that uses more data from another class
  than from its own. It probably belongs in the other class.

- **Inappropriate Intimacy:** Two classes that access each other's
  private details extensively. They're either one class pretending to
  be two, or they need a clearer interface between them.

- **Message Chains:** `a.getB().getC().getD().doThing()` — a long
  chain of navigations. The caller knows too much about the object
  graph. If any intermediate structure changes, the caller breaks.

- **Middle Man:** A class that delegates almost everything to another
  class. If most methods just forward to a delegate, the middleman
  adds indirection without value.

- **Parallel Inheritance Hierarchies:** Every time a subclass is
  added to one hierarchy, a corresponding subclass must be added to
  another. The hierarchies are coupled and should probably be merged
  or restructured.

### Data Problems

- **Data Clumps:** Groups of data that always appear together (e.g.,
  `startDate`/`endDate`/`timezone` as three separate parameters
  everywhere). They should be a single structure.

- **Primitive Obsession:** Using primitive types (strings, ints) for
  domain concepts. An email address is not a string — it has
  validation rules, formatting expectations, and semantic meaning. A
  user ID is not an integer.

- **Refused Bequest:** A subclass that inherits methods it doesn't
  want or use. The inheritance hierarchy is wrong — maybe composition
  would be better.

### Change-Pattern Smells

These are detected by observing how the code changes over time, not
by reading it once. During review, ask whether the change itself
exhibits these patterns.

- **Divergent Change:** One class is frequently modified for
  different, unrelated reasons. This violates SRP — the class has
  multiple responsibilities.

- **Shotgun Surgery:** A single logical change requires editing many
  different classes/files. The responsibility is scattered instead of
  cohesive.

- **Combinatorial Explosion:** Adding a new dimension (new file
  format, new output target) requires changes proportional to all
  existing dimensions. The design lacks proper abstraction at the
  variation points.

### Bloaters

- **Large Class:** A class with too many fields, too many methods,
  or too many lines. It's doing too much and should be split.

- **Long Method:** (Repeated from above because it's both a method
  smell and a bloater.) Functions over ~20 lines warrant scrutiny.
  Functions over ~50 lines are almost certainly doing too much.

- **God Class:** The extreme case of Large Class — one class that
  knows everything and does everything. Common in codebases that
  grew without refactoring.


## Framework 5 — Cognitive Complexity

Cognitive complexity measures how hard code is for a human to
understand. It was developed by SonarSource as an improvement over
cyclomatic complexity.

### How It Differs from Cyclomatic Complexity

Cyclomatic complexity counts the number of linearly independent paths
through the code. It treats all control flow equally — a simple
`if/else` and a deeply nested `if` inside a `for` inside a `try`
both add the same count.

Cognitive complexity weights for nesting. A deeply nested condition is
harder to understand than a flat one, and the metric reflects that.

### The Three Rules

**Rule 1 — Increment for flow breaks:** Each of the following adds
+1 to the complexity score:

- `if`, `else if`, `else`
- `for`, `while`, `do...while`, `loop`
- `match`/`switch` (the whole block, not each arm)
- `catch`/`except`
- `break` or `continue` to a label
- Logical operator sequences that mix `&&` and `||`:
  `a && b` = +1, `a && b && c` = +1 (same operator, no extra cost),
  `a && b || c` = +2 (operator switch adds a cost)
- Ternary operator (`? :`)
- `goto`

**Rule 2 — Increment for nesting:** Each level of nesting adds an
additional +1 to every flow break inside it. An `if` at the top level
costs 1. An `if` inside a `for` costs 2 (1 for the `if` + 1 for
being nested in the `for`). An `if` inside a `for` inside a `try`
costs 3.

This is the key insight — nesting compounds difficulty geometrically,
not linearly.

**Rule 3 — Ignore readability aids:** The following do NOT add to
complexity because they make code easier to read, not harder:

- `else` after `if` when it simplifies understanding
  (but `else if` does increment because it's a new condition)
- Early returns / guard clauses (these reduce nesting)
- Null coalescing operators (`??`, `?.`)
- Functions extracted from complex code (the extraction reduces
  complexity of the caller)

### Threshold

The generally accepted threshold is **15 per function**. Functions
above 15 should be refactored. Functions above 25 are almost
certainly doing too much. Functions below 5 are usually fine.

When reviewing, don't compute exact scores — use the rules as a
heuristic. If a function has three levels of nesting with conditions
at each level, it's probably over the threshold regardless of the
exact count.

### Reducing Cognitive Complexity

- Extract nested logic into named helper functions. The name acts as
  documentation and the nesting resets.
- Replace nested conditions with early returns (guard clauses).
- Replace complex boolean expressions with named boolean variables:
  `let isEligible = age >= 18 && hasConsent && !isBanned;`
- Replace `if/else if/else if` chains with lookup tables, maps, or
  pattern matching where the language supports it.


## Framework 6 — The Newspaper Test

Robert C. Martin's "newspaper test" (from Clean Code) says that
reading a source file should be like reading a newspaper article:
the headline and most important information first, details later.

### Public Above Private

Public-facing API — the functions and types that other modules call —
should appear at the top of the file. Private helper functions,
internal implementation details, and utility code should appear below.

A reader opening the file should immediately see what this module
offers. They should not have to scroll past 200 lines of internal
helpers to find the public interface.

### Callers Above Callees

If function `A` calls function `B`, then `A` should appear above `B`
in the file. The reader encounters the high-level orchestration
first, then can drill into the details if needed.

This creates a natural top-down reading order. The main function or
entry point at the top, the leaf-level utilities at the bottom.

### Abstractions Before Details

Type definitions, interfaces, and traits should appear before their
implementations. The reader sees the contract first, then the
concrete behavior.

Constants and configuration should appear near the top (after
imports) because they parameterize the behavior that follows.

### When Files Fail the Newspaper Test

Signs of failure during review:

- The reader has to jump around the file to follow the logic.
- Helper functions are defined above the functions that use them,
  forcing bottom-up reading.
- Private implementation details are interleaved with public API.
- The file has no discernible organization — functions appear in
  the order they were written, not the order they should be read.

This isn't about rigid rules — some languages and codebases have
conventions that differ (e.g., Rust typically puts `impl` blocks
after struct definitions, which is fine). The principle is about
readability and discoverability within whatever structure the
language and project use.


## Applying the Frameworks

Run the passes in this order. Each pass assumes the previous pass
found no blocking issues — if a higher-priority pass finds serious
problems, note the remaining passes as "not evaluated due to
higher-priority findings."

### Recommended Pass Order

1. **Design review (Google #1):** Is the change in the right place?
   Are the abstractions appropriate? If the design is wrong, nothing
   else matters — send it back.

2. **Functional correctness (Concern pass #1):** Does it work? Trace
   the paths, check the edges. If it doesn't work correctly, other
   feedback is premature.

3. **Security (Concern pass #2):** Assume hostile input. Check
   injection, auth, secrets, data exposure. Security problems block
   regardless of everything else.

4. **SOLID check:** Walk through the five principles as questions.
   Note violations, but weigh them — not every violation warrants a
   blocking comment. A minor SRP issue is a suggestion; a major LSP
   violation is a blocker.

5. **Code smells scan:** Look for the major smells. These are often
   symptoms of the SOLID violations found in the previous pass.
   Focus on smells that affect the changed code, not pre-existing
   smells in surrounding code (though those can be noted as
   follow-up work).

6. **Cognitive complexity check:** For any function that looks
   complex, apply the three rules. Functions over the threshold
   should be flagged with specific suggestions for reduction.

7. **Newspaper test:** Check file organization. This is a quick
   scan — is the public API discoverable? Can the file be read
   top-down? Note issues but don't block on them unless the
   organization is genuinely confusing.

8. **Performance (Concern pass #3):** Check algorithmic complexity,
   database patterns, memory, I/O. Performance issues in hot paths
   block; performance issues in cold paths are suggestions.

9. **Maintainability (Concern pass #4):** Is this code easy to
   change later? Will a new team member understand it? This
   synthesizes findings from the previous passes.

10. **Style and naming (Google #5-7, Concern pass #5):** The lowest
    priority. Only flag what automated tools can't catch. Don't
    spend review capital on formatting preferences.

### Severity Levels

Classify each finding:

- **Blocker:** Must be fixed before merge. Design problems,
  correctness bugs, security vulnerabilities, test gaps for
  critical paths.
- **Should fix:** Not blocking but should be addressed in this PR.
  Significant complexity, missing edge-case tests, naming that
  will cause confusion.
- **Suggestion:** Take it or leave it. Minor style issues,
  alternative approaches that are roughly equivalent, notes about
  future improvements.
- **Note:** Not actionable now but worth recording. Observations
  about surrounding code, potential future refactoring, things
  to watch for.

### Output Format

Present findings grouped by severity, not by framework. The reviewer
doesn't care which framework surfaced a finding — they care whether
it blocks the merge. Within each severity level, order by the pass
that found it (higher-priority passes first).

For each finding, include:

- The file and location (line number or function name).
- What was found, stated concretely. Not "this is complex" but
  "this function has four levels of nesting with conditions at
  each level."
- Why it matters. Not "violates SRP" but "if the email provider
  changes, this payment processing function must also change."
- A suggested fix, when one is apparent. Not always necessary for
  blockers where the right fix requires discussion.

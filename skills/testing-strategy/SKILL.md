---
name: testing-strategy
description: >
  Test strategy design using shape models (pyramid, trophy, diamond),
  Marick's testing quadrants, risk-based prioritization, RCRCRC regression
  selection, Beck's 12 test desiderata, Google's test size classification,
  and specialized techniques (contract, property-based, mutation testing).
  Adapts to architecture: monoliths get the pyramid, frontends get the
  trophy, microservices get the diamond. Use when designing test suites,
  evaluating test portfolio balance, choosing test types for a feature,
  or when the user invokes /testing-strategy. Not for writing individual
  tests (use testing-philosophy) or running tests (use qa-web/qa-cli).
user-invocable: true
---

# Testing Strategy

Design test suites that catch real bugs, run fast enough to use
constantly, and don't collapse under refactoring. The approach depends
on what's being tested and what the architecture looks like.

## Framework 1 — Shape Models

Three competing models for how to distribute tests across scope levels.
None is universally correct. The architecture determines which shape
fits.

### The Test Pyramid (Cohn, 2009)

Mike Cohn's original model from *Succeeding with Agile*. A triangle
with three layers:

```
    /  E2E  \          Few, slow, expensive
   /----------\
  / Integration \      Some, moderate speed
 /----------------\
/    Unit Tests    \   Many, fast, cheap
```

**The principle:** more tests at the bottom, fewer at the top. Unit
tests are fast, isolated, and cheap to write. E2E tests are slow,
brittle, and expensive to maintain. A healthy suite has many unit
tests, some integration tests, and few E2E tests.

**Recommended ratio:** roughly 70% unit, 20% integration, 10% E2E.
These numbers aren't sacred — they're a starting orientation.

**When it works:** traditional backend applications, libraries, CLIs,
anything where business logic lives in functions that can be called
in isolation. Monoliths with well-defined module boundaries.

**When it breaks down:** when the interesting bugs live at boundaries
between components, not inside them. When units are trivial but their
composition is complex. When the system is mostly glue code.

### The Test Trophy (Dodds, 2019)

Kent C. Dodds proposed this for frontend and UI-heavy applications.
Named because the shape — narrow at the bottom and top, wide in the
middle — looks like a trophy.

```
    /  E2E  \          Few
   /----------\
  /            \
 / Integration  \      Most investment here
  \            /
   \----------/
    \  Unit  /         Few (static analysis catches the rest)
     \------/
      Static           Type system, linters
```

**The principle:** integration tests give the best return on investment
for UI code. Unit testing a React component in isolation tells you
almost nothing about whether the user experience works. Integration
tests (rendering a feature with its real children, hitting a real API
mock) catch the bugs that actually ship.

**The static layer:** TypeScript, ESLint, and similar tools catch an
entire category of bugs (typos, type mismatches, unused variables)
that unit tests used to catch. This moved the effective floor upward.

**When it works:** single-page applications, component-based UIs,
frontend-heavy architectures where the rendering layer is the product.

**When it breaks down:** backend systems with complex business logic
that genuinely benefits from isolated unit testing. Systems where
"integration" is ambiguous because everything integrates with
everything.

### The Test Diamond / Honeycomb (Spotify, ~2018)

Spotify's engineering team described this shape for microservice
architectures. Wide in the middle, narrow at top and bottom.

```
    / E2E \            Very few (cross-service)
   /--------\
  /          \
 / Integration\        Most tests here
 \            /
  \----------/
   \  Unit  /          Few (services are thin)
    \------/
```

**The principle:** in a microservice architecture, individual services
are often thin — a REST handler, some validation, a database call,
maybe a call to another service. There's barely any logic to unit test.
The interesting behavior lives at the seams: does this service correctly
call that service? Does the data flow through the pipeline intact?

Integration tests — tests that exercise a service with its real
database and mocked external dependencies — are where the bugs live.
Unit tests cover algorithmic logic if it exists. E2E tests verify
critical user journeys across service boundaries, but sparingly
because they're slow and hard to debug.

**When it works:** microservices, thin services, API gateways, systems
where services are primarily glue between a database and other services.

**When it breaks down:** monoliths, systems with substantial business
logic in a single service, libraries.

### Choosing a shape

| Architecture | Shape | Why |
|---|---|---|
| Monolith with business logic | Pyramid | Logic is testable in isolation |
| SPA / component UI | Trophy | Integration catches UI bugs; static catches type errors |
| Microservices / thin services | Diamond | Integration is the interesting layer |
| Library / CLI tool | Pyramid | Public API is the test surface |
| Data pipeline | Diamond | Transformations compose; test the composition |
| Mixed (monolith + SPA) | Pyramid backend, trophy frontend | Different layers have different economics |

## Framework 2 — Marick's Testing Quadrants

Brian Marick's model (later popularized by Lisa Crispin and Janet
Gregory in *Agile Testing*) classifies tests along two axes:

**Axis 1:** Business-facing vs. technology-facing
**Axis 2:** Supporting development vs. critiquing the product

```
                    Business-facing
                         |
         Q2              |           Q3
   Functional tests      |      Exploratory testing
   Story tests           |      Usability testing
   Prototypes            |      UAT
   Simulations           |      Alpha/beta
                         |
  Supporting ────────────┼──────────── Critiquing
  development            |             the product
                         |
         Q1              |           Q4
   Unit tests            |      Performance testing
   Component tests       |      Load testing
   Integration tests     |      Security testing
                         |      "-ility" testing
                         |
                    Technology-facing
```

**Q1 (technology-facing, supporting development):** unit, component,
and integration tests. Written by developers, run in CI. These are
the pyramid/trophy/diamond tests. They support development by catching
regressions fast.

**Q2 (business-facing, supporting development):** functional tests,
story tests, acceptance tests. Written with or by the business.
They verify that the system does what was asked for. Often automated,
sometimes manual. BDD-style tests live here.

**Q3 (business-facing, critiquing the product):** exploratory testing,
usability testing, UAT. These aren't scripted — they're creative
investigation. Humans finding problems that automated tests wouldn't
think to look for. The QA skill (SFDPOT, FEW HICCUPPS) operates here.

**Q4 (technology-facing, critiquing the product):** performance, load,
security, accessibility testing. Non-functional requirements that
require specialized tools and techniques.

**The insight:** most teams over-invest in Q1 and under-invest in
Q2-Q4. A suite of 2,000 passing unit tests tells you the code works
as programmed. It tells you nothing about whether the product is
useful, performant, secure, or pleasant to use. A healthy test
portfolio has coverage in all four quadrants.

**How to audit:** list every test and testing activity. Place each in
a quadrant. If any quadrant is empty, that's a gap. If one quadrant
has 95% of the effort, the portfolio is unbalanced.

## Framework 3 — Risk-Based Test Selection

Not everything deserves equal testing investment. Risk-based testing
allocates effort proportional to two factors:

**Likelihood of failure × Impact of failure = Risk**

### Likelihood factors

- **Complexity**: more complex code breaks more often
- **Change frequency**: code that changes often has more opportunities
  for regression
- **Developer experience**: new team members, unfamiliar technology
- **Dependencies**: external services, third-party libraries
- **History**: code that has broken before will break again

### Impact factors

- **User visibility**: does the user see the failure directly?
- **Data integrity**: can the failure corrupt or lose data?
- **Revenue**: does the failure affect transactions or billing?
- **Security**: does the failure expose sensitive data?
- **Recovery difficulty**: how hard is it to fix once it's broken?

### The risk matrix

```
              High Impact    Low Impact
High Likelihood  [CRITICAL]     [Important]
Low Likelihood   [Important]    [Low priority]
```

**Critical (high likelihood, high impact):** maximum test coverage.
Multiple test types. Monitored in production.

**Important (high on one axis):** solid automated coverage. At least
unit and integration tests.

**Low priority (low on both):** basic happy-path coverage. Don't
waste time on edge cases for low-risk code.

### Applying it

For each feature or module:
1. Rate likelihood (1-5) and impact (1-5)
2. Multiply for a risk score (1-25)
3. Allocate test effort proportional to the score
4. Review quarterly — risk profiles change as code stabilizes or
   gains new dependencies

## Framework 4 — RCRCRC Regression Selection

James Bach's heuristic for deciding what to retest after a change.
When you can't retest everything (and you usually can't), prioritize:

- **Recent**: code changed in this release or sprint. Fresh changes
  are the most likely source of new bugs.
- **Core**: primary user workflows. The features that define the
  product. If these break, nothing else matters.
- **Risky**: complex areas, areas with history of defects, areas
  using new or unfamiliar technology.
- **Configuration-sensitive**: behavior that varies by environment,
  settings, feature flags, user roles. The same code behaves
  differently under different configurations.
- **Repaired**: bugs that were recently fixed. Fixes often introduce
  new bugs or incomplete fixes regress.
- **Chronic**: areas that break repeatedly. Some code is just fragile.
  Until it's rewritten, it needs retesting every release.

**How to use it:** after each change, score each area of the system
against these six criteria. The areas that score highest on the most
criteria get retested first. This isn't a formula — it's a
prioritization heuristic for when time is limited.

## Framework 5 — Kent Beck's Test Desiderata

From Beck (2019). Twelve properties of good tests. The critical
insight: *these properties trade off against each other.* No test
can maximize all twelve simultaneously. The art of testing is choosing
which properties to prioritize for a given context.

### The twelve properties

1. **Isolated** — tests don't affect each other. Can run in any order,
   in parallel. Failure of one test doesn't cascade.

2. **Composable** — tests can run in any combination: the full suite,
   a subset, a single test. No test requires another test to run first.

3. **Deterministic** — same code, same test, same result every time.
   No flakiness. A test that fails intermittently is worse than no
   test — it trains people to ignore failures.

4. **Fast** — fast enough to run frequently. "Frequently" is relative:
   unit tests should be sub-second, E2E tests might take minutes. The
   measure is whether developers actually run them before committing.

5. **Writable** — cheap to create. Low ceremony, minimal boilerplate.
   If writing a test takes longer than writing the feature, fewer
   tests will be written.

6. **Readable** — the intention is clear to someone who didn't write
   the test. A failing test should communicate *what went wrong*
   without requiring the reader to trace through helper functions.

7. **Behavioral** — tests describe *what* the system does, not *how*
   it does it. Testing behavior means refactoring internals doesn't
   break tests.

8. **Structure-insensitive** — changes to code structure (extracting
   methods, reorganizing modules) don't require test changes. The
   complement of behavioral: if tests are behavioral, they're
   naturally structure-insensitive.

9. **Automated** — no human judgment required to determine pass/fail.
   The test runner reports green or red. No "check that the output
   looks right."

10. **Specific** — when a test fails, you know what's broken. One
    test, one failure reason. A test that fails for five different
    reasons requires debugging to figure out which one.

11. **Predictive** — passing tests give confidence the code works in
    production. The test environment is representative enough that
    results transfer to real conditions.

12. **Inspiring** — passing tests give you confidence to make changes.
    This is the subjective experience of trusting the suite. If you
    don't trust the tests, you'll manually verify anyway, and the
    tests add cost without reducing fear.

### Key trade-offs

These are the tensions that make test design non-trivial:

**Fast vs. Predictive:** the fastest tests are the most isolated from
production reality. The most predictive tests (real database, real
network, real infrastructure) are the slowest. Every test suite sits
somewhere on this spectrum.

**Behavioral vs. Specific:** testing behavior at a high level makes
tests structure-insensitive but makes failures vague. Testing at a
low level makes failures specific but couples tests to implementation
details.

**Writable vs. Readable:** the fastest way to write a test (copy-paste,
magic helpers, heavy setup abstraction) often makes it harder to read.
The most readable tests (explicit setup, clear assertions, no hidden
state) take longer to write.

**Isolated vs. Predictive:** isolated tests mock away real
dependencies, which is fast and deterministic but less representative
of production. Predictive tests use real dependencies, which is slow
and potentially flaky but catches integration bugs.

### Using the desiderata

When designing a test, ask: which of these twelve properties matter
most for this specific test? A unit test for a pure function should
maximize fast, isolated, deterministic, specific. An E2E smoke test
should maximize predictive and inspiring at the expense of fast and
isolated.

When evaluating an existing test suite, rate each property 1-5 across
the suite. Low scores identify systemic problems: a suite that's
writable but not readable will accumulate tests nobody can maintain.
A suite that's fast but not predictive will pass while production
burns.

## Framework 6 — Google's Test Size Classification

From *Software Engineering at Google* (Winters, Manshreck, Wright,
2020). Google classifies tests by constraints on execution
environment, not by what's being tested.

### Small tests

- Run in a single process
- No I/O: no disk, no network, no database
- No sleep or waiting
- Must complete in seconds
- Hermetic and deterministic by design

A test that exercises a complex algorithm but stays in one process
with no I/O is "small," even if it tests substantial logic.

### Medium tests

- Run on a single machine
- Can access localhost: database, file system, local services
- No access to external or remote services
- Must complete in minutes

A test that verifies database queries against a local PostgreSQL
instance is "medium."

### Large tests

- Can span multiple machines
- Can access real external services
- Can take minutes to hours
- Used for full system integration, performance, reliability testing

**The mapping to shapes:**
- Small ≈ unit tests (pyramid base)
- Medium ≈ integration tests (pyramid middle)
- Large ≈ E2E tests (pyramid top)

**The important distinction:** size is about execution constraints,
not about scope. A test that checks a config value but reads a file
is "medium." A test that exercises an entire state machine in memory
is "small." This is cleaner than arguing about what counts as "unit"
vs. "integration."

**Google's recommended ratio:** roughly 80% small, 15% medium,
5% large. This varies by project — services that are mostly glue
will have more medium tests.

## Framework 7 — Specialized Techniques

### Contract testing

Tests at service boundaries that verify both sides agree on the
interface. Instead of running service A and service B together (slow,
brittle), you test each side independently against a shared contract.

**How it works:**
1. The consumer (caller) writes tests describing what it expects
   from the provider: "when I send GET /users/1, I expect a JSON
   object with fields name (string) and email (string)."
2. These expectations are recorded as a contract (a JSON file).
3. The provider runs the contract against its real implementation.
4. If the provider satisfies the contract, the consumer's
   expectations are met — even though they never ran together.

**When to use it:** microservices, any system with API boundaries
between independently deployed components. Especially valuable when
E2E tests across services are too slow or too flaky.

**When not to use it:** monoliths where both sides are in the same
codebase and deployed together. The overhead of contract management
isn't justified when you can just write an integration test.

### Property-based testing

Instead of specifying individual input/output examples, you specify
properties that should hold for *all* inputs, and the testing
framework generates random inputs to try to falsify the property.

**Example:** instead of testing `sort([3,1,2]) == [1,2,3]`, you test:
- For any list, `sort(list)` has the same length as `list`
- For any list, `sort(list)` is ordered (each element ≤ the next)
- For any list, `sort(list)` contains the same elements as `list`

The framework generates hundreds of random lists, including edge
cases (empty list, one element, duplicates, already sorted, reverse
sorted).

**When to use it:**
- Functions with large input spaces where example-based tests can't
  cover enough ground
- Serialization/deserialization: `deserialize(serialize(x)) == x`
- Data transformations: invariants like "output has same count as
  input"
- Parsers: "valid input never crashes," "parse(format(x)) == x"

**When not to use it:** UI code, I/O-heavy code, code where the
interesting bugs are about specific interaction sequences rather than
input/output relationships.

**The shrinking insight:** when property-based testing finds a failing
input, it automatically shrinks it to the minimal failing case. A
failing list of 200 elements gets reduced to the 3-element list that
reproduces the bug. This is often more valuable than the test itself.

### Mutation testing

Tests your tests. A mutation testing tool makes small changes to your
source code (mutants) — flipping a `>` to `>=`, removing a line,
changing a constant — and runs your test suite against each mutant. If
the tests still pass with the mutation, the mutant "survived" — meaning
your tests don't actually verify that behavior.

**Mutation score:** percentage of mutants killed (detected by tests).
A score of 85% means 15% of mutations went undetected — those are gaps
in test coverage.

**Why it's better than code coverage:** code coverage tells you which
lines were executed. Mutation testing tells you which lines were
*verified*. A test that calls a function but doesn't assert on its
output will show 100% coverage but kill 0% of mutants.

**When to use it:**
- Evaluating whether an existing test suite actually catches defects
- Identifying which code paths are executed but not verified
- High-risk code where you need confidence the tests are real

**When not to use it:** routinely in CI (too slow for most projects).
Better as a periodic audit — run monthly, fix the worst gaps.

## Applying the Frameworks

For a new project or feature:

1. **Choose a shape** (Framework 1) based on architecture. This sets
   the ratio of test types.
2. **Audit quadrant coverage** (Framework 2). Ensure testing activity
   exists in all four quadrants, not just Q1.
3. **Prioritize by risk** (Framework 3). Allocate more effort to
   high-risk areas.
4. **Size tests** (Framework 6). Classify each test as small, medium,
   or large. Track the ratio.
5. **Evaluate test quality** (Framework 5). Score the suite against
   Beck's twelve desiderata. Address the lowest-scoring properties.

For regression after changes:

6. **Select what to retest** (Framework 4) using RCRCRC when full
   regression isn't feasible.

For test suite health audits:

7. **Run mutation testing** (Framework 7) periodically on critical
   code to verify tests actually catch defects.
8. **Check for missing techniques** (Framework 7). Should this system
   have contract tests? Would property-based testing cover an input
   space that examples can't?

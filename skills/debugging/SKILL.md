---
name: debugging
description: >
  Structured debugging using scientific method, 5 Whys, Ishikawa fishbone
  diagrams, fault tree analysis, delta debugging, wolf fence bisection,
  Kepner-Tregoe IS/IS NOT analysis, and blameless incident investigation.
  Adapts to context: simple bugs get wolf fence, complex failures get
  fishbone + fault tree, production incidents get timeline reconstruction
  and contributing factor analysis. Use when investigating bugs, failures,
  or incidents, or when the user invokes /debug. Not for code review.
user-invocable: true
---

# Debugging

Investigate bugs, failures, and incidents systematically. The approach
depends on what's broken and how much is known about the failure.

## Phase 0 — Triage

Before applying any technique, establish the basics:

1. **What is the expected behavior?** Be precise. "It should work" is
   not a specification.
2. **What is the actual behavior?** Exact error messages, screenshots,
   logs. Not summaries — artifacts.
3. **Is this reproducible?** Always/sometimes/once. Reproducibility
   determines which techniques apply.
4. **What changed?** Recent deploys, config changes, dependency updates,
   data migrations. The answer "nothing" is almost always wrong.

Based on triage, select a technique from the sections below. Simple,
reproducible bugs rarely need fishbone diagrams. Production incidents
with unclear cause rarely yield to wolf fence alone.

---

## 1. Scientific Debugging

The foundation everything else builds on. Andreas Zeller formalized
this in *Why Programs Fail*, but the core is just the scientific method
applied to code.

### The cycle

```
Observe failure
    → Form hypothesis ("The null pointer comes from the cache lookup
       returning None when the key contains unicode")
    → Design experiment that can FALSIFY the hypothesis
    → Run experiment
    → Analyze results
    → Refine or reject hypothesis
    → Repeat
```

### Zeller's principles

- **Every bug has a cause.** Bugs don't appear from nowhere. Something
  in the code, data, environment, or timing produces the failure.
  Nondeterministic bugs have deterministic causes — the
  nondeterminism is usually timing, concurrency, or uninitialized memory.

- **Isolate the cause.** A hypothesis must be specific enough to test.
  "Something is wrong with the database layer" is an observation, not
  a hypothesis. "The connection pool exhausts because the health check
  query holds a connection for 30s under network partition" is testable.

- **Experiments must be able to fail.** If an experiment can only confirm
  your hypothesis, it's worthless. Design experiments where a specific
  result would *disprove* what you believe. This is the hardest part
  and the part most people skip.

- **One change at a time.** Changing two things and seeing the bug
  disappear tells you nothing about which change fixed it. Worse, the
  two changes might mask each other — one fixes the bug, the other
  introduces a new one that happens to look the same.

### Example

**Observation:** API returns 500 on POST /orders after deploy.

**Bad hypothesis:** "The deploy broke something." (Unfalsifiable, too vague.)

**Good hypothesis:** "The new validation middleware rejects requests
where `shipping_address` is nested under `customer` instead of at
the top level, because the schema migration in PR #412 moved the
field but the middleware still references the old path."

**Experiment:** Send a POST with `shipping_address` at both locations.
If the old location works and the new one fails, the hypothesis is
confirmed. If both fail, something else is wrong.

---

## 2. Five Whys

Taiichi Ohno's technique from the Toyota Production System. Deceptively
simple, genuinely useful for a specific class of problem, and genuinely
dangerous when misapplied.

### Ohno's classic example

> **Problem:** The machine stopped.
>
> **Why?** A fuse blew due to overload.
> **Why?** The bearing wasn't lubricated enough.
> **Why?** The lubrication pump wasn't working properly.
> **Why?** The pump shaft was worn.
> **Why?** No strainer was installed, so metal shavings got in.
>
> **Root cause:** Missing strainer. **Fix:** Install a strainer.

The power is in refusing to stop at the first plausible cause. "A fuse
blew" is true but useless — replacing the fuse doesn't prevent
recurrence.

### When it works

- Linear causal chains where each step has a single dominant cause
- Operational problems where you need to find the systemic gap
- Post-incident analysis where the proximate cause is obvious but
  the organizational cause isn't
- When the people in the room have direct knowledge of each "why"

### When it fails

- **Multiple contributing causes.** Five Whys assumes a single chain.
  If the failure requires A AND B AND C, following one chain misses
  the others entirely. Use fishbone or fault tree instead.

- **Premature convergence.** People stop at the first "why" that sounds
  like something they can fix, or worse, at the first "why" that
  blames someone. "Why did it fail? Because the developer didn't
  test it" is not root cause analysis — it's finger-pointing with
  extra steps.

- **Complex systems.** In distributed systems, the causal chain is
  rarely linear. A cascading failure might involve a thundering herd
  triggered by a retry storm triggered by a slow database triggered
  by a missing index triggered by a schema migration that removed it.
  Five Whys can trace one path through this, but the actual cause is
  the interaction between all of them.

### How to do it well

At each "why," ask: "Is this the ONLY reason?" If not, branch. If you
find yourself branching more than twice, switch to fishbone.

---

## 3. Ishikawa Fishbone Diagram

Kaoru Ishikawa's cause-and-effect diagram. The "fishbone" shape
organizes potential causes into categories, preventing tunnel vision.

### The 6M categories, mapped to software

In manufacturing, the six categories are Man, Machine, Method, Material,
Measurement, and Mother Nature (Environment). For software:

| Manufacturing | Software equivalent | Examples |
|---|---|---|
| Man | **People** | Developer error, miscommunication, knowledge gaps, on-call fatigue |
| Machine | **Infrastructure** | Hardware failure, cloud provider issue, network, DNS, certificates |
| Method | **Process** | Deployment procedure, testing gaps, review process, rollback plan |
| Material | **Code & Data** | Bug in code, corrupt data, schema mismatch, dependency version |
| Measurement | **Observability** | Missing metrics, misleading alerts, log gaps, wrong thresholds |
| Mother Nature | **Environment** | Traffic spike, third-party outage, timezone edge case, leap second |

### How to build one systematically

1. Write the problem statement on the right (the fish's "head").
   Be specific: not "site is slow" but "P99 latency for /api/search
   exceeds 2s during business hours since March 15."

2. Draw the six category branches.

3. For each category, brainstorm specific potential causes. Don't
   evaluate yet — just generate. Bad ideas provoke good ones.

4. For each potential cause, ask "why would this happen?" and add
   sub-branches. Two or three levels of depth is usually enough.

5. Now evaluate. For each leaf, ask: "Do we have evidence for or
   against this?" Mark each as confirmed, ruled out, or unknown.

6. Investigate the unknowns, starting with whichever is cheapest to
   check.

### Example: P99 latency spike

```
                                    ┌─ People ──── New team member deployed without perf testing
                                    │              On-call missed gradual degradation
                                    │
                                    ├─ Infra ───── DB replica lag increased
                                    │              Pod memory limit too low → GC pressure
                                    │
Problem: P99 > 2s ◄────────────────├─ Process ──── No load test in CI pipeline
on /api/search                      │              Canary deploy disabled last sprint
since March 15                      │
                                    ├─ Code ────── N+1 query in new search filter
                                    │              Missing index on `products.category_id`
                                    │
                                    ├─ Observ. ──── Latency alert threshold was 5s, not 2s
                                    │              No per-endpoint breakdown in dashboard
                                    │
                                    └─ Environ. ── March 15 = start of spring sale traffic
                                                   CDN config change by provider
```

The value is in *seeing all the categories at once*. Without the
diagram, investigations tend to anchor on the first plausible cause
(usually Code) and ignore Process, Observability, and Environment.

---

## 4. Fault Tree Analysis

Originally developed for ICBM launch safety in the 1960s. Works
backward from an undesired event (the "top event") through logical
gates to find the minimal combinations of basic events that can
cause the failure.

### AND/OR gates

- **OR gate:** The parent event occurs if ANY child event occurs.
  Most failures are OR gates — there are multiple independent ways
  things can break.

- **AND gate:** The parent event occurs only if ALL child events
  occur simultaneously. These represent failures that require
  multiple things to go wrong. AND gates are where your defenses
  live — or should.

### Building a fault tree

```
        [Data loss on write]          ← Top event
              |
           OR gate
         /        \
[Primary DB      [Replication
 write fails]     fails silently]
    |                  |
 OR gate            AND gate
 /     \           /        \
[Disk  [OOM     [Replica    [Monitoring
full]  kill]     lag > 60s]  doesn't alert]
```

Read this tree: Data loss occurs if the primary write fails (due to
disk full OR OOM kill) OR if replication fails silently (which
requires BOTH replica lag AND monitoring failure). The AND gate is
critical — if monitoring works, silent replication failure can't
happen.

### Minimal cut sets

A *cut set* is any combination of basic events (leaf nodes) that
causes the top event. A *minimal* cut set has no unnecessary events —
every event in it is required.

From the tree above:
- {Disk full} — a single-event cut set, meaning disk full alone
  causes data loss
- {OOM kill} — another single-event cut set
- {Replica lag > 60s, Monitoring doesn't alert} — a two-event cut
  set, requires both

### Finding single points of failure

Any minimal cut set with a single event is a **single point of
failure**. These are your highest-priority fixes. In the example,
both "disk full" and "OOM kill" are single points of failure for
data loss. The replication path is better defended because it requires
two things to go wrong.

### When to use fault trees

- Safety-critical systems where you need to enumerate failure modes
- Post-incident analysis where you want to understand *all* the
  ways the incident could have happened, not just the one that did
- Capacity planning — understanding which component failures cascade
- Architecture review — finding single points of failure before
  they bite

---

## 5. Delta Debugging

Andreas Zeller's algorithm for automatically minimizing a failure-
inducing input (or change set). The insight: if a test case with 1000
lines triggers a bug, there's usually a much smaller subset that
also triggers it. Delta debugging finds that subset.

### The ddmin algorithm

Given a set of changes C that causes a failure, and a test function
that reports PASS, FAIL, or UNRESOLVED:

1. Split C into n partitions (start with n=2).
2. Test each partition individually.
3. If a partition fails, recurse on that partition (it contains the
   cause).
4. If no partition fails, test the complement of each partition.
5. If a complement fails, recurse on that complement.
6. If nothing fails individually, increase granularity (n = 2n) and
   repeat.
7. Stop when you can't reduce further — every remaining element is
   necessary.

### Complexity

O(n log n) tests in the worst case, where n is the number of changes.
For a 1000-line diff, that's roughly 10,000 tests — feasible if each
test is fast, prohibitive if each test takes minutes.

### Practical applications

- **Minimizing test cases.** A 500-line input that crashes the parser
  can usually be reduced to 5-10 lines that still crash it. Easier
  to understand, easier to fix, better regression test.

- **Bisecting change sets.** Given a set of commits between "working"
  and "broken," delta debugging can find the minimal set that causes
  the failure. Unlike `git bisect`, this handles cases where the bug
  requires multiple commits interacting.

- **Reducing configuration.** A complex config file that triggers
  a startup failure can be minimized to the specific combination of
  settings that conflict.

### Example

**Input:** 800-line JSON payload causes a 500 error.

1. Split into two 400-line halves. Test each. First half passes,
   second half returns UNRESOLVED (malformed JSON).
2. Test complement of first half (= second half). UNRESOLVED.
   Test complement of second half (= first half). PASS. Increase
   granularity to 4 partitions of 200 lines each.
3. Partition 3 (lines 401-600) alone: FAIL. Recurse.
4. Split partition 3 into two 100-line halves. First half: PASS.
   Second half: FAIL. Recurse.
5. Continue until reaching: lines 487-491 (a nested array with
   duplicate keys) cause the failure. 5 lines out of 800.

---

## 6. Wolf Fence / Binary Search Bisection

"There's a wolf somewhere in Alaska. Build a fence down the middle.
Listen for the wolf. It's in the north half. Build another fence
down the middle of the north half. Repeat."

### The technique

Binary search applied to debugging. Works on anything that has a
linear ordering: lines of code, commits in history, time, config
entries, input records.

### git bisect (the most common application)

```bash
git bisect start
git bisect bad                    # current commit is broken
git bisect good v2.3.0            # this tag was known working
# Git checks out the midpoint. Test it.
git bisect good                   # or "git bisect bad"
# Repeat until Git identifies the first bad commit.
git bisect reset                  # return to original HEAD
```

For automated bisection with a test script:
```bash
git bisect start HEAD v2.3.0
git bisect run ./test_the_thing.sh
```

The script should exit 0 for good, 1-124 (except 125) for bad,
and 125 for "skip" (can't test this commit — e.g., won't compile).

### Complexity

O(log n). For 1000 commits, roughly 10 steps. For 1,000,000 commits,
roughly 20 steps.

### Beyond git bisect

The same logic applies anywhere:

- **Commenting out code.** Comment out the bottom half of a function.
  Bug gone? It's in the bottom half. Still there? Top half. Repeat.

- **Log injection.** Add a log statement halfway through the
  execution path. Is the state correct there? If yes, bug is
  downstream. If no, bug is upstream. Repeat.

- **Input bisection.** Process the first half of the input file.
  Failure? Bug is triggered by something in the first half. Repeat.

### When wolf fence doesn't work

- The failure requires interaction between two distant parts (bisecting
  either part alone won't reproduce it)
- The search space isn't linearly ordered
- The "test" is expensive or flaky
- There are multiple bugs overlapping

---

## 7. Rubber Duck Debugging

Explain the problem out loud to an inanimate object (traditionally
a rubber duck). The technique sounds silly and works embarrassingly
well.

### Why it works

**System 1 vs System 2 (Kahneman).** When reading code, the brain
uses fast, pattern-matching System 1 thinking. You *see* what you
expect to see, not what's there. Explaining the code forces a shift
to slow, deliberate System 2 processing. You have to actually read
each line instead of scanning past it.

**Verbal overshadowing.** The act of translating visual/spatial
understanding into sequential language forces linearization. Code
execution is sequential; visual scanning is not. When you explain
"first this happens, then this happens," you're simulating execution
in the order the computer actually does it, which exposes ordering
assumptions you didn't know you were making.

**The generation effect.** Producing information (speaking it) creates
stronger cognitive engagement than consuming it (reading it). You
literally process the code more deeply when explaining it than when
reading it.

### How to do it

1. State what the code is supposed to do.
2. Walk through it line by line, explaining what each line actually
   does (not what you intended it to do).
3. At each step, state what the program state should be. Variables,
   flags, collections — be explicit.
4. When you find a discrepancy between "should" and "does," that's
   your bug.

The duck is not optional. Explaining "in your head" doesn't work as
well because it's too easy to skip steps. The external commitment of
speaking (or typing to a chat) forces completeness.

---

## 8. Kepner-Tregoe IS/IS NOT Analysis

Charles Kepner and Benjamin Tregoe's structured problem analysis
technique. The core insight: precisely defining what a problem IS
and IS NOT generates *distinctions* that point directly to the cause.

### The four dimensions

| Dimension | IS | IS NOT |
|---|---|---|
| **WHAT** | What object/system has the problem? What is the defect? | What similar objects/systems do NOT have the problem? What similar defects are NOT occurring? |
| **WHERE** | Where geographically/logically is the problem observed? | Where is it NOT observed? |
| **WHEN** | When was it first observed? When does it occur? Any pattern? | When does it NOT occur? When was it last known to be working? |
| **EXTENT** | How many units affected? How much of each unit? What's the trend? | How many are NOT affected? How much is NOT affected? |

### How the distinctions point to cause

The magic is in the IS NOT column. Every IS NOT entry is something
the cause must *not* affect. This eliminates hypotheses rapidly.

### Example: Login failures

| Dimension | IS | IS NOT |
|---|---|---|
| WHAT | Login with email/password fails with 403 | Login with SSO works fine. Login with email/password on mobile app works. |
| WHERE | Web app, all browsers | Mobile app. Internal admin panel. |
| WHEN | Started Tuesday 3pm. Happens on every attempt. | Was working Monday. No pattern within the day. |
| EXTENT | 100% of email/password web logins. | 0% of SSO logins. 0% of mobile logins. 0% of admin logins. |

**Distinctions:** The cause must:
- Affect web but not mobile or admin
- Affect email/password but not SSO
- Have changed Tuesday around 3pm
- Be 100% reproducible (not intermittent)

**Investigation:** What changed Tuesday at 3pm that affects web
email/password auth but not mobile, SSO, or admin? Check the deploy
log. A CSRF token rotation was deployed to the web app at 2:47pm.
The mobile app doesn't use CSRF tokens. SSO bypasses the login form.
The admin panel uses a different session store. The CSRF token
rotation broke the login form's token validation.

Without the IS/IS NOT analysis, the investigation might have spent
hours checking the auth service, the database, the load balancer —
all of which affect mobile and SSO equally and therefore cannot be
the cause.

---

## 9. Incident Investigation

For production incidents where the goal is understanding what happened,
why, and how to prevent recurrence. This is not debugging a single
bug — it's analyzing a system failure.

### Timeline reconstruction

The single most valuable artifact in incident investigation.

1. **Gather raw data.** Logs, metrics, alerts, deploy history, chat
   messages, on-call pages, customer reports. Timestamps for everything.

2. **Build the timeline.** Chronological sequence of events with
   timestamps. Include:
   - System events (deploys, config changes, scaling events)
   - Alerts and pages
   - Human actions (who did what, when, based on what information)
   - Customer-visible impact start and end

3. **Annotate with "known at the time."** For each human action, note
   what information was available to the person at that moment. This
   is critical for blameless analysis — decisions that look obviously
   wrong in hindsight were often reasonable given the information
   available.

### Example timeline

```
14:23  Deploy v2.47 to production (automated, all checks green)
14:31  Error rate on /api/checkout rises from 0.1% to 2%
14:35  PagerDuty alert: "checkout error rate > 1%"
14:37  On-call acknowledges, begins investigation
14:38  On-call checks deploy diff — 14 files changed, nothing
       obviously related to checkout
14:42  On-call checks database metrics — all normal
14:45  Error rate rises to 8%
14:47  On-call decides to rollback v2.47
14:49  Rollback complete
14:51  Error rate returns to 0.1%
14:55  Incident resolved, begin investigation
```

### Contributing factor analysis

Avoid "root cause" language. Complex system failures rarely have a
single root cause. Instead, identify *contributing factors* — things
that, if different, would have prevented or mitigated the incident.

Categories of contributing factors:

- **Triggering cause.** The proximate event. (The deploy introduced
  a bug in the payment validation path.)
- **Latent conditions.** Things that were already wrong but hadn't
  caused a failure yet. (The checkout service had no circuit breaker
  for payment validation failures.)
- **Missing defenses.** Safeguards that should have existed. (No
  integration test covered the payment validation path with the new
  field format.)
- **Amplifying factors.** Things that made the impact worse. (The
  retry logic turned a 2% error rate into an 8% error rate by
  hammering the already-failing service.)
- **Detection delays.** Why it wasn't caught sooner. (The alert
  threshold was 1%, but the error rate grew gradually and took 4
  minutes to cross the threshold.)

### Blameless postmortem format

```markdown
## Incident: [Title]
**Date:** YYYY-MM-DD
**Duration:** X hours Y minutes
**Severity:** SEV-N
**Impact:** [Customer-facing impact in plain language]

## Timeline
[Chronological events as above]

## Contributing Factors
1. [Triggering cause]
2. [Latent condition]
3. [Missing defense]
...

## What Went Well
- [Things that worked — fast detection, effective rollback, etc.]

## Action Items
| Item | Owner | Priority | Due |
|------|-------|----------|-----|
| Add integration test for payment validation | @dev | P1 | Sprint 23 |
| Implement circuit breaker on checkout→payment | @dev | P1 | Sprint 23 |
| Lower alert threshold to 0.5% | @ops | P2 | This week |
| Review retry configuration across all services | @arch | P2 | Sprint 24 |

## Lessons Learned
[What this incident taught us about our systems, processes, or
assumptions that we didn't know before]
```

The "What Went Well" section matters. Incident investigation that
only finds failures breeds a culture of hiding problems.

---

## 10. The Debugging Mindset

What distinguishes effective debuggers from ineffective ones isn't
intelligence or experience — it's disposition.

### What good debuggers do

- **Read the error message.** The entire error message. Including the
  stack trace. Including the part after "caused by." This sounds
  obvious and yet the majority of debugging time is spent chasing
  theories that the error message already rules out.

- **Check assumptions.** "I know this value is non-null here" — prove
  it. Add an assertion. Print it. Good debuggers are pathologically
  distrustful of their own understanding.

- **Reproduce before theorizing.** A bug you can't reproduce is a bug
  you can't verify a fix for. Reproduction comes first, always,
  even when the cause seems obvious.

- **Change one thing at a time.** Then test. Then change the next
  thing. Changing three things at once and seeing the bug disappear
  means you've added two unnecessary changes and can't explain why
  the fix works.

- **Keep notes.** What was tried, what was observed, what was ruled
  out. Debugging is search, and search without memory revisits the
  same states repeatedly.

- **Question the environment.** Is this the right branch? The right
  build? The right config? The right database? "It works on my
  machine" is a diagnosis, not a dismissal — it means the
  environment differs, and the difference matters.

### What bad debuggers do

- Fix the symptom, not the cause. (Catch the exception, swallow it,
  move on.)
- Change things at random until it works, then commit without
  understanding why.
- Blame the tools, the framework, the compiler, the OS — before
  checking their own code.
- Skip reproduction. "I think it's this" → change → "seems to work
  now" → deploy → surprised when it recurs.
- Anchor on the first hypothesis and seek only confirming evidence.

### When to step away

The sunk cost of debugging time creates a powerful gravitational pull
to keep going. Sometimes the most productive thing to do is stop.

Step away when:
- The same hypothesis has been tested three different ways and keeps
  being "almost confirmed." It's wrong. Step away, let it go.
- You've been at it for more than two hours without new information.
  Fresh eyes — yours or someone else's — will find it in ten minutes.
- You're making changes you can't explain. If you're editing code to
  "see what happens" without a hypothesis, you're not debugging,
  you're thrashing.
- You're angry at the code. Emotion narrows attention. Walk away.

---

## When to Use What

| Situation | Start with | Add if needed |
|---|---|---|
| Simple reproducible bug, known area | Wolf fence / bisect | Rubber duck if stuck |
| Reproducible bug, unknown area | Scientific debugging | Wolf fence to narrow, then rubber duck |
| Flaky / intermittent failure | Kepner-Tregoe IS/IS NOT | Scientific debugging with timing hypotheses |
| Complex failure, many possible causes | Ishikawa fishbone | Fault tree for critical paths |
| Regression after changes | git bisect (wolf fence) | Delta debugging if multiple changes interact |
| Large failing input | Delta debugging | Scientific debugging on the minimized case |
| Production incident, unclear scope | Timeline reconstruction | Kepner-Tregoe to narrow, fishbone for causes |
| Post-incident analysis | Contributing factor analysis | 5 Whys for each contributing factor |
| Architecture review / prevention | Fault tree analysis | Fishbone to ensure coverage of all categories |
| "I have no idea what's happening" | Rubber duck → scientific method | Kepner-Tregoe if rubber duck doesn't unstick |

No technique is universal. The skill is in recognizing which situation
you're in and picking the technique that fits, rather than reaching
for the same hammer every time.

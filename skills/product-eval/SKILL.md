---
name: product-eval
description: >
  Structured product evaluation using Jobs-to-be-Done analysis, falsifiability
  testing, scope discipline, primary-deliverable critique, guardrail-metric audit, and
  risk identification. Use when reviewing PRDs, product briefs, feature specs,
  or phase plans. Produces severity-rated findings on hypothesis quality,
  scope coherence, success criteria, sequencing, and gaps. Not for code
  review, UI design, or architecture evaluation.
user-invocable: true
---

# Product Eval

Evaluate a product document — PRD, spec, brief, phase plan — through
structured frameworks applied in sequence. The goal is not to improve
the document's prose, but to find where the product reasoning breaks:
where the hypothesis is unfalsifiable, where the scope creates the
opposite of focus, where the success metrics reward the wrong behavior,
where a non-goal is actually load-bearing.

Findings are severity-rated. Every finding names a specific claim,
gap, or contradiction in the document — no generic product advice.

---

## Phase 1 — Hypothesis Falsifiability

A product hypothesis is only useful if it can be proven false. A
hypothesis that is structured to always confirm itself wastes the
experiment.

For each stated hypothesis or goal:

1. **State the falsifiable form.** What would have to be true for the
   hypothesis to be wrong? If no answer exists, the hypothesis is
   unfalsifiable. That is a finding.

2. **Check the proving mechanism.** Is the stated evidence sufficient
   to confirm or disconfirm? Direct user behavior (would a target
   user adopt it over their current solution) is evidence.
   "Stakeholders expressed interest" is not evidence — it measures
   reception, not impact.

3. **Check for post-hoc flexibility.** If success criteria are vague
   enough that any outcome can be framed as partial success, the
   hypothesis is effectively unfalsifiable. Look for phrases like
   "demonstrates traction," "shows progress," "provides evidence," or
   "tests the architecture" without a specific threshold.

4. **Gate vs. goal.** The gate is binary (hit or miss). The goal is
   the north star. If a document treats a long-horizon outcome as
   the immediate phase gate, the team will misjudge success.

Severity guide:
- **BLOCKER**: Gate is entirely unfalsifiable; no way to determine at
  deadline whether it was hit.
- **MAJOR**: Gate is falsifiable but success criteria are vague enough
  that reasonable people will disagree on the verdict.
- **MINOR**: Gate is clear but a secondary signal is ambiguous.

---

## Phase 2 — Scope Coherence

Scope failures come in two kinds: too much and hidden dependencies.
Both produce the same outcome — missed gates and post-hoc rationalization.

### 2a. Scope creep audit

List every requirement, user story, breadth test, stretch goal, and
"qualitative experiment" in the document. For each:

- Is it required for the stated gate? If no, is it explicitly gated
  on the primary gate being hit first?
- Does it share engineering resources with the primary deliverable? If yes,
  it competes for the gate, not after it.
- Is it described as "qualitative only" or "exploratory" but receiving
  the same sprint-level attention as the primary deliverable? That is scope
  creep with a label.

### 2b. Hidden dependency audit

List every "this enables" or "this depends on" relationship in the
document — explicit or implied. For each:

- Is the dependency already built? If no, is it on the critical path?
- Is the dependency owned by a different team or system? If yes, is
  there a named owner and a delivery commitment?
- Does a stretch goal implicitly require infrastructure that the
  primary deliverable doesn't? If yes, the stretch goal's real cost is
  hidden.

### 2c. Reception-as-success patterns

The most dangerous scope addition is one whose success is measured
by reception rather than user outcome. Flag features whose stated
proof of success is "people were impressed" or "demo went well"
rather than a concrete user outcome (time saved, error reduced,
work completed).

---

## Phase 3 — Jobs-to-be-Done Coherence

JTBD asks: what is the user trying to accomplish, and does this product
actually help them accomplish it? The framework resists feature-level
thinking and forces the evaluator to trace from user outcome to
product mechanism.

For each stated primary user and JTBD:

1. **Name the functional, emotional, and social job.** The functional
   job is the task. The emotional job is how the user wants to feel
   during it. The social job is how they want to be perceived afterward.
   Most PRDs only state the functional job. Missing emotional and social
   jobs produce products that are "correct" but not "trusted."

2. **Trace the mechanism.** For the stated functional job, is there
   a direct mechanism in the product that addresses it? Or is there
   a mechanism that addresses a proxy (e.g., a surface-level metric
   that correlates with the real outcome but isn't the outcome)?

3. **Check for job substitution.** Is the product solving the job the
   user has, or the job the team wishes the user had? This appears as
   "users don't yet realize they want X" reasoning without evidence
   that users would, in fact, want X.

4. **Check hiring criteria.** Users "hire" a product when it does a
   job better than their current solution. What is the current
   solution? Is the product meaningfully better? "Meaningfully" means
   on the dimension the user cares about, not the dimension the team
   finds interesting.

---

## Phase 4 — Success Metrics Audit

Good metrics are specific, owned, and anti-gameable. Bad metrics
produce incentives to optimize the metric rather than the outcome.

For each stated metric or leading indicator:

1. **Specificity.** Does it have a named numerator, denominator, and
   time window? A metric is specific if it names exactly what counts,
   over what window, and how the count is gathered. "Demonstrates
   traction" is not specific.

2. **Ownership.** Who measures it? Who decides whether it was hit?
   If ownership is ambiguous, the gate will be disputed at the worst
   moment.

3. **Gaming resistance.** Can the metric be hit while the underlying
   problem gets worse? A ratio metric can be gamed by degrading the
   denominator. A volume metric (count of tasks completed) can be
   gamed by selecting easy tasks. Name the gaming path and check
   whether it is guarded.

4. **Guardrail metrics.** A guardrail is "if we see X, it means we
   failed even if the headline metric is green." Does the document name
   any? If not, flag it — guardrail metrics force teams to be honest
   about what they're actually optimizing.

5. **Capacity metrics.** If a north-star metric tracks a rate or
   capacity (throughput per unit, latency at percentile, etc.),
   check that the measurement baseline is set before work starts
   and the re-measurement is scheduled. North-star metrics that are
   "tracked but not gated" often never get re-measured.

---

## Phase 5 — Non-Goals Audit

Non-goals fail in two ways: they are actually load-bearing (removing
them would break something implied by the goals), or they are stated
but not respected (scope added later contradicts them).

For each stated non-goal:

1. **Is it actually non-load-bearing?** Read the goals and requirements.
   Does any goal implicitly require the non-goal to be partially
   built? Watch for the pattern where the non-goal is the *full*
   version of a feature ("X is a non-goal") but a requirement
   commits to a *partial* version of it ("ship a minimal X for
   evaluation"). The partial version still requires the
   infrastructure the non-goal claims to defer.

2. **Is there a stated enforcement mechanism?** Non-goals without
   an enforcement mechanism are aspirations. If the document has a
   course-correction rule ("if gate is missed, pause secondary explorations"),
   that is enforcement. If the non-goal is simply declared without a
   corresponding guardrail, flag it.

3. **Are any non-goals in tension with the primary user's JTBD?**
   A non-goal that removes a major friction point for the primary user
   will create pressure to "just add it" mid-sprint. Name that tension.

---

## Phase 6 — Risk and Assumption Audit

Every PRD rests on assumptions. The ones that aren't named are the
ones that will break first.

For each named risk:
- Is the mitigation actually a mitigation, or is it a restatement of
  the risk? ("Invest in quality" does not mitigate "the system
  isn't good enough.")
- Is the mitigation owned and scheduled, or is it aspirational?

For unnamed assumptions (the more important audit):
- What must be true for the primary deliverable to succeed? List them.
- Which of those are not verified? Those are risks.
- Categories of common unnamed assumptions: user behavior, technical
  dependencies, team capacity, environmental conditions.

---

## Phase 7 — Sequencing and Pacing

The critical path is the longest chain of dependent tasks. A proving
gate that depends on unbuilt infrastructure is a gate with a hidden
dependency.

For each phase or milestone:

1. **What must exist before this milestone begins?** Is it built?
2. **What is the longest dependency chain?** Does the milestone's
   timebox accommodate the full chain, including ramp-up, iteration,
   and review?
3. **Is the gate timed to produce a real signal?** A gate at "end of
   Week 2" is only meaningful if there's time after it to either
   course-correct or expand. A gate at the last day of the timebox
   produces a retrospective, not a course-correction.
4. **Are secondary explorations sequenced after the gate, or before?** If
   secondary explorations share engineering time with the primary deliverable, they
   are before the gate regardless of how the document describes them.

---

## Output Format

Produce findings organized by severity across all phases. Do not
group by phase — a reader should be able to act on the top findings
without reading the full report.

```
## Findings

### BLOCKER

**[Short name]**
[One sentence naming the specific claim, gap, or contradiction.]
[One to three sentences explaining why it matters for the gate or the hypothesis.]
Location: [Section or requirement in the document.]

### MAJOR

...

### MINOR

...

### OBSERVATIONS (no action required)

...
```

Do not produce findings for things the document handles well.
Do not produce generic advice ("tighten scope," "add more metrics").
Every finding must cite a specific claim in the document.

If the document is well-reasoned and the findings are all MINOR or
below, say so explicitly. A clean bill of health is a valid output.

---

## Non-negotiables

- Never produce a finding that is just "this is ambitious." Ambition
  is not a problem; unfalsifiability is.
- Never suggest adding features, requirements, or users.
- Never endorse a gate without checking whether it is falsifiable.
- Never treat "stakeholders were excited" as evidence of product-market
  fit. Reception is not outcome.
- Never produce more than ten findings. More than ten means the
  evaluator is listing, not prioritizing. Prioritize ruthlessly.
- Read the stated non-goals before producing any "missing feature"
  finding. If the feature is a non-goal, it is not a finding that it
  is missing.

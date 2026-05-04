---
name: design-review
description: >
  Systematic design evaluation for web UIs using Nielsen's heuristics,
  Gestalt principles, and cognitive walkthrough. Dispatches sub-agents
  as independent evaluators, aggregates severity-rated findings, runs
  fresh-eyes sessions. Use when reviewing a UI's design quality, after
  visual changes, or when the user invokes /design-review. Not for
  functional QA (use qa-web) or code review.
user-invocable: true
---

# Design Review

Evaluate a web UI's design quality through structured heuristic
evaluation. Sub-agents are independent evaluators who don't see each
other's findings until aggregation.

## Phase 1 — Cognitive Walkthrough

For each primary user task, walk the happy path step by step. At
each step, answer four questions:

1. Will the user try the right thing? (Is the sub-goal obvious?)
2. Will the user notice the correct action? (Is the control visible?)
3. Will the user understand what it does? (Does the label/affordance communicate?)
4. Will the user see progress? (Does the system give feedback?)

A "no" at any step is a finding. Document which question failed
and why.

## Phase 2 — Heuristic Evaluation (Nielsen's 10)

Dispatch sub-agents as independent evaluators. Each evaluator reviews
the entire interface against all 10 heuristics, documenting violations
with severity ratings.

**The heuristics** (summary — load `references/nielsens-heuristics.md`
for full explanations, violation examples, and severity guidance):

1. **Visibility of system status** — is the user informed of what's happening?
2. **Match between system and real world** — does it use the user's language, not internal jargon?
3. **User control and freedom** — can mistakes be undone? Are there exits?
4. **Consistency and standards** — do similar things look and work the same?
5. **Error prevention** — does the design prevent problems before they happen?
6. **Recognition over recall** — are options visible, not memorized?
7. **Flexibility and efficiency** — are there shortcuts for experts?
8. **Aesthetic and minimalist design** — does every element earn its place?
9. **Help users recover from errors** — are error messages clear and constructive?
10. **Help and documentation** — is it searchable, task-focused, concrete?

**Severity scale:**
- 0: Not a usability problem
- 1: Cosmetic — fix if time permits
- 2: Minor — low priority
- 3: Major — high priority, fix before release
- 4: Catastrophe — must fix immediately

## Phase 3 — Visual Evaluation (Gestalt + Norman)

Evaluate visual design using perceptual and interaction oracles
(load `references/visual-design-principles.md` for full explanations
and violation examples):

**Gestalt (perceptual consistency):**
- **Proximity** — are related items grouped spatially?
- **Similarity** — do similar functions look similar?
- **Figure-ground** — is content clearly distinguished from chrome?
- **Focal point** — does the most important element get the most emphasis?
- **Common region** — do boundaries create correct groupings?

**Norman (interaction quality):**
- **Affordances** — does each element communicate what's possible?
- **Signifiers** — are interactive elements visibly interactive?
- **Mapping** — do controls relate naturally to their effects?
- **Feedback** — does every action produce visible confirmation?
- **Constraints** — are impossible actions prevented?

**Component state coverage:**
For each interactive component, verify all states exist:
- [ ] Default
- [ ] Hover
- [ ] Focus (keyboard)
- [ ] Active/pressed
- [ ] Disabled (if applicable)
- [ ] Error (if applicable)
- [ ] Loading (if applicable)
- [ ] Empty (if applicable)

## Phase 4 — Fresh Eyes

Spawn a sub-agent with NO context from phases 1-3. It opens the app
cold and evaluates:

1. First impression — what draws the eye? Intentional?
2. Typography hierarchy — can you tell what's most important?
3. Visual consistency — spacing, alignment, color usage
4. Information architecture — does the organization make sense?
5. One sentence: does this feel designed or default?

## Phase 5 — Aggregate and Prioritize

Merge findings from all evaluators. Deduplicate. For each finding:
- Which heuristic(s) violated
- Severity rating (0-4)
- Specific element and location
- Screenshot evidence

Fix everything, severity 4 first, then 3, then 2, then 1.
Re-evaluate affected areas after fixes. Loop until clean.

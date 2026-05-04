---
name: qa-web
description: >
  Exploratory QA for web UIs using agent-browser. Enumerates testable
  areas via SFDPOT heuristics, dispatches sub-agents as independent
  checkers, evaluates results against consistency oracles (FEW HICCUPPS),
  runs fresh-eyes sessions. Use when verifying a web app before shipping,
  after major changes, or when the user invokes /qa. Not for unit tests
  or API testing.
user-invocable: true
---

# QA

Investigate a web UI for problems. Sub-agents perform checks; the
orchestrating agent does the testing — interpreting results, recognizing
patterns, deciding what matters.

## Phase 1 — Survey (SFDPOT)

Open the target in agent-browser. Map coverage using Bach's heuristic:

- **Structure**: pages, routes, components, navigation hierarchy
- **Function**: what each interactive element does (buttons, links,
  toggles, inputs, accordions, file trees, tabs)
- **Data**: what content is displayed, is it rendered correctly
  (markdown as HTML, code in monospace, dates readable)
- **Interfaces**: links between pages, external links, API calls
- **Platform**: viewports (1440, 768, 375), dark mode if applicable
- **Operations**: user workflows end-to-end (not just individual clicks)
- **Time**: loading states, transitions, dynamic content updates

Produce a numbered checklist. Group by risk — what's most likely to be
broken and most damaging if it is. Present to the user for review.

## Phase 2 — Check (sub-agents)

Dispatch sub-agents for each flow. Each sub-agent is a checker — it
executes a specific sequence and reports pass/fail. It does NOT
interpret or make judgment calls.

Sub-agent charter format:

```
Open [URL] in agent-browser. Set viewport to [WxH].

Execute:
1. [action]  → expect [result]
2. [action]  → expect [result]
...

For each step: screenshot before, act, screenshot after.
Report: step, expected, actual, pass/fail, screenshot paths.
Save to /tmp/qa/[flow-name]/
```

Parallelize independent flows. Sequence dependent ones.

## Phase 3 — Evaluate (oracles)

Review sub-agent results against FEW HICCUPPS consistency oracles
(load `references/few-hiccupps.md` for full explanations, diagnostic
questions, violation examples, and prioritization guidance):

- **Familiar**: does it work like things that work?
- **Explainable**: can the behavior be explained?
- **World**: does it match how the real world works?
- **History**: is it consistent with previous versions?
- **Image**: does it match the project's intended quality?
- **Comparable products**: does it work like similar tools?
- **Claims**: does it match what the docs say?
- **User desires**: does it do what users actually want?
- **Product**: is it internally consistent?
- **Purpose**: does it serve its stated purpose?
- **Standards**: WCAG 2.2, semantic HTML, responsive design

A check can pass (element responds to click) while the test fails
(the response doesn't make sense). The oracles catch the difference.

## Phase 4 — Fresh eyes

Spawn a sub-agent with NO context from phases 1-3. It opens the app
cold and explores freely for 10-15 actions, reporting:

1. What draws the eye first — intentional?
2. What feels broken, even if it "works"
3. What's inconsistent across pages
4. What's missing that should be obvious
5. One sentence: would you trust this product?

This catches problems that structured checking misses — the gestalt
issues that only surface when someone encounters the app without
expectations.

## Phase 5 — Report and fix

Categorize findings:

- **Blocking**: broken flows, crashes, data loss
- **Degraded**: works but wrong (bad rendering, wrong content, misleading)
- **Cosmetic**: spacing, alignment, minor visual issues
- **Accessibility**: keyboard traps, missing labels, contrast failures

Fix everything. Blocking first, then degraded, then accessibility,
then cosmetic. Re-check affected flows after each fix. Loop until
all categories are clean — not just the top two.

## Regression (RCRCRC)

After changes, prioritize retesting:

- **Recent**: code changed in this session
- **Core**: primary user workflows
- **Risky**: complex or fragile areas
- **Configuration**: viewport/theme/state-dependent behavior
- **Repaired**: previously broken, just fixed
- **Chronic**: areas that keep breaking

## Accessibility checklist

Run on every QA pass:

- [ ] Every interactive element reachable by keyboard
- [ ] Focus indicator visible, never obscured
- [ ] Images have alt text (or are decorative and hidden)
- [ ] Color contrast ≥ 4.5:1 for text, ≥ 3:1 for UI
- [ ] Touch targets ≥ 24x24 CSS pixels
- [ ] No information conveyed by color alone

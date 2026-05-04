---
name: research
description: >
  Structured research using Cynefin classification, PRISMA-style search
  accountability, Zettelkasten note processing, grounded theory coding,
  ACH for competing hypotheses, and source triangulation. Adapts approach
  to problem type: complicated problems get deep expert analysis, complex
  problems get broad probing with emergent pattern recognition. Use when
  investigating a topic, evaluating tools, building a knowledge base, or
  when the user invokes /research. Not for quick lookups or single-source
  answers.
user-invocable: true
---

# Research

Investigate a topic systematically. The approach depends on the type
of question.

## Phase 1 — Classify the question (Cynefin)

Before searching, determine what kind of problem this is:

**Complicated** (known unknowns): the answer exists somewhere. An
expert could tell you. You need to find and analyze the right sources.
- Strategy: depth over breadth. Find authoritative sources, analyze
  carefully.
- Example: "How does Leptos 0.8 handle client-side routing?"

**Complex** (unknown unknowns): there's no single right answer. You
need to probe from multiple angles and see what patterns emerge.
- Strategy: breadth over depth. Diverse sources, look for convergence.
- Example: "What's the best approach for documentation that stays
  in sync with code?"

**Clear** (known knowns): this doesn't need research. The answer is
a fact. Look it up directly.
- Example: "What's the Flesch-Kincaid formula?"

If the question is clear, answer it directly. If complicated, go
deep on fewer sources. If complex, go broad and synthesize.

## Phase 2 — Define search strategy (PRISMA-inspired)

Before gathering sources, define:

1. **Search terms** — what to search for, in what sources
2. **Inclusion criteria** — what makes a source relevant (date range,
   source type, topic scope)
3. **Exclusion criteria** — what to skip (outdated, off-topic,
   unreliable)
4. **Synthesis approach** — how findings will be combined

Pre-commitment prevents cherry-picking. Write these down before
the first search.

## Phase 3 — Gather sources

Search systematically. For each source found:

### Evaluate credibility (CRAAP test)

- **Currency** — when published? Is timeliness relevant?
- **Relevance** — does it address the specific question?
- **Authority** — who wrote it? Credentials? Publisher reputation?
- **Accuracy** — claims verifiable? Evidence provided? Peer-reviewed?
- **Purpose** — inform, persuade, sell? Bias disclosed?

Don't evaluate sources in isolation. **Lateral reading**: check what
*other* sources say about this source's claims. A polished website
with no external citations is less credible than a rough blog post
that other experts reference.

### Triangulate

No significant claim should rest on a single source. Require
corroboration from at least two independent sources using different
methods or perspectives.

The evidence hierarchy:
1. Primary sources (original data, official docs, code)
2. Expert analysis (practitioner writing, peer-reviewed research)
3. Secondary reporting (journalism, summaries, tutorials)
4. Opinion (blog posts, social media, forum answers)

### Track what you exclude

Maintaining a search log is as important as maintaining findings.
Record: sources found → sources screened → sources excluded (with
reason) → sources included. This makes the research process
auditable.

## Phase 4 — Process sources (Zettelkasten)

For each included source, produce:

**Literature note**: what the source says, in your own words. Not
quotes — your interpretation. Include your reaction: what's
surprising, what confirms prior knowledge, what contradicts it.

**Atomic claims**: decompose the source into individual claims or
findings. One claim per note. Each claim should be self-contained
and understandable without context.

**Links**: connect each claim to related claims from other sources.
Every link must include **link context** — *why* these claims are
related, not just that they are.

### Progressive depth

Not every source needs deep processing. Apply Forte's progressive
summarization:

- Layer 1: capture the raw material
- Layer 2: on first pass, mark what's relevant to the question
- Layer 3: on synthesis pass, extract the load-bearing claims
- Layer 4: write a one-line summary of what this source contributes
- Layer 5: incorporate into the deliverable

Most sources stay at layer 1-2. Only the most relevant reach layer
4-5. Process to the depth needed, not to maximum depth.

## Phase 5 — Synthesize (Grounded Theory + ACH)

### Bottom-up synthesis (grounded theory coding)

1. **Open coding**: label every atomic claim from Phase 4. What is
   this about? What category does it belong to? Don't use
   predetermined categories — let them emerge.

2. **Axial coding**: find relationships between codes. Group them
   into themes. What conditions, contexts, and consequences
   surround each theme?

3. **Selective coding**: identify the core narrative. What central
   finding ties everything together? Relate all themes to it.

### Competing hypotheses (ACH)

When multiple interpretations emerge:

1. List all plausible hypotheses
2. Build a consistency matrix: evidence as rows, hypotheses as
   columns. Mark each cell: consistent (C), inconsistent (I),
   neutral (N).
3. Focus on **diagnostic evidence** — evidence that distinguishes
   between hypotheses. Evidence consistent with all hypotheses
   tells you nothing.
4. The surviving hypothesis is the one **least burdened by
   inconsistent evidence**, not the one with the most confirming
   evidence.

This directly counters confirmation bias. Don't seek to confirm
your preferred interpretation — seek to disconfirm all of them.

### Saturation check

Stop gathering new sources when new material stops generating new
codes or categories. This is theoretical saturation — additional
sources confirm existing themes but don't create new ones.

## Phase 6 — Self-audit (Feynman Technique)

Before delivering findings, explain the core conclusions in plain
language. No jargon, no hand-waving.

Wherever the explanation becomes vague or relies on technical terms
as a crutch, that's a gap in understanding — not a gap in
vocabulary. Go back and research that specific area deeper.

The inability to explain something simply is a comprehension failure,
not a communication failure.

## Phase 7 — Deliver

Structure the output:

1. **The question** — what was investigated
2. **The answer** — core finding, stated plainly
3. **The evidence** — key sources and what each contributed
4. **The gaps** — what's still uncertain, what would resolve it
5. **The search log** — what was searched, found, excluded, included

The deliverable should make the research process transparent and
auditable. A reader should be able to trace any claim back to its
source and understand why that source was included.

---
name: writing-review
description: >
  Orchestrates writing evaluation by classifying content and dispatching
  the appropriate framework skills. Classifies as tutorial, reference,
  explanation, guide, persuasive, or essay, then runs sentence-level
  checks (Orwell/Williams), docs evaluation (DQTI/Diátaxis) if
  applicable, readability metrics, voice consistency, and fresh-eyes
  review. Use when reviewing any user-facing text, when the user invokes
  /writing-review, or before shipping docs. Not for code or design review.
user-invocable: true
---

# Writing Review

Orchestrate writing evaluation. Classify the content, pick which
framework skills apply, run them, synthesize findings.

## Step 1 — Classify

Read the content. Determine its type:

| Type | Signals | Run these skills |
|---|---|---|
| **Tutorial** | Step-by-step, imperative, code blocks | writing-sentence-level + writing-docs-eval |
| **Reference** | Tables, exhaustive listings, factual | writing-sentence-level + writing-docs-eval |
| **Explanation** | Why/how, conceptual, narrative | writing-sentence-level + writing-docs-eval |
| **How-to guide** | Task-oriented, assumes competence | writing-sentence-level + writing-docs-eval |
| **Landing/marketing** | Persuasive, conversion goal | writing-sentence-level (AIDA/PAS mode) |
| **README** | First impression, mixed | writing-sentence-level + writing-docs-eval |
| **Essay/opinion** | Argument, voice, perspective | writing-sentence-level |

## Step 2 — Run framework skills

Dispatch sub-agents to run each applicable skill independently.
They don't see each other's findings.

## Step 3 — Fresh eyes

Spawn a sub-agent with NO prior context. It reads cold and reports:

1. After the first paragraph, do you want to keep reading?
2. What's the single main point? One sentence.
3. Where did you get confused or re-read?
4. What's missing that you expected?
5. Quote the weakest sentence and why.
6. Quote the strongest sentence and why.

## Step 4 — Synthesize and fix

Merge findings from all evaluators. Deduplicate. For each finding:
- Which framework flagged it
- Severity (blocks shipping / degrades quality / cosmetic)
- The specific text
- A rewrite

Fix everything. Recheck affected sections. Loop until clean.

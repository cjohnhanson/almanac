---
name: doc-coauthoring
description: >
  Structured co-authoring workflow for substantial writing: design docs,
  RFCs, ADRs, specs, proposals, long-form prose. Four stages: context
  gathering, section-by-section drafting, reader testing, fact-check.
  Use when the user wants to write something substantial collaboratively.
  Not for reference documentation (use doc-editing) or quick edits.
user-invocable: true
allowed-tools: Read, Edit, Write, WebSearch, WebFetch, Agent
---

# Doc Co-Authoring

Collaborative writing workflow. The user directs; the agent drafts.
Nothing gets written without the user shaping it first.

## Before writing

If there's an existing draft, read it. Understand what's already been
written before starting.

## Stage 1: Context

Before writing anything, establish:

1. **What is this?** Design doc, RFC, spec, proposal, something else.
2. **Who reads it?** Audience and what they already know.
3. **What should it do?** The effect on the reader.
4. **What's out of scope?**

Then the user dumps everything they have — notes, links, half-formed
thoughts. Don't organize it yet.

After the dump, ask 5-10 clarifying questions based on gaps. The user
can answer in shorthand ("1: yes, 2: no, 3: see the linked source").
Keep going until the shape is visible — not every detail, but enough
to know what sections exist and which one is hardest.

**Exit condition:** Questions show understanding of the subject, not just
the metadata. Edge cases and trade-offs can be discussed without needing
basics explained.

## Stage 2: Drafting

Section by section. Start with the hardest — the one with the most unknowns
or the most at stake. Introduction comes last.

For each section:

1. **Ask** what belongs here. Short clarifying questions.
2. **Brainstorm** 5-15 possible points, framings, or angles. Include things
   from the context dump or research notes the user may have forgotten.
3. **User curates.** They pick, cut, combine. ("Keep 1,4,7. Drop 3. Combine
   5 and 9.") Freeform feedback works too — parse it and proceed.
4. **Draft** the section. Follow the writing-style rules in `CLAUDE.md` and load the `anti-slop` skill.
5. **Refine** via surgical edits (`str_replace`). Don't rewrite surrounding
   sections. Don't reprint the whole document.

After the first section draft, tell the user: instead of editing the file
directly, describe what to change. ("Remove the X paragraph — covered
earlier." "Make the third graf more concrete.") This teaches style
preferences for later sections.

Repeat for all sections.

After 80% of sections are done, re-read the full document and flag:
- Flow problems between sections
- Redundancy
- Filler or slop — sentences not doing work

Write the introduction last.

## Stage 3: Reader testing

Spawn a sub-agent with only the document text and no conversation context.

The sub-agent should:

1. Answer 5-10 questions a reader would realistically ask
2. Flag anything ambiguous, unclear, or assumed
3. Identify internal contradictions

If the sub-agent gets things wrong or surfaces gaps, loop back to Stage 2
for those sections.

## Stage 4: Fresh-eyes fact-check (multiple passes)

Every claim in the document must be verified against the actual system.
The drafting context is biased toward believing what it just wrote; a
fresh sub-agent with only the doc and the codebase will catch claims
that are plausible but wrong.

Dispatch one sub-agent per claim category. Each pass:

1. Reads the document.
2. Reads the code, configs, or external sources needed to verify the
   claims in that category.
3. Reports every claim that is wrong, stale, or unverifiable.
4. Does not attempt to fix. Only reports.

Minimum pass set:

- **Commands.** Every shown CLI command, Makefile target, or shell
  snippet exists and produces the described output.
- **Config and settings.** Every env var, setting, and file path
  referenced matches the project's actual config files and the
  filesystem.
- **Behavior.** Every claim about how the system behaves matches the
  code path.
- **Versions and URLs.** Every dependency version, package name, and
  external URL resolves and matches.

If a pass surfaces any issue, fix the doc and re-run that pass.
Iterate until all passes come back clean.

Only after every pass is clean, the document is ready for the user's
final read.

## Rules

- Never present a complete draft without going through sections individually
- If the user says "just write it," push back once — propose the outline,
  flag the hardest section, offer to start there. If they insist, comply
  but flag that a review pass should follow.
- Follow the writing-style rules in `CLAUDE.md` and the `anti-slop` skill.
- Don't editorialize about the writing unless asked.
- When uncertain about direction, ask. Don't guess.

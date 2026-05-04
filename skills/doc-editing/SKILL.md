---
name: doc-editing
description: >
  Guides collaborative writing of project documentation. Enforces a
  section-by-section drafting process where the user directs and curates
  rather than receiving a finished document. Applies Diátaxis type
  discipline, project file conventions, and source-verified technical
  claims. Triggered when asked to write, draft, or create docs, guides,
  tutorials, references, or explanations. Not triggered for inline code
  comments, commit messages, PR descriptions, or README updates.
user-invocable: true
---

# Documentation Writing

## The core problem this skill solves

Without guidance, documentation gets written in one shot and presented as
finished. The user becomes a reviewer instead of a co-author. This skill
enforces a collaborative process where the user shapes the document at
every stage.

## Process: three phases, no shortcuts

### Phase 1 — Context (do this before writing anything)

Determine these four things. Ask the user if any are unclear:

1. **Doc type** — exactly one of: tutorial, guide, reference, explanation
2. **Audience** — who reads this and what do they already know?
3. **Purpose** — what should the reader be able to do or understand after?
4. **Scope** — what's explicitly out of bounds?

Don't start drafting until type, audience, and purpose are settled. If
the user's request implies a type (e.g., "write a reference for the CLI"),
confirm it rather than assuming.

### Phase 2 — Structure and drafting

1. Propose a section outline. Wait for approval or changes.
2. Start with the hardest section — the one most likely to require
   discussion. Don't start with the introduction.
3. For each section:
   a. Brainstorm 2-3 approaches or framings. Present them briefly.
   b. User picks, combines, or redirects.
   c. Draft that section only.
   d. Refine via surgical edits. Don't rewrite surrounding sections.
4. Write the introduction last, once the shape of the document is clear.

**The user selects and directs. The agent drafts.** Never present a
complete document without having gone through sections individually.

If the user says "just write it" or similar, push back once: propose the
outline, flag the hardest section, and offer to start there. If they
insist on a single-pass draft, comply — but flag that a review pass
should follow.

### Phase 3 — Review

After all sections are drafted:

1. Re-read the full document for flow, redundancy, and slop.
2. Check type discipline (see below).
3. For reference docs, run the verification step (see below).
4. If the document is substantial (>500 words), offer to spawn a
   sub-agent to read it cold and flag anything confusing. The sub-agent
   shouldn't have seen the drafting conversation — the point is to catch
   curse-of-knowledge blind spots.

## File conventions

Documentation lives wherever the project's convention puts it
(per-tool `docs/` directories, a top-level `docs/`, or whatever
the repo already does — match what's there).

Every doc file requires YAML frontmatter:

```yaml
---
title: "The document title"
description: "One-sentence summary for indexes and search"
type: tutorial | guide | reference | explanation
---
```

File naming: lowercase, hyphens, descriptive. Match the title loosely.
Example: `cli-configuration-reference.md` for a doc titled
"CLI Configuration Reference."

Internal links use relative paths between docs.

## Type discipline

One document, one Diátaxis type. This is the rule that gets broken most
often, and it matters because mixed-type docs serve no audience well.

The failure modes to watch for:

- **Reference material leaking into tutorials.** A tutorial that includes
  a project structure tree diagram, a full config field listing, or an
  exhaustive API surface table. If it's describing *what exists* rather
  than *what to do next*, it's reference. Extract it.

- **Reference material leaking into explanations.** An explanation that
  includes a 30-line lifecycle phase table or a complete flag listing.
  The explanation should discuss *why* things work the way they do and
  link to the reference for the specifics.

- **How-to steps leaking into reference.** A reference doc that starts
  walking the reader through a workflow. That's a guide. Split it.

When type mixing is detected during drafting or review, flag it and
propose a split. The extracted material becomes its own doc with its own
frontmatter.

## Reference doc verification

**Every technical claim in a reference doc must be verified against actual
source code, CLI `--help` output, or test fixtures.**

This means:

- Before documenting a config field, grep for it in the codebase. Confirm
  the field name, type, default value, and behavior.
- Before documenting a CLI flag, run `<tool> --help` or read the argument
  parser source. Confirm the flag exists, its syntax, and its effect.
- Before documenting an API surface, read the actual type definitions.
  Don't rely on memory or inference.

If something can't be verified (e.g., the code is in a private dependency
or the behavior is only observable at runtime), mark it explicitly:
`<!-- unverified: [reason] -->`. Don't silently guess.

The default assumption should be that any technical detail written
from memory is wrong until confirmed against source.

## When this skill applies

This skill is relevant when the task involves writing, drafting, or
substantially revising a documentation file. It applies to:

- New docs from scratch
- Major rewrites of existing docs
- Splitting a mixed-type doc into proper single-type docs

It does not apply to:

- Quick edits to fix a typo or update a version number
- Inline code comments or docstrings
- README files (unless they're being treated as proper docs)
- PR descriptions, commit messages, or issue write-ups

## Checklist (for self-verification before presenting work)

- [ ] Frontmatter includes title, description, and type
- [ ] File is in the project's documentation directory, named with lowercase hyphens
- [ ] Document serves exactly one Diátaxis type
- [ ] No section belongs to a different type without being flagged
- [ ] Every technical claim in reference docs is verified against source
- [ ] No corporate blog openings or forced structure
- [ ] Introduction was written last (or at minimum, revised last)
- [ ] User directed the structure and curated section content

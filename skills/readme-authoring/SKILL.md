---
name: readme-authoring
description: >
  Write a README that someone landing cold can use in 30 seconds. Lead
  with what it is and why someone would use it; show usage with a real
  example; cross-link to related work; cut clever openings. Use when
  writing or rewriting a project README.
user-invocable: true
---

# README Authoring

A README has one job: a stranger lands on the GitHub page, reads the
top half, and either knows whether this tool helps them or moves on.
Every line either earns that judgment or wastes it.

## The first three lines

The first paragraph answers two questions: **what is it** and **why
would I use it**. Plain language, no jokes, no quotes, no dictionary
definitions, no decorative emoji in the title. If a stranger can't tell
what the project does after reading the first paragraph, the README has
already failed.

Bad: `> 🎶 A tisket, a tasket 🎶 ...` (cute, but not informative)
Bad: `> An almanac (historically spelled almanack) is a regularly...`
(dictionary definition cold-open)
Good: `Plaintext, git-tracked issue tracking. Issues are markdown files
with YAML frontmatter, stored in your repo.`

## Required sections

Every README in this style needs:

1. **Title + one-paragraph description** — what + why
2. **Install** — one command if possible
3. **Usage** — at least one runnable example, not just CLI help text
4. **License** — one line

That's the floor. Most READMEs need only those four.

Optional sections, in this order if used:

- **How it works** — one paragraph or example, only if non-obvious from usage
- **Configuration** — only if there's something to configure
- **Documentation** — links to deeper docs, if any exist
- **Related** — sibling projects in the same ecosystem
- **Threat model** / **Caveats** — if there's something the user must know

## Voice

Tight, plain, technical. Match the project's actual character without
performing it. Avoid:

- Self-referential cleverness in titles or openings
- Banned anti-slop phrases (see `anti-slop` skill)
- Marketing voice ("powerful", "seamless", "robust")
- Apologetic disclaimers in the first 3 paragraphs
- Feature lists before establishing problem/value

For libraries: lead with a code example after the description. For
CLIs: lead with the simplest invocation. The reader should see the
shape of the tool within the first screen.

## Cross-linking

If sibling projects exist, link to them in a `## Related` section near
the bottom. Each link gets one short clause explaining why the reader
might also want it. Don't link to projects that aren't actually
related — boilerplate "see also" sections are noise.

## Tone consistency across an ecosystem

When multiple READMEs serve the same audience, they should sound like
one author wrote them. Same section names, same voice level, same
emoji discipline (none, or one decorative emoji per title at most).
Inconsistency reads as carelessness.

## Process

When writing or rewriting:

1. Read the project's source enough to write the description from
   ground truth, not the existing README.
2. Draft the description first. If it's longer than two sentences,
   cut.
3. Write the install command. Verify it works.
4. Write a usage example. Verify it runs.
5. Add only the optional sections that earn their place.
6. Read the result aloud. Cut anything that sounds like it's trying to
   be clever.

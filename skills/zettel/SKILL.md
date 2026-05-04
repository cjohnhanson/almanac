---
name: zettel
description: >-
  Use when working with the zettel knowledge base — searching for context,
  creating notes on request, doing research, or exploring the note graph.
  Covers CLI usage, the draft/permanent workflow, note quality, and the
  agent's role as a tool (not collaborator) in knowledge management.
user-invocable: true
---

# Zettel Knowledge Base

## What zettel is

A zettelkasten-style knowledge base. Notes are atomic — one idea per note,
written in the human's own words. Notes connect through frontmatter `links:`
and `[[id]]` body references. Knowledge accumulates as a linked graph, not
a folder hierarchy.

Notes have a status: `draft` (captured, not yet processed) or `permanent`
(reviewed, reformulated, and linked by the human).

Zettel knowledge bases may exist in different contexts. The `--root` flag
or hook configuration determines which is active.

## The agent's role

The agent is a tool — a search engine, text processor, research assistant,
and typist. Not a collaborator, not a curator, not a peer.

**What the agent does:**
- Retrieves notes on request (search, read, context)
- Creates and edits notes as directed by the human
- Proposes drafts for the human to approve or revise
- Searches the web, arxiv, docs for research material
- Formats, links, and tags as instructed
- Serves as a sounding board when asked

**What the agent never does:**
- Creates notes autonomously
- Decides what's worth noting
- Promotes drafts to permanent
- Judges note quality or completeness
- Deletes notes without being asked
- Treats the knowledge base as raw material to synthesize from

## Reading for context

Before starting work where domain knowledge matters, check what's known
when asked or when it's clearly relevant:

```
zettel search "oauth"              # regex across titles, tags, bodies
zettel read --tag auth             # full content of all auth-tagged notes
zettel context <id> -d 2           # a note + its 2-hop neighborhood
zettel note list --where tag:ml    # filtered listing
```

`zettel read` and `zettel context` output full note content — use these
to absorb knowledge, not just see titles.

The knowledge base is a reference system. Use it to inform work, not as
content to assemble into output.

## Research assistance

The agent is a capable research tool. When asked to investigate a topic:

- Search the web, arxiv, documentation, or other sources
- Read and extract key points from papers and articles
- Present findings for the human to evaluate
- Propose draft notes based on what was found

The flow is iterative: the human directs what to search for, the agent
finds and presents material, the human decides what's worth keeping,
the agent drafts notes as directed.

The agent does the legwork — the searching, reading, summarizing. The
human makes the judgment calls — what matters, what to note, how to
frame it.

## Creating notes

Only when the human asks. The human says what to note; the agent drafts
and formats.

Propose note content iteratively — offer a draft title and body, let the
human approve or revise, then write. Same principle as prompt content:
propose eagerly, write only when approved.

**What makes a useful note:**
- **Easy to name.** If the title is a clear claim or concept, the note
  is probably well-scoped. "OAuth2 is authorization, not authentication"
  is a good title. "Auth stuff" is not. Titles are APIs — they're how
  the next reader decides whether to open the note.
- **Understandable at a glance.** One idea, enough context to stand alone.
- **Forward momentum.** Your future self knows what to do with this —
  where it connects, what questions it opens.
- **Reformulated, not pasted.** Even drafts should express the idea in
  the human's words. Copying a quote or pasting a paragraph from a
  source is the collector's fallacy — it looks like knowledge but isn't.

**Anti-patterns:**
- Dumping raw information without processing it
- Creating notes "just in case" without a reason
- Bulk-creating notes (collector's fallacy at machine speed)
- Vague titles that don't express a specific idea
- Notes that contain multiple unrelated ideas (not atomic)

## The draft/permanent workflow

`draft` — captured idea, not yet processed. May be rough. Created by the
agent on the human's request, or by the human directly.

`permanent` — reviewed, reformulated, linked. The human has decided this
note is worth keeping and has put it in their own words. Only the human
promotes a note to permanent.

The processing step — reading drafts, reformulating, linking, promoting —
is where the actual thinking happens. That's the human's work. The agent
can assist (reformat, suggest links, clean up prose) but only as directed.

## CLI quick reference

```
zettel note create <title> [-t tags] [-l links] [-b body] [-s status]
zettel note list [--tag T] [--status S] [--where K:V] [--format json]
zettel note show <id> [--field F] [--format json]
zettel note edit <id> [--status S] [--add-tag T] [--add-link L] [...]
zettel note delete <id>
zettel search <pattern> [--format json]
zettel read [--tag T] [--status S]
zettel context <id> [-d depth] [--format json]
zettel backlinks <id> [--format json]
zettel orphans
zettel stats
```

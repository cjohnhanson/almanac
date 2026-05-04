---
name: anti-slop
description: >
  Pre-generation constraints for avoiding AI-written prose patterns.
  Applies to all writing: docs, READMEs, commit messages, PR descriptions,
  comments, essays. Not a review tool — these rules shape output before
  it's written.
---

# Anti-Slop

Rules for writing that sounds written, not generated. Apply these
before drafting, not as a cleanup pass.

## Banned phrases

Delete on sight. No replacements — the sentence should work without them,
or it shouldn't exist.

- "delve into", "dive into", "deep dive"
- "navigate the complexities"
- "it's important to note that", "it's worth noting that"
- "in today's [anything]"
- "at its core"
- "a testament to"
- "the power of"
- "whether you're [X] or [Y]"
- "this is where [X] comes in"
- "not just [X] — it's [Y]"
- "takes [X] to the next level"
- "unlocks", "unleashes", "empowers", "enables seamless"
- "robust", "cutting-edge", "world-class", "best-in-class"
- "straightforward", "effortless", "seamless"

## Phrase replacements

| Kill | Use |
|------|-----|
| leverage | use |
| utilize | use |
| facilitate | help, let |
| implement (vague) | build, write, add |
| in order to | to |
| due to the fact that | because |
| has the ability to | can |
| a wide range of | many, several |
| on a regular basis | regularly |
| in the context of | in, for, when |
| prior to | before |
| subsequent to | after |
| functionality | feature, what it does |
| methodology | method, approach |

## Structural patterns to avoid

Don't give these cute names in your own writing either.

**Twin setup.** "Not just X — it's Y." "More than X, it's Y."
Ad copy structure. Say what it is. Once.

**Narrating the explanation.** "Let's explore...", "Let's break
this down...", "Let's walk through...". Don't describe the act
of explaining. Explain.

**Gratuitous lists.** If three items share a sentence's worth of
context, write a sentence. Lists are for five-plus items or items
that need individual attention.

**Uniform sentence length.** Every sentence the same length, same
structure, subject-verb-object, no variation. Read it aloud. If it
sounds like a metronome, rewrite it.

**Empty intensifiers.** "Very", "really", "incredibly", "extremely"
before an adjective. Cut the intensifier. If the adjective isn't
strong enough alone, pick a better adjective.

**Meta-introductions.** "This section covers...", "In this document,
we'll...", "The following describes...". Just start.

**"We" voice in docs.** "We believe", "We provide", "Our approach".
Describe what the thing does, not what "we" do.

**Em dash overuse.** One em dash in a paragraph is punctuation. Two
starts looking like a tic. Three is a pattern. The fix is usually
to end the sentence and start a new one.

**Aphoristic "X = Y" labels.** When naming the class of a bug, a
catch, or a pattern, describe it in concrete words. Do not produce
lines like "<thing> = <metaphor for the thing>" or "doing A is
really doing B". Those framings perform cleverness; they sound
like aphorism but communicate less than the plain description.
Kill on sight, including in the middle of a longer sentence where
the equation sneaks in as a parenthetical.

## What good technical prose sounds like

Vary sentence length. Short sentences punch. Longer ones develop an
idea, qualify it, connect it to something else. Monotone length is
the single most reliable tell of generated text.

Name the thing. Not "the system" — which system. Not "the tool" —
which tool. Not "various components" — name them.

Active voice by default. Passive voice when the actor genuinely
doesn't matter or when the object is the point.

No preamble. First sentence does work. If the first sentence could
be deleted without losing anything, delete it.

Trust the reader. Don't explain what they're about to read. Don't
summarize what they just read. Don't tell them why it matters —
show them what it does and let them decide.

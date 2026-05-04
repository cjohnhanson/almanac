---
name: writing-sentence-level
description: >
  Sentence-level writing evaluation using Orwell's 6 rules, Williams'
  clarity diagnostics, readability metrics, and persuasion frameworks
  (AIDA/PAS). Evaluates individual sentences and paragraphs for clarity,
  concision, voice, and structure. Use as part of writing-review or
  independently when checking prose quality. Not for document-level
  structure (use writing-docs-eval).
---

# Sentence-Level Writing Evaluation

Evaluate prose quality sentence by sentence, paragraph by paragraph.
Three layers: Orwell for concision and word choice, Williams for
clarity and structure, readability metrics for audience fit.

## Layer 1 — Orwell's 6 Rules

From "Politics and the English Language" (1946). These are revision
heuristics — apply them to every sentence, flag violations, suggest
rewrites.

### Rule 1: No dead metaphors

A dead metaphor is a figure of speech so overused that the original
image has been lost. The writer uses it as a ready-made phrase rather
than choosing words for their meaning.

Examples of dead metaphors to flag:
- "stand shoulder to shoulder"
- "play into the hands of"
- "no axe to grind"
- "fishing for compliments"
- "tip of the iceberg"
- "level playing field"
- "move the needle"
- "deep dive" (in non-literal contexts)
- "low-hanging fruit"
- "circle back"

The test: if the image the metaphor describes has nothing to do with
the point being made, it's dead. Replace with plain language.

**Not a violation:** original, vivid metaphors chosen for their
specific image. Orwell's rule targets lazy reuse, not metaphor itself.

### Rule 2: Short words over long ones

If a short word conveys the same meaning, use it.

| Long | Short |
|---|---|
| utilize | use |
| facilitate | help |
| implement | do, build |
| approximately | about |
| functionality | feature |
| methodology | method |
| in order to | to |
| leverage (verb) | use |
| prior to | before |
| subsequent to | after |
| commence | start |
| terminate | end |
| endeavor | try |
| regarding | about |
| demonstrate | show |

The test: read the long word aloud. Would a person actually say it in
conversation? If not, the short word is better.

### Rule 3: Cut every word that can be cut

If a sentence works without a word, the word shouldn't be there.

Common cuttable phrases:

| Wordy | Cut to |
|---|---|
| it is important to note that | (delete entirely) |
| the fact that | (delete or restructure) |
| in terms of | (restructure) |
| at this point in time | now |
| due to the fact that | because |
| in the event that | if |
| for the purpose of | to, for |
| a large number of | many |
| is able to | can |
| has the ability to | can |
| make use of | use |
| on a daily basis | daily |
| at the present time | now |
| in close proximity to | near |
| it should be noted that | (delete) |

The test: read the sentence with the suspect word or phrase removed.
Does the meaning change? If not, cut it.

### Rule 4: Active voice over passive

**Active:** The agent writes tests.
**Passive:** Tests are written by the agent.

Active voice is shorter, clearer, and names who does what. Passive
hides the actor and adds words.

How to spot passive: look for forms of "to be" (is, was, were, been,
being) followed by a past participle (usually -ed or -en). "The file
**was created**" is passive. "The command **creates** the file" is
active.

**When passive is acceptable:**
- The actor is genuinely unknown: "The server was compromised."
- The actor is less important than the action: "Requests are routed
  to the nearest server."
- Orwell's Rule 6 applies — active sounds worse.

### Rule 5: Everyday words over jargon

If there's an everyday English equivalent, use it. This doesn't mean
avoiding all technical terms — it means not using technical terms when
plain ones work.

| Jargon | Everyday |
|---|---|
| performant | fast |
| scalable | handles growth |
| leverage | use |
| paradigm | model, approach |
| synergy | working together |
| optimize | improve |
| deliverable | result, output |
| stakeholder | (name the actual group) |
| operationalize | put into practice |
| ideate | think, brainstorm |

**Exception:** domain-specific terms that have no simpler equivalent.
"Worktree" in git, "assertion" in testing, "frontmatter" in markdown —
these are precise terms that plain language can't replace without
losing meaning. Use them, but define them at first use.

### Rule 6: Break any rule to avoid sounding barbarous

This is the escape valve. If following rules 1-5 produces an awkward,
unnatural, or ugly sentence, break the rule. The purpose of the rules
is better writing, not mechanical compliance.

The test: read the revised sentence aloud. If it sounds worse than the
original, keep the original.

## Layer 2 — Williams' Clarity Diagnostics

From "Style: Lessons in Clarity and Grace" by Joseph Williams. The
central insight: unclear writing buries characters in prepositional
phrases and turns actions into nouns.

### The characters-as-subjects diagnostic

For each sentence, check: do the first 7-8 words name the main
character (who or what the sentence is about) doing the main action?

**Clear:** "The guard **rejects** edits to source files."
(Character = guard, action = rejects, both in the first 7 words)

**Unclear:** "The rejection of source file edits is performed by
the guard system when the phase is restrictive."
(Character buried in "by the guard system," action turned into a
noun "rejection," real verb is the meaningless "is performed")

### The nominalization test

Nominalizations are verbs or adjectives turned into nouns, usually
ending in -tion, -ment, -ness, -ity, -ence, -ance.

| Nominalization | Verb it hides |
|---|---|
| investigation | investigate |
| implementation | implement |
| establishment | establish |
| development | develop |
| utilization | use |
| completion | complete |
| determination | determine |
| improvement | improve |
| assessment | assess |
| configuration | configure |

When a nominalization appears as the subject of a sentence, the real
actor is usually missing and the real action is hidden.

**Fix:** find the hidden verb, find the hidden actor, make the actor
the subject and the verb the main verb.

"The **implementation** of the feature **was completed** by the team."
→ "The team **implemented** the feature."

### The old-new principle

Sentences should begin with information the reader already knows
(old) and end with information that's new. This creates cohesion —
each sentence connects to the previous one.

**Good flow:**
"Missouri runs tests in sandboxed temp directories. **These
directories** are created fresh for each test path and deleted
afterward."

(Second sentence starts with "these directories" — old info from
the first sentence — and ends with new info about lifecycle.)

**Bad flow:**
"Fresh creation and deletion after each test path characterizes the
sandboxed temp directories. Missouri runs tests in them."

(Second sentence starts with new info and ends with old info. The
reader has to hold the new info in memory while waiting for context.)

### Undefined definite references

A definite noun phrase ("the guard," "the registry," "the scrubber")
presupposes the reader already knows what it refers to. When writing,
the referent may be in the LLM's context window but absent from the
document. The reader encounters "the guard" and has to guess what
guard.

The test: for every "the [noun]" that isn't a common English word
(the server, the file), check whether that noun was introduced earlier
in the document. If not, either introduce it first or use an
indefinite form ("a guard that...") at first mention.

This is distinct from the old-new principle. Old-new is about sentence
ordering within a paragraph. This is about document-level
presupposition: does the reader have the knowledge this reference
assumes?

Common failure pattern: the writer has been discussing a concept in
conversation or in other files, uses "the X" as if the reader was
present for that context, and the reader wasn't.

### Paragraph structure

Each paragraph should have:
1. A topic sentence stating the paragraph's point
2. Supporting sentences that develop the point
3. A consistent subject — don't change topics mid-paragraph

The topic sentence is usually first. If the reader can't state the
paragraph's point after reading just the first sentence, the topic
sentence is missing or buried.

## Layer 3 — Readability Metrics

Quantitative measures of text difficulty. Use as signals, not verdicts.

### Flesch-Kincaid Grade Level

Measures the US school grade level needed to understand the text.
Based on average sentence length and average syllables per word.

| Grade | Audience | Examples |
|---|---|---|
| 6-8 | General public | Consumer products, marketing |
| 8-10 | Informed public | Journalism, popular nonfiction |
| 10-12 | Technical audience | Developer docs, engineering |
| 12-14 | Specialist | Academic papers, legal |
| 14+ | Expert | Dense technical/theoretical |

**Target for developer documentation:** grade 8-12. Lower is fine if
accuracy isn't sacrificed. Higher needs justification.

### Sentence length

Average sentence length is the strongest single predictor of
readability. Target: 15-20 words average. Flag any sentence over 35
words — it almost certainly needs splitting.

Count sentences in each paragraph and compute the average. Report
paragraphs that exceed the target.

### Paragraph length

Long paragraphs (6+ sentences) intimidate readers and bury
information. Short paragraphs (1-2 sentences) create emphasis but
fragment ideas if overused.

Flag paragraphs over 6 sentences. Suggest splitting at the natural
topic break.

## Layer 4 — Persuasion Frameworks (when applicable)

Apply only when the content has a conversion or persuasion goal
(landing pages, marketing copy, READMEs trying to convince someone
to use the tool).

### AIDA (Attention → Interest → Desire → Action)

1. **Attention** — does the opening break through noise? Is there a
   hook that makes the reader stop scrolling?
2. **Interest** — does the body sustain engagement with relevant
   information, data, or story?
3. **Desire** — does it transform interest into want? Benefits, not
   features. What changes for the reader?
4. **Action** — is there a clear, specific call to action? Is the
   friction to act low?

Failure mode: strong hook but no CTA. Or CTA with no desire-building.

### PAS (Problem → Agitate → Solution)

1. **Problem** — does it name the reader's pain specifically? Does it
   show understanding of their situation?
2. **Agitate** — does it make the problem feel urgent? What happens if
   nothing changes?
3. **Solution** — does it present the offering as the resolution?

PAS works better than AIDA when the audience already knows they have
a problem. It's more emotionally charged.

Failure mode: jumping straight to solution without establishing the
problem. Or naming a problem the reader doesn't recognize.

## Applying the layers

For each piece of content:

1. Run Layer 1 (Orwell) on every sentence. Flag violations.
2. Run Layer 2 (Williams) on every paragraph. Flag buried characters
   and nominalized actions.
3. Run Layer 3 (readability) on the whole piece. Report metrics.
4. Run Layer 4 (persuasion) only if the content has a conversion goal.
5. For each violation, provide the specific text and a rewrite.
6. Severity: blocks shipping / degrades quality / cosmetic.

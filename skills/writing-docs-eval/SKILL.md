---
name: writing-docs-eval
description: >
  Documentation-specific evaluation using IBM DQTI nine characteristics,
  Diátaxis per-type quality criteria, and Baker's Every Page is Page One
  principles. Evaluates whether docs are easy to use, understand, and
  find, whether they serve their type well, and whether topics stand
  alone. Use as part of writing-review or independently when evaluating
  technical documentation quality. Not for sentence-level prose (use
  writing-sentence-level).
---

# Documentation Evaluation

Evaluate documentation at the document and system level. Three
frameworks: IBM DQTI for overall quality dimensions, Diátaxis for
type-specific quality, Baker's EPPO for topic independence.

## Framework 1 — IBM DQTI Nine Characteristics

From "Developing Quality Technical Information" (IBM Press, Michelle
Carey et al.). Nine named quality dimensions organized into three
user needs. The closest thing documentation has to Nielsen's 10
usability heuristics.

Each characteristic is evaluated as a question. "Yes" means the doc
meets the standard. "No" is a finding.

### Easy to Use

#### Task orientation

Is the documentation written from the user's perspective?

Good task orientation:
- Presents information in the order users need it, not the order
  the system is built
- Explains *why* before *how* — gives the practical reason for
  each step
- Organizes by user tasks, not by product features
- Each section answers "how do I do X?" rather than "what is X?"

Violation examples:
- A tutorial organized by module names instead of user goals
- Reference docs that list internal function names the user never
  calls
- Steps that say "configure the FrobnicatorService" without saying
  why the user would want to

#### Accuracy

Do the documented steps produce the documented results?

Check:
- Do code samples compile/run when copy-pasted?
- Do commands produce the output shown?
- Do screenshots match the current product state?
- Are version numbers, defaults, and config values current?
- Are deprecated features marked as deprecated?
- Is there a "last updated" date?

This is the most concrete dimension — it can be verified by
literally running the documented steps. A single inaccurate code
sample destroys trust in the entire document.

#### Completeness

Does the doc include everything the user needs — and nothing they
don't?

Check:
- Are prerequisites listed?
- Are error cases documented? (Not just the happy path)
- Are limitations and known issues disclosed?
- Are all parameters/flags/options documented with types, defaults,
  and descriptions?
- Is there a "what's NOT covered" section? (Prevents false
  expectations)

Completeness does NOT mean exhaustiveness. Including irrelevant
information violates completeness just as much as omitting necessary
information. The user's task determines what's "complete."

### Easy to Understand

#### Clarity

Can the user understand the content on first read?

Check:
- Are terms used consistently? (Don't call it "server" then
  "machine" then "instance")
- Are new terms defined at first use?
- Are pronouns unambiguous? (If "it" or "this" could refer to
  multiple things, use the noun)
- Is the meaning of each sentence clear without re-reading?

Clarity failures are often caused by:
- Nominalized verbs ("the implementation" instead of "implement")
- Passive voice hiding the actor
- Long sentences with multiple nested clauses
- Undefined acronyms or jargon

#### Concreteness

Does the doc use specific examples rather than abstract descriptions?

Check:
- Are there code examples for each concept?
- Are examples realistic (real data, plausible scenarios)?
- Are abstract claims illustrated with concrete instances?
- Can the reader copy an example and modify it for their case?

**Bad (abstract):** "Configure the tool with appropriate settings."
**Good (concrete):** "Set `timeout` to `30` in `config.yml`."

#### Style

Is the writing readable and appropriately toned?

Check:
- Active voice used predominantly?
- Sentences short enough to parse in one read?
- Tone appropriate for audience? (Not condescending for experts,
  not assuming for beginners)
- Consistent voice throughout? (Not switching between chatty and
  formal)

### Easy to Find

#### Organization

Does the structure match how users look for information?

Check:
- Is the most important information first? (Inverted pyramid)
- Do headings describe the content below them?
- Is the hierarchy logical? (Do sections at the same level cover
  comparable topics?)
- Are transitions between sections smooth?
- Can a reader jump into any section without reading everything
  before it?

#### Retrievability

Can a user find specific information without reading the whole doc?

Check:
- Is there a table of contents?
- Are headings descriptive enough to scan? ("Configuration" is too
  vague; "Configure timeout settings" is scannable)
- Can the user search for a term and find the relevant section?
- Are cross-references provided to related topics?
- Is the doc broken into sections small enough to bookmark?

#### Visual effectiveness

Does the layout support comprehension?

Check:
- Is code in code blocks with syntax highlighting?
- Are tables used for structured data (not buried in prose)?
- Is whitespace used to separate logical sections?
- Are headings visually distinct from body text?
- Do diagrams or illustrations appear where the concept is spatial
  or complex?
- Is the page readable on mobile?

## Framework 2 — Diátaxis Per-Type Quality

First classify the document as one of four types. Then evaluate
against the quality criteria for that type. A doc that serves two
types should be split.

### Tutorial evaluation

Tutorials are learning-oriented. The reader acquires a skill by
doing, under guidance.

| Criterion | What it means |
|---|---|
| Meaningful goal | The reader achieves something they care about, not a toy exercise |
| Visible results | Every step produces output the reader can verify |
| Perfect reliability | Every step works every time on a clean setup |
| Concrete, not abstract | No theory, no alternatives, no digressions |
| Inclusive language | "We" — the writer and reader are doing this together |
| Rapid progress | The reader sees progress within the first few minutes |
| No choices | The tutorial makes decisions for the reader — one path |

Violation examples:
- A tutorial that presents three ways to do each step
- A tutorial that works on the author's machine but not a clean setup
- A tutorial that explains architecture before showing a working result
- A tutorial that ends with "now you can explore the options" without
  having built anything concrete

### How-to guide evaluation

Guides are task-oriented. The reader has a specific goal and wants
to achieve it.

| Criterion | What it means |
|---|---|
| Goal-oriented title | "How to configure authentication" not "Authentication" |
| Assumes competence | Doesn't re-explain basics the reader knows |
| Addresses real complexity | Handles edge cases, not just the clean path |
| Conditional structure | "If you want X, do Y" — adapts to reader's situation |
| Complete solution | The reader can finish their task using only this guide |

Violation examples:
- A guide that explains what authentication is before showing how
  to configure it
- A guide titled "Authentication" instead of "How to configure
  authentication"
- A guide that only covers the simple case and says "see the docs
  for advanced usage"

### Reference evaluation

Reference is information-oriented. The reader looks up a specific
detail.

| Criterion | What it means |
|---|---|
| Austere, factual | No opinions, no narrative, no instruction |
| Mirrors product structure | Organized the same way the product is organized |
| Consistent format | Every entry follows the same template |
| Complete | Every parameter, flag, option, error code documented |
| Includes examples | Brief usage examples, not tutorials |

Violation examples:
- Reference that starts explaining "when you might want to use this"
- Reference with some entries fully documented and others bare
- Reference organized alphabetically when the product has a logical
  grouping

### Explanation evaluation

Explanations are understanding-oriented. The reader wants to know
*why*.

| Criterion | What it means |
|---|---|
| Reflective, not instructional | Discusses reasons, history, tradeoffs |
| Broader perspective | Steps back from the specific to the general |
| Multiple perspectives | Acknowledges alternatives and tradeoffs |
| Background context | Design decisions, constraints, history |
| Bounded | Doesn't absorb tutorial, guide, or reference content |

Violation examples:
- An explanation that turns into a step-by-step tutorial mid-way
- An explanation that lists every CLI flag (that's reference)
- An explanation that says "this is the best approach" without
  acknowledging tradeoffs

## Framework 3 — Baker's Every Page is Page One

From Mark Baker. Seven principles for evaluating whether topics
work independently — critical for web-based docs where readers
arrive from search, not from page 1.

### Self-contained

The topic functions alone. It doesn't depend on the reader having
read previous topics. No "as mentioned earlier" or "see the
previous section" — if context is needed, either include it or
link to it.

Test: can a reader land on this page from a Google search and
understand it without clicking backward?

### Specific purpose

One topic, one job. If the topic tries to serve two purposes
(explain AND instruct, or reference AND guide), it should be split.

Test: can you describe what this page does in one sentence without
using "and"?

### Context establishment

The topic orients readers who arrive from anywhere. The first
paragraph tells them what this page covers, who it's for, and what
they need to know before proceeding.

Test: does the first paragraph answer "am I in the right place?"

### Rich linking

Navigation through subject affinity, not hierarchy. Related topics
are linked inline where the concepts appear, not just in a sidebar.

Test: can the reader navigate to any related topic from within the
content, without using the nav?

### Reader qualification

The topic addresses qualified readers while helping unqualified
ones recognize gaps. It doesn't try to teach prerequisites — it
tells the reader what they need to know and links to where they
can learn it.

Test: if a reader lacks a prerequisite, do they find out in the
first few sentences rather than failing mid-page?

## Applying the frameworks

For each document:

1. Classify as tutorial, guide, reference, or explanation (Diátaxis).
2. Evaluate against the nine DQTI characteristics. Flag violations.
3. Evaluate against the type-specific Diátaxis criteria. Flag
   violations.
4. If the doc is part of a web-based doc set, evaluate against
   Baker's EPPO principles.
5. For each violation, specify the characteristic, the specific text,
   and a suggested fix.
6. Severity: blocks shipping / degrades quality / cosmetic.

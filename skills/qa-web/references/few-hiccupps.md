# FEW HICCUPPS Consistency Oracles

Quick-reference for applying James Bach's consistency oracles during
exploratory QA. Each oracle names a source of expectations. A bug is
detected when observed behavior is inconsistent with that source.

---

## Familiar

**Consistency with patterns the tester has seen before.**

A violation occurs when something works differently from the way similar
things typically work in software. This isn't about any specific product —
it's about general software literacy. If a tester with broad experience
says "that's weird," Familiar is the oracle being invoked.

*Diagnostic question*: Does this behave the way experienced users of
software in general would expect?

**Example violations**:
- A modal dialog closes when clicking inside its body, not just the X or
  overlay — most modals don't do this.
- Pressing Enter in a form field triggers navigation instead of submission.
- Right-clicking a link doesn't offer "Open in new tab" because the
  element isn't actually an anchor tag.

**Real finding vs. false positive**: Familiar is the weakest oracle when
used alone. "This feels weird to me" may reflect limited exposure rather
than a real problem. Strengthen it by pairing with another oracle
(Comparable Products, Standards). If the behavior can be explained by a
deliberate, documented design choice, it's likely a false positive.

---

## Explainable

**Consistency with the ability to articulate a coherent reason for
the behavior.**

A violation occurs when observed behavior can't be explained by any
reasonable model of how the software works. If something happens and
neither the tester nor a developer could construct a plausible reason,
the behavior is suspicious.

*Diagnostic question*: Can a reasonable explanation be constructed for
why the software behaves this way?

**Example violations**:
- A search returns different result counts on consecutive identical
  queries with no data changes.
- A button is disabled on first page load but enabled after a refresh
  with no state change.
- A form saves successfully but the saved data differs from what was
  entered.

**Real finding vs. false positive**: Unexplainable behavior is strong
evidence of a bug — but first, check for async operations, caching, or
race conditions that might provide an explanation. If the behavior is
reproducible and no explanation emerges, escalate.

---

## World

**Consistency with how things work in the real world.**

A violation occurs when the software models real-world concepts
incorrectly. Dates, geography, money, time zones, names, physical
constraints — the software's representation should match reality.

*Diagnostic question*: Does this match how things actually work outside
the software?

**Example violations**:
- A date picker allows selecting February 30th.
- A shipping calculator shows delivery to an island nation via ground
  freight.
- A currency field truncates to two decimal places for Japanese yen
  (which has no decimal subdivision).

**Real finding vs. false positive**: World violations are almost always
real findings. The main false positive is when the software intentionally
simplifies a real-world concept and the simplification is documented.
Check whether the domain model is deliberately constrained before filing.

---

## History

**Consistency with past behavior of the same product.**

A violation occurs when the current version behaves differently from a
previous version without a documented, intentional reason. Regressions
live here.

*Diagnostic question*: Did this used to work differently, and was the
change intentional?

**Example violations**:
- A keyboard shortcut that worked in the previous release now does
  nothing.
- Pagination previously showed 25 items per page, now shows 10, with no
  setting change or release note.
- A previously instant action now shows a loading spinner for 3+ seconds.

**Real finding vs. false positive**: History is strong when change is
confirmed and unintentional. False positives come from comparing against
stale memory — verify with actual prior behavior (screenshots, release
notes, version history) rather than recollection.

---

## Image

**Consistency with the organization's desired public identity.**

A violation occurs when the product's quality, polish, or behavior
contradicts the image the organization wants to project. A company
selling enterprise security software shouldn't have a login page with
broken CSS.

*Diagnostic question*: Would this embarrass the team or undermine trust
if a customer or journalist saw it?

**Example violations**:
- A banking app shows a stock photo placeholder on its transaction page.
- A design tool's own marketing site has misaligned grid elements.
- An error message exposes a stack trace or internal service name to
  end users.

**Real finding vs. false positive**: Image is subjective but useful. The
test is whether someone outside the team would notice and draw a negative
conclusion. Internal tools with low external visibility get more slack.
Customer-facing products get none.

---

## Comparable Products

**Consistency with similar products in the same category.**

A violation occurs when the product handles something differently from
how established competitors handle it, without a clear reason for the
divergence.

*Diagnostic question*: How do the top 2-3 products in this space handle
this same interaction?

**Example violations**:
- A text editor doesn't support Cmd/Ctrl+Z for undo when every
  comparable editor does.
- A project management tool requires saving a form manually when
  competitors auto-save.
- A checkout flow asks for shipping info after payment when every
  major e-commerce site does the opposite.

**Real finding vs. false positive**: Comparable Products is strong when
the convention is near-universal. It's weak when there's genuine
variation across the category. If comparable products disagree with each
other, this oracle doesn't help.

---

## Claims

**Consistency with what the product explicitly says about itself.**

A violation occurs when the product's behavior contradicts its own
documentation, tooltips, marketing copy, error messages, or help text.
The product made a promise and broke it.

*Diagnostic question*: Does the behavior match what the product's own
text says should happen?

**Example violations**:
- A tooltip says "Click to download CSV" but clicking triggers a PDF
  download.
- The docs say the API supports pagination but the endpoint ignores
  page parameters.
- An error message says "Try again in a few minutes" but the error
  persists for hours.

**Real finding vs. false positive**: Claims violations are nearly always
real findings — either the behavior or the claim needs to change. The
only false positive is stale documentation that hasn't caught up with
an intentional change, which is itself a finding (just a docs bug).

---

## User Desires

**Consistency with what users actually want to accomplish.**

A violation occurs when the software technically works but fails to
serve the user's actual goal. The feature functions correctly in
isolation but doesn't solve the problem the user brought to it.

*Diagnostic question*: If a real user did this to accomplish their goal,
would they succeed or be frustrated?

**Example violations**:
- An export function produces a CSV that can't be opened in Excel due to
  encoding issues — the feature "works" but defeats its purpose.
- A search returns results sorted by creation date when users clearly
  want relevance ranking.
- A password reset flow requires the user to remember their current
  password.

**Real finding vs. false positive**: User Desires requires knowing (or
having reasonable assumptions about) what users want. Without user
research or stated requirements, this oracle relies on empathy and
inference. Pair it with Claims or Purpose for stronger evidence.

---

## Product

**Consistency of the product with itself.**

A violation occurs when one part of the product contradicts another
part. Internal inconsistency — different pages handle the same concept
differently, or the same action produces different results in different
contexts.

*Diagnostic question*: Does this part of the product agree with other
parts of the same product?

**Example violations**:
- Date format is MM/DD/YYYY on one page and DD/MM/YYYY on another.
- Deleting an item requires confirmation in one list view but not
  another.
- The sidebar navigation shows a feature that the main menu doesn't,
  or vice versa.

**Real finding vs. false positive**: Product consistency violations are
almost always real findings. The only exception is when different
sections intentionally serve different audiences with different
conventions, and that's explicitly designed. Otherwise, inconsistency
within a product is a bug.

---

## Purpose

**Consistency with the product's reason for existing.**

A violation occurs when a feature or behavior undermines the core
purpose of the product. Not a matter of broken functionality — a matter
of the product working against its own mission.

*Diagnostic question*: Does this help or hinder the fundamental reason
someone would use this product?

**Example violations**:
- A note-taking app reformats pasted text in ways that lose the
  original structure.
- A privacy-focused browser leaks referrer headers by default.
- A collaboration tool doesn't notify team members when shared content
  changes.

**Real finding vs. false positive**: Purpose violations are among the
most serious. False positives only arise from misunderstanding the
product's actual purpose — check the product's positioning and stated
mission before invoking this oracle.

---

## Statutes and Standards

**Consistency with laws, regulations, and external standards.**

A violation occurs when the product fails to meet applicable legal
requirements (statutes, regulations, acts) or formal voluntary
standards — WCAG, RFC specs, platform conventions, or industry norms
with normative force. Statutes carry legal obligation; standards carry
professional or contractual obligation. Both are external authorities
the product must satisfy.

*Diagnostic question*: Does this meet the applicable formal standards
for this type of product?

**Example violations**:
- Form inputs lack associated labels (WCAG 1.3.1).
- A cookie consent banner doesn't actually block tracking until consent
  is given (GDPR).
- A responsive layout breaks below 320px viewport width (WCAG 1.4.10
  reflow).

**Real finding vs. false positive**: Statutes and standards violations
are objective and verifiable. The main question is whether the statute
or standard actually applies — not every product must meet every
standard, and not every regulation applies to every jurisdiction or
domain. Check which statutes and standards are in scope before citing
them.

---

## Prioritizing across oracles

**Editorial note:** The ranking below is practical guidance for applying
these oracles during QA work. It is not part of Bach and Bolton's
framework. They explicitly reject fixed oracle ranking — oracles are
"fallible and context-dependent; to be applied, not followed." Any
oracle can be the most important one in the right situation. This
ranking reflects editorial judgment about which oracles most often
produce actionable findings in typical web product work.

When a finding triggers multiple oracles, that's signal about severity.

**Strongest evidence of a real bug** (one of these alone is usually
sufficient):
- Claims — the product contradicts its own words
- Product — the product contradicts itself
- Statutes and Standards — a formal or legal requirement is unmet
- World — reality is modeled incorrectly

**Strong evidence, best paired with another oracle**:
- Purpose — the product undermines its own mission
- User Desires — real users would fail at their goal
- Explainable — no one can explain the behavior
- History — a confirmed, unintentional regression

**Supporting evidence, not sufficient alone**:
- Image — looks bad but might be acceptable for the context
- Comparable Products — others do it differently, but there's no
  single right way
- Familiar — feels wrong, but that's subjective

A finding that violates Claims + Product + User Desires is almost
certainly a real bug. A finding that only violates Familiar might just
be an unconventional design choice. When writing up findings, name which
oracles are violated — it makes the case without editorializing.

# Visual Design Principles Reference

Phase 3 evaluation oracles. For each principle: what it describes, how to
check it, a concrete violation, and the fix.

---

## Gestalt Perceptual Principles

These describe how the human visual system organizes elements into groups
and structures before conscious thought kicks in. Violations feel "off"
even when users can't articulate why.

### Proximity

Elements placed near each other are perceived as belonging together.
Spatial distance is the strongest grouping cue — it overrides color,
shape, and size.

**How to evaluate:** Measure the gap between related elements versus the
gap between unrelated elements. Related items should be closer to each
other than to anything else. Check form labels against their inputs,
section headers against their content, and action buttons against the
content they act on.

**Violation:** A form where the label "Email" sits equidistant between
the password input above and the email input below. The user has to
read labels carefully because spatial grouping doesn't disambiguate.

**Fix:** Reduce the gap between each label and its input to roughly
one-third of the gap between field groups. The label-to-input distance
should be noticeably smaller than the input-to-next-label distance.

### Similarity

Elements that share visual properties (color, size, shape, typography)
are perceived as related or equivalent in function.

**How to evaluate:** Identify all interactive element types (primary
actions, secondary actions, links, navigation items). Check that
elements of the same type look the same, and elements of different
types look different. Watch for cases where visual similarity implies
a relationship that doesn't exist.

**Violation:** A dashboard where "Delete Account" is styled identically
to "Save Changes" — same size, same button shape, same font weight.
The only difference is color (red vs. blue), but the shared shape
and size signal equivalent importance and frequency of use.

**Fix:** Differentiate destructive actions structurally, not just by
color. Make "Delete Account" a text button or outline button while
"Save Changes" remains a filled primary button. Shape and prominence
should reflect how often and how casually the action should be taken.

### Figure-Ground

The visual system separates the scene into a foreground object of
attention (figure) and everything else (ground). Ambiguity about
which is which causes disorientation.

**How to evaluate:** Identify the primary content on each screen.
Check that it reads as the clear foreground against navigation,
chrome, and decorative elements. Modals and overlays should have
unambiguous depth separation. Watch for competing visual layers
where neither clearly sits in front.

**Violation:** A modal dialog opens over a page, but the backdrop
overlay is nearly transparent and the modal has no shadow or border.
The form fields in the modal visually merge with the form fields
on the page behind it. The user can't tell which inputs are active.

**Fix:** Apply a dimmed backdrop (opacity 0.5+) behind the modal.
Add elevation cues — a drop shadow or solid border — to the modal
container. The background page should feel recessed and inert.

### Closure

The visual system perceives incomplete shapes as complete, filling in
missing information to see a whole. The brain closes gaps in contours,
borders, and patterns automatically.

**How to evaluate:** Check that incomplete visual elements are
intentionally incomplete and that users correctly perceive the intended
whole. Look at icon designs that rely on implied shapes, progress
indicators that show partial completion, carousels that hint at
off-screen content by showing partial items, and loading states that
use skeleton shapes. Verify that the implied completion matches what
the user should expect.

**Violation:** A horizontal carousel shows exactly three cards with
no visual hint that more exist — no partial fourth card peeking in
from the edge, no fade, no indicator dots. Users perceive the set as
complete and never scroll.

**Fix:** Show a sliver of the fourth card at the viewport edge so
the row looks intentionally incomplete. The partial element triggers
closure — users perceive a longer row and understand they can scroll
to see the rest.

### Continuity

The eye follows lines, curves, and directional flow, preferring smooth
continuous paths over abrupt changes. Elements arranged along a line or
curve are perceived as related.

**How to evaluate:** Trace the visual flow of navigation sequences,
step indicators, timelines, and reading order. Check that alignment
guides the eye along the intended path. Look for places where the
flow breaks unexpectedly — elements that interrupt a visual line, step
indicators that change direction, or content sections that break the
reading axis without reason.

**Violation:** A five-step progress indicator wraps to a second line
after step 3, with no visual connector between step 3 and step 4.
Users perceive two separate processes instead of one continuous flow.

**Fix:** Either keep all steps on one line (adjust sizing or use a
compact representation) or add a visible connector — a line or arrow —
that bridges the wrap point so the eye follows the flow across the
line break.

### Common Fate

Elements that move together — or change together — are perceived as
a group. Shared motion is a powerful grouping cue that overrides
static properties like color or proximity.

**How to evaluate:** Watch animations, transitions, and dynamic state
changes. Check that elements which belong together animate in unison
(same direction, same timing). Look at loading skeletons to verify
that related placeholder regions pulse or shimmer together. In
multi-select interfaces, check that selected items respond
identically to group actions. Watch for unrelated elements that
accidentally share an animation and get perceived as grouped.

**Violation:** A loading skeleton for a card grid has each card's
shimmer animation on a slightly different phase. Some cards appear
to pulse together while others are out of sync, creating a false
perception that certain cards are related and others aren't.

**Fix:** Synchronize the shimmer animation across all skeleton cards
so they pulse in unison, reinforcing that they're equivalent
placeholders for equivalent content.

### Common Region

Elements enclosed within a shared boundary (box, background color,
card) are perceived as a group, even if proximity alone wouldn't
group them.

**How to evaluate:** Check that card boundaries, section backgrounds,
and bordered regions contain exactly the items that belong together
logically. Watch for cards that contain unrelated items, or related
items split across different regions. Verify that nested regions
have clear visual hierarchy (outer vs. inner).

**Violation:** A settings page where "Notification Preferences" and
"Billing Information" sit inside the same card with no visual
separator. Users assume these are related and expect changes to
one section to affect the other.

**Fix:** Split into separate cards, each with its own heading. If
they must share a container for layout reasons, add a visible
divider and distinct sub-headers so the boundary between groups
is unambiguous.

---

## Norman's Interaction Design Principles

These describe the qualities that make interactive elements
understandable and usable. Where Gestalt is about seeing, Norman
is about doing.

### Affordances

An affordance is a relationship between an object's properties and
a person's capabilities that determines how the object could be used.
A flat surface affords pushing. A handle affords pulling. In a UI,
affordances communicate what actions are possible.

**How to evaluate:** For each interactive element, ask: does its
visual form suggest the correct interaction? Buttons should look
pushable. Text fields should look typeable. Sliders should look
draggable. Check that non-interactive elements don't accidentally
suggest interactivity (underlined text that isn't a link, card
shadows that suggest clickability on static content).

**Violation:** A pricing page displays plan options as elevated cards
with hover effects, but the cards themselves aren't clickable — the
user has to find a small "Select" link at the bottom of each card.
The card's visual properties (shadow, hover animation) afford
clicking, but clicking the card body does nothing.

**Fix:** Either make the entire card a click target (matching the
affordance), or remove the hover effect and elevation so the card
reads as a static container with an explicit action button.

### Signifiers

Signifiers indicate where and how to act. Affordances are about what's
possible; signifiers are about what's communicated. A door handle is
an affordance. A "Pull" sign is a signifier.

**How to evaluate:** Tab through the interface with a keyboard. Every
interactive element should be visually distinguishable from static
content without hovering. Check for: underlines or color on links,
cursor changes on hover, visible focus indicators, icon conventions
(chevrons for expandable items, "x" for close). Ask whether a new
user could identify every clickable element from a static screenshot.

**Violation:** A navigation menu uses plain text for both section
headers (not clickable) and sub-page links (clickable). Same font,
same color, same weight. The only way to discover which items are
links is to hover over each one and watch the cursor.

**Fix:** Style links with a distinct color and underline (or underline
on hover at minimum). Give section headers a different weight or size
that signals "label, not link." Interactive elements must be visually
coded as interactive without requiring interaction to discover.

### Mapping

Mapping is the relationship between controls and their effects. Good
mapping means the spatial or conceptual arrangement of controls
corresponds naturally to the arrangement of outcomes.

**How to evaluate:** Check that controls are positioned near what they
affect. Verify that directional controls match the direction of the
effect (a "move up" button should be above a "move down" button).
For forms, check that the submit button is near the form content,
not detached at the bottom of the page. For settings, check that
toggles are adjacent to the feature they control.

**Violation:** A list reordering interface has "Move Up" and "Move Down"
buttons in a toolbar at the top of the page, far from the selected
list item. The buttons are arranged horizontally (left for up, right
for down) even though the list is vertical.

**Fix:** Place movement controls inline with each list item. Orient
them vertically (up arrow above, down arrow below) to match the
direction of the list. Or better: support drag-and-drop, which is
a direct mapping — you move the item by literally moving it.

### Feedback

Every action should produce an immediate, perceptible response. Users
need confirmation that the system received their input and is acting
on it. Silence breeds uncertainty.

**How to evaluate:** Perform every interactive action and check for
a response: button clicks should produce visual change (press state,
loading indicator, result). Form submissions should confirm success
or report failure. Long-running operations need progress indicators.
Check timing — feedback within 100ms feels instant, within 1 second
feels responsive, beyond 1 second needs a loading state.

**Violation:** A user clicks "Save" on a profile form. The button
does nothing visible — no loading spinner, no disable state, no
success message. The data saves in the background, but the user
clicks "Save" three more times because nothing confirmed the first
click worked.

**Fix:** On click: disable the button and show a spinner or "Saving..."
text. On success: show a brief confirmation ("Changes saved") via
toast or inline message. On failure: re-enable the button and display
the error. The user should never wonder whether their action registered.

### Constraints

Constraints limit the actions available to the user, preventing errors
by making incorrect actions impossible rather than merely discouraged.
Norman identifies four types: physical constraints (a USB plug only
fits one way), cultural constraints (red means danger, a handshake
means greeting), semantic constraints (the meaning of the situation
restricts what makes sense — a rearview mirror only makes sense facing
backward), and logical constraints (what's left must be right — if
all other options are filled, the remaining one goes in the remaining
slot). Physical constraints are the strongest; logical constraints
work by elimination.

**How to evaluate:** For each invalid state, check what happens when
the user tries to reach it. Can they submit an empty required field?
Can they navigate to step 3 before completing step 1? Can they delete
an item that other items depend on? Prefer disabling or hiding invalid
options over showing error messages after the fact.

**Violation:** A multi-step wizard allows the user to click any step
tab at any time. Jumping to step 4 without completing steps 1-3
results in a blank form with no context. The user has to figure out
they need to go back and complete earlier steps.

**Fix:** Disable future step tabs until their prerequisites are
complete. Show completed steps as clickable (for review) and the
current step as active, but render future steps as visually inert
with no hover or click behavior. The interface should make the
correct sequence the only available path.

### Conceptual Models

A conceptual model is the user's understanding of how a system works.
When the model the interface suggests matches the actual system
behavior, the interface feels intuitive. When they diverge, users
make confident predictions that turn out wrong — and blame themselves.

**How to evaluate:** For each major feature, ask: what does a user
think is happening when they perform this action? Compare that mental
model to the actual system behavior. Check for metaphors that break
down (a "folder" that can't contain other folders, a "trash" that
auto-empties without warning). Look for features where users
consistently mispredict outcomes — that's a conceptual model mismatch.

**Violation:** A cloud document editor shows a "Save" button, implying
that changes aren't saved until clicked. But the system auto-saves
continuously, and the "Save" button actually triggers "save a named
version." Users click it frantically during edits, thinking their
work is at risk, and end up with dozens of meaningless named versions.

**Fix:** Remove the "Save" button and replace it with a persistent
"All changes saved" indicator. Offer "Save as version" (or similar)
as a separate, clearly labeled action. The interface should communicate
the auto-save model rather than borrowing the manual-save metaphor.

# Nielsen's 10 Usability Heuristics — Evaluation Reference

This reference covers each heuristic in enough depth to conduct a
structured evaluation. For each: what it means, what to check, what
violations look like, and how to rate severity.

Severity ratings use the standard 0-4 scale throughout:
- **0** — Not a usability problem
- **1** — Cosmetic only; fix if time permits
- **2** — Minor usability problem; low priority
- **3** — Major usability problem; important to fix, should be given high priority
- **4** — Usability catastrophe; imperative to fix before product can be released

### Severity assessment methodology

Nielsen's severity ratings are determined by three factors:

- **Frequency** — How common is the problem? Is it encountered by many
  users on many tasks, or is it rare?
- **Impact** — How difficult is it for users to overcome the problem once
  they encounter it? Can they work around it easily, or does it block them?
- **Persistence** — Is it a one-time problem that users can overcome once
  they know about it, or does it hurt them every time they encounter it?

Beyond these three, market impact also matters: the problem's effect on
the product's competitive position, user satisfaction, and the
organization's credibility. A severity-2 issue in a commodity feature
may matter more than a severity-3 issue in a niche workflow if it
damages trust at scale.

---

## 1. Visibility of System Status

The system should keep users informed about what is going on through
appropriate feedback within reasonable time. Without status visibility,
users cannot form accurate mental models of the system's state, leading
to repeated actions, premature abandonment, or data loss. The feedback
must be timely — a loading indicator that appears two seconds after the
user clicked is too late to prevent a second click.

**What to check:**
- Every user action produces visible feedback within ~100ms
- Long operations (>1s) show progress indicators, not just spinners
- System state is unambiguous: is a form saved or unsaved? Is a process running or complete?
- Network-dependent operations show loading, success, and failure states
- Background processes surface their status somewhere discoverable

**Violation examples:**
- A "Save" button gives no visual confirmation after clicking. The user clicks again, creating a duplicate record. No toast, no disabled state, no checkmark — nothing acknowledges the action happened.
- A file upload starts but shows no progress bar or percentage. The user waits 30 seconds staring at an unchanged screen, then navigates away, killing the upload silently.
- A multi-step checkout process has no step indicator. The user has no idea whether they are on step 2 of 3 or step 2 of 7.

**Severity distinctions:**
- **1**: Feedback exists but is visually subtle (e.g., a toast appears in a corner nobody watches)
- **2**: Feedback is missing for low-stakes actions (e.g., "added to favorites" has no confirmation)
- **3**: Feedback is missing for important operations (e.g., form submission gives no success/failure signal)
- **4**: Absence of feedback leads to data loss or destructive repeated actions (e.g., duplicate payment submission)

---

## 2. Match Between System and Real World

The system should use the user's language, concepts, and mental models
rather than internal system terminology. This goes beyond word choice —
it includes logical ordering (alphabetical vs. workflow-ordered),
cultural conventions (date formats, currency symbols), and metaphors
that map to real-world experience. When the system speaks its own
language, users must translate, and translation errors become usage errors.

**What to check:**
- Labels, menu items, and headings use domain language, not developer/database terminology
- Information is ordered in a way that matches user expectations (chronological, by importance, by workflow step), not by database ID or creation date
- Icons and metaphors correspond to widely understood real-world concepts
- Units, formats, and conventions match the target audience (e.g., mm/dd/yyyy vs. dd/mm/yyyy)
- Error messages describe the problem in user terms, not system terms

**Violation examples:**
- A healthcare app labels a section "Encounter Records" instead of "Visit History." Patients don't call their doctor visits "encounters."
- An e-commerce checkout asks users to enter their "billing entity" instead of "billing name" or "name on card."
- A dashboard shows timestamps in UTC with no timezone label. Users in Chicago interpret 2:00 PM UTC as 2:00 PM local time and miss a deadline.

**Severity distinctions:**
- **1**: Mildly technical label that users can figure out from context (e.g., "Repository" in a developer tool used by non-developers)
- **2**: Jargon that causes momentary confusion but doesn't block task completion (e.g., "SKU" shown to end consumers alongside a product name)
- **3**: Terminology mismatch that causes users to select wrong options or enter wrong data (e.g., "Ship To" vs. "Bill To" confusion due to unclear labels)
- **4**: System language causes users to make irreversible errors (e.g., "Purge" button that users interpret as "clear my view" but actually deletes records permanently)

---

## 3. User Control and Freedom

Users frequently perform actions by mistake or change their minds.
The system must provide clearly marked exits — undo, cancel, back,
close — so users never feel trapped. Control also means users can
navigate freely without being forced through a rigid sequence when
the task doesn't require it. A system that traps users erodes trust
and increases error anxiety.

**What to check:**
- Every dialog, modal, and overlay has a visible close/cancel mechanism
- Destructive actions are reversible (undo) or require confirmation
- Multi-step flows allow backward navigation without losing entered data
- Users can abandon a process at any point and return to a known safe state
- Browser back button behaves predictably (doesn't break state, doesn't skip steps)
- Forced sequences exist only when genuinely required by the domain (e.g., payment before shipping)

**Violation examples:**
- A modal overlay has no close button and no backdrop-click-to-dismiss. The only way out is completing the form or refreshing the page, losing all other in-progress work.
- A multi-page form wizard doesn't let users go back to step 2 from step 4. Catching a typo in an earlier field requires restarting the entire flow.
- A bulk email tool has "Send" with no confirmation and no undo. A misclick sends 10,000 emails.

**Severity distinctions:**
- **1**: Exit exists but is hard to find (e.g., close button is an unlabeled X in low contrast)
- **2**: A non-critical action cannot be undone (e.g., removing an item from a wishlist is permanent)
- **3**: Users get trapped in a flow and must restart to correct mistakes (e.g., no back button in a multi-step form)
- **4**: No undo or confirmation for destructive actions that affect other users or external systems (e.g., publishing, deleting, sending)

---

## 4. Consistency and Standards

Users should not have to wonder whether different words, actions, or
visual treatments mean the same thing. Consistency operates at two
levels: internal (within the product) and external (with platform
conventions). Internal inconsistency forces users to relearn the
interface with every screen. External inconsistency violates learned
expectations from other software.

**What to check:**
- Same action, same label everywhere (don't mix "Delete," "Remove," "Trash," "Clear" for the same operation)
- Same visual treatment for same function (all primary actions look the same, all destructive actions look the same)
- Layout patterns are consistent across pages (navigation in the same position, page titles in the same spot)
- Interactive element behavior is consistent (do all dropdowns work the same way? All modals?)
- Platform conventions are respected (e.g., links are underlined or blue, checkboxes are square, radio buttons are round)
- Keyboard shortcuts follow platform norms (Cmd+S saves, Escape closes modals)

**Violation examples:**
- A SaaS app uses a red button for "Delete" on one page and a red button for "Submit" (the primary action) on another. Users hesitate on every red button, unsure if it's destructive.
- The primary navigation is a left sidebar on the dashboard, a top bar on the settings page, and a hamburger menu on the reports page. Three pages, three navigation paradigms.
- Date pickers across the app use three different components: a calendar popup, a text input with validation, and a dropdown with month/day/year selectors.

**Severity distinctions:**
- **1**: Minor visual inconsistency with no functional impact (e.g., slightly different border radius on buttons in different sections)
- **2**: Inconsistent labeling that causes momentary confusion (e.g., "Settings" in the nav, "Preferences" in the footer, both going to the same page)
- **3**: Behavioral inconsistency where the same gesture does different things in different contexts (e.g., swiping left archives in one list and deletes in another)
- **4**: Inconsistency that causes users to take destructive action based on expectations set elsewhere in the app

---

## 5. Error Prevention

Even better than good error messages is a design that prevents errors
from occurring in the first place. Error prevention operates at two
levels: eliminating error-prone conditions (slips) and checking for
them before the user commits to an action (mistakes). Slips are
unconscious errors (typos, misclicks); mistakes are conscious errors
based on wrong mental models.

**What to check:**
- Destructive actions require a distinct gesture (not adjacent to common actions, use confirmation dialogs or undo)
- Input fields constrain to valid values where possible (date pickers instead of free text, dropdowns instead of typed codes)
- Real-time validation catches errors before form submission
- The system prevents invalid states (e.g., can't set end date before start date)
- Dangerous zones are visually separated from routine actions
- Defaults are safe (opt-in to risky behavior, not opt-out)

**Violation examples:**
- A "Delete Account" button sits directly below "Save Profile" with identical styling. The only difference is the label. No confirmation dialog follows the delete.
- A date range filter accepts an end date that precedes the start date, then shows zero results with no explanation. The user thinks there's no data rather than catching the date inversion.
- A quantity field in a shopping cart accepts negative numbers. Entering -1 and checking out causes a credit instead of a charge, or an application error.

**Severity distinctions:**
- **1**: Missing constraint on low-risk input (e.g., a phone field accepts letters but server-side validation catches it harmlessly)
- **2**: Error-prone layout that occasionally causes wrong-target clicks on non-destructive actions
- **3**: No validation on important input that leads to failed submissions or wasted effort
- **4**: No safeguard against destructive or irreversible actions (e.g., deletion without confirmation, publishing without review)

---

## 6. Recognition Rather Than Recall

The system should minimize memory load by making objects, actions,
and options visible. Users should not need to remember information
from one screen to apply it on another. Recognition (seeing and
choosing) is cognitively cheaper than recall (retrieving from memory).
Every piece of information the user must hold in working memory is a
potential error source.

**What to check:**
- Available actions are visible, not hidden behind unmarked menus or memorized shortcuts
- Form fields show expected format (placeholder, helper text, example)
- Related information needed for a decision is visible on the same screen (don't make users memorize data from a previous page)
- Recently used or frequently accessed items are surfaced
- Navigation is persistent and visible, not hidden until hovered or scrolled
- Instructions for complex tasks are visible at point of use, not in a separate help page

**Violation examples:**
- A report builder requires users to type exact field names from memory into a text box. There's no autocomplete, no browsable list of available fields, and the field names don't match the column headers users see in the data table.
- An admin panel requires remembering a 6-character permission code (e.g., "RW-USR") to assign roles. There's no dropdown, no tooltip explaining codes, and no legend on the page.
- A settings page has 40 options organized into unlabeled tabs. After changing a setting, users must remember which tab it was on to find it again — there's no search and no recent-changes view.

**Severity distinctions:**
- **1**: Information is available but requires an extra click to reveal (e.g., full file path shown on hover, not inline)
- **2**: Users must remember information between steps in a short flow (e.g., a confirmation page doesn't repeat the details being confirmed)
- **3**: Core workflow requires memorizing codes, IDs, or field names that the system could present as selectable options
- **4**: Critical decisions must be made from memory with no way to see the relevant data on the current screen

---

## 7. Flexibility and Efficiency of Use

Accelerators — invisible to novice users — can speed up expert
interaction. The system should cater to both inexperienced and
experienced users. This doesn't mean feature bloat; it means
providing shortcuts, customization, and automation for frequent
actions while keeping the basic path simple. A system that only
serves beginners becomes frustrating at scale.

**What to check:**
- Keyboard shortcuts exist for frequent actions
- Power-user features (bulk actions, saved searches, templates) are available without cluttering the default view
- Frequently repeated tasks can be automated or templated
- Personalization is possible (customizable dashboards, saved filters, default views)
- Touch targets and click areas are appropriately sized for the input method
- Tab order through forms is logical

**Violation examples:**
- A project management tool requires 4 clicks to change a task's status: open task → click edit → scroll to status dropdown → select → save. There's no inline status toggle or keyboard shortcut despite users doing this hundreds of times per day.
- An admin interface with tables of data provides no bulk selection. To disable 50 user accounts, an admin must open each one individually, toggle the status, and save. Fifty times.
- A CRM has no keyboard navigation. Every interaction requires precise mouse targeting, making it inaccessible to power users who work primarily via keyboard.

**Severity distinctions:**
- **1**: Missing shortcuts for uncommon actions (the feature exists, just takes a few extra clicks)
- **2**: A moderately frequent task requires unnecessary steps that experts would notice
- **3**: High-frequency tasks have no efficient path, forcing repetitive multi-step sequences throughout the workday
- **4**: The interface is fundamentally hostile to efficient use — no keyboard access, no bulk operations, no way to reduce repetitive work for tasks performed constantly

---

## 8. Aesthetic and Minimalist Design

Every extra unit of information on a screen competes with relevant
information and diminishes its relative visibility. This is not about
visual attractiveness — it's about information design. Dialogues
should not contain information that is irrelevant or rarely needed.
The heuristic targets signal-to-noise ratio, not subjective beauty.

**What to check:**
- Every visible element serves a purpose (either informational, navigational, or interactive)
- Visual hierarchy clearly distinguishes primary content from secondary content from chrome
- Whitespace is used to create breathing room and group related elements
- Decorative elements do not compete with functional elements for attention
- Information density is appropriate for the context (data-heavy dashboards can be dense; onboarding flows should not be)
- Content is scannable — headings, bullet points, and visual breaks exist where walls of text would otherwise form

**Violation examples:**
- A dashboard crams 15 metrics above the fold, all in identical card styles with no visual hierarchy. The three metrics that actually matter get the same visual weight as twelve that nobody checks.
- A marketing site's pricing page has animated backgrounds, floating shapes, and gradient borders that make the actual plan comparison table hard to read. The decoration actively fights the content.
- An error dialog includes a full stack trace, a correlation ID, a timestamp, and a "learn more" link alongside the one sentence the user actually needs: "Your session expired. Please log in again."

**Severity distinctions:**
- **1**: Minor visual clutter that doesn't impede task completion (e.g., a footer with rarely-useful links)
- **2**: Noise that slows scanning but doesn't prevent finding information (e.g., too many badges and labels on list items)
- **3**: Critical information is buried under irrelevant content, requiring effort to extract (e.g., an important warning lost in a dense paragraph)
- **4**: Visual noise actively misleads or prevents users from identifying essential information or controls

---

## 9. Help Users Recognize, Diagnose, and Recover from Errors

Error messages should be expressed in plain language (no codes),
precisely indicate the problem, and constructively suggest a solution.
A good error message answers three questions: what happened, why it
happened, and what to do about it. Users encountering errors are
already frustrated — a bad error message compounds the frustration
and blocks recovery.

**What to check:**
- Error messages are written in plain language, not technical jargon or error codes
- The message identifies what specifically went wrong (not just "An error occurred")
- The message suggests a concrete next step or fix
- Validation errors appear next to the relevant field, not only at the top of the form
- Error states are visually distinct and persistent (not a toast that disappears in 3 seconds while the user is still reading)
- The system preserves user input after errors (form data isn't cleared on failed submission)

**Violation examples:**
- A form submission fails with "Error 422: Unprocessable Entity" displayed in a red banner at the top of the page. No indication of which field is wrong. The user re-checks all 12 fields looking for the problem.
- A login failure shows "Invalid credentials" for both wrong-email and wrong-password cases. The user can't tell whether they mistyped their password or are using the wrong email entirely.
- A payment fails with "Transaction declined. Please try again." No guidance on whether to try a different card, contact their bank, or check the billing address. "Try again" will fail again for the same reason.

**Severity distinctions:**
- **1**: Error message is correct but could be more helpful (e.g., "Invalid input" when "Enter a valid email address" would be better)
- **2**: Error message is vague, requiring users to experiment to find the problem
- **3**: Error message is misleading or provides no recovery path for a common error scenario
- **4**: Error causes data loss (form cleared on submission failure) or the error is invisible (silent failure with no message at all)

---

## 10. Help and Documentation

Even though it's better if the system can be used without
documentation, it may be necessary to provide help. Any such
information should be easy to search, focused on the user's task,
list concrete steps, and not be too large. This heuristic also
covers onboarding, tooltips, contextual help, and empty states —
any place the system teaches the user how to proceed.

**What to check:**
- Help content is searchable
- Help is contextual — accessible from the screen where it's relevant, not only from a global help center
- Instructions are task-oriented (how to do X) rather than feature-oriented (what button Y does)
- Empty states provide guidance (what to do first, how to populate this view)
- Onboarding exists for complex features and can be re-accessed later
- Tooltips and inline help are available for non-obvious controls

**Violation examples:**
- A complex analytics tool has a help center with hundreds of articles organized by product feature. A user trying to build a cohort analysis must know the feature is called "Segmentation" to find the right article. There's no task-oriented entry point like "How do I analyze user groups?"
- A dashboard's empty state shows a blank white area with no message. First-time users stare at nothing with no idea how to populate it. No "Get started" prompt, no sample data, no link to import.
- A data export feature has a "Format" dropdown with options like "CSV," "TSV," "Parquet," and "NDJSON." No tooltips explain what each format is or when to use it. Users unfamiliar with Parquet must leave the app to search.

**Severity distinctions:**
- **1**: Help exists but is slightly hard to find (e.g., buried in a submenu)
- **2**: Help content is present but generic or feature-oriented rather than task-oriented
- **3**: No contextual help for a complex feature that most users will struggle with; the only option is trial and error
- **4**: No help or documentation exists for a critical workflow, and the interface is not self-explanatory enough to compensate

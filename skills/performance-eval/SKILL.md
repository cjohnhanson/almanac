---
name: performance-eval
description: >
  Web performance evaluation using RAIL model, Core Web Vitals (LCP, INP,
  CLS), performance budgets, critical rendering path analysis, Lighthouse
  scoring, and anti-pattern detection. Use when evaluating web application
  performance, setting performance budgets, diagnosing slowness, or when
  the user invokes /performance-eval. Not for backend/database performance.
user-invocable: true
---

# Web Performance Evaluation

Evaluate front-end web performance through structured analysis of user-centric
metrics, rendering behavior, resource loading, and known anti-patterns.

Steve Souders' golden rule applies: **80-90% of end-user response time is
spent on the front end.** The HTML document typically accounts for only
10-20% of total load time. Everything else — stylesheets, scripts, images,
fonts, third-party resources — is where the time goes. This means front-end
optimization yields disproportionate returns. Backend optimization matters,
but if you haven't looked at the front end first, you're optimizing the
wrong thing.

This skill covers the front-end side. For server response time, database
query performance, or API latency, use other tools.

---

## 1 — RAIL Model

RAIL is Google's user-centric performance model. It frames performance
around four types of interaction, each with a budget defined by human
perception thresholds. The acronym stands for Response, Animation, Idle,
Load.

### Response — under 50ms

When a user taps a button, toggles a control, or starts an interaction,
the system must provide visible feedback within 50 milliseconds. This is
the threshold at which humans perceive a system as instantaneous. Beyond
100ms, the connection between action and reaction feels broken.

The 50ms budget is tighter than the 100ms perception threshold because
the browser needs time to process the response — event handlers, style
recalculation, paint. If your JavaScript takes 50ms, the total time
including browser work may push close to 100ms.

Practically: event handlers should complete in under 50ms. If the work
takes longer, defer the heavy computation and show an immediate
acknowledgment (spinner, state change, optimistic UI update).

### Animation — under 10ms per frame

Animations, scrolling, and drag interactions must produce a new frame
every 16ms (60fps). But the browser needs roughly 6ms of overhead for
compositing and painting each frame, leaving approximately 10ms for
JavaScript work per frame.

This is the hardest budget to meet because it's per-frame, not per-event.
A single long task during an animation causes a visible jank — a dropped
frame that the user perceives as stuttering.

Common violations: running layout-triggering JavaScript during scroll
handlers, animating properties that trigger layout (width, height, top,
left) instead of compositor-friendly properties (transform, opacity),
and synchronous style reads interleaved with writes (layout thrashing).

### Idle — under 50ms chunks

Idle time is when the user isn't actively interacting. This is the
window for deferred work: analytics, prefetching, lazy loading,
non-critical initialization.

The constraint: idle work must be broken into chunks of 50ms or less.
The reason is the Response budget — if a user interaction arrives while
an idle task is running, the system must be able to respond within 50ms.
A 200ms idle task means the user could wait up to 200ms for a response
if their interaction lands at the wrong moment.

Use `requestIdleCallback` where available, or break work into small
chunks dispatched via `setTimeout(fn, 0)` or `postMessage` patterns.
Always yield back to the main thread between chunks.

### Load — under 5 seconds

The page should be interactive within 5 seconds on a mid-tier mobile
device over a 3G connection. "Interactive" means the main content is
visible and the page responds to input — not that every resource has
finished loading.

This is the loosest budget and the one most often exceeded. The 5s
target assumes a slow baseline (Moto G4 class device, 400ms RTT,
1.6 Mbps). On fast connections and devices, aim for under 2 seconds.

Key strategies: deliver critical content first, defer non-essential
resources, minimize main-thread work during load, use server-side
rendering or static generation for initial content.

---

## 2 — Core Web Vitals

Core Web Vitals are Google's standardized metrics for real-world user
experience. They measure loading, interactivity, and visual stability.
These are field metrics — they're measured from real user sessions via
the Chrome User Experience Report (CrUX), not just lab tools.

All Core Web Vitals are evaluated at the **75th percentile**. This means
75% of page loads must meet the threshold, not the average or median.
The 75th percentile was chosen because it captures the experience of
users on slower devices and connections without being dominated by
extreme outliers.

### Largest Contentful Paint (LCP)

LCP measures loading performance — specifically, how long until the
largest visible content element finishes rendering. The "largest" element
is typically a hero image, heading block, or video poster.

| Rating            | Threshold  |
|-------------------|------------|
| Good              | ≤ 2.5s    |
| Needs improvement | ≤ 4.0s    |
| Poor              | > 4.0s    |

What counts as the LCP element: `<img>`, `<image>` inside SVG,
`<video>` poster images, elements with `background-image` via CSS,
and block-level text elements (`<h1>`, `<p>`, etc.).

Common LCP killers: slow server response (high TTFB), render-blocking
CSS/JS, large unoptimized images, and client-side rendering. Fix by
preloading the LCP image, using responsive images with `srcset`, serving
modern formats (WebP, AVIF), and ensuring the LCP element is in the
initial HTML rather than injected by JavaScript.

### Interaction to Next Paint (INP)

INP measures interactivity — the latency between a user interaction
(click, tap, keypress) and the next visual update. Unlike its
predecessor FID, INP considers ALL interactions throughout the page
lifecycle, not just the first one.

| Rating            | Threshold  |
|-------------------|------------|
| Good              | ≤ 200ms   |
| Needs improvement | ≤ 500ms   |
| Poor              | > 500ms   |

INP replaced First Input Delay (FID) as a Core Web Vital in March 2024.
FID only measured the delay before processing the first interaction —
it didn't measure processing time, and it ignored every interaction
after the first. A page could score well on FID but feel sluggish
because subsequent interactions were slow. INP fixes both problems:
it measures the full input-to-paint latency, and it reports the worst
interaction (or near-worst, at the 98th percentile for pages with many
interactions).

INP captures three phases: input delay (main thread may be busy),
processing time (event handler execution), and presentation delay
(handler completion to next paint). Optimize by breaking up long tasks,
reducing main-thread work, debouncing rapid interactions, and moving
heavy computation to web workers.

### Cumulative Layout Shift (CLS)

CLS measures visual stability — how much the visible content shifts
unexpectedly during the page lifecycle. Every time a visible element
moves without user interaction, it contributes to the CLS score.

| Rating            | Threshold  |
|-------------------|------------|
| Good              | ≤ 0.1     |
| Needs improvement | ≤ 0.25    |
| Poor              | > 0.25    |

CLS is unitless. It's calculated as `impact fraction × distance
fraction` for each layout shift, summed within session windows. A
session window is a burst of shifts with less than 1 second between
them and a maximum duration of 5 seconds. The CLS score is the largest
session window's total, not the sum of all shifts.

Common CLS offenders: images/iframes without explicit dimensions,
dynamically injected content above the fold (ads, banners), web fonts
causing FOIT/FOUT, and late-loading components that push content down.
Fix by setting explicit dimensions on media, reserving space for dynamic
content with `min-height` or `aspect-ratio`, using `font-display: swap`
with size-adjusted fallbacks, and applying CSS `contain` for layout
isolation.

---

## 3 — Performance Budgets

A performance budget is a threshold that the team commits to not
exceeding. Without budgets, performance degrades incrementally — each
feature adds "just a little" weight until the page is slow and nobody
can point to a single cause. Budgets make performance a constraint
rather than an afterthought.

### Types of Budgets

**Quantity-based budgets** limit resource size and count: JS 170 KB
compressed (roughly 700 KB parse/compile work), images 500 KB for
content pages (1 MB for image-heavy), total page under 1.5 MB, under
50 HTTP requests, third-party scripts tracked separately (most common
bloat source), web fonts under 100 KB (subset to characters used).

**Timing-based budgets** limit user-perceived milestones: FCP under
1.8s, TTI under 3.8s, LCP under 2.5s, TBT under 200ms.

**Rule-based budgets** set score thresholds: Lighthouse Performance
at least 90, WebPageTest SpeedIndex under 3.0s, CrUX "good" origin
summary at least 75% for all three Core Web Vitals.

### Setting Budgets

**Benchmark competitors, then set your budget 20% better.** If
competitors' median JS size is 300 KB, your budget should be 240 KB.

For new projects without competitive benchmarks, starting points:
JS 170 KB (compressed), CSS 50 KB, images 500 KB, total page 1.5 MB,
requests 50, LCP 2.5s, TBT 200ms, CLS 0.1. Adjust after collecting
real user data.

### Enforcing Budgets

Budgets that aren't enforced are suggestions. Build-time enforcement
(`bundlesize`, `size-limit`, webpack `performance.maxAssetSize`) is
non-negotiable — it's the only check that catches regressions before
they ship. Supplement with Lighthouse CI on PRs and RUM dashboards
with regression alerts in production.

---

## 4 — Critical Rendering Path

The critical rendering path is the sequence of steps the browser takes
to convert HTML, CSS, and JavaScript into pixels on screen. Understanding
it is prerequisite to understanding why pages are slow.

### The Steps

**1. HTML parsing → DOM construction.** The browser parses HTML into
tokens and builds the DOM tree incrementally. `<script>` tags without
`defer` or `async` pause the parser because JS might modify the DOM
via `document.write()`.

**2. CSS parsing → CSSOM construction.** The browser fetches and parses
CSS into the CSSOM. Unlike the DOM, the CSSOM is NOT incremental — it
must be fully constructed before rendering proceeds. This is why CSS
is render-blocking: a single slow stylesheet blocks the entire page
from painting.

**3. Render tree construction.** The DOM and CSSOM are combined into a
render tree containing only visible elements. `display: none` elements
are excluded; `visibility: hidden` elements are included (they occupy
layout space).

**4. Layout (reflow).** The browser calculates exact pixel positions
and sizes for each element. Layout is expensive and cascading — changing
a parent's width triggers recalculation for all children.

**5. Paint.** Pixels are filled in per layer — text, colors, images,
borders, shadows. Complex operations (shadows, gradients, filters)
cost more.

**6. Composite.** Painted layers are combined into the final image.
Elements on their own compositor layer (`transform`, `opacity`,
`will-change`, `position: fixed`) can be moved without triggering
layout or paint — handled entirely by the GPU.

### What Blocks Rendering

**CSS is always render-blocking** — the browser won't paint until the
CSSOM is complete. **JavaScript is parser-blocking by default** — a
`<script>` tag pauses HTML parsing until the script downloads and
executes, and the browser waits for CSS before executing JS (since JS
might access computed styles). **Fonts can block text rendering** —
with `font-display: block` (the default), text is invisible until the
font loads (FOIT).

### Optimization Strategies

**Inline critical CSS.** Extract the CSS needed for above-the-fold
content and inline it in `<style>` tags in the `<head>`. Load the
remaining CSS asynchronously. This eliminates the render-blocking
network round trip for initial paint.

Tools: `critical` (npm package), Critters (webpack plugin).

**Defer and async JavaScript.** `<script>` blocks parsing and
rendering. `<script async>` downloads in parallel, executes immediately
when ready (no order guarantee). `<script defer>` downloads in parallel,
executes after parsing in document order. Use `defer` for DOM-dependent
scripts, `async` for independent scripts (analytics, ads). Never use
bare `<script>` for non-critical JavaScript.

**Preload critical resources.** Use `<link rel="preload">` for
resources the browser won't discover until later — fonts referenced
in CSS, images in CSS backgrounds, JavaScript modules imported
dynamically. Specify the `as` attribute (font, image, script) so the
browser can prioritize correctly.

**Preconnect to required origins.** Use `<link rel="preconnect">` for
third-party origins to complete DNS + TCP + TLS handshakes early.
Particularly valuable for font providers, CDNs, and API endpoints.

**Reduce critical resource count.** Every resource in the critical path
adds latency. Combine, inline, or eliminate. The fastest request is the
one that's never made.

---

## 5 — Lighthouse Scoring

Lighthouse is the de facto lab performance auditing tool. It simulates
a mid-tier mobile device on a throttled connection (Moto G4, slow 4G)
and produces a performance score from 0-100.

### Metric Weights (Lighthouse 12)

The performance score is a weighted average of five metrics:

| Metric                     | Weight |
|----------------------------|--------|
| Total Blocking Time (TBT)  | 30%    |
| Largest Contentful Paint   | 25%    |
| Cumulative Layout Shift    | 25%    |
| Speed Index (SI)           | 10%    |
| First Contentful Paint     | 10%    |

**Total Blocking Time (TBT)** is the sum of blocking time for all long
tasks during page load. A long task is any main-thread task exceeding
50ms. The blocking time is the portion exceeding 50ms — a 70ms task
contributes 20ms of blocking time. TBT is the lab proxy for INP; it
measures how much the main thread is monopolized during load.

**Speed Index (SI)** measures how quickly content is visually displayed
during load. It captures the visual completeness over time — a page
that renders progressively scores better than one that's blank for 3
seconds then renders all at once. Measured in milliseconds.

**First Contentful Paint (FCP)** is the time from navigation to the
first text or image paint. It's the earliest signal that the page is
loading. A good FCP is under 1.8 seconds.

### Scoring Ranges

| Range   | Score    | Interpretation                                |
|---------|----------|-----------------------------------------------|
| Good    | 90–100   | Page meets performance best practices         |
| Needs improvement | 50–89 | Optimization opportunities exist      |
| Poor    | 0–49     | Significant performance problems              |

### Lighthouse Caveats

Lighthouse is a lab tool — it measures controlled conditions, not real
user experience. Scores vary between runs; run 3-5 times and take the
median. The throttling profile is directionally correct but doesn't
perfectly replicate real devices. Lighthouse doesn't test post-load
interactions (INP) or real network conditions. Always consult field
data (CrUX, RUM) alongside lab results. Run in incognito or via the
Node CLI to avoid extension interference.

---

## 6 — Performance Anti-Patterns

When evaluating a page, look for these known anti-patterns. Each one
has an outsized impact on performance and is usually fixable.

### Layout Thrashing

Layout thrashing occurs when JavaScript repeatedly reads layout
properties (offsetHeight, getBoundingClientRect, scrollTop) and then
writes style changes, forcing the browser to recalculate layout
synchronously on each read. The fix: batch all reads first, then batch
all writes. This pattern can turn a 1ms operation into 100ms+ with
enough elements. Use FastDOM or similar read/write batching libraries
in complex UIs.

### Excessive DOM Size

A DOM with more than **1,500 nodes** is excessive. Also flag more than
**800 nodes** in a single subtree or depth exceeding **32 levels**.
Large DOMs slow style calculations, layout, and paint — every node has
a cost even when not visible. Mitigate with virtualized lists, lazy
rendering of off-screen sections, simplified markup, and replacing
wrapper divs with CSS Grid/Flexbox.

### Unoptimized Images

Images are typically the heaviest resources on a page. Five common
problems: no modern formats (JPEG/PNG instead of WebP/AVIF, 25-50%
savings lost), no responsive images (2000px image in a 400px viewport
— use `srcset`/`sizes`), no lazy loading (below-fold images loading
eagerly — use `loading="lazy"`), no explicit dimensions (causes layout
shifts), and no compression (quality 100 when 80 is indistinguishable).

### Render-Blocking Resources

Resources that block rendering delay first paint. Offenders: external
stylesheets without media queries (a `media="print"` sheet won't block
screen rendering), synchronous `<script>` tags in `<head>`, CSS
`@import` (creates sequential requests — use `<link>` instead), and
web font files without `font-display` set.

### Too Many HTTP Requests

Even with HTTP/2 multiplexing, each request has overhead. Pages with
more than **50 requests** during initial load deserve scrutiny. Look
at the waterfall for request chains (A loads B loads C). Common causes:
granular unbundled modules, excessive third-party scripts, individual
SVG requests replacing sprites, and CSS `@import` chains.

### Long Tasks

Any main-thread task exceeding **50 milliseconds** is a long task. Long
tasks block input response and frame painting. Chrome DevTools marks
them with a red corner in the Performance tab. Common causes: large JS
execution during load, synchronous XHR, complex DOM manipulation,
expensive style recalculations, heavy third-party scripts. Break them
up with `setTimeout`, `requestIdleCallback`, or `scheduler.yield()`.

### Uncompressed Resources

Text resources (HTML, CSS, JS, JSON, SVG) should always be served
compressed. Gzip is the minimum; Brotli provides 15-20% better ratios.
Check for `Content-Encoding: gzip` or `br` in response headers. Missing
compression on text resources is among the highest-impact, lowest-effort
fixes available.

### No Caching Strategy

Resources without proper cache headers are re-downloaded on every visit.
Look for missing `Cache-Control` headers, `no-store` on static assets
with content hashes, short `max-age` on infrequently-changing assets,
and missing `ETag`/`Last-Modified` for conditional requests.

The ideal pattern: immutable hashed static assets get
`Cache-Control: public, max-age=31536000, immutable`. The HTML document
gets `Cache-Control: no-cache` (meaning "revalidate every time," not
"don't cache") to ensure users always get the latest version pointing
to current hashed assets.

---

## 7 — Evaluation Procedure

When evaluating a page or application's performance, work through these
phases in order. Each phase builds on the previous one.

### Phase 1 — Gather Data

Collect both lab and field metrics before forming conclusions.

- Run Lighthouse 3-5 times. Use the median scores.
- Check CrUX data (via PageSpeed Insights or BigQuery) for field
  metrics. If the site has insufficient traffic for CrUX, note this —
  lab data alone is incomplete.
- Record the waterfall from the Network tab.
- Record the Performance tab trace during load and during key
  interactions.

### Phase 2 — Assess Against Budgets

Compare collected metrics against performance budgets (if the team has
them) or against the default thresholds documented in Section 3.

Flag any metric that exceeds its budget. Prioritize Core Web Vitals
(LCP, INP, CLS) first, then supporting metrics (FCP, TBT, SI).

### Phase 3 — Anti-Pattern Scan

Walk through each anti-pattern in Section 6. For each one found,
document:

- What the anti-pattern is
- Where it occurs (specific resources, specific code paths)
- The measured impact (how much time/size/shift it contributes)
- The recommended fix

### Phase 4 — Critical Path Analysis

Examine the critical rendering path:

- How many critical resources are there?
- What's the total critical path length (round trips)?
- What's the total critical bytes?

Identify optimization opportunities per Section 4.

### Phase 5 — Report

Produce findings ordered by expected impact (highest first). Each
finding: metric affected, current value, target value, root cause,
specific recommendation, and estimated improvement. The goal is a
prioritized list the team can work through top to bottom.

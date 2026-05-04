---
name: library-first-eval
description: >
  Audit for homegrown implementations of solved problems. Walks every
  module looking for custom code that reimplements what an existing
  dependency or a well-maintained open-source library already provides.
  Grounded in the principle that homegrown reimplementations of
  solved problems are a maintenance liability. Use after major feature
  work lands, or when the user invokes /library-first-eval.
user-invocable: true
---

# Library-First Eval

Audit every module in the worktree for custom code that should be a
library call.

## Phase 1 — Dependency inventory

1. Read every package manifest in the project (`Cargo.toml`,
   `pyproject.toml`, `package.json`, `go.mod`, etc.). List every direct
   dependency and its documented purpose.
2. For each dependency, note the surface it covers (e.g., `serde`
   covers ser/de; `tokio` covers async runtime; `clap` covers CLI
   parsing; `reqwest` covers HTTP).

## Phase 2 — Homegrown-implementation scan

Walk every source directory in the project. For every file, ask:

- Does this module contain custom logic for a problem that one of the
  project's existing dependencies already solves? If yes, flag it.
- Does this module contain custom logic for a problem that a
  well-maintained open-source library (3k+ GitHub stars) solves, even
  if that library is not currently a dependency? If yes, flag it.

Concrete patterns to catch (examples by language; pick the ones that
apply to the project's stack):

- **Configuration via scattered env-var reads.** Centralize at
  startup via a typed config struct (`config-rs`, `figment`, or a
  `serde`-backed struct in Rust; `pydantic-settings` in Python).
  Scattered reads bypass validation and make the config surface
  non-discoverable.
- **Hand-rolled JSON, YAML, or TOML parsing** when `serde_json` /
  `serde_yaml` / `toml` (Rust), the standard library (Python), or
  `JSON.parse` / `js-yaml` (JS) already exist.
- **Hand-rolled token formats, signature schemes, or credential
  flows** when `jsonwebtoken` (Rust), `pyjwt` / `jose` (Python),
  or `jose` (JS) already exist.
- **Hand-rolled HTTP retry / backoff** when `backoff` / `tower::retry`
  (Rust), `tenacity` (Python), or `p-retry` (Node) are available.
- **Hand-rolled state machines** when `statig` or `rust-fsm` (Rust)
  or `transitions` (Python) already exist.
- **Hand-rolled CLI argument parsing** when `clap` (Rust), `argparse`
  / `click` / `typer` (Python), or `commander` / `yargs` (Node) are
  available. Bespoke arg-string-splitting in `main` is a flag.
- **Custom date formatting** when `chrono` / `time` (Rust),
  `datetime.strftime` (Python), or `date-fns` / `dayjs` / `Intl`
  (JS) exist.
- **Hand-rolled async coordination** (channels, oneshots, broadcast
  fan-out, task cancellation, timeouts) when `tokio::sync::*` (Rust)
  or `asyncio` primitives (Python) already cover it.
- **Custom base64 / hex / URL encoding helpers** when the standard
  library covers it (`base64` / `hex` crates in Rust; stdlib
  `base64` in Python; `Buffer.from` / `btoa` in JS).
- **Hand-rolled regex utilities** for problems a parser library
  already solves (e.g. URL parsing via `url` crate, not regex).
- **Duplicated test-fixture wiring** when the project's real
  initialization function can be called with overrides instead.

## Phase 3 — Severity rating

For each flagged instance:

- **Blocker**: the custom implementation gates a trust boundary (auth,
  crypto, access control) where the library's audited implementation
  would prevent a class of vulnerability.
- **Major**: the custom implementation is >20 lines and the library
  equivalent is documented and available. Maintenance burden is real;
  edge cases are likely unhandled.
- **Minor**: the custom implementation is <20 lines, the library
  equivalent exists but the gain from switching is marginal (e.g.,
  a three-line `base64url` helper vs importing a library for one
  call).

## Phase 4 — Remediation

For each blocker or major, name the specific library, the specific
API call that replaces the custom code, and the files that need to
change.

**Bad:**

> "Consider using a library for this."

**Good:**

> Major: `<crate>/src/<module>.rs:NN-NN` hand-rolls retry-with-backoff
> using a sleep loop and a hand-tuned multiplier. Replace with an
> exponential-backoff helper from the project's chosen retry library
> (e.g. `backoff` or `tower::retry` in Rust; `tenacity` in Python;
> `p-retry` in Node) driving the existing HTTP call. Files: the
> caller, the retry module, the corresponding test.

## Phase 5 — Report

Severity-rated list. Each finding: file:line, what was reimplemented,
which library covers it, the specific API call, and files to change.
Under 600 words.

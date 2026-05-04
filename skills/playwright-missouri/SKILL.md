---
name: playwright-missouri
description: >-
  Use when writing missouri tests that need browser interaction — testing
  web UIs, verifying rendered state, or automating browser workflows as
  part of state graph transitions. Covers state directory layout, Playwright
  script structure, comparators, and determinism.
user-invocable: false
---

# Playwright Browser Testing with Missouri

## Philosophy

Missouri's state graph model applies directly to browser testing. Browser
state — storage, accessibility tree, screenshots — serializes to files in
state directories. Playwright scripts are transition commands. Missouri
compares the resulting files. No new missouri primitives needed.

The accessibility tree is the primary representation of page state, not
the DOM. It captures semantic meaning — what a user (or agent) actually
sees — and is stable across minor DOM changes.

## State Directory Layout

Browser state lives in a `browser/` subdirectory within each state:

```
state-a/
  .missouri/missouri.yml
  .tisket/...              # app data (if testing tisket web)
  tisket.yml
  browser/
    storage.json           # cookies + localStorage (Playwright storageState)
    aria.yml               # accessibility tree snapshot
    screenshot.png         # visual baseline
```

Not every state needs all files. A state before login might only have
`screenshot.png`. A state after login adds `storage.json`. Include only
what's meaningful for the transition being tested.

## Playwright Scripts

Scripts are PEP 723 inline-metadata Python files run with `uv run`:

```python
# /// script
# requires-python = ">=3.12"
# dependencies = ["playwright"]
# ///

import os, sys, json, yaml
from playwright.sync_api import sync_playwright

action = sys.argv[1]
port = os.environ["PORT"]

with sync_playwright() as p:
    browser = p.chromium.launch()

    # Restore state if storage.json exists
    storage_path = "browser/storage.json"
    context = browser.new_context(
        storage_state=storage_path if os.path.exists(storage_path) else None
    )
    page = context.new_page()
    page.goto(f"http://localhost:{port}")

    # ... perform interaction based on action ...

    # Serialize state
    os.makedirs("browser", exist_ok=True)
    context.storage_state(path="browser/storage.json")
    page.screenshot(path="browser/screenshot.png", full_page=True)

    # ARIA snapshot
    snapshot = page.accessibility.snapshot()
    with open("browser/aria.yml", "w") as f:
        yaml.dump(snapshot, f, default_flow_style=False)

    browser.close()
```

### Browser installation

Playwright needs browser binaries. Handle this in missouri's `setup:` block
so it runs once before all test paths:

```yaml
setup:
  - name: "install playwright browsers"
    command: "uv run playwright install chromium"
```

## Missouri Configuration

### Transition with browser interaction

```yaml
transitions:
  - name: "create issue from board"
    command: "uv run tests/browser/test.py create-issue"
    target: "../issue-created"
    services:
      - command: "clc-api serve --port 0 --root ."
    comparators:
      files:
        - path: "browser/screenshot.png"
          command: "pixel-diff"
```

The `services:` key starts the API server. `$PORT` is injected into the
Playwright script's environment. The script interacts with the app at
`http://localhost:$PORT`, then serializes browser state to `browser/`.
Missouri compares the resulting files against the target state directory.

### Assertion with browser verification

```yaml
assertions:
  - name: "board shows new issue"
    command: "uv run tests/browser/test.py verify-board"
    services:
      - command: "clc-api serve --port 0 --root ."
```

Assertions verify properties of the current state. The Playwright script
navigates, checks conditions, and exits 0 or non-zero.

## Comparators

### Screenshots — pixel-diff

```bash
#!/bin/sh
# pixel-diff: compare two PNGs with tolerance
actual="$1"
expected="$2"
uv run --with Pillow python3 -c "
from PIL import Image
import sys
a = Image.open(sys.argv[1])
b = Image.open(sys.argv[2])
if a.size != b.size:
    print(f'size mismatch: {a.size} vs {b.size}', file=sys.stderr)
    sys.exit(1)
diff = sum(abs(pa-pb) for pa,pb in zip(a.tobytes(), b.tobytes()))
threshold = len(a.tobytes()) * 0.01  # 1% tolerance
if diff > threshold:
    print(f'pixel diff {diff} exceeds threshold {threshold}', file=sys.stderr)
    sys.exit(1)
" "$actual" "$expected"
```

Put this in `.missouri/bin/pixel-diff` and mark it executable. Adjust
the threshold per project — 1% is a starting point.

### ARIA tree — text diff

ARIA snapshots as YAML are directly text-diffable. Missouri's default
byte comparison works. No custom comparator needed unless fields like
timestamps need ignoring.

### Storage — JSON diff

`storage.json` is JSON. Missouri's default byte comparison works for
exact matches. For fuzzy matching (session IDs, CSRF tokens), use a
custom comparator or the `ignore` comparator on the file.

## State Restoration

Playwright's `storageState` handles cookies and localStorage restoration
automatically via `browser.new_context(storage_state="browser/storage.json")`.

For sessionStorage (not included in `storageState`), use `addInitScript`:

```python
if os.path.exists("browser/session.json"):
    session = json.load(open("browser/session.json"))
    context.add_init_script(f"""
        for (const [k, v] of {json.dumps(list(session.items()))})
            sessionStorage.setItem(k, v);
    """)
```

## Determinism

Browser tests are inherently less deterministic than CLI tests. These
techniques help:

- **Font rendering**: Run in Docker/Linux for visual regression consistency
  across environments. macOS and Linux render fonts differently.
- **Animations**: Disable CSS animations:
  `page.emulate_media(reduced_motion="reduce")`
- **Time**: Playwright Clock API: `page.clock.install(time=datetime(2026, 1, 1))`
  freezes time for consistent timestamps in snapshots.
- **Random IDs**: Use comparator scripts that normalize or ignore them,
  or use the ARIA tree (which uses semantic names, not generated IDs).
- **Network timing**: The `services:` primitive runs the API server
  locally — latency is consistent. No external network calls during tests.
- **Viewport**: Set explicit viewport size in context creation:
  `browser.new_context(viewport={"width": 1280, "height": 720})`

## When NOT to Use This

- API behavior testing → use `curl` in missouri transitions directly
  (already covered by clc-api's missouri tests)
- Component logic testing → use `wasm-bindgen-test` for Rust/WASM unit tests
- This pattern is for testing the rendered, interactive web application
  end-to-end through a real browser

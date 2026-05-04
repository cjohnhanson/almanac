---
name: qa-cli
description: >-
  Exploratory QA for CLI tools. Enumerates testable areas via SFDPOT
  heuristics adapted for command-line interfaces, dispatches sub-agents
  as independent checkers, evaluates results against consistency oracles,
  runs fresh-eyes sessions. Use when verifying a CLI tool before shipping,
  after major changes, or when the user invokes /qa-cli.
user-invocable: true
---

# CLI QA

Investigate a CLI tool for problems. Sub-agents perform checks; the
orchestrating agent does the testing — interpreting results, recognizing
patterns, deciding what matters.

## Phase 1 — Survey (SFDPOT for CLI)

Run `<tool> --help` and explore the command tree. Map coverage:

- **Structure**: subcommands, nesting depth, help text at each level,
  flag naming conventions (--long vs -s), global vs local flags
- **Function**: what each command does — create, list, show, edit,
  delete, search. Does it do what the help text says?
- **Data**: output formatting (text vs JSON), field completeness,
  timestamp formats, encoding, empty state handling
- **Interfaces**: exit codes (0 success, non-zero failure), stderr
  vs stdout separation, piping compatibility, machine-readable output
- **Platform**: behavior with/without TTY (color, formatting),
  relative vs absolute paths, missing config files, permissions
- **Operations**: multi-step workflows end-to-end (init → create →
  edit → list → delete). State mutations in expected order.
- **Time**: timestamps, ordering stability, concurrent access

Produce a numbered checklist. Group by risk — what's most likely to be
broken and most damaging if it is. Present to the user for review.

## Phase 2 — Check (sub-agents)

Dispatch sub-agents for each flow. Each sub-agent is a checker — it
executes a specific sequence and reports pass/fail. It does NOT
interpret or make judgment calls.

Sub-agent charter format:

```
Set up a temp directory. Initialize the tool.

Execute:
1. [command]  → expect [exit code, stdout pattern, stderr pattern]
2. [command]  → expect [exit code, stdout pattern, stderr pattern]
...

For each step report: command, expected, actual, pass/fail.
Save transcript to /tmp/qa-cli/[flow-name].txt
```

Parallelize independent flows (each in its own temp dir).
Sequence dependent ones.

### What to check in each flow

- **Happy path**: does the command succeed with valid input?
- **Error path**: does it fail gracefully with bad input? Useful
  error message? Correct exit code?
- **Edge cases**: empty input, very long input, special characters,
  unicode, whitespace-only values
- **Idempotency**: running the same command twice — does it behave
  sensibly? (create should reject duplicates, edit should be re-runnable)
- **State transitions**: does the tool's state look right after each
  command? (check files on disk, list output, show output)

## Phase 3 — Evaluate (oracles)

Review sub-agent results against consistency oracles:

- **Familiar**: does it work like similar CLI tools? (git, cargo, etc.)
- **Explainable**: can every behavior be explained by the docs?
- **History**: is it consistent with previous versions?
- **Claims**: does it match what `--help` and docs say?
- **Product**: is it internally consistent? Same flag names, same
  output format, same error style across all subcommands?
- **Purpose**: does it serve its stated purpose?
- **Standards**: conventional exit codes (0/1/2), stderr for errors,
  stdout for output, no color when piped

Specific CLI consistency checks:

- Flag naming: `--format` everywhere or `--output` everywhere, not both
- Short flags: do `-t`, `-s`, etc. conflict across subcommands?
- Error messages: do they name the problem and suggest a fix?
- JSON output: valid JSON? Consistent schema across commands?
- Empty states: does `list` with no items output nothing (not an error)?
- Help text: accurate? Reflects actual behavior?

## Phase 4 — Fresh eyes

Spawn a sub-agent with NO context from phases 1-3. Give it only the
tool name and a goal:

```
You have access to a CLI tool called <tool>. Your goal is to
<accomplish X>. You have no documentation. Figure it out from
--help and experimentation.

Report:
1. Was --help sufficient to get started?
2. What confused you?
3. What error messages were unhelpful?
4. What did you expect to work that didn't?
5. One sentence: is this tool ready for users?
```

This catches problems that structured checking misses — confusing
flag names, missing commands, workflows that feel wrong even when
they technically work.

## Phase 5 — Report and fix

Categorize findings:

- **Blocking**: commands that crash, corrupt data, or silently fail
- **Degraded**: works but wrong (bad output, misleading errors,
  incorrect exit codes)
- **Cosmetic**: help text typos, inconsistent spacing, minor output
  formatting
- **Missing**: commands or flags that should exist but don't

Fix blocking and degraded issues. Re-check only the affected flows.
Loop until clean.

## Regression (RCRCRC)

After changes, prioritize retesting:

- **Recent**: commands changed in this session
- **Core**: primary user workflows
- **Risky**: complex commands with many flags
- **Configuration**: behavior that depends on config files or env vars
- **Repaired**: previously broken, just fixed
- **Chronic**: commands that keep having issues

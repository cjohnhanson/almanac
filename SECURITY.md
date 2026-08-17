# Security policy

## Reporting

Do not open a public issue for a vulnerability.

Report it privately:
**https://github.com/cjohnhanson/almanac/security/advisories/new**

That opens a thread only you and the maintainer can read.

Include what an attacker gains, what they must already control to get
it, the affected commit, and steps that reproduce it.

## What happens next

almanac has one maintainer, so response is best effort. Expect a reply
within a week.

A confirmed report gets a fix and an advisory published together. You
are credited unless you ask otherwise.

## Scope

almanac indexes agent skills from pluggable sources, and serves them over the Model Context Protocol. A source may be a local path, a git repository, or an https prefix.

A skill library is third-party content by design, and a skill is instructions an agent reads and may act on. What a library can reach, and what a skill can say, are both boundaries worth attacking.

In scope:

- A document, a declaration, or a name reaching outside the directory
  it should be confined to.
- A fetch reaching a host or a path that no declaration named.
- Reading untrusted content leading to code execution.
- A skill or a reference resolving to a file outside the library that declared it.
- The MCP server answering for a path outside the library it serves.

Out of scope:

- A dependency advisory with no exploitable path through this tool.
  Report it to that dependency.
- Denial of service from a malformed local file, where the caller
  already controls that file.

## Known boundaries

Documented limits are not vulnerabilities. `src/confined.rs` carries a
`# What this does not cover` section in its module documentation. Read
it before reporting a traversal issue.

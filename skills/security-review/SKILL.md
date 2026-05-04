---
name: security-review
description: >
  Security evaluation using OWASP Top 10, STRIDE threat modeling, DREAD
  risk rating, and a structured review checklist covering input validation,
  auth, session management, cryptography, error handling, logging, data
  protection, and dependencies. Use when reviewing code for security,
  evaluating an application's security posture, or when the user invokes
  /security-review. Not for penetration testing or compliance auditing.
user-invocable: true
---

# Security Review

Evaluate code and application security through structured analysis.
Three frameworks provide coverage from different angles: OWASP Top 10
identifies the most common vulnerability classes, STRIDE models threats
systematically, and DREAD rates risk severity. The structured checklist
ties it all together into concrete review items.

Work through all four sections. Findings should reference specific code
locations, not abstract possibilities.

---

## Section 1 — OWASP Top 10 (2021)

For each category, review the codebase for the specific patterns
described. A category is not "clear" until every check has been
actively investigated — not just assumed absent.

### A01: Broken Access Control

The most common vulnerability class. Users acting outside their
intended permissions.

**What to check:**
- Can URL paths or API endpoints be accessed without authentication
  by simply navigating to them directly?
- Are access control checks enforced server-side, or do they rely on
  client-side hiding of UI elements?
- Can a user view or edit another user's data by modifying an ID
  parameter (IDOR)? Example: changing `/api/users/123/profile` to
  `/api/users/456/profile`.
- Are CORS headers overly permissive (`Access-Control-Allow-Origin: *`
  on authenticated endpoints)?
- Can HTTP methods be bypassed? (e.g., a DELETE endpoint that only
  checks auth on POST but not DELETE)
- Are directory listings disabled on static file servers?
- Do JWT tokens get validated for expiration, audience, and issuer —
  not just signature?
- Is rate limiting applied to prevent enumeration of resources?

### A02: Cryptographic Failures

Sensitive data exposed through weak or missing cryptography.

**What to check:**
- Is data transmitted over plaintext HTTP anywhere (including internal
  service-to-service calls)?
- Are passwords hashed with a modern algorithm (bcrypt, scrypt, argon2)
  with appropriate work factors? MD5 and SHA-family are not acceptable
  for password hashing.
- Are cryptographic keys hardcoded in source? Search for patterns like
  `secret_key = "..."`, `API_KEY`, `private_key` in code and config.
- Is sensitive data stored in plaintext in databases (SSNs, credit
  cards, health records)?
- Are deprecated protocols in use (SSL 2/3, TLS 1.0/1.1)?
- Are random values generated with cryptographically secure PRNGs?
  `Math.random()` (JS), `random.random()` (Python), and `rand()`
  (C) are not cryptographically secure.
- Are encryption modes appropriate? ECB mode is almost never correct.
  Look for AES-GCM or AES-CBC with HMAC.
- Are certificates validated properly, or is verification disabled
  (`verify=False`, `NODE_TLS_REJECT_UNAUTHORIZED=0`)?

### A03: Injection

Untrusted data sent to an interpreter as part of a command or query.

**What to check:**
- SQL: Are queries constructed with string concatenation or
  interpolation? Every database query should use parameterized
  statements. Example of vulnerable code:
  `query = f"SELECT * FROM users WHERE id = {user_input}"`
- NoSQL: Are MongoDB queries built from user input without
  sanitization? `{"$gt": ""}` in a password field bypasses auth.
- OS command: Is `os.system()`, `subprocess.call(shell=True)`,
  `exec()`, or backtick execution used with user input?
- LDAP: Are LDAP filters built from unescaped user input?
- XPath: Are XPath expressions constructed from user input?
- Template: Can user input reach template rendering? (Server-side
  template injection via Jinja2, Twig, Freemarker, etc.)
- ORM: Even with ORMs, are there `.raw()`, `.extra()`, or literal
  SQL fragments that include user input?

### A04: Insecure Design

Flaws in architecture and design rather than implementation bugs.
These cannot be fixed by perfect implementation — the design itself
is the problem.

**What to check:**
- Are there business logic flows that can be abused? Example: a
  checkout flow where price is passed from client to server instead
  of looked up server-side.
- Is there rate limiting on resource-intensive operations (password
  reset, account creation, file upload)?
- Are trust boundaries clearly defined? Does the application
  distinguish between authenticated/unauthenticated, admin/user,
  internal/external?
- Is there a "fail open" pattern anywhere? (System grants access
  when a security check fails rather than denying.)
- Are security requirements documented in user stories or design
  docs, not just assumed?
- Can multi-step processes be completed out of order or with steps
  skipped?

### A05: Security Misconfiguration

Default configs, incomplete setups, open cloud storage, unnecessary
features enabled.

**What to check:**
- Are default credentials still active? (admin/admin, root/root,
  default API keys from documentation)
- Are stack traces or debug information exposed to users in
  production? (`DEBUG=True`, `FLASK_DEBUG=1`, detailed error pages)
- Are unnecessary HTTP methods enabled (TRACE, OPTIONS responding
  with too much)?
- Are security headers present? Check for:
  - `Content-Security-Policy`
  - `X-Content-Type-Options: nosniff`
  - `X-Frame-Options` or CSP `frame-ancestors`
  - `Strict-Transport-Security`
  - `Referrer-Policy`
- Are default sample applications or documentation endpoints deployed
  to production?
- Are directory listings enabled on web servers?
- Is cloud storage (S3 buckets, GCS buckets) publicly accessible when
  it shouldn't be?
- Are container images running as root?

### A06: Vulnerable and Outdated Components

Using libraries, frameworks, or other software with known
vulnerabilities.

**What to check:**
- Run dependency audit tools: `npm audit`, `pip-audit`, `cargo audit`,
  `bundler-audit`. Are there known CVEs in dependencies?
- Are dependencies pinned to specific versions, or using loose ranges
  that could pull in compromised versions?
- Are there dependencies that are unmaintained (no commits in 2+
  years, archived repositories)?
- Is the runtime itself (Node.js, Python, JVM, etc.) on a supported
  version receiving security patches?
- Are there vendored/copied library files that won't receive updates
  through normal dependency management?
- Is there a lockfile committed (`package-lock.json`, `poetry.lock`,
  `Cargo.lock`) to ensure reproducible builds?

### A07: Identification and Authentication Failures

Broken authentication mechanisms.

**What to check:**
- Is brute force protection in place? (Account lockout, progressive
  delays, CAPTCHA after N failures)
- Are passwords checked against a list of known-compromised passwords
  or common passwords?
- Is multi-factor authentication available for sensitive operations?
- Are session tokens regenerated after login? (Prevents session
  fixation.)
- Are password recovery flows secure? Does the reset token expire?
  Is it single-use? Can the security question be guessed?
- Are credentials transmitted over encrypted channels only?
- Is the "remember me" token separate from the session token and
  properly scoped?
- Do session tokens appear in URLs (query strings, referrer headers)?

### A08: Software and Data Integrity Failures

Code and infrastructure not protected against integrity violations.

**What to check:**
- Are CI/CD pipelines secured? Can an attacker modify build scripts
  or deployment configs?
- Is deserialization of untrusted data happening? (Java
  `ObjectInputStream`, Python `pickle.loads()`, PHP `unserialize()`,
  Ruby `Marshal.load()`) — all of these can lead to RCE.
- Are software updates verified with signatures or checksums?
- Are subresource integrity (SRI) hashes used for CDN-loaded scripts
  and stylesheets?
- Are dependencies pulled from trusted registries? Could a dependency
  confusion attack substitute a private package with a public one of
  the same name?
- Is unsigned or unverified code being dynamically loaded (`eval()`,
  `Function()`, dynamic `import()` from user input)?

### A09: Security Logging and Monitoring Failures

Insufficient logging makes attacks undetectable and undiagnosable.

**What to check:**
- Are authentication events logged? (Login success, login failure,
  logout, privilege escalation)
- Are authorization failures logged? (Access denied events)
- Are input validation failures logged?
- Are logs protected against injection? (Can a user craft input that
  creates fake log entries or injects newlines?)
- Are logs stored securely and tamper-evident?
- Is sensitive data excluded from logs? (Passwords, tokens, PII
  should never appear in logs.)
- Are alerts configured for suspicious patterns? (Brute force
  attempts, impossible travel, privilege escalation)
- Is there a retention policy? Are logs kept long enough for forensic
  analysis?

### A10: Server-Side Request Forgery (SSRF)

Application fetches a remote resource based on user-supplied URL
without validation.

**What to check:**
- Does the application fetch URLs provided by users? (Preview
  generators, webhook URLs, file importers, PDF renderers)
- Is there allowlisting of permitted domains/IPs for outbound
  requests?
- Are internal/private IP ranges blocked? (`10.x`, `172.16-31.x`,
  `192.168.x`, `127.x`, `169.254.169.254` metadata endpoint, `fd00::/8`)
- Can URL schemes be abused? (`file://`, `gopher://`, `dict://`)
- Are redirects followed on user-supplied URLs? Open redirects can
  bypass domain allowlists.
- Is DNS rebinding possible? (URL resolves to allowed IP at check
  time, then to internal IP at fetch time.)

---

## Section 2 — STRIDE Threat Modeling

STRIDE categorizes threats by what an attacker achieves. Apply each
threat category to the relevant system elements. The goal is to
enumerate threats — not just the ones that seem likely, but all that
are structurally possible.

For each identified threat, document: the threat category, the
affected element, the attack vector, and the recommended mitigation.

### S — Spoofing (Threats to Authentication)

**Applies to:** External entities, processes, data flows that cross
trust boundaries.

An attacker pretends to be something or someone they're not.

**Specific threats to look for:**
- Forged authentication tokens (JWTs with `alg: none`, stolen API
  keys, replayed session cookies)
- IP address spoofing to bypass IP-based access controls
- DNS spoofing to redirect traffic to attacker-controlled servers
- Email spoofing in password reset or notification flows
- Phishing attacks leveraging the application's own open redirect
  vulnerabilities

**Mitigations:**
- Strong authentication mechanisms (MFA, certificate-based auth)
- Token validation including algorithm, expiration, issuer, audience
- Mutual TLS for service-to-service communication
- DKIM/SPF/DMARC for email
- Input validation on all redirect URLs

### T — Tampering (Threats to Integrity)

**Applies to:** Data stores, data flows, processes.

An attacker modifies data, code, or configuration without
authorization.

**Specific threats to look for:**
- Modification of data in transit (man-in-the-middle on unencrypted
  connections)
- Direct modification of database records bypassing application logic
- Tampering with client-side state (cookies, local storage, hidden
  form fields) that the server trusts
- Modification of log files to cover tracks
- Altering configuration files or environment variables
- Supply chain attacks modifying dependencies

**Mitigations:**
- TLS for all data in transit
- Digital signatures or HMAC on sensitive data
- Database access controls (application uses least-privilege DB user)
- Server-side validation of all client-supplied data
- Integrity monitoring on critical files and configurations
- Code signing and verified builds

### R — Repudiation (Threats to Non-Repudiation)

**Applies to:** External entities, processes (especially those
performing sensitive operations).

An attacker performs an action and denies having done it, and the
system cannot prove otherwise.

**Specific threats to look for:**
- Critical operations (financial transactions, data deletion,
  privilege changes) without audit trails
- Logs that can be modified or deleted by the users they track
- Actions performed through shared accounts or generic credentials
- Systems where timestamps are not reliably synchronized
- Missing correlation IDs across distributed operations

**Mitigations:**
- Comprehensive, tamper-evident audit logging
- Digital signatures on critical transactions
- Centralized, append-only log storage
- Individual accountability (no shared accounts)
- Synchronized, trustworthy timestamps (NTP)
- Correlation IDs for distributed transaction tracing

### I — Information Disclosure (Threats to Confidentiality)

**Applies to:** Data stores, data flows, processes.

An attacker gains access to information they shouldn't have.

**Specific threats to look for:**
- Verbose error messages revealing stack traces, database schemas,
  internal IPs, or file paths
- API responses that include more data than the client needs
  (over-fetching, missing field-level authorization)
- Timing side channels that reveal whether a user exists (login
  response time differs for valid vs. invalid usernames)
- Metadata leakage (HTTP headers revealing server versions, Git
  directories exposed, `.env` files accessible)
- Cached sensitive data accessible to other users (shared caches,
  browser back button showing authenticated content)
- Memory dumps or core dumps containing sensitive data

**Mitigations:**
- Generic error messages for end users, detailed errors to logs only
- Explicit response shaping (return only needed fields)
- Constant-time comparison for secrets
- Stripping unnecessary headers, securing metadata
- Appropriate cache-control headers on sensitive responses
- Secrets management (vault, not environment variables or files)

### D — Denial of Service (Threats to Availability)

**Applies to:** Processes, data stores, data flows.

An attacker makes the system unavailable to legitimate users.

**Specific threats to look for:**
- Resource exhaustion through unbounded queries (missing pagination,
  no query complexity limits on GraphQL)
- Algorithmic complexity attacks (regex denial of service via
  catastrophic backtracking, hash collision attacks)
- File upload without size limits consuming disk space
- Connection pool exhaustion from slow clients (Slowloris)
- Cascading failures from lack of circuit breakers in
  microservice architectures
- Application-level DDoS through expensive operations (search,
  report generation, bulk exports)

**Mitigations:**
- Rate limiting and throttling at multiple layers
- Input size limits (request body, file uploads, query parameters)
- Timeouts on all external calls and database queries
- Circuit breakers and bulkheads in distributed systems
- Regex review for catastrophic backtracking (avoid nested
  quantifiers like `(a+)+`)
- Queue-based processing for expensive operations
- Autoscaling and load shedding

### E — Elevation of Privilege (Threats to Authorization)

**Applies to:** Processes (especially where trust boundaries exist).

An attacker gains capabilities beyond what they're authorized for.

**Specific threats to look for:**
- Vertical privilege escalation: regular user gains admin access
  (e.g., changing a `role` field in a request body or JWT)
- Horizontal privilege escalation: user A accesses user B's resources
  (IDOR — insecure direct object reference)
- Exploiting race conditions in permission checks (TOCTOU — time of
  check to time of use)
- Default or overly broad permissions (new features default to
  "allow" instead of "deny")
- Container or process running with unnecessary privileges
  (root in containers, overly broad IAM roles)
- SQL injection or command injection escalating to system access

**Mitigations:**
- Principle of least privilege everywhere (DB users, IAM roles,
  file permissions, container users)
- Server-side authorization on every request, not just at the UI
  layer
- Role-based or attribute-based access control with deny-by-default
- Input validation to prevent injection-based escalation
- Run processes and containers as non-root
- Regular permission audits

---

## Section 3 — DREAD Risk Rating

After identifying threats and vulnerabilities, rate each finding
using DREAD to prioritize remediation. Each factor is scored 1-3.
Total score ranges from 5 (low risk) to 15 (critical risk).

### Scoring Guide

**Damage Potential** — How bad is it if the vulnerability is exploited?
- 1: Minimal — minor data leakage, cosmetic defacement, inconvenience
  to individual users
- 2: Significant — access to sensitive data of individual users,
  limited financial impact, partial service disruption
- 3: Critical — complete system compromise, bulk PII exposure,
  significant financial loss, full service destruction

**Reproducibility** — How easy is it to reproduce the attack?
- 1: Difficult — requires specific timing, race conditions, unusual
  configuration, or physical access
- 2: Moderate — requires authentication, specific preconditions, or
  some insider knowledge
- 3: Trivial — can be reproduced every time with a simple request or
  publicly available exploit

**Exploitability** — How much skill/tooling is needed to exploit it?
- 1: Advanced — requires custom exploit development, deep technical
  knowledge, chaining multiple vulnerabilities
- 2: Intermediate — requires some security knowledge and standard
  tools (Burp Suite, sqlmap)
- 3: Novice — can be exploited with a web browser or simple curl
  command, or automated scanners find it

**Affected Users** — How many users are impacted?
- 1: Few — individual user or small subset under specific conditions
- 2: Some — significant subset, users of a specific feature, or
  users in a specific role
- 3: All — all users, or all users of a major feature, or all
  admins

**Discoverability** — How easy is it to find the vulnerability?
- 1: Hard — requires source code access, deep application knowledge,
  or sophisticated analysis
- 2: Moderate — discoverable by examining network traffic, error
  messages, or with automated scanning
- 3: Obvious — visible in URL, publicly documented, or found by
  casual browsing

### Risk Levels

- **12-15: Critical** — fix immediately, consider taking the system
  offline if needed
- **8-11: High** — fix in the current sprint, block release if not
  addressed
- **5-7: Low** — schedule for upcoming work, acceptable to ship with
  a tracked remediation plan

### Example Rating

A SQL injection in the login form:
- Damage: 3 (full database access)
- Reproducibility: 3 (every time with crafted input)
- Exploitability: 3 (sqlmap automates it)
- Affected Users: 3 (all users' data exposed)
- Discoverability: 2 (requires testing the input field)
- **Total: 14/15 — Critical**

---

## Section 4 — Structured Review Checklist

Work through each section. Check items are not aspirational — they
are concrete things to verify in the code. Mark each as pass, fail,
or not applicable with a brief note.

### 4.1 Input Validation

- [ ] All user input is validated server-side (client-side validation
      exists only for UX, never for security)
- [ ] Validation uses allowlists (accepted patterns) rather than
      denylists (rejected patterns)
- [ ] Input length limits are enforced on all fields
- [ ] File uploads validate file type by content (magic bytes), not
      just extension or MIME type from the client
- [ ] File uploads enforce size limits
- [ ] Uploaded files are stored outside the web root and served
      through a handler, not directly
- [ ] JSON/XML parsers are configured with size limits and depth
      limits to prevent billion-laughs and similar attacks
- [ ] Regular expressions used for validation are reviewed for
      catastrophic backtracking (no nested quantifiers on
      unbounded input)
- [ ] Rich text input is sanitized through a proven library (not
      custom regex), stripping or escaping HTML/JS

### 4.2 Authentication

- [ ] Passwords are hashed with bcrypt, scrypt, or argon2 (not
      MD5, SHA-1, SHA-256, or any unsalted hash)
- [ ] Failed login attempts are rate-limited or trigger progressive
      delays
- [ ] Authentication tokens are generated with sufficient entropy
      (at least 128 bits from a CSPRNG)
- [ ] Password reset tokens are single-use and expire within a
      reasonable time (15-60 minutes)
- [ ] "Remember me" functionality uses a separate, long-lived token
      — not an extended session
- [ ] Account lockout after repeated failures cannot be weaponized
      for denial of service (use progressive delays over hard lockout)
- [ ] API authentication uses tokens (OAuth, API keys) — not
      session cookies — for non-browser clients
- [ ] Default credentials do not exist in any environment

### 4.3 Authorization and Access Control

- [ ] Authorization is enforced server-side on every request, not
      just by hiding UI elements
- [ ] Access control follows deny-by-default: new endpoints require
      explicit permission grants
- [ ] Object-level authorization is checked: user A cannot access
      user B's objects by changing an ID parameter
- [ ] Function-level authorization is checked: non-admin users
      cannot call admin endpoints
- [ ] Field-level authorization is checked where needed: API
      responses don't include fields the requesting user shouldn't see
- [ ] Horizontal and vertical privilege escalation paths have been
      explicitly tested
- [ ] Role/permission changes take effect immediately (not cached
      indefinitely)
- [ ] Admin functionality is on a separate interface or behind
      additional authentication

### 4.4 Session Management

- [ ] Session IDs are generated server-side with a CSPRNG
- [ ] Session tokens are not present in URLs
- [ ] Sessions are invalidated on logout (server-side, not just
      client-side cookie deletion)
- [ ] Sessions have an absolute timeout (maximum lifetime regardless
      of activity)
- [ ] Sessions have an idle timeout (expire after period of
      inactivity)
- [ ] Session tokens are regenerated after authentication (prevents
      session fixation)
- [ ] Concurrent session limits exist where appropriate (or users
      can view and revoke active sessions)
- [ ] Session cookies use `Secure`, `HttpOnly`, and `SameSite`
      attributes

### 4.5 Cryptography

- [ ] TLS 1.2+ is used for all network communication (including
      internal services)
- [ ] Certificates are valid, not self-signed in production, and
      verified by clients
- [ ] Sensitive data at rest is encrypted with AES-256-GCM or
      equivalent
- [ ] Encryption keys are stored in a secrets manager — not in
      source code, config files, or environment variables
- [ ] Key rotation is possible without system downtime
- [ ] Random values for security purposes use CSPRNG
      (`secrets` module in Python, `crypto.randomBytes` in Node,
      `SecureRandom` in Java/Ruby)
- [ ] No custom or "homegrown" cryptographic implementations exist
- [ ] Deprecated algorithms are not in use (DES, 3DES, RC4, MD5
      for integrity, SHA-1 for signatures)

### 4.6 Error Handling

- [ ] Errors return generic messages to users — no stack traces,
      database errors, file paths, or internal IPs
- [ ] Error handling is centralized (not ad-hoc try/catch in every
      function)
- [ ] All code paths have error handling — no unhandled exceptions
      that crash the process or reveal information
- [ ] Authentication and authorization errors are intentionally
      vague (don't distinguish "user not found" from "wrong
      password")
- [ ] Errors don't create side channels (response timing doesn't
      differ based on which step failed)
- [ ] Failed operations don't leave the system in an inconsistent
      state (use transactions, cleanup on failure)

### 4.7 Logging

- [ ] Security events are logged: auth success/failure, access
      control failures, input validation failures, admin actions
- [ ] Logs include enough context: timestamp, user ID, IP address,
      action, resource, result
- [ ] Sensitive data is excluded from logs: no passwords, tokens,
      session IDs, PII, credit card numbers
- [ ] Log injection is prevented: user input in log messages is
      sanitized (no newlines or control characters that could forge
      entries)
- [ ] Logs are written to a centralized, append-only store (not
      just local files that an attacker can delete)
- [ ] Log retention meets operational and compliance requirements
- [ ] Alerting is configured for high-severity security events

### 4.8 Data Protection

- [ ] PII and sensitive data are identified and classified (know
      what you have and where it lives)
- [ ] Data minimization is practiced: don't collect or store data
      that isn't needed
- [ ] Sensitive data has defined retention periods with automated
      deletion
- [ ] Database connections use least-privilege accounts (the app
      should not connect as DBA)
- [ ] Backups are encrypted and access-controlled
- [ ] Data export/download features enforce authorization and rate
      limiting
- [ ] Cross-tenant data isolation is verified in multi-tenant
      systems (queries are always scoped to tenant)
- [ ] Caching of sensitive data uses appropriate TTLs and
      access controls

### 4.9 Dependencies and Supply Chain

- [ ] All dependencies are declared in a manifest with a committed
      lockfile
- [ ] Known vulnerabilities are checked: `npm audit`, `pip-audit`,
      `cargo audit`, or equivalent has been run
- [ ] Dependencies are reviewed for maintenance status (abandoned
      projects, low download counts, recent ownership transfers)
- [ ] No dependencies are loaded from URLs at runtime (CDN scripts
      without SRI, dynamically fetched code)
- [ ] Private package names cannot be confused with public ones
      (namespace squatting / dependency confusion)
- [ ] Build and CI pipelines use pinned dependency versions, not
      "latest"
- [ ] Container base images are from trusted sources and pinned by
      digest, not just tag

### 4.10 Configuration

- [ ] Secrets are not in source code or version control (search for
      hardcoded keys, passwords, tokens)
- [ ] Environment-specific configuration is separate from code
      (not `if production` branches with hardcoded values)
- [ ] Debug/development features are disabled in production
- [ ] Default deny: firewalls, security groups, and network policies
      block everything not explicitly needed
- [ ] Administrative interfaces are not exposed to the public
      internet
- [ ] HTTPS is enforced (HTTP redirects to HTTPS, HSTS is enabled)
- [ ] Container images run as non-root users
- [ ] File and directory permissions follow least privilege

---

## Section 5 — Cross-Reference Matrix

This table maps how the three frameworks overlap. A finding in one
area should trigger investigation in the related areas.

```
OWASP Category          | STRIDE Threats     | Checklist Sections
------------------------|--------------------|--------------------------
A01 Broken Access Ctrl  | S, E               | 4.3, 4.4
A02 Cryptographic Fail  | T, I               | 4.5, 4.8
A03 Injection           | T, E               | 4.1, 4.6
A04 Insecure Design     | all (design-level) | 4.1, 4.2, 4.3, 4.4
A05 Security Misconfig  | I, D, E            | 4.10, 4.6
A06 Vulnerable Comps    | T, E               | 4.9
A07 Auth Failures       | S, E               | 4.2, 4.4
A08 Integrity Failures  | T, R               | 4.9, 4.5
A09 Logging Failures    | R                  | 4.7
A10 SSRF               | S, I               | 4.1, 4.10
```

**How to use this matrix:**

When a vulnerability is found through one framework, the matrix
identifies where to look through the other frameworks. For example:

- A SQL injection finding (A03) should prompt investigation of
  Tampering and Elevation of Privilege threats (STRIDE), plus a
  thorough review of input validation (4.1) and error handling
  (4.6) in the checklist.
- An SSRF finding (A10) maps to Spoofing and Information Disclosure
  (STRIDE), meaning the investigation should also check whether the
  same endpoint could be used to spoof internal service identity or
  exfiltrate data from internal systems. Check input validation
  (4.1) and configuration (4.10) for related weaknesses.
- A finding of missing audit logs (A09) maps to Repudiation (STRIDE),
  which means there should also be investigation into whether any
  security-critical operations lack accountability — not just
  whether logging exists but whether it's sufficient to reconstruct
  what happened.

---

## Output Format

For each finding, document:

1. **Title** — one-line description
2. **Location** — file path and line number(s)
3. **Framework reference** — which OWASP category, STRIDE threat,
   and/or checklist item
4. **Description** — what the vulnerability is, concretely
5. **Proof of concept** — how an attacker would exploit it (specific
   request, input, or sequence of actions)
6. **DREAD score** — individual factor scores and total
7. **Remediation** — specific fix, with code example if applicable
8. **Verification** — how to confirm the fix works

Sort findings by DREAD score descending. Critical findings first.

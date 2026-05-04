---
name: architecture-eval
description: >
  Architecture evaluation using ATAM quality attribute scenarios, ISO 25010
  quality characteristics, coupling/cohesion analysis, SOLID at the system
  level, Conway's Law alignment, C4 model layered evaluation, ADR review,
  and technical debt assessment. Use when evaluating system architecture,
  reviewing design decisions, or when the user invokes /architecture-eval.
  Not for code-level review (use code-review-eval).
user-invocable: true
---

# Architecture Evaluation

Evaluate system architecture through structured frameworks applied in
sequence. Each section is a lens — the same architecture looks different
through each one, and the contradictions between lenses are where the
interesting findings live.

The evaluation proceeds in eight phases. Findings from earlier phases
inform later ones, so order matters.

---

## Phase 1 — ATAM (Architecture Tradeoff Analysis Method)

ATAM exists because architecture is fundamentally about tradeoffs.
Optimizing one quality attribute degrades another. The goal is to make
those tradeoffs explicit and deliberate rather than accidental.

### Step 1: Build the Utility Tree

The utility tree is a hierarchical decomposition of "utility" (the
overall goodness of the system) into quality attributes, then into
concrete scenarios.

```
Utility
├── Performance
│   ├── (H,H) API responds to 95th-percentile queries in <200ms under normal load
│   ├── (M,H) Batch jobs complete within the nightly maintenance window
│   └── (L,M) Dashboard renders initial page in <1s
├── Modifiability
│   ├── (H,H) A new payment provider can be integrated in <2 developer-weeks
│   └── (M,M) UI theme changes don't require backend deployment
├── Availability
│   ├── (H,H) System remains available during single-node failure
│   └── (M,L) Degraded mode serves read traffic during database failover
└── Security
    ├── (H,H) No PII exposed in logs or error responses
    └── (H,M) Authentication failures rate-limited to prevent brute force
```

Each scenario gets two ratings in `(importance, difficulty)` format:
- **Importance**: How much stakeholders care (H/M/L)
- **Difficulty**: How hard the architecture makes it to achieve (H/M/L)

Scenarios rated `(H,H)` are the ones most worth analyzing — high
stakeholder importance AND high architectural difficulty. These are
where the architecture is most strained.

### Step 2: Identify Sensitivity Points

A sensitivity point is an architectural decision that significantly
affects one quality attribute. Ask: "If this decision changed, would
this quality attribute be noticeably affected?"

Examples:
- "The choice to use synchronous database calls is a sensitivity point
  for performance under load"
- "The shared authentication middleware is a sensitivity point for
  availability — if it fails, everything fails"

### Step 3: Identify Tradeoff Points

A tradeoff point is an architectural decision that is simultaneously
a sensitivity point for multiple quality attributes — changing it
improves one but degrades another.

Examples:
- "Adding request-level caching improves performance but creates a
  sensitivity point for data consistency"
- "Decomposing the monolith improves modifiability but degrades
  operational simplicity and adds network-partition failure modes"

### Step 4: Catalog Risks and Non-Risks

A **risk** is an architecturally significant decision that hasn't been
validated against its scenario. A **non-risk** is one that has been
validated (through testing, proof of concept, production data, etc.).

Document each as:
- The decision
- The quality attribute scenario it affects
- Why it's a risk (unvalidated assumption) or non-risk (evidence)

### Deliverable

A table of the top-priority `(H,H)` scenarios with their sensitivity
points, tradeoff points, and risk status.

---

## Phase 2 — ISO 25010 Quality Characteristics

ISO 25010 defines eight top-level quality characteristics, each with
sub-characteristics. Walk through every one. The value isn't that the
taxonomy is perfect — it's that it forces attention to dimensions that
would otherwise be missed.

### 1. Functional Suitability

Does the system do what it's supposed to do?

- **Functional completeness** — Does the function set cover all
  specified tasks and user objectives? Look for gaps between what's
  specified and what's implemented.
- **Functional correctness** — Does the system provide correct results
  with the needed degree of precision? Where are the correctness
  boundaries (floating point, eventual consistency, etc.)?
- **Functional appropriateness** — Does the system facilitate the
  accomplishment of tasks? Or does it technically fulfill requirements
  while making the actual work harder?

### 2. Performance Efficiency

How well does the system use resources?

- **Time behavior** — Response times, throughput rates, processing
  times. Measure under realistic load, not just happy-path.
- **Resource utilization** — CPU, memory, disk, network, database
  connections. Are resources used proportionally to work done, or are
  there pathological cases?
- **Capacity** — What are the maximum limits? More importantly, what
  happens at and beyond those limits — graceful degradation or cliff?

### 3. Compatibility

How well does the system coexist with others?

- **Co-existence** — Can it share its environment (hardware, network,
  runtime) with other systems without interference? Watch for port
  conflicts, resource contention, dependency version conflicts.
- **Interoperability** — Can it exchange information and use the
  information it receives from other systems? Evaluate API contracts,
  data formats, protocol versions, schema evolution strategy.

### 4. Usability

How effectively and efficiently can users operate the system? For
architecture evaluation, "users" includes developers and operators,
not just end users.

- **Appropriateness recognizability** — Can users recognize whether
  the system is appropriate for their needs? Is the system's purpose
  clear from its interface?
- **Learnability** — How much effort to learn to use the system? For
  developer-facing systems: how long until a new team member is
  productive?
- **Operability** — How easy is it to operate and control? Evaluate
  deployment, configuration, monitoring, and incident response.
- **User error protection** — Does the system guard against user
  mistakes? For APIs: does it reject invalid input clearly, or
  silently do the wrong thing?
- **User interface aesthetics** — (Less relevant at architecture level,
  but note if architectural decisions constrain UI possibilities.)
- **Accessibility** — Does the architecture support accessible
  interfaces, or do architectural decisions make accessibility harder?

### 5. Reliability

How well does the system maintain its level of performance?

- **Maturity** — How reliably does it meet needs under normal
  operation? Track mean time between failures.
- **Availability** — Is the system operational and accessible when
  required? Evaluate the availability design: redundancy, failover,
  health checks.
- **Fault tolerance** — Does the system operate as intended despite
  hardware or software faults? Evaluate failure modes, blast radius,
  circuit breakers, bulkheads.
- **Recoverability** — Can the system recover data and re-establish
  desired state after interruption or failure? Evaluate backup
  strategy, data durability, recovery time objectives.

### 6. Security

How well does the system protect information and resist unauthorized
access?

- **Confidentiality** — Is data accessible only to those authorized?
  Evaluate encryption at rest and in transit, access control models,
  secrets management.
- **Integrity** — Does the system prevent unauthorized modification of
  data or programs? Evaluate input validation, checksums, audit trails.
- **Non-repudiation** — Can actions or events be proven to have taken
  place? Evaluate audit logging, digital signatures, tamper evidence.
- **Accountability** — Can actions of an entity be traced uniquely to
  that entity? Evaluate authentication strength, session management,
  identity propagation across services.
- **Authenticity** — Can the identity of a subject or resource be
  proved to be the one claimed? Evaluate certificate management, token
  validation, API key rotation.

### 7. Maintainability

How effectively and efficiently can the system be modified?

- **Modularity** — Is the system composed of discrete components such
  that a change to one has minimal impact on others? This is the
  architectural quality attribute.
- **Reusability** — Can components be used in more than one system or
  context? Evaluate coupling to specific deployment environments,
  configuration approaches, dependency management.
- **Analysability** — How effectively can the impact of a change be
  assessed? Evaluate observability, logging, tracing. Can you figure
  out what went wrong after the fact?
- **Modifiability** — Can the system be effectively modified without
  introducing defects or degrading quality? Evaluate separation of
  concerns, abstraction boundaries, extension points.
- **Testability** — Can test criteria be established and tests
  performed to determine whether those criteria have been met?
  Evaluate test infrastructure, mocking boundaries, environment parity.

### 8. Portability

How effectively can the system be transferred between environments?

- **Adaptability** — Can the system be adapted for different hardware,
  software, or other operational environments? Evaluate hard-coded
  assumptions about infrastructure.
- **Installability** — Can the system be installed or uninstalled
  successfully? Evaluate deployment complexity, dependency management,
  rollback capability.
- **Replaceability** — Can the system replace another specified system
  for the same purpose? Evaluate interface compatibility, data
  migration paths, contract adherence.

### Deliverable

For each characteristic, rate: Satisfactory / Needs Improvement /
Unsatisfactory. Only flag sub-characteristics where there's a
meaningful architectural concern — the point isn't to fill every box
but to catch blind spots.

---

## Phase 3 — Coupling and Cohesion Analysis

Coupling and cohesion are the two most fundamental structural
properties of a system. High cohesion within components and low
coupling between them is the goal, but the specific types matter
more than a vague "high/low" assessment.

### Coupling Types (worst to best)

Evaluate which types of coupling exist between the system's major
components. Each type represents a different kind of dependency, and
they get progressively less damaging:

1. **Content coupling** — One module directly modifies or depends on
   the internal workings of another. Reaching into another service's
   database, modifying another module's internal state, monkey-patching.
   This is architectural cancer. If found, flag immediately.

2. **Common coupling** — Multiple modules share the same global data.
   Shared mutable state, global configuration objects mutated at
   runtime, shared database tables with no ownership boundary. Changes
   to the shared data structure ripple unpredictably.

3. **External coupling** — Multiple modules depend on the same
   externally-imposed data format, protocol, or interface. Shared
   dependency on a wire format, specific database schema, external
   API contract. Manageable but creates coordination costs for changes.

4. **Control coupling** — One module controls the behavior of another
   by passing control information (flags, mode parameters). "Pass
   `is_admin=true` to skip validation" — the caller needs to know the
   callee's internal logic to use it correctly.

5. **Stamp coupling** — Modules share a composite data structure but
   each only uses part of it. Passing an entire User object when only
   the email is needed. Creates implicit dependencies on the full
   structure even though only a subset matters.

6. **Data coupling** — Modules communicate only through parameters,
   and each parameter is an elementary piece of data. This is the
   target state for most inter-component communication. Clean
   interfaces, minimal data exchange, no hidden dependencies.

### Cohesion Types (worst to best)

Evaluate the internal cohesion of each major component. Why are these
things together?

1. **Coincidental cohesion** — Elements are grouped arbitrarily. The
   "utils" package. The "helpers" module. The "common" directory.
   Nothing connects these elements except someone needed a place to
   put them. Split ruthlessly.

2. **Logical cohesion** — Elements are grouped because they're
   categorically similar, not because they work together. "All
   validators in one module" even though they validate unrelated
   things. "All API handlers in one file" regardless of domain.

3. **Temporal cohesion** — Elements are grouped because they happen at
   the same time. Startup initialization code, shutdown cleanup code.
   Sometimes unavoidable, but shouldn't be the primary organization.

4. **Procedural cohesion** — Elements are grouped because they follow
   a specific execution sequence. Better than temporal (there's an
   ordering reason), but the elements still don't share a purpose.

5. **Communicational cohesion** — Elements are grouped because they
   operate on the same data. All operations on the user profile in one
   module. Getting warmer — at least there's a shared subject.

6. **Sequential cohesion** — Elements are grouped because the output
   of one feeds the input of the next. A data transformation pipeline.
   Strong relationship, but the grouping is about data flow rather
   than domain meaning.

7. **Functional cohesion** — Every element contributes to a single,
   well-defined function. The module does one thing and everything in
   it exists to support that one thing. This is the target.

### How to Evaluate

For each major component (service, module, package — whatever the
architectural unit is):

1. List what it contains and what it does
2. Ask: "Why are these things together?" — the answer reveals cohesion type
3. Ask: "What does this component need from others?" — the answer reveals coupling type
4. Document the coupling type for each dependency edge and cohesion
   type for each component

### Deliverable

A coupling/cohesion map: each component labeled with its cohesion
type, each dependency edge labeled with its coupling type. Flag any
content or common coupling as immediate risks. Flag coincidental or
logical cohesion as refactoring candidates.

---

## Phase 4 — SOLID at the Architecture Level

SOLID principles were formulated for object-oriented class design, but
they apply with equal force at the system architecture level. The
evaluation questions change but the underlying principles don't.

### Single Responsibility Principle

**At class level:** A class should have one reason to change.
**At architecture level:** A service/component should have one business
capability owner.

Evaluation questions:
- If a single business requirement changes, how many components need
  to be modified?
- Does each component have exactly one team or stakeholder who "owns"
  its reason to exist?
- Are there components that change for unrelated business reasons?
  (e.g., a component that changes both when billing rules change AND
  when notification formats change)
- Can a change to one business capability be deployed independently?

### Open/Closed Principle

**At class level:** Open for extension, closed for modification.
**At architecture level:** New capabilities should be addable without
modifying existing components.

Evaluation questions:
- Can a new payment method be added without modifying the payment
  processing service? (Or whatever the domain equivalent is.)
- Are extension points explicit (plugin interfaces, event buses,
  webhook registrations) or does extension require forking/patching?
- When the last three features were added, did they require modifying
  existing components or adding new ones?
- Are there components that have become "god services" because every
  new feature requires changing them?

### Liskov Substitution Principle

**At class level:** Subtypes must be substitutable for their base types.
**At architecture level:** Components that implement the same interface
must be interchangeable without breaking consumers.

Evaluation questions:
- If a service is replaced with a different implementation of the same
  API contract, do consumers break? (This tests whether the contract
  is actually honored or whether consumers depend on implementation
  details.)
- Can you swap the database implementation, message broker, or cache
  layer without changes propagating beyond the adapter?
- Are there "leaky abstractions" where consumers depend on behavior
  not specified in the contract?
- Do different instances of the same service type (e.g., different
  regional deployments) behave consistently enough to be substitutable?

### Interface Segregation Principle

**At class level:** Clients shouldn't depend on interfaces they don't use.
**At architecture level:** Service interfaces should be client-specific,
not general-purpose.

Evaluation questions:
- Do consumers use all the endpoints/methods of the services they
  depend on, or do they depend on large interfaces where they use a
  small fraction?
- Are there "kitchen sink" APIs that serve every possible consumer
  rather than having focused interfaces per consumer type?
- Would a change to an unused part of a service's API require
  consumers to update their client libraries?
- Could the API be split into multiple focused interfaces that
  different consumer types depend on independently?

### Dependency Inversion Principle

**At class level:** Depend on abstractions, not concretions.
**At architecture level:** High-level policy components should not
depend on low-level mechanism components. Both should depend on
abstractions (contracts, interfaces, protocols).

Evaluation questions:
- Does the domain/business logic depend on infrastructure details
  (specific database, specific message broker, specific cloud
  provider)?
- Is there an explicit boundary between business policy and technical
  mechanism?
- Could the infrastructure layer be replaced without rewriting
  business logic?
- Do dependency arrows point inward (toward the domain) or outward
  (toward infrastructure)?

### Deliverable

For each principle, a verdict: Honored / Partially Honored / Violated,
with specific evidence. Focus on violations — they're the findings.

---

## Phase 5 — Conway's Law Alignment

Conway's Law states: "Any organization that designs a system will
produce a design whose structure is a copy of the organization's
communication structure." This isn't a suggestion — it's an
observation about an extremely strong force. Fighting it is usually
futile. Understanding it is diagnostic.

### Diagnostic Use

Map the system's component boundaries against the team boundaries.
Answer these questions:

1. **Does each component have a clear owning team?** If a component
   is co-owned by multiple teams, expect coordination overhead,
   inconsistent design decisions, and slow change velocity. Shared
   ownership is a design smell at the architecture level.

2. **Do component boundaries align with team boundaries?** If team A
   owns services X, Y, and Z, are X/Y/Z tightly coupled in a way
   that makes sense for one team but would be painful if split across
   teams? Conversely, if X is owned by team A but tightly coupled to
   Y owned by team B, expect friction.

3. **Do communication paths between teams match integration paths
   between components?** If team A and team B rarely talk, but their
   services are tightly integrated, the integration will be fragile.
   If two teams communicate frequently but their services have no
   integration, maybe they should be one team (or one service).

4. **Where does the architecture violate Conway's Law, and what's the
   cost?** Every misalignment between org structure and system
   structure is paying a tax. Quantify it: coordination meetings,
   cross-team PRs, blocked deployments, integration failures.

### The Inverse Conway Maneuver

If the desired architecture doesn't match the current org structure,
the Inverse Conway Maneuver says: restructure the teams to match the
desired architecture, and let Conway's Law work in your favor.

This is relevant to architecture evaluation because:
- If the evaluation reveals that the architecture needs to change, the
  team structure may need to change first (or simultaneously).
- Recommending an architectural change without considering org
  structure is recommending something that Conway's Law will resist.
- The feasibility of an architectural recommendation is partly a
  function of whether the org structure supports it.

### What to Document

- A side-by-side map: team structure vs. component structure
- Alignment points (where they match) and misalignment points (where
  they don't)
- The cost of each misalignment, estimated in developer friction
- Whether the Inverse Conway Maneuver is relevant to any recommended
  changes

---

## Phase 6 — C4 Model Layered Evaluation

The C4 model (Context, Containers, Components, Code) provides four
zoom levels for describing and evaluating architecture. Different
concerns live at different levels, and evaluating the wrong concern
at the wrong level wastes effort.

### Level 1: System Context

**What it shows:** The system as a single box, surrounded by the
users and external systems it interacts with.

**What to evaluate at this level:**
- Are all external actors (users, systems) identified? Missing an
  external dependency is a risk.
- Are the integration points with external systems well-defined? What
  happens when an external system is unavailable?
- Is the system boundary clear? Can you draw a single line around
  "the system" without ambiguity?
- Are trust boundaries identified? Which external actors are trusted,
  which are untrusted, and where does the system enforce that
  distinction?

### Level 2: Container

**What it shows:** The major runtime units (applications, databases,
message brokers, file systems) that make up the system, and how they
communicate.

**What to evaluate at this level:**
- Is each container's responsibility clear and singular? (Single
  Responsibility at the container level.)
- Are the communication protocols between containers appropriate?
  Synchronous where latency matters, asynchronous where resilience
  matters.
- Is the technology choice for each container justified? Does the
  technology match the container's requirements, or was it chosen by
  default?
- Are there containers that are single points of failure? What's the
  blast radius if each container goes down?
- Is the deployment model clear? Can each container be deployed
  independently?

### Level 3: Component

**What it shows:** The major structural building blocks inside each
container — the modules, services, or layers within a single
deployable unit.

**What to evaluate at this level:**
- Is the internal structure of each container well-organized?
  (Cohesion analysis from Phase 3 applies here.)
- Are the component boundaries clean? (Coupling analysis from Phase 3
  applies here.)
- Is there a consistent internal architectural style (layered,
  hexagonal, clean architecture, etc.)? Is it followed consistently or
  eroded?
- Are cross-cutting concerns (logging, authentication, error handling)
  handled consistently?

### Level 4: Code

**What it shows:** The implementation details within a single
component — classes, functions, data structures.

**What to evaluate at this level:**
- Code-level evaluation is out of scope for this skill. Use
  code-review-eval for this level.
- However, note when architectural concerns are only visible at the
  code level (e.g., an abstraction that exists in the architecture
  diagram but not in the code, or tight coupling that's hidden behind
  a nominally clean interface).

### Deliverable

Findings organized by C4 level. Each finding should be at the
appropriate level — don't report code-level issues as architecture
issues, and don't report context-level issues as component issues.

---

## Phase 7 — Technical Debt Assessment

Technical debt is the gap between the system's current state and the
state it would need to be in to make the next set of changes easy.
It's not inherently bad — like financial debt, it's a tool. The
problem is unmanaged, untracked, or unintentional debt.

### Taxonomy of Technical Debt

Classify identified debt by type:

1. **Deliberate/Prudent** — "We know this is a shortcut, and we'll
   pay it back after launch." Conscious decision with a plan. Track
   it, schedule the payback.

2. **Deliberate/Reckless** — "We don't have time for design." No plan
   to pay it back. Accumulates interest rapidly. Usually the most
   expensive kind.

3. **Inadvertent/Prudent** — "Now we know how we should have built
   it." Unavoidable learning. The system worked fine for previous
   requirements but the world changed. Not a failure of engineering.

4. **Inadvertent/Reckless** — "What's layering?" Debt from lack of
   knowledge or skill. Often the hardest to address because the team
   may not recognize it.

### Identification Methods

Look for these signals in the architecture:

- **Dependency cycles** — Component A depends on B depends on C
  depends on A. Always debt. Always expensive.
- **Shotgun surgery** — A single business change requires coordinated
  modifications across many components. Indicates poor separation of
  concerns.
- **Divergent change** — A single component changes for many different
  business reasons. Indicates too many responsibilities.
- **Dead code at the architecture level** — Components, APIs, or data
  stores that are maintained but unused. Cost without value.
- **Workarounds and "temporary" solutions** — Any component whose name
  or documentation says "temporary," "legacy," "old," or "v1" while a
  "v2" exists alongside it.
- **Missing abstraction layers** — Business logic that directly
  manipulates infrastructure, UI code that contains business rules,
  data access scattered throughout the codebase.
- **Inconsistent patterns** — Some components follow pattern A, others
  follow pattern B, for no principled reason. Every inconsistency is
  cognitive load.

### Prioritization

For each debt item, estimate:

- **Impact (I)** — How much does this debt slow down current and
  near-future work? (1-5 scale)
- **Spread (S)** — How much of the codebase is affected? Localized
  debt is cheap to service; widespread debt compounds. (1-5 scale)
- **Accrual rate (A)** — How fast is this debt getting worse? Some
  debt is stable; some compounds with every change. (1-5 scale)
- **Remediation cost (C)** — How expensive is it to pay back? (1-5
  scale, inverted: 1 = expensive, 5 = cheap)

**Priority score = (I + S + A) * C**

Higher scores should be addressed first — they represent high-impact,
fast-growing, widespread debt that's relatively cheap to fix. Low
scores are either low-impact or expensive to fix (or both).

### Deliverable

A debt register: each item classified by type (from the taxonomy),
described concretely, scored with the prioritization formula, and
assigned a recommended action (pay back now, schedule payback, accept
and monitor, or write off).

---

## Phase 8 — ADR (Architecture Decision Record) Review

ADRs document the decisions that shaped the architecture. Reviewing
them tells you not just what was decided but why, what alternatives
were considered, and what constraints were in play. Missing ADRs are
as informative as existing ones.

### Expected ADR Format

A well-structured ADR contains:

- **Title** — Short descriptive name, numbered (e.g., "ADR-0012:
  Use PostgreSQL for primary data store")
- **Status** — Proposed, Accepted, Deprecated, Superseded
- **Context** — The forces at play. What problem was being solved?
  What constraints existed? This is the most important section —
  without context, the decision can't be evaluated.
- **Decision** — What was decided. Stated clearly and concisely.
- **Consequences** — What follows from the decision. Both positive
  and negative. Honest consequences are the sign of a mature ADR
  practice.
- **Alternatives considered** — What other options were evaluated and
  why they were rejected. Without this, the decision looks arbitrary.

### How ADRs Serve as Evaluation Artifacts

During architecture evaluation, ADRs are evidence:

1. **Coverage** — Are the major architectural decisions documented?
   List the decisions that should exist (choice of database, service
   boundaries, communication patterns, authentication approach, etc.)
   and check which ones have ADRs. Missing ADRs are undocumented
   decisions — high risk for inadvertent debt.

2. **Currency** — Are ADRs up to date? An ADR that says "Accepted"
   but describes a system state that no longer exists is misleading.
   Superseded decisions should link to their replacements.

3. **Quality of reasoning** — Do the ADRs show genuine analysis of
   tradeoffs, or are they post-hoc justifications? Look for:
   - Multiple alternatives seriously considered (not straw men)
   - Honest negative consequences acknowledged
   - Context that explains the constraints, not just the preference

4. **Consistency** — Do the ADRs tell a coherent story? Or do later
   decisions contradict earlier ones without acknowledging the
   contradiction?

5. **Traceability** — Can the current architecture be explained by
   following the chain of ADRs? If significant aspects of the system
   have no decision trail, those are the areas most likely to contain
   inadvertent debt.

### When ADRs Don't Exist

If the project has no ADRs, the evaluation should:

1. Reconstruct the major decisions by examining the architecture
2. Document them as "inferred ADRs" with unknown context
3. Flag the absence of ADRs as a maintainability concern
4. Recommend establishing ADR practice going forward, starting with
   documenting the decisions that are most at risk of being revisited

### Deliverable

A review of existing ADRs noting coverage gaps, currency issues, and
reasoning quality. Inferred ADRs for undocumented decisions.

---

## Aggregation and Final Report

After all eight phases, produce a consolidated evaluation:

1. **Top findings** — The 5-10 most significant architectural
   concerns, cross-referenced to the phase(s) that identified them.
   Findings that appear in multiple phases are more significant.

2. **Tradeoff map** — From ATAM (Phase 1), the key tradeoffs in the
   architecture with their current balance point and whether that
   balance is appropriate.

3. **Quality profile** — From ISO 25010 (Phase 2), a summary of
   which quality characteristics are well-served and which are
   underserved by the current architecture.

4. **Structural health** — From coupling/cohesion (Phase 3) and
   SOLID (Phase 4), the overall structural quality and the specific
   components that need attention.

5. **Organizational alignment** — From Conway's Law (Phase 5),
   whether the architecture and org structure support each other.

6. **Debt inventory** — From Phase 7, the prioritized debt register
   with recommended actions.

7. **Decision health** — From Phase 8, whether architectural
   decisions are well-documented and well-reasoned.

Present findings with evidence. Every finding should trace back to a
specific observation in a specific phase. Architectural evaluation
that can't point to evidence is just opinion.

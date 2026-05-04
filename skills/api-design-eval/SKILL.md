---
name: api-design-eval
description: >
  API design evaluation using Richardson REST maturity model, Google/
  Zalando/Microsoft guidelines, JSON:API patterns, and developer
  experience heuristics. Evaluates naming, versioning, pagination,
  error handling, consistency, and the "guess test." Use when reviewing
  API designs, evaluating endpoints, or when the user invokes
  /api-design-eval. Not for implementation review (use code-review-eval).
---

# API Design Evaluation

Evaluate API design quality across nine dimensions. Each dimension
includes a framework, concrete examples of good and bad practice,
and evaluation criteria. The goal is not pedantic REST purity — it's
whether developers can use this API correctly on the first try without
reading a novel of documentation.

## Framework 1 — Richardson REST Maturity Model

Leonard Richardson's four-level model describes how "RESTful" an API
actually is. Most APIs claiming to be REST sit at Level 1 or 2. The
model is useful not as a purity test but as a diagnostic: lower
maturity levels tend to predict specific categories of developer
confusion.

### Level 0 — The Swamp of POX

One URI, one HTTP method (usually POST), operation specified in the
request body. This is RPC tunneled over HTTP.

```
POST /api
{
  "action": "getUser",
  "userId": 42
}
```

Everything goes to one endpoint. The HTTP layer is just a transport
tunnel. Error codes are all 200 with an application-level status
buried in the response body. Caching is impossible because every
request hits the same URL. Tooling (browsers, proxies, CDNs) can't
help because nothing meaningful is expressed in the HTTP layer.

When it's actually fine: internal microservice RPC (gRPC, Twirp),
GraphQL — these are intentionally not REST and shouldn't pretend to be.

### Level 1 — Resources

Distinct URIs for distinct things, but still just one HTTP method.

```
POST /users/42
{ "action": "get" }

POST /users/42
{ "action": "delete" }
```

The improvement over Level 0 is that each resource has an identity.
You can look at the URL and know what thing is being acted upon. But
the operation is still embedded in the body, so tooling still can't
distinguish reads from writes, and caching is still broken.

### Level 2 — HTTP Verbs

Resources + proper HTTP methods + status codes. This is where most
production APIs live and where most should aim.

```
GET    /users/42          → 200 with user object
POST   /users             → 201 with created user + Location header
PUT    /users/42          → 200 with updated user
DELETE /users/42          → 204
GET    /users?status=active → 200 with filtered collection
```

HTTP semantics are used correctly: GET is safe (no side effects),
PUT/DELETE are idempotent, POST is neither. Status codes communicate
outcome (201 Created, 404 Not Found, 409 Conflict). Caches work.
Browser dev tools are useful. Proxies can make intelligent decisions.

### Level 3 — Hypermedia Controls (HATEOAS)

Responses include links that tell clients what they can do next.

```json
{
  "id": 42,
  "name": "Ada Lovelace",
  "status": "active",
  "_links": {
    "self": { "href": "/users/42" },
    "deactivate": { "href": "/users/42/deactivate", "method": "POST" },
    "orders": { "href": "/users/42/orders" }
  }
}
```

The client doesn't hardcode URL patterns — it follows links from
responses. In theory this makes the API evolvable without breaking
clients. In practice, very few APIs implement this well, and most
clients ignore the links and hardcode paths anyway. Worth considering
for APIs with complex state machines (order workflows, approval
processes) where the valid next actions change based on current state.

**Evaluation criteria:** Identify the maturity level. Level 2 is the
baseline expectation for a public REST API. If it's below Level 2,
flag it. If it claims Level 3 but links are incomplete or clients
can't actually navigate by following them, that's worse than honest
Level 2.

## Framework 2 — Resource Design

Good resource design means a developer can look at a URL and predict
what it does, what it returns, and how to interact with it.

### URI Structure

Resources are nouns, not verbs. The URL identifies *what*, the HTTP
method specifies *how*.

```
Good:  GET  /orders/123
Bad:   GET  /getOrder?id=123
Bad:   POST /fetchOrderDetails
```

Collections are plural. Individual resources use an identifier
segment under the collection.

```
/users              → collection
/users/42           → individual resource
/users/42/orders    → sub-collection scoped to user 42
/users/42/orders/7  → specific order belonging to user 42
```

Nesting should rarely go deeper than two levels. If the path looks
like `/users/42/orders/7/items/3/variants/12`, the sub-resources
probably deserve their own top-level collections with query filters.

### CRUD Mapping

| Operation | Method | Path | Status | Notes |
|---|---|---|---|---|
| List | GET | /resources | 200 | Supports filtering, pagination |
| Create | POST | /resources | 201 | Returns created resource + Location header |
| Read | GET | /resources/:id | 200 | |
| Replace | PUT | /resources/:id | 200 | Full replacement, all fields required |
| Partial update | PATCH | /resources/:id | 200 | Only changed fields sent |
| Delete | DELETE | /resources/:id | 204 | Idempotent — deleting twice returns 204 or 404 |

### Custom Methods

Sometimes CRUD doesn't fit. When the operation is an action rather
than a state change on a resource, custom methods are appropriate.
Google's API design guide recommends a colon syntax:

```
POST /users/42:deactivate
POST /orders/123:cancel
POST /reports:generate
```

The colon separates the resource identifier from the action, making
it clear this isn't a sub-resource. This is preferable to verbs in
the URL path (`/deactivateUser/42`) because it preserves the
resource-centric URL structure while acknowledging that not
everything maps cleanly to CRUD.

**Evaluation criteria:** Can you look at any endpoint and immediately
know what resource it acts on, what operation it performs, and what
it returns? If not, the resource design needs work.

## Framework 3 — Naming Conventions

Inconsistent naming is the single most common API design problem and
the one that most reliably predicts developer frustration.

### URI Paths

- **Plural nouns:** `/users`, `/orders`, `/line-items` — not
  `/user`, `/order`, `/lineItem`
- **Kebab-case for multi-word segments:** `/line-items`, `/api-keys`
  — not `/lineItems`, `/line_items`, `/LineItems`
- **Lowercase always:** `/users/42/shipping-addresses` — not
  `/Users/42/ShippingAddresses`
- **No file extensions:** `/users/42` — not `/users/42.json`
- **No trailing slashes:** `/users` — not `/users/`
- **No CRUD verbs:** `/users` with POST — not `/createUser`

### JSON Property Names

Pick one convention and use it everywhere. The two viable options:

- **camelCase:** `firstName`, `createdAt`, `lineItems` — matches
  JavaScript convention, used by Google, Microsoft, and most
  JS-heavy ecosystems
- **snake_case:** `first_name`, `created_at`, `line_items` — matches
  Python/Ruby convention, used by Stripe, GitHub, and JSON:API

The wrong answer is mixing them. If `createdAt` appears in one
response and `updated_at` in another, trust is gone.

### Query Parameters

Follow the same convention as JSON properties. If your JSON uses
camelCase, your query params should too.

```
GET /users?sortBy=createdAt&pageSize=20
GET /users?sort_by=created_at&page_size=20
```

Common parameter names to standardize across all endpoints:
- Filtering: field name directly (`status=active`, `type=premium`)
- Sorting: `sort` or `sortBy` with direction (`sort=createdAt:desc`)
- Pagination: see Framework 4
- Searching: `q` for full-text, field-specific filters for structured

### Headers

Custom headers use the `X-` prefix (deprecated but still common) or
a namespaced prefix. The convention that matters is consistency.

```
X-Request-Id: abc-123
X-Correlation-Id: def-456
```

Standard headers follow HTTP convention: `Content-Type`,
`Authorization`, `Accept`, `If-None-Match`.

**Evaluation criteria:** Pick any five endpoints at random. Are the
naming conventions identical across all five? If not, every
inconsistency is a finding.

## Framework 4 — Pagination Patterns

Any collection endpoint that could return more than ~100 items needs
pagination. The choice of pattern has significant consequences for
performance and developer experience.

### Offset/Limit

```
GET /users?offset=40&limit=20
```

Simple, familiar, maps directly to SQL `OFFSET/LIMIT`. Developers
understand it immediately. But: skipping rows is O(n) in most
databases, so page 1000 is dramatically slower than page 1. Items
inserted or deleted between page fetches cause duplicates or missed
items. Fine for small datasets with infrequent writes. Poor choice
for large, actively-changing collections.

### Page/Size

```
GET /users?page=3&size=20
```

Syntactic sugar over offset/limit (page 3 with size 20 = offset 40).
Same performance characteristics and consistency problems. Slightly
more intuitive for UI-driven pagination with numbered page buttons.

### Cursor-Based

```
GET /users?limit=20
→ { "data": [...], "cursor": "eyJpZCI6NDJ9" }

GET /users?limit=20&after=eyJpZCI6NDJ9
```

The cursor is an opaque token encoding the position in the result
set (typically the last seen ID or a composite key). The server
decodes it and uses a `WHERE id > ?` clause instead of `OFFSET`,
which is O(1) regardless of position. No duplicate or missed items
from concurrent writes. The tradeoff: no random page access (can't
jump to page 50), so it doesn't work for UIs with numbered page
buttons. But most modern UIs use infinite scroll or "load more,"
which cursor pagination handles perfectly.

### Keyset

```
GET /users?limit=20&createdAfter=2024-01-15T00:00:00Z&idAfter=42
```

Similar to cursor-based but with explicit, human-readable parameters
instead of an opaque token. More transparent — developers can see and
manipulate the pagination state. But the API is coupled to the sort
order, and changing the sort key means changing the pagination
parameters. Works well when the sort order is fixed or there are very
few sort options.

### Response Envelope

Regardless of pattern, pagination responses should include metadata:

```json
{
  "data": [...],
  "pagination": {
    "total": 1042,
    "limit": 20,
    "hasMore": true,
    "nextCursor": "eyJpZCI6NDJ9"
  }
}
```

Including `total` is expensive (requires a COUNT query) and optional
for cursor-based pagination. If omitted, document that it's omitted.

**Evaluation criteria:** Is cursor-based pagination used for
large/active collections? Is the pagination metadata complete? Are
the parameter names consistent across all collection endpoints?

## Framework 5 — Error Handling

Error responses are where APIs most often fail developers. A good
error response tells the client: what went wrong, which field or
parameter caused it, what to do about it, and provides a stable
machine-readable code for programmatic handling.

### RFC 9457 — Problem Details for HTTP APIs

The standard format. Use it or something structurally equivalent.

```json
{
  "type": "https://api.example.com/errors/validation-failed",
  "title": "Validation Failed",
  "status": 422,
  "detail": "The 'email' field must be a valid email address.",
  "instance": "/users",
  "errors": [
    {
      "field": "email",
      "code": "invalid_format",
      "message": "Must be a valid email address. Received: 'not-an-email'"
    }
  ]
}
```

Key fields:
- **type**: A stable URI identifying the error category. Clients can
  switch on this for programmatic handling. Does not need to resolve
  to an actual web page (though it's nice if it does).
- **title**: Short human-readable summary of the error type. Same for
  all instances of this error type.
- **status**: The HTTP status code (repeated for convenience when the
  response is logged without headers).
- **detail**: Human-readable explanation specific to this occurrence.
  Include the actual problematic value when safe to do so.
- **instance**: The request path or a URI identifying this specific
  occurrence.

### Bad vs Good Error Responses

Bad — the client has nothing to work with:
```json
{ "error": "Bad Request" }
{ "error": true, "message": "Something went wrong" }
{ "code": 500 }
```

Bad — mixed formats (validation errors look different from auth
errors look different from not-found errors):
```json
{ "errors": ["invalid email"] }          // validation
{ "error": { "code": "UNAUTHORIZED" } }  // auth
{ "message": "Not Found" }               // 404
```

Good — consistent structure, machine-readable codes, human-readable
context, field pointers:
```json
{
  "type": "https://api.example.com/errors/insufficient-funds",
  "title": "Insufficient Funds",
  "status": 422,
  "detail": "Account balance is $12.50, but the transfer requires $50.00.",
  "errors": [
    {
      "field": "amount",
      "code": "insufficient_funds",
      "message": "Transfer amount exceeds available balance by $37.50"
    }
  ]
}
```

### Status Code Usage

- **400** — malformed request syntax (unparseable JSON, wrong
  Content-Type)
- **401** — authentication missing or invalid (token expired, no
  credentials)
- **403** — authenticated but not authorized for this resource/action
- **404** — resource does not exist (or the caller isn't authorized
  to know it exists)
- **409** — conflict with current resource state (duplicate creation,
  concurrent edit)
- **422** — request is syntactically valid but semantically wrong
  (validation failures)
- **429** — rate limited (include `Retry-After` header)
- **500** — server error (never expose stack traces or internal
  details)

**Evaluation criteria:** Is the error format consistent across all
endpoints? Does every error include a machine-readable code, a
human-readable message, and a pointer to the problematic field when
applicable? Are status codes used correctly?

## Framework 6 — Versioning Strategies

Every API will change. The versioning strategy determines how painful
those changes are for both the API team and consumers.

### Strategy Comparison

| Strategy | Example | Pros | Cons |
|---|---|---|---|
| **URL path** | `/v1/users` | Simple, visible, easy to route | Entire API surface duplicated per version, encourages big-bang breaking changes |
| **Header** | `Accept: application/vnd.api+json;version=2` | Clean URLs, fine-grained | Invisible to browsers, harder to test, easy to forget |
| **Query param** | `/users?version=2` | Easy to test in browser | Pollutes query space, caching complications |
| **Evolution** | No explicit version, additive changes only | No version management overhead | Requires discipline, eventually hits limits |

URL path versioning is the most common and the easiest for developers
to understand. Evolution (no explicit versioning, only additive
non-breaking changes) is increasingly preferred by APIs that can
maintain the discipline — Stripe is the notable example, using
date-based API versions as a compatibility layer.

### Breaking vs Non-Breaking Changes

**Breaking changes** — require a new version or careful migration:
- Removing a field from a response
- Renaming a field
- Changing a field's type (string → integer)
- Changing a field's semantic meaning
- Adding a required field to a request
- Removing an endpoint
- Changing a URL pattern
- Changing authentication requirements
- Changing error response format
- Tightening validation (rejecting previously-accepted input)

**Non-breaking changes** — safe to make without a new version:
- Adding a new optional field to a request
- Adding a new field to a response
- Adding a new endpoint
- Adding a new optional query parameter
- Adding a new enum value (if clients handle unknown values)
- Loosening validation (accepting previously-rejected input)
- Adding new error types (if clients handle unknown types)

The gray area: adding a new enum value is technically non-breaking
only if clients are written to tolerate unknown values. In practice,
many clients use strict deserialization and will break. This is why
API guidelines should document expectations for client resilience.

**Evaluation criteria:** Is there a clear versioning strategy? Is it
applied consistently? Are breaking changes identified and handled
through the versioning mechanism?

## Framework 7 — Idempotency

An idempotent operation produces the same result whether called once
or multiple times. This matters because networks are unreliable —
clients need to safely retry requests without causing duplicate
side effects.

### Inherent Idempotency by Method

- **GET** — always idempotent and safe (no side effects)
- **HEAD** — always idempotent and safe
- **OPTIONS** — always idempotent and safe
- **PUT** — idempotent by definition (replacing a resource with the
  same representation yields the same state)
- **DELETE** — idempotent (deleting an already-deleted resource is
  a no-op, whether that returns 204 or 404 is a style choice)
- **POST** — NOT inherently idempotent (each call may create a new
  resource)
- **PATCH** — NOT inherently idempotent (depends on the patch format
  — `{"count": 5}` is idempotent, `{"increment": 1}` is not)

### Idempotency-Key for POST

For non-idempotent operations (especially resource creation and
payment processing), the `Idempotency-Key` header pattern provides
safety:

```
POST /payments
Idempotency-Key: unique-client-generated-key-abc123
Content-Type: application/json

{ "amount": 5000, "currency": "usd" }
```

How it works:

1. The client generates a unique key (UUID) and sends it with the
   request.
2. The server checks if it has seen this key before.
3. If new: process the request, store the key with the response,
   return the response.
4. If seen: return the stored response without re-executing the
   operation.

Implementation considerations:
- Keys should expire after a reasonable window (24-48 hours)
- The stored response must include the status code and full body
- Concurrent requests with the same key should be serialized (one
  processes, others wait or get 409)
- The key is scoped to the authenticated client — different clients
  can use the same key without collision

Stripe's implementation is the reference standard: keys expire after
24 hours, concurrent duplicate requests get 409, and the stored
response is replayed byte-for-byte.

**Evaluation criteria:** Are POST endpoints that create resources or
trigger side effects protected by idempotency keys? Is the mechanism
documented? Are PUT and DELETE truly implemented as idempotent?

## Framework 8 — Developer Experience

Technical correctness is necessary but not sufficient. An API can be
perfectly RESTful and still be miserable to use. These heuristics
evaluate the human side of the design.

### Time to First Call (TTFC)

How long does it take a new developer to make a successful API call
from zero? This includes: finding the docs, understanding auth,
getting credentials, constructing a request, and getting a
meaningful response. The best APIs target under 5 minutes.

Friction points that inflate TTFC:
- Auth setup that requires contacting a human (approval workflows,
  manual key provisioning)
- Docs that explain concepts before showing a working example
- No curl/httpie example on the first page
- Sandbox/test environments that require separate signup
- API keys that take hours or days to provision

### The Guess Test

Give a developer the base URL and the documentation for two or three
endpoints. Then ask them to predict the URL, method, request body,
and response shape for a fourth endpoint they haven't seen. If they
can guess correctly, the API is consistent. If they can't, the API
has consistency problems.

This is the single most revealing test of API design quality. It
works because it tests whether the API follows its own patterns.
Inconsistency is the source of most developer frustration — not
complexity, not missing features, not even poor documentation.

### Consistency Audit

Check these across every endpoint:
- Same naming convention (camelCase everywhere or snake_case
  everywhere — never both)
- Same pagination parameters and response shape
- Same error response format
- Same envelope structure (if using one)
- Same date format (ISO 8601, always with timezone)
- Same null handling (omit null fields, or include them with null
  value — pick one)
- Same authentication mechanism
- Same rate limiting headers

Every inconsistency is a context switch that costs developer time
and erodes trust in the API.

### Error Quality

When something goes wrong, does the error response help the
developer fix the problem without searching external resources?

Good error messages:
- Name the specific field that's wrong
- Show the invalid value that was received
- State what was expected
- Suggest what to do ("Did you mean X?" or "See docs at URL")

The test: can a developer who has never read the docs fix their
request using only the error response? If yes, the errors are good.

### Forgiveness (Postel's Law)

"Be conservative in what you send, be liberal in what you accept."

A forgiving API:
- Accepts both `camelCase` and `snake_case` field names (if
  reasonable for the ecosystem)
- Trims whitespace from string inputs
- Accepts dates in multiple formats and normalizes
- Ignores unknown fields in request bodies rather than rejecting
  them
- Treats empty strings and null equivalently where semantically
  appropriate
- Returns helpful errors when being strict is necessary

Forgiveness isn't about accepting garbage — it's about not punishing
developers for trivial variations that don't affect intent. The
balance point: accept variations silently when the intent is
unambiguous, return clear errors when the intent is unclear.

**Evaluation criteria:** Estimate TTFC from the documentation. Run
the guess test mentally with three known endpoints and one unknown.
Count inconsistencies across five random endpoints. Evaluate error
quality by reading five error responses. Assess forgiveness by
identifying places where the API rejects valid intent due to
formatting pickiness.

## Framework 9 — Backward Compatibility and Deprecation

APIs are contracts. Breaking a contract damages trust proportional
to the number of clients affected and how much warning they received.

### What Constitutes a Breaking Change

See the list in Framework 6. The key insight: breaking changes are
defined by client impact, not by server implementation. If clients
break, it was a breaking change, regardless of whether the team
classified it as one.

### Deprecation Lifecycle

A responsible deprecation follows four stages:

**Preview / Beta**
- The feature is available but explicitly marked as unstable
- Clients accept the risk of breaking changes
- Response headers include `Sunset: <date>` or a beta warning
- Documentation marks the feature clearly

**General Availability (GA)**
- The feature is stable and covered by the API's compatibility
  promise
- Breaking changes require a new API version

**Deprecated**
- The feature still works but is marked for removal
- Response headers include `Deprecation: true` and `Sunset: <date>`
- Documentation shows the replacement and migration path
- API responses may include a warning in a standard location
  (e.g., a `warnings` array in the response envelope)
- Monitoring tracks usage to identify clients that need migration
  help

**Sunset**
- The feature is removed
- Requests return 410 Gone with a response body pointing to the
  replacement
- The sunset date was communicated at least 6-12 months in advance
  for public APIs

### Migration Support

When deprecating, provide:
- A migration guide (old → new mapping for every affected endpoint)
- Codemods or SDK helpers when feasible
- A deprecation timeline with hard dates
- Usage analytics so the team knows who's still calling deprecated
  endpoints
- A way for affected clients to request a deadline extension for
  genuine hardship cases

**Evaluation criteria:** Are deprecated features marked in both
documentation and response headers? Is there a clear timeline? Is
there a migration guide? Does the API use Sunset headers? Has any
feature been removed without a deprecation period?

## Running the Evaluation

Apply each of the nine frameworks. For each, assign one of:

- **Pass** — meets the standard with no significant issues
- **Concern** — meets the standard partially, specific issues noted
- **Fail** — does not meet the standard

Present findings grouped by severity (Fail first, then Concern).
For each finding, include:
1. Which framework and which specific criterion
2. The specific endpoint(s) or pattern affected
3. A concrete example of the problem
4. A concrete example of what the fix looks like

Do not pad findings with praise. If an API passes a framework
cleanly, a single line noting the pass is sufficient. Developers
reading this evaluation are looking for problems to fix, not
validation.

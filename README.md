# AppHub — exploratory backend for an alternative iOS app marketplace

> A learning project I started after coming across the Onside Rust role. The
> problem space looked genuinely interesting, so instead of just reading about
> it I started building a small backend to actually try the patterns the role
> centers on — **highload, async Rust, and the fintech-adjacent parts**
> (payments, ledger, payouts). One measurable step at a time.

This is **work in progress and intentionally honest**: it's an exploration, not
a product. Several external integrations (payment provider, notarization) are
mocked behind traits so I can focus on the architecture and the load behavior
rather than third-party onboarding.

---

## Why this exists

I wanted to understand a marketplace backend by building one, not by reading
about it. The interesting tension in this domain is that it has **three very
different load profiles** living in the same system, and each scales
differently:

- **Read-heavy, spiky** — catalog browsing and binary distribution.
- **Write-heavy, contention-prone** — the money ledger.
- **Bursty background work** — analytics, notarization, payment webhooks.

The goal of this repo is to feel each of those for myself, measure it, and learn
the right tool for each — rather than reaching for "just add servers".

## What it is

- A place to try real patterns under real (load-tested) numbers.
- Incremental: every step adds **one** thing and produces **one** measurable result.

## What it isn't

- Not production-ready, not audited, not feature-complete.
- External providers are mocked; this is about the backend's own behavior.

---

## Tech stack

Chosen to mirror the kind of stack this domain uses in production:

- **Rust** — `axum`, `tokio`, `sqlx`
- **PostgreSQL** — primary store, compile-time-checked queries, migrations
- **Redis** — cache-aside for read-heavy paths
- **NATS** — async events / queue-based load leveling
- **Observability** — Prometheus + Grafana, OpenTelemetry traces
- **Docker / Docker Compose** — local environment; Kubernetes later

## Approach

The thing I care about most here is **method, not feature count**:

1. Build the smallest increment that runs.
2. Load-test it (`k6`), record the numbers.
3. Change one thing, re-test, compare before/after.
4. Write down *why* I made the call and what the trade-off was
   (see [`docs/decisions.md`](docs/decisions.md)).

Each stage is a self-contained, sprint-style doc under
[`docs/stages/`](docs/stages/), and a step is only "done" once there's a
measurement or a clear outcome behind it.

---

## Roadmap

Checkboxes reflect actual progress, updated as I go. Done stages link to their
sprint doc; later stages get linked as they're written.

### Part A — Marketplace core & highload

- [x] **0. Foundation** — runnable skeleton: service + database + local infra, up with one command — [doc](docs/stages/00-Foundation.md)
- [x] **1. App Catalog** — the first domain: create / list / get applications, cleanly layered — [doc](docs/stages/01-catalog.md)
- [x] **2. Performance Baseline** — measure the catalog under load *before* any optimization — [doc](docs/stages/02-Performance.md) · [results](docs/baseline/02-performance.md)
- [x] **3. Caching Layer** — cache-aside on the read path; validated before/after vs the baseline — [doc](docs/stages/03-Caching.md) · [results](docs/baseline/03-caching.md)
- [x] **4. Write-Heavy Load** — stress the write path; find where writes stop scaling (the read/write asymmetry) — [doc](docs/stages/04-WriteHeavyLoad.md) [results](docs/baseline/04-writeheavyload.md)
- [ ] **5. Async Processing** — move non-critical side effects off the request path via a queue — [doc](docs/stages/05-AsyncProcessing.md)
- [ ] **6. Data Aggregation** — derived read models updated from events (a taste of CQRS) — [doc](docs/stages/06-DataAggregation.md)
- [ ] **7. System Observability** — metrics, logs, traces; RED/USE; correlate the async paths — [doc](docs/stages/07-Observability.md)
- [ ] **8. Scaling & Infrastructure** — horizontal scaling; stateless-vs-stateful asymmetry — [doc](docs/stages/08-ScalingInfrastructure.md)

### Part B — Resilience & distributed behavior

- [ ] **9. Load & Stress Scenarios** — push the whole system to its limit; backpressure, degradation, the "death point" *(planned)*
- [ ] **10. Reliability & Failure Handling** — break parts on purpose; retries, timeouts, graceful degradation, recovery *(planned)*
- [ ] **11. Data Consistency & Tradeoffs** — eventual consistency as the norm, idempotency, reconciliation *(planned)*

### Part C — Fintech core (the part the role actually centers on)

- [ ] **12. Purchases & Idempotency** — purchase flow with idempotency keys, signed webhooks, a mocked payment provider *(planned)*
- [ ] **13. Double-entry Ledger** — balances derived from immutable entries; tackling hot-row contention (why not `UPDATE balance = balance - x`) *(planned)*
- [ ] **14. Payouts** — saga + transactional **outbox**; reconciliation against the provider *(planned)*

### Capstone

- [ ] **15. Production Readiness** — pull highload *and* money together: deploy strategy, mature monitoring, real-load readiness *(planned)*

### Later

- [ ] Split into separate services (catalog / analytics) — service boundaries, network calls
- [ ] Kubernetes, autoscaling on queue depth

---

## Benchmarks

The point of the project — recorded as I reach each step, on the same machine
for comparability.

| Stage | Scenario | RPS | p95 latency | Notes |
|-------|----------|-----|-------------|-------|
| 2 | `GET /apps` list (no cache, 150 VUs) | 1 425 /s | 92 ms | first number |
| 2 | `GET /apps/:id` get (no cache, 150 VUs) | 1 801 /s | 82 ms | PK lookup, fastest path |
| 2 | `POST /apps` create (50 VUs) | 612 /s | 132 ms | write tail; pool contention visible |
| 3 | `GET /apps/:id` with cache-aside (150 VUs) | 3 691 /s | 41 ms | +105% rps vs stage 2; 100-ID working set fits in cache |
| 3 | `GET /apps` list with cache-aside (150 VUs) | 2 136 /s | 60 ms | +50% rps; short TTL → stampede tail (max 572 ms) |
| 4 | catalog write path under sustained load | _TBD_ | _TBD_ | where writes stop scaling |
| 5 | side effects moved off the request path | _TBD_ | _TBD_ | API latency decoupled |

*(Numbers filled in as each stage lands.)*

---

## Getting started

See [`docs/development.md`](docs/development.md).

---

## Status & honesty note

This is a personal learning repo, built in evenings, to get hands-on with the
architecture behind an alternative iOS marketplace. It's incomplete by design —
I'm walking the roadmap above one rung at a time and recording what I find.
Payment and notarization providers are mocked. Feedback and pointers very
welcome.

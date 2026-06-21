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

Each highload step is only "done" once there's a measurement behind it.

---

## Roadmap

Checkboxes reflect actual progress, updated as I go.

### Part A — the marketplace core (read paths, async, observability)

- [ ] **0. Skeleton** — `axum` + Postgres + Docker Compose; `POST /apps`, `GET /apps`, `GET /apps/:id`
- [ ] **1. Baseline load test** — `k6` on `GET /apps`, record RPS / latency / CPU
- [ ] **2. Redis cache-aside** — re-run the test, record before/after
- [ ] **3. Downloads (sync)** — `POST /downloads` writing straight to Postgres
- [ ] **4. Observe the coupling** — load it; see how a synchronous write on the hot path ties API latency/availability to the DB *(the lesson is decoupling, not "Postgres can't do N inserts")*
- [ ] **5. NATS** — API publishes a download event, a worker persists it; API responds immediately
- [ ] **6. Analytics aggregation** — worker rolls up `daily_downloads`; `GET /stats` avoids heavy `COUNT` (a taste of CQRS)
- [ ] **7. Metrics** — Prometheus + Grafana, RED metrics (rate / errors / duration)
- [ ] **8. Tracing** — OpenTelemetry across `Request → axum → sqlx → NATS`

### Part B — the fintech core (the part the role actually centers on)

- [ ] **9. Purchases** — `POST /purchases` with an `Idempotency-Key`, mocked payment provider, signed webhook handler
- [ ] **10. Double-entry ledger** — balances derived from immutable entries; tackling **hot-row contention** (why not `UPDATE balance = balance - x`)
- [ ] **11. Payouts** — saga + transactional **outbox** *(contrast with step 5: for money you can't fire-and-forget — the state change and the event must be atomic)*

### Later

- [ ] Split into separate services (catalog / analytics) — service boundaries, network calls
- [ ] Kubernetes, autoscaling on queue depth

---

## Benchmarks

The point of the project — recorded as I reach each step, on the same machine
for comparability.

| Step | Scenario | RPS | p99 latency | Notes |
|------|----------|-----|-------------|-------|
| 1 | `GET /apps` baseline (Postgres only) | _TBD_ | _TBD_ | first number |
| 2 | `GET /apps` with Redis cache-aside | _TBD_ | _TBD_ | before/after |
| 4 | `POST /downloads` sync write | _TBD_ | _TBD_ | hot-path coupling |
| 5 | `POST /downloads` async via NATS | _TBD_ | _TBD_ | API decoupled |

*(Numbers filled in as each step lands.)*

---

## Repo layout

Starts deliberately small — a single service — and grows only when a step
demands it (the split into multiple services is itself Stage 10, not a day-one
decision).

```
apphub/
├── src/                # axum app: routes, handlers, state
├── migrations/         # sqlx migrations
├── load/               # k6 scenarios + recorded results
├── docs/
│   └── decisions.md    # why I chose X over Y, and the trade-off
├── docker-compose.yml  # postgres (+ redis, nats, observability as stages land)
└── README.md
```

---

## Running locally

```bash
docker compose up -d        # postgres (and friends as stages are added)
sqlx migrate run
cargo run
```

Load tests:

```bash
k6 run load/get_apps.js
```

*(Commands evolve with the stages; this reflects the current state.)*

---

## Status & honesty note

This is a personal learning repo, built in evenings, to get hands-on with the
architecture behind an alternative iOS marketplace. It's incomplete by design —
I'm walking the roadmap above one rung at a time and recording what I find.
Payment and notarization providers are mocked. Feedback and pointers very
welcome.

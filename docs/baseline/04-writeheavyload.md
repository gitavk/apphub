# Performance Baseline — Stage 4: Write-Heavy Load

Measured: 2026-06-22  
Server: `cargo run` (debug build), single process, connection pool size 10  
Cache: Valkey 8 in Docker  
DB: Postgres 18 in Docker, default config  
Tool: k6

---

## Results

### update_apps — PATCH /apps/:id (150 VUs, 100 shared rows)

| metric     | value      |
|------------|------------|
| rps        | 605 /s     |
| p50        | 16.70 ms   |
| p90        | 333 ms     |
| p95        | 405 ms     |
| max        | 591 ms     |
| error rate | 0.00 %     |
| threshold  | ✓ p95 < 500 ms |

### write_heavy — POST /apps (150 VUs, unique rows each time)

| metric     | value      |
|------------|------------|
| rps        | 595 /s     |
| p50        | 20.45 ms   |
| p90        | 366 ms     |
| p95        | 416 ms     |
| max        | 653 ms     |
| error rate | 0.00 %     |
| threshold  | ✓ p95 < 2000 ms |

### mixed — readers (100 VUs) + writers (30 VUs) simultaneously

| metric                | readers    | writers    |
|-----------------------|------------|------------|
| peak VUs              | 100        | 30         |
| rps (combined: 1 948) | ~1 680     | ~268       |
| p95                   | 63 ms      | 89 ms      |
| max                   | 2 600 ms   | 2 830 ms   |
| error rate            | 0.00 %     | 0.00 %     |
| threshold             | ✓ p95 < 200 ms | ✓ p95 < 1000 ms |

---

## Comparison against Stage 2 baseline

| scenario            | Stage 2 rps | Stage 4 rps | Stage 2 p95 | Stage 4 p95 | VUs       |
|---------------------|-------------|-------------|-------------|-------------|-----------|
| create (write_heavy)| 612 /s      | 595 /s      | 132 ms      | 416 ms      | 50 → 150  |
| update_apps         | —           | 605 /s      | —           | 405 ms      | — → 150   |
| list (mixed reader) | 1 425 /s    | ~1 680 /s   | 92 ms       | 63 ms       | 150 → 100 |

---

## Observations

**Throughput saturated at the connection pool.** `write_heavy` reached 595 rps
at 150 VUs — virtually identical to `create_app` at 50 VUs in Stage 2 (612 rps).
Tripling the VUs added zero throughput and tripled p95 from 132 ms to 416 ms.
The pool (10 connections) is the hard ceiling: beyond ~10 inflight writes, every
additional VU queues. This is the first write bottleneck, and it is not in
Postgres — it is in the pool configuration.

**Row-level lock contention is the second bottleneck.** `update_apps` hammers
the same 100 rows from 150 VUs simultaneously. The p50 is 17 ms (fast lock
acquisition), but the p95 is 405 ms — a 24× spread. When multiple writers race
for the same row, they form a queue behind the row lock. Each waiter inherits
the latency of every writer ahead of it. This is qualitatively different from
`write_heavy` where each request targets a fresh row: with unique rows there is
no row-lock contention, only pool contention.

**Reads are almost completely insulated from write pressure.** In `mixed`, the
list_apps reader p95 was 63 ms — nearly identical to Stage 3 standalone (60 ms),
even with 30 concurrent writers active. This is the direct payoff of caching:
reads go to Valkey and barely touch the connection pool, so write pressure does
not bleed into read latency. Without cache this would not hold.

**The max latency tail (2.6–2.8 s) in mixed is a warning.** Although p95 is
fine, occasional requests spiked past two seconds. This happens when a cache
miss (list page TTL expires) coincides with high write load — the request joins
the pool queue behind a batch of writers. In production this would occasionally
produce a slow user experience even though median and p95 look healthy.

**Why writes don't scale the way reads did.** Reads scaled with caching because
a cached response requires no DB connection — the working set can be served from
Valkey at memory speed, and adding VUs just adds Valkey round-trips. Writes
cannot be copied away: every write must land on the primary, acquire a lock,
flush WAL, and return. The only way to raise write throughput is to either
reduce per-write latency (batching, async WAL), increase the pool size, or move
work off the synchronous request path — which is exactly what Stage 5 does.

---

## The bottleneck to address in Stage 5

The connection pool (size 10) is saturated at ~600 rps for writes, long before
Postgres itself is stressed. Moving non-critical write side-effects (analytics
events, audit records) off the synchronous request path via a queue decouples the
caller from the DB write. The immediate `POST /apps` path becomes lighter; the
heavy work happens asynchronously. That is the core of Stage 5.

---

## Reproducibility

```bash
make load-write     # update_apps → write_heavy → mixed
make load-baseline  # re-run read scenarios for comparison
```

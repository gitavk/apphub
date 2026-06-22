# Performance Baseline — Stage 3: Caching Layer

Measured: 2026-06-22  
Server: `cargo run` (debug build), single process  
Cache: Valkey 8 (Redis-compatible) in Docker, default config  
DB: Postgres 18 in Docker, default config, pool size 10  
Seed data: ~1 010 rows in `apps` at test start  
Tool: k6, same load shapes as Stage 2

---

## Before / After

### list_apps — GET /apps?page=N&per_page=20 (pages 1–50, random)

| metric     | Stage 2 (no cache) | Stage 3 (cache TTL 30 s) | delta     |
|------------|-------------------|--------------------------|-----------|
| rps        | 1 425 /s          | 2 136 /s                 | **+50%**  |
| p50        | 29.66 ms          | 19.52 ms                 | −34%      |
| p90        | 84.95 ms          | 55.30 ms                 | −35%      |
| p95        | 92.16 ms          | 60.03 ms                 | **−35%**  |
| max        | 286 ms            | 572 ms                   | +100% ⚠   |
| error rate | 0.00 %            | 0.00 %                   | —         |

### get_app — GET /apps/:id (100 IDs sampled in setup)

| metric     | Stage 2 (no cache) | Stage 3 (cache TTL 300 s) | delta      |
|------------|-------------------|---------------------------|------------|
| rps        | 1 801 /s          | 3 691 /s                  | **+105%**  |
| p50        | 22.03 ms          | 9.98 ms                   | −55%       |
| p90        | 72.11 ms          | 34.01 ms                  | −53%       |
| p95        | 81.74 ms          | 41.06 ms                  | **−50%**   |
| max        | 179 ms            | 125 ms                    | −30%       |
| error rate | 0.00 %            | 0.00 %                    | —          |

### create_app — POST /apps (write path, no cache)

| metric     | Stage 2 (no cache) | Stage 3 (unchanged) | delta |
|------------|-------------------|---------------------|-------|
| rps        | 612 /s            | 596 /s              | −3%   |
| p50        | 5.84 ms           | 5.86 ms             | ≈ 0   |
| p95        | 131 ms            | 135 ms              | ≈ 0   |
| error rate | 0.00 %            | 0.00 %              | —     |

The −3% on create_app is noise; the write path was intentionally left untouched.

---

## Observations

**get_app gained the most (+105% rps, −50% p95).** The scenario fetches 100
IDs once in `setup()` and hammers those same 100 keys for the full 2m50s. With
a 300s TTL the entire working set fits in cache after the first 100 requests
and never evicts during the run. Every subsequent request is a Valkey round-trip
with no Postgres involvement — the p50 drop from 22ms to 10ms reflects exactly
that: one local network hop instead of a DB query + serialization.

**list_apps gained less (+50% rps, −35% p95).** The scenario distributes
requests across 50 random pages. Each page is a separate cache key with a 30s
TTL, so pages expire roughly twice per minute under load. After each expiry the
first request to that page hits Postgres again, which is why the **max latency
doubled (286ms → 572ms)**: a cache miss under 150 concurrent VUs sees more DB
contention than a cold run without cache, because the rest of the traffic that
was previously spread across the DB is now hitting Valkey instead — concentrating
the DB load onto the few misses. This is a small-scale manifestation of the
**cache stampede**: multiple VUs can race to refill the same expired key at once.

**create_app is unchanged and is still the bottleneck.** At 50 VUs it saturates
the write path (596 rps, p95=135ms) before either read scenario shows any strain.
The connection pool and synchronous WAL flush remain the limiting factor for
writes — caching does not address this, and Stage 4 is where it gets examined.

**The max latency increase on list_apps is a warning sign.** The p95 improved,
but the tail (max) got worse. In production this matters for the slowest users.
A mitigation would be probabilistic early expiry (refresh the cache slightly
before TTL expires, rather than letting all VUs race on the miss) or a longer
TTL with explicit invalidation on write.

---

## Reproducibility

```bash
make cache-flush     # clear Valkey state before each run
make load-baseline   # runs all three scenarios in sequence
```

# Performance Baseline — Stage 2

Measured: 2026-06-22  
Server: `cargo run` (debug build), single process  
DB: Postgres 18 in Docker, default config, pool size 10  
Seed data: 1 010 rows in `apps`  
Tool: k6, run from localhost against `http://localhost:3000`

---

## Scenarios

All three scenarios use a four-stage ramp:

| Stage      | Duration | list/get VUs | create VUs |
|------------|----------|-------------|------------|
| Light      | 30 s     | → 10        | → 5        |
| Moderate   | 60 s     | → 50        | → 20       |
| Stress     | 60 s     | → 150       | → 50       |
| Ramp-down  | 20 s     | → 0         | → 0        |

---

## list_apps — GET /apps?page=N&per_page=20

| metric        | value      |
|---------------|------------|
| VUs (peak)    | 150        |
| rps           | 1 425 /s   |
| p50           | 29.66 ms   |
| p90           | 84.95 ms   |
| p95           | 92.16 ms   |
| max           | 286 ms     |
| error rate    | 0.00 %     |
| iterations    | 242 351    |
| threshold     | ✓ p95 < 1000 ms |

---

## get_app — GET /apps/:id

IDs fetched via `setup()` from `GET /apps?per_page=100` before the run.

| metric        | value      |
|---------------|------------|
| VUs (peak)    | 150        |
| rps           | 1 801 /s   |
| p50           | 22.03 ms   |
| p90           | 72.11 ms   |
| p95           | 81.74 ms   |
| max           | 179 ms     |
| error rate    | 0.00 %     |
| iterations    | 306 273    |
| threshold     | ✓ p95 < 500 ms |

---

## create_app — POST /apps

Each iteration inserts a unique `bundle_id` (`com.load.<VU>.<ITER>.<counter>`).

| metric        | value      |
|---------------|------------|
| VUs (peak)    | 50         |
| rps           | 612 /s     |
| p50           | 5.84 ms    |
| p90           | 109 ms     |
| p95           | 131 ms     |
| max           | 315 ms     |
| error rate    | 0.00 %     |
| iterations    | 104 129    |
| threshold     | ✓ p95 < 1000 ms |

---

## First observations

**Reads are fast and stable.** Both `get_app` and `list_apps` clear their thresholds with headroom — p95 of 82 ms and 92 ms respectively against ceilings of 500 ms and 1 000 ms. `get_app` is the fastest scenario (primary-key lookup vs. paginated scan + serialisation of 20 rows).

**Writes show high latency variance.** `create_app` has a p50 of only 6 ms but a p95 of 132 ms — a 22× spread. The tail rises even though peak VUs are capped at 50 (a third of the read scenarios). This points to write-path contention: each INSERT must acquire a row lock, flush WAL, and return the new UUID, all while competing with concurrent writers for the connection pool.

**create_app is the first limiting factor.** At 50 VUs it already saturates the pool more than reads do at 150 VUs. Raising concurrency further on writes would push p95 past 1 000 ms before the read scenarios show any degradation. The connection pool (size 10) and synchronous WAL flush are the likely bottleneck.

**No errors across all three scenarios.** Zero 4xx/5xx responses and zero timeouts over ~650 000 total requests. The service is correct under load; the opportunity is purely latency and write throughput.

---

## Reproducibility

```bash
make seed-load      # load 1 000 rows (idempotent if truncated first)
make load-baseline  # runs all three scenarios in sequence
```

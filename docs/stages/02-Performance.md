# Stage 2 — Performance Baseline

**Status:** planned · **Effort:** 1–2 evenings · **Depends on:** Stage 1 (Catalog) · **Unlocks:** Stage 3 (Caching)

## Goal

Establish a measurable performance baseline for the App Catalog. At this stage
the system is **not** being improved — it is being observed. No functional
changes are expected; the point is to understand how the current implementation
behaves under load and to define the reference metrics every later optimization
will be judged against.

> You cannot optimize what you have not measured.

This baseline is the anchor for the whole high-load story — without a
trustworthy *before*, every later *after* is just a claim.

---

## Learning Objectives

Become comfortable with:

* how a backend behaves under concurrent load
* reading latency as a distribution (p50 / p95 / p99), not as an average
* how throughput stops scaling as traffic increases
* where bottlenecks tend to appear first
* why "it works locally" says nothing about performance

---

## What We Measure

Three scenarios that reflect real catalog usage:

* listing applications — the read-heavy operation
* fetching a single application — a lookup
* creating an application — a write, at lower frequency

Each is run at increasing load — light, moderate, stress — watching for
**degradation patterns** rather than chasing exact numbers. For each, record:

* throughput (requests per second)
* the latency distribution — median through the tail (p95 / p99) and the worst case, not the average
* error rate, if any appears
* resource saturation on both the service and the database (CPU, connections)

The environment is fixed and documented so results stay comparable across
stages. Averages are de-emphasized on purpose — they hide exactly the tail
behavior that matters under load.

---

## Deliverables

### Benchmarking

* [ ] Defined scenarios (read / lookup / write)
* [ ] Repeatable load simulation at increasing levels
* [ ] Recorded results (throughput, latency percentiles, errors, resource use)

### Analysis & Reproducibility

* [ ] A short baseline report — scenarios, observed metrics, behavior under load, first conclusions
* [ ] Reproducibility notes — how the tests were run, so they can be repeated later
* [ ] The first bottleneck named and explained

---

## Success Criteria

Stage 2 is complete when:

1. Baseline performance is measured across the three scenarios.
2. Behavior under increasing load is documented — where latency rises and where throughput stops scaling.
3. Results are recorded as percentiles, not averages.
4. The measurements can be reliably reproduced.
5. The first limiting factor is identified and explained.

---

## Exit Criteria

Before moving to Stage 3:

* the baseline is reproducible and trustworthy
* the report and method are documented
* the main bottleneck is identified

After this stage you can answer three questions: how does the system behave
under load today, where does it degrade first, and what is the *before
optimization* baseline? Stage 3 introduces caching and is judged against it:
**did it actually move the numbers, and at what cost?**

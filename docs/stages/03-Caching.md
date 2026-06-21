# Stage 3 — Caching Layer

**Status:** planned · **Effort:** 2–3 evenings · **Depends on:** Stage 2 (Performance Baseline) · **Unlocks:** Stage 4 (Write-heavy Load)

## Goal

Introduce a caching layer to speed up the read-heavy parts of the catalog. The
catalog is a natural fit: its data changes far less often than it is read, so
the same results are served over and over. This stage is about reducing pressure
on the primary store and improving latency under concurrent traffic.

The scope is deliberately narrow — **read optimization only**. Writes still go
straight to the primary store and are not touched here.

---

## Learning Objectives

Become comfortable with:

* why repeated reads become expensive at scale
* the role caching plays in a high-load system
* the trade-off between freshness and performance
* the new costs caching introduces — staleness, an extra failure mode, and the risk of a stampede when a hot entry expires
* how a cache changes the shape of the architecture

---

## Approach

The pattern is **cache-aside**: a read checks the cache first; on a miss it loads
from the primary store and populates the cache, so the next identical read is
served without touching the database.

Three decisions drive the stage:

* **What to cache** — chosen from the Stage 2 results: the hottest and slowest read paths, not everything.
* **How to invalidate** — a time-to-live, an explicit invalidation when the underlying data changes, or both. This is where the freshness-vs-performance trade-off actually lives.
* **What to watch** — behavior when a popular entry expires and many requests reach for the store at once.

The win is only real if it shows up against the Stage 2 baseline — measured, not
assumed.

---

## Deliverables

### Caching

* [ ] Cacheable read paths identified from the Stage 2 results
* [ ] Cache-aside serving for at least the hottest read endpoint(s)
* [ ] A defined invalidation strategy (TTL and/or on-write)

### Validation

* [ ] The same Stage 2 scenarios re-run with caching in place
* [ ] A before/after comparison — latency, throughput, load on the primary store
* [ ] Observations: where caching helped, where it didn't, and the trade-offs seen

---

## Success Criteria

Stage 3 is complete when:

1. At least the hottest read path is served via cache-aside.
2. The improvement is measurable against the Stage 2 baseline, on the same scenarios.
3. Cache behavior under repeated load is understood.
4. Invalidation is defined and consistent — cached data does not go permanently stale.
5. A clear cached-vs-uncached comparison is recorded.

---

## Exit Criteria

Before moving to Stage 4:

* the improvement is validated against Stage 2, not assumed
* the invalidation strategy is documented and behaves consistently
* the trade-offs introduced (freshness, the new failure mode) are noted

After this stage you can answer: which parts of the system benefit most from
caching, how much of the gain comes from taking pressure off the database, and
what you gave up to get it. Stage 4 turns to the opposite problem — a
**write-heavy** path, where caching no longer saves you.

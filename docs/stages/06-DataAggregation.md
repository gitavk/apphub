# Stage 6 — Data Aggregation

**Status:** planned · **Effort:** 2–4 evenings · **Depends on:** Stage 5 (Async Processing) · **Unlocks:** Stage 7 (System Observability)

## Goal

Put the events from Stage 5 to use: turn raw system activity into structured,
query-ready views. Instead of computing summaries on demand — an expensive
aggregate over raw data every time it's asked for — the system maintains
**precomputed derived views** that are updated as events arrive.

The scope is read optimization for analytical and summary use. These views are
**derived from processed events, not from direct user requests**, and they live
off the operational hot path.

---

## Learning Objectives

Become comfortable with:

* why raw event data alone is not enough for fast analytics
* how a precomputed read model turns an expensive aggregate into a cheap lookup
* what a derived data model is, and how it stays in sync with its source
* separating operational workloads (serving requests) from analytical ones (summaries, trends)
* read models as the query side of **CQRS** — the same events feed a separate, read-optimized view

---

## Approach

Three moves:

* **Decide what to precompute** — from the Stage 5 events, find the summaries that are read often and expensive to compute on demand (counts and trends per application, activity over time).
* **Maintain a read model** — a separate, query-optimized representation, updated **incrementally** as events arrive and kept off the operational store's hot path.
* **Serve cheap queries** — analytical endpoints return the precomputed result, turning an expensive aggregate (a large `COUNT`, say) into something close to a point read.

The costs to respect, and they tie directly back to Stage 5:

* the read model is **eventually consistent** — it lags the events slightly
* updates must be **idempotent** — at-least-once delivery means a replayed event must not double-count
* the view should be **rebuildable** from the source of truth, so it can recover if it drifts or the aggregation logic changes

---

## Deliverables

### Aggregation

* [ ] Aggregation needs identified from the Stage 5 events (frequent, expensive-to-compute summaries)
* [ ] At least one derived read model, updated incrementally from events
* [ ] Analytical endpoint(s) serving precomputed results, off the operational hot path

### Design & Validation

* [ ] An aggregation design note — what is aggregated, how the view is structured, how it updates from events
* [ ] A comparison: precomputed query vs the same result computed on demand
* [ ] Idempotent updates verified — a replayed event does not corrupt the aggregate

---

## Success Criteria

Stage 6 is complete when:

1. At least one derived read model exists and updates from system events.
2. Queries against it are faster than computing the same result from raw data on demand.
3. Updates are idempotent — replayed events do not double-count.
4. Analytical endpoints are usable and stay off the operational hot path.
5. The separation between raw events and derived state is clear.

---

## Exit Criteria

Before moving to Stage 7:

* the read model is rebuildable from the source of truth
* the eventual-consistency lag is understood and acceptable for the use case
* the precomputed-vs-on-demand difference is documented

After this stage you can answer: why systems keep precomputed views instead of
computing on demand, how that separation helps scalability, and what
distinguishes operational from analytical data. The system now maintains derived
state — and **Stage 7 turns that same instinct inward**, making the system's own
behavior observable through metrics and monitoring.

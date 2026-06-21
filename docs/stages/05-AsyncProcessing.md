# Stage 5 — Async Processing

**Status:** planned · **Effort:** 2–4 evenings · **Depends on:** Stage 4 (Write-Heavy Load) · **Unlocks:** Stage 6 (Data Aggregation)

## Goal

Take the write pressure exposed in Stage 4 and relieve it by moving work **off
the request path**. The idea is to decouple the user-facing write from its
secondary side effects: the request does only the essential work and returns
immediately, while the rest is handed off to be processed in the background.

The scope is deliberate. **Primary operations stay synchronous and immediately
consistent** — only non-critical side effects move to asynchronous execution.
This stage is where the system first becomes event-driven.

---

## Learning Objectives

Become comfortable with:

* why a synchronous request path becomes the bottleneck under write load
* queue-based load leveling — using a queue to absorb bursts so the API stays responsive
* eventual consistency, and the immediacy trade-off it introduces
* delivery semantics — at-least-once means a consumer must be safe to run twice (idempotent)
* backpressure — what happens when the background worker can't keep up

---

## Approach

Three moves:

* **Identify the deferrable work** — review the Stage 4 write paths and find side effects that aren't needed for the user's response and can tolerate a slight delay.
* **Split the path** — the request performs the essential write and returns; the secondary work is published to a queue rather than done inline.
* **Process in the background** — a worker consumes the queue independently of requests, runs continuously, and never blocks API responsiveness.

This is **queue-based load leveling**: the queue absorbs spikes so the API stays
fast even when the secondary work is slow. What it costs: the side effect lands a
moment later (eventual consistency), the same event may arrive more than once
(so the worker must be **idempotent**), and the backlog can grow if the worker
falls behind (something to watch).

> One deliberate boundary: this simple hand-off is safe **because the deferred
> work is non-critical** — an occasional duplicate or delay is tolerable. When
> money is involved in later stages, this isn't enough: the state change and the
> event must be made atomic. That stronger guarantee (a transactional outbox) is
> introduced when the ledger arrives, not here.

---

## Deliverables

### Async flow

* [ ] Deferrable side effects identified from the Stage 4 write paths
* [ ] At least one write path split — essential write synchronous, secondary work enqueued
* [ ] A background worker processing deferred tasks independently, with idempotent handling

### Validation

* [ ] Before/after comparison against Stage 4 — API latency and stability under write load
* [ ] Backlog growth and processing delay observed under spikes
* [ ] Notes on the consistency trade-off introduced

---

## Success Criteria

Stage 5 is complete when:

1. At least one write flow defers its secondary work to async processing.
2. API responsiveness under write load is improved or stabilized versus Stage 4.
3. Background processing runs independently of request handling and is safe to retry.
4. Behavior under spikes is measured — backlog growth and processing delay.
5. The trade-off between immediacy and eventual consistency is understood.

---

## Exit Criteria

Before moving to Stage 6:

* the async path is validated against Stage 4, not assumed
* backlog and backpressure behavior is observed and documented
* the consistency trade-off — and where this simple hand-off would *not* be safe — is noted

After this stage you can answer: why high-load systems push work to the
background, what decoupling side effects from requests buys you, and what it
costs in consistency. The system is now event-driven — and **Stage 6 puts those
events to use**, aggregating them so expensive queries don't run on the hot path.

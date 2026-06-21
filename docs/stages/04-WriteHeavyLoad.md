# Stage 4 — Write-Heavy Load

**Status:** planned · **Effort:** 2–3 evenings · **Depends on:** Stage 3 (Caching Layer) · **Unlocks:** Stage 5 (Async Processing)

## Goal

Shift focus from read optimization to **write pressure**. Where Stage 2 measured
how the catalog behaves under reads, this stage does the same for writes: observe
how the system holds up under continuous creation and update load, and where it
starts to strain.

The scope stays within the catalog domain — creating applications, updating their
metadata, and mixed read+write traffic. The caching from Stage 3 stays in place
but is no longer the focus; the point here is that **caching does nothing for
writes**, and that's exactly what makes this stage interesting.

---

## Learning Objectives

Become comfortable with:

* why writes are inherently more expensive than reads — durability, index maintenance, contention
* why writes can't be offloaded the way reads can — a cache or a replica serves a copy, but a write has to land on the primary
* what happens as write volume grows, and what write amplification means
* how writes interact with reads under concurrent load
* the difference between a read-scaling problem and a write-scaling problem

---

## What We Measure

This is the write-side twin of Stage 2. Define scenarios where:

* application creation happens continuously
* updates to existing records are frequent and concurrent
* reads still happen, but no longer dominate

Apply **sustained** load at increasing levels and record:

* write latency and throughput, and where throughput stops scaling
* the effect of write pressure on read performance
* resource saturation on the primary store (CPU, connections, locks)
* how behavior changes over time, not just at a single instant

The honest framing matters: the goal is not to "watch the database die" at some
low number, but to find **where write capacity is bounded and what couples to
it** — because that boundary is what the next stage is designed to move.

---

## Deliverables

### Benchmarking

* [ ] Defined write-heavy scenarios (continuous creation, updates, mixed read+write)
* [ ] Sustained write load applied at increasing levels
* [ ] Recorded results — write latency/throughput, effect on reads, behavior over time

### Analysis

* [ ] A write-load report comparing against the read-heavy baseline
* [ ] The first write bottleneck named and explained
* [ ] The key insight captured: why writes don't scale the way reads did

---

## Success Criteria

Stage 4 is complete when:

1. Write-heavy scenarios run reliably under sustained load.
2. Write latency and throughput limits are measured.
3. The effect of writes on read performance is observed.
4. The first limiting factor under write pressure is identified.
5. Observations are documented and reproducible.

---

## Exit Criteria

Before moving to Stage 5:

* the write bottleneck is identified and explained
* the interaction between reads and writes is understood
* it is clear why the next step is decoupling, not more caching

After this stage you can answer: what happens when writes dominate, how write
pressure bleeds into read performance, and where the system degrades first under
sustained mutation. Reads scaled because they could be served from copies — writes
can't be copied away, so **Stage 5 moves the work off the request path** so the
write path stops being the limiter.

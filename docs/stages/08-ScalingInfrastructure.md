# Stage 8 — Scaling & Infrastructure

**Status:** planned · **Effort:** 3–5 evenings · **Depends on:** Stage 7 (System Observability) · **Unlocks:** Stage 9 (Load & Failure Scenarios)

## Goal

Move the system from a single machine to a distributed, production-like
deployment, and understand how its behavior changes when components no longer
share one local setup. The work is **scaling what already exists**, not adding
functionality — the behavior stays the same, the deployment model changes.

The whole stage is driven by Stage 7: scale the parts the signals show are
actually constrained, not the parts that feel slow.

---

## Learning Objectives

Become comfortable with:

* what changes when a system runs across multiple instances
* why stateless services scale horizontally and stateful ones don't
* why the database tends to become the first scaling bottleneck
* the new costs of being distributed — network calls, partial failure, coordination
* the relationship between deployment topology and system behavior

---

## Approach

* **Scale the stateless part first.** Run multiple API instances behind a load balancer. This works *because* the API holds no local state — state lives in the shared store, cache, and queue — so adding instances raises request capacity nearly linearly.
* **Separate the components** so each can scale on its own: request-serving, the data store, and background processing as independent units.
* **Hit the asymmetry.** The stateless API scales out; the shared, stateful database does not — the same wall as Stage 4. It becomes the first ceiling, and more API instances can even make it *worse*, since each adds connections against a fixed limit (a case for connection multiplexing). Reads can lean on replicas; writes stay bounded by the primary.
* **Mind the new costs.** Network calls and partial failures between components, coordination overhead, and multiple workers competing for the same queue — which only stays correct because consumers are idempotent (Stage 5 again).
* **Measure it.** Re-run the load scenarios and compare the multi-node setup against the single-instance baseline.

---

## Deliverables

### Scaling

* [ ] API running as multiple instances behind a load balancer
* [ ] Components separated into independently runnable units (request-serving, store, background)
* [ ] Per-component scaling limits identified, guided by the Stage 7 signals

### Analysis

* [ ] A scaling report — what scales cleanly, what becomes the bottleneck, and the new distributed costs
* [ ] A deployment overview — how components deploy, how traffic flows, how responsibilities split
* [ ] A multi-node vs single-node comparison on the load scenarios (latency, throughput, failure behavior)

---

## Success Criteria

Stage 8 is complete when:

1. The API runs across multiple instances under a distributed deployment.
2. System behavior is evaluated multi-node versus single-node on the same scenarios.
3. Each component's scaling characteristics are understood.
4. The bottleneck in the distributed setup is identified, with the reason why.
5. The differences and new costs of the distributed setup are documented.

---

## Exit Criteria

Before moving to Stage 9:

* the stateless-vs-stateful scaling asymmetry is understood and documented
* the distributed bottleneck is identified from the signals, not guessed
* the new failure modes introduced by distribution are noted

After this stage you can answer what "scaling horizontally" actually does, which
parts scale well and which don't, and where a distributed architecture starts
introducing *new* bottlenecks. The system is now distributed and
production-like — and **Stage 9 stresses it on purpose**, breaking parts to see
how it fails and recovers.

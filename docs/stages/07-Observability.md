# Stage 7 — System Observability

**Status:** planned · **Effort:** 2–4 evenings · **Depends on:** Stage 6 (Data Aggregation) · **Unlocks:** Stage 8 (Scaling & Infrastructure)

## Goal

Make the system **measurable from the inside**. Up to now it has been observed
from the outside, through load tests. This stage turns that inward: the system
should expose enough about its own behavior that you can tell what's happening
under load without guessing.

The scope is introspection, not new business logic. The point is to make the
system's behavior visible, interpretable, and traceable — especially across the
asynchronous paths from Stages 5–6, where work now happens out of band and is
harder to see.

---

## Learning Objectives

Become comfortable with:

* observing a system under production-like load rather than testing it from outside
* the three complementary signals — metrics, logs, and traces — and what each is for
* telling a **symptom** apart from a **root cause**
* following a single action end-to-end across asynchronous components
* reasoning about system health continuously from signals, not by manual inspection

---

## Approach

**The three signals, each answering a different question:**

* **Metrics** — aggregate trends over time; the basis for alerting (is latency rising, is the error rate climbing).
* **Logs** — discrete, contextual events; the detail of *what* happened.
* **Traces** — the path of a single request across components; *where* the time went.

**Apply the methods.** Use RED — rate, errors, duration — for the
request-serving paths, and watch resource saturation (utilization, saturation,
errors) on the store, the connection pool, and the queue. RED shows the
*symptom* (latency up); saturation points at the *root cause* (which resource is
maxed out). That contrast is the whole skill.

**Correlate across the async paths.** A single user action now fans out —
request → event → background processing → aggregation update. A shared
correlation/trace identifier is what lets you follow it end-to-end. The
event-driven system is harder to observe precisely because work happens out of
band, so this matters more here than anywhere earlier.

**Watch it under stress.** Re-run the load scenarios and observe how components
degrade *independently*, where slowdowns or errors appear first, and how they
propagate.

---

## Deliverables

### Signals

* [ ] Key signals defined across the three pillars — metrics, logs, traces
* [ ] RED metrics for request-serving paths; resource saturation for the store, pool, and queue
* [ ] Correlated end-to-end visibility across the async paths (a shared trace/correlation id)

### Analysis

* [ ] An observability report — signals identified, how behavior is interpreted, failure patterns under load
* [ ] A system behavior map — how a request flows, how events propagate, how derived data updates
* [ ] Under load: where the system degrades first and how errors propagate, documented

---

## Success Criteria

Stage 7 is complete when:

1. System behavior is observable during execution across metrics, logs, and traces.
2. Failure and degradation patterns are identifiable under load.
3. A single action can be followed end-to-end across the asynchronous paths.
4. Differences in how components degrade under pressure are visible.
5. System health can be reasoned about from the signals, not guessed at.

---

## Exit Criteria

Before moving to Stage 8:

* a symptom can be traced to a root cause from the signals (RED symptom → saturated resource)
* the asynchronous paths are traceable end-to-end
* failure and degradation patterns under load are documented

After this stage you can answer: what is happening inside the system right now
under load, where it is slowing down or failing, and how its parts interact under
pressure. This completes the move from a functional system to a **measurable
distributed system** — and **Stage 8 acts on that visibility**, scaling the parts
the signals show are actually constrained.

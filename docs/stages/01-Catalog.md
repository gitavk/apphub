# Stage 1 — App Catalog

**Status:** planned · **Effort:** 2–3 evenings · **Depends on:** Stage 0 (Foundation) · **Unlocks:** Stage 2 (Baseline Benchmarking)

## Goal

Introduce the first business domain: the application catalog — the read-heavy
core of the marketplace. Users browse applications far more often than they
download or buy them, which is exactly why the catalog is the natural place to
start the performance story that later stages build on.

This stage is about building that domain **cleanly**: a minimal domain model,
a clear separation between the HTTP, domain, and storage layers, and an API that
can create, store, and query applications. It deliberately stops at a working,
well-structured catalog — measuring how fast it is, making it faster, and
refining how listings behave under load belong to the stages that follow.

---

## Learning Objectives

Become comfortable with:

* designing a small, honest domain model
* isolating data access from the transport layer (repository-style boundaries)
* request validation and consistent error handling
* mapping cleanly between the HTTP, domain, and storage layers
* pagination fundamentals

---

## Domain

An **Application** represents a published app in the marketplace. Its initial
shape is intentionally minimal — identity, name, a unique bundle identifier,
developer, description, and creation time. The unique bundle identifier is the
one real domain rule worth enforcing early.

Everything else a real catalog eventually needs — versions, uploads, reviews,
categories, ownership — is deferred to later stages. The model should be
realistic enough to evolve, not complete.

---

## Deliverables

### Database

* [ ] Catalog schema and migration
* [ ] Indexes supporting the main lookup and listing patterns

### API

* [ ] Create application
* [ ] List applications (paginated)
* [ ] Get application by id
* [ ] Request validation
* [ ] Consistent error responses

### Structure

* [ ] Domain model
* [ ] Repository layer isolating data access from the transport
* [ ] HTTP handlers

### Developer Experience

* [ ] Documented example requests
* [ ] Seed data for local testing

---

## Success Criteria

Stage 1 is complete when:

1. Applications can be created through the API.
2. Applications persist across restarts.
3. Applications can be listed through a paginated endpoint.
4. Applications can be fetched individually.
5. Invalid requests return meaningful errors.
6. The API can be exercised entirely from documented examples.

---

## Exit Criteria

Before moving to Stage 2:

* catalog functionality is stable
* schema and API contracts are documented
* local seed data exists
* the structure feels comfortable to extend, with no refactoring blocking future work

Stage 2 establishes a performance baseline and answers one simple question:
**how fast is the catalog before any optimization is introduced?**

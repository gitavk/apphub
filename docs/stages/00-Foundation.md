# Stage 0 — Foundation

**Status:** in progress · **Effort:** 1–2 evenings · **Depends on:** — · **Unlocks:** Stage 1 (App Catalog)

## Goal

Establish a stable, production-minded foundation for AppHub. The project itself
is exploratory, but the foundation should be built with the habits a real
service needs, so later high-load and architectural experiments can build on it
without rework.

The aim of this stage is **not** marketplace functionality, but a development
environment, project structure, and local infrastructure that everything else
will sit on. By the end, the project runs locally with a single command and
exposes a working HTTP service connected to a database. Anything beyond *"the
service starts, connects, and reports healthy"* belongs to a later stage.

---

## Learning Objectives

Get comfortable with the foundational concerns every service shares, regardless
of features:

* structuring a backend project cleanly
* running the service and its database together in containers
* connecting to the database through a managed connection pool
* versioned database migrations
* configuration through the environment
* structured, readable logging
* a smooth one-command local workflow

---

## Deliverables

### Infrastructure

* [ ] Container orchestration for local development
* [ ] Database service
* [ ] Application service
* [ ] Environment-based configuration

### Application

* [ ] Project scaffold initialized
* [ ] HTTP server running
* [ ] Health endpoint
* [ ] Structured logging
* [ ] Configuration loading

### Database

* [ ] Migration system in place, verified with an initial migration
* [ ] Database connection pool established

### Developer Experience

* [ ] README with local setup instructions
* [ ] Single-command task runner (Makefile or equivalent)
* [ ] Local development workflow documented

---

## Success Criteria

Stage 0 is complete when:

1. A single command starts all required services.
2. The application connects to the database successfully.
3. The health endpoint returns HTTP 200.
4. Migrations can be applied reliably.
5. Logs are visible and readable.
6. A new developer can run the project using only the README.

---

## Exit Criteria

Before moving to Stage 1:

* the foundation is stable and reproducible from a clean checkout
* the local development workflow is documented
* migrations run reliably
* the repository structure is settled
* bootstrap-phase rough edges are cleaned up

Stage 1 introduces the first business domain: the **App Catalog**.

# Development Guide

## Prerequisites

- [Rust](https://rustup.rs) (stable)
- [Docker](https://docs.docker.com/get-docker/) with Compose plugin
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
- [k6](https://k6.io/docs/get-started/installation/) — load testing (required for `make load*` targets)

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

## First-time setup

```bash
cp .env.example .env
```

## Running locally

Start everything in one command:

```bash
make dev
```

This starts the Postgres container, waits for it to be healthy, applies migrations, and runs the server on `localhost:3000`.

## Day-to-day workflow

| Command        | What it does                              |
|----------------|-------------------------------------------|
| `make dev`     | Start infra + migrate + run server        |
| `make down`    | Stop Docker infrastructure                |
| `make migrate` | Apply pending migrations                  |
| `make build`   | Build debug binary                        |
| `make check`   | `cargo check` + Clippy                    |
| `make fmt`     | Format code                               |
| `make test`    | Run tests                                 |
| `make ci`      | Full check: fmt + clippy + tests          |
| `make clean`   | Remove build artifacts                    |
| `make seed-load` | Insert 1 000 load-test rows into DB     |
| `make load SCENARIO=<name>` | Run a single k6 scenario   |
| `make load-baseline` | Run all three scenarios back-to-back |

## Verify the service is up

```bash
curl http://localhost:3000/health
# {"status":"ok","db_time":"2026-06-21T18:00:00+00:00"}
```

## Environment variables

| Variable       | Default                                          | Description          |
|----------------|--------------------------------------------------|----------------------|
| `DATABASE_URL` | —                                                | Postgres connection string (required) |
| `PORT`         | `3000`                                           | HTTP port            |
| `RUST_LOG`     | `info`                                           | Log level filter     |

## Project layout

```
src/
  main.rs       — server entry point: logging, config, pool, router
  config.rs     — Config struct loaded from environment
  db.rs         — connection pool setup
migrations/     — sqlx migration files
docs/           — stage docs and this guide
compose.yml     — local infrastructure (Postgres; Redis + NATS added in later stages)
Makefile        — task runner
```

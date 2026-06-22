# ==================================================================================== #
# HELPERS
# ==================================================================================== #

## help: print this help message
.PHONY: help
help:
	@echo 'Usage:'
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' | sed -e 's/^/ /'

# ==================================================================================== #
# DEVELOPMENT
# ==================================================================================== #

## dev: start infra, migrate, and run the server
.PHONY: dev
dev:
	@docker compose up -d
	@echo 'Waiting for postgres...'
	@until docker compose exec -T postgres pg_isready -U apphub -d apphub > /dev/null 2>&1; do sleep 1; done
	@sqlx migrate run
	@cargo run

## build: build debug binary
.PHONY: build
build:
	@cargo build

## check: cargo check + clippy
.PHONY: check
check:
	@cargo check && cargo clippy

## fmt: format code
.PHONY: fmt
fmt:
	@cargo fmt

## test: run tests
.PHONY: test
test:
	@cargo test

# ==================================================================================== #
# INFRASTRUCTURE
# ==================================================================================== #

## down: stop local infrastructure
.PHONY: down
down:
	@docker compose down

# ==================================================================================== #
# DATABASE
# ==================================================================================== #

## migrate: apply pending sqlx migrations
.PHONY: migrate
migrate:
	@sqlx migrate run

## seed: load sample app data
.PHONY: seed
seed:
	@docker compose exec -T postgres psql -U apphub -d apphub -f /dev/stdin < seed/apps.sql

# ==================================================================================== #
# LOAD TESTING
# ==================================================================================== #

## cache-flush: clear all cache keys (dev convenience)
.PHONY: cache-flush
cache-flush:
	@docker compose exec -T valkey valkey-cli FLUSHDB

## load: run a k6 load scenario  (SCENARIO=list_apps | get_app | create_app)
.PHONY: load
load:
	@k6 run load/$(SCENARIO).js

## seed-load: insert 1 000 load-test rows
.PHONY: seed-load
seed-load:
	@docker compose exec -T postgres psql -U apphub -d apphub -f /dev/stdin < seed/load.sql

## load-baseline: run all three baseline scenarios back-to-back
.PHONY: load-baseline
load-baseline:
	@k6 run load/list_apps.js
	@k6 run load/get_app.js
	@k6 run load/create_app.js

## load-write: run write-heavy scenarios (update_apps, write_heavy, mixed)
.PHONY: load-write
load-write:
	@k6 run load/update_apps.js
	@k6 run load/write_heavy.js
	@k6 run load/mixed.js

# ==================================================================================== #
# QUALITY CONTROL
# ==================================================================================== #

## ci: full check — format, linting and tests
.PHONY: ci
ci:
	@cargo fmt --check && cargo clippy -- -D warnings && cargo test -- --test-threads=1

# ==================================================================================== #
# BUILD
# ==================================================================================== #

## clean: remove build artifacts
.PHONY: clean
clean:
	@cargo clean

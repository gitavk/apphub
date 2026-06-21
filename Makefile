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

## run: start the HTTP server
.PHONY: run
run:
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

## up: start local infrastructure (postgres, redis, nats, …)
.PHONY: up
up:
	@docker compose up -d

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

# ==================================================================================== #
# LOAD TESTING
# ==================================================================================== #

## load: run a k6 load scenario  (SCENARIO=get_apps)
.PHONY: load
load:
	@k6 run load/$(SCENARIO).js

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

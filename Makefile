SHELL := /usr/bin/env bash
COMPOSE ?= docker compose

API_PORT ?= $(if $(PERCEPTIONLAB_API_PORT),$(PERCEPTIONLAB_API_PORT),8080)
API_URL ?= http://127.0.0.1:$(API_PORT)
LOKI_PORT ?= $(if $(PERCEPTIONLAB_LOKI_PORT),$(PERCEPTIONLAB_LOKI_PORT),3100)
LOKI_URL ?= http://127.0.0.1:$(LOKI_PORT)
ALLOY_PORT ?= $(if $(PERCEPTIONLAB_ALLOY_PORT),$(PERCEPTIONLAB_ALLOY_PORT),12345)
ALLOY_URL ?= http://127.0.0.1:$(ALLOY_PORT)
PERCEPTIONLAB_API_BASE_URL ?= $(API_URL)

.PHONY: help up infra api web worker-once seed smoke down ps logs logs-api logs-loki loki-ready loki-query alloy-ready quality

help:
	@printf "PerceptionLab local commands\n"
	@printf "  make up           Start Postgres, Loki, Alloy, API, then the web dashboard\n"
	@printf "  make infra        Start Postgres, Loki, and Alloy only\n"
	@printf "  make api          Start Postgres, Loki, Alloy, and API in Docker Compose\n"
	@printf "  make web          Start the React/Vite operations dashboard\n"
	@printf "  make worker-once  Process one queued training job from PostgreSQL\n"
	@printf "  make seed         Seed the running API with the configured demo dataset\n"
	@printf "  make smoke        Run the product object-recognition smoke flow\n"
	@printf "  make loki-ready   Check Loki readiness\n"
	@printf "  make loki-query   Query Loki labels\n"
	@printf "  make logs         Follow all Docker Compose logs\n"
	@printf "  make down         Stop the Docker Compose stack\n"

up: api
	PERCEPTIONLAB_API_BASE_URL=$(PERCEPTIONLAB_API_BASE_URL) npm run web:dev

infra:
	$(COMPOSE) up -d postgres loki alloy

api:
	$(COMPOSE) up -d postgres loki alloy api

web:
	PERCEPTIONLAB_API_BASE_URL=$(PERCEPTIONLAB_API_BASE_URL) npm run web:dev

worker-once:
	cd worker && PERCEPTIONLAB_DATABASE_URL=$${PERCEPTIONLAB_DATABASE_URL:-postgres://perceptionlab:perceptionlab@127.0.0.1:55432/perceptionlab} PERCEPTIONLAB_ARTIFACT_ROOT=$${PERCEPTIONLAB_ARTIFACT_ROOT:-/media/jerem/ubuntu1/perceptionlab/artifacts} UV_CACHE_DIR=../.perceptionlab/cache/uv uv run perception-worker process-once --repository-backend postgres --trainer tiny_torch

seed:
	PERCEPTIONLAB_API_BASE_URL=$(API_URL) node scripts/seed-demo-dataset.mjs

smoke:
	PERCEPTIONLAB_API_BASE_URL=$(API_URL) npm run demo:fire

down:
	$(COMPOSE) down

ps:
	$(COMPOSE) ps

logs:
	$(COMPOSE) logs -f

logs-api:
	$(COMPOSE) logs -f api

logs-loki:
	$(COMPOSE) logs -f loki alloy

loki-ready:
	curl -fsS "$(LOKI_URL)/ready"

loki-query:
	curl -fsG "$(LOKI_URL)/loki/api/v1/labels"

alloy-ready:
	curl -fsS "$(ALLOY_URL)/-/ready"

quality:
	npm run quality
	cargo test --manifest-path api/Cargo.toml --workspace

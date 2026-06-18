SHELL := /usr/bin/env bash
COMPOSE ?= docker compose

API_PORT ?= $(if $(PERCEPTIONLAB_API_PORT),$(PERCEPTIONLAB_API_PORT),8080)
API_URL ?= http://127.0.0.1:$(API_PORT)
LOKI_PORT ?= $(if $(PERCEPTIONLAB_LOKI_PORT),$(PERCEPTIONLAB_LOKI_PORT),3100)
LOKI_URL ?= http://127.0.0.1:$(LOKI_PORT)
ALLOY_PORT ?= $(if $(PERCEPTIONLAB_ALLOY_PORT),$(PERCEPTIONLAB_ALLOY_PORT),12345)
ALLOY_URL ?= http://127.0.0.1:$(ALLOY_PORT)
PERCEPTIONLAB_API_BASE_URL ?= $(API_URL)

.PHONY: help up infra api api-real web worker-once seed smoke down ps logs logs-api logs-loki loki-ready loki-query alloy-ready quality

help:
	@printf "PerceptionLab local commands\n"
	@printf "  make up           Start Postgres, Loki, Alloy, API, then the web dashboard\n"
	@printf "  make infra        Start Postgres, Loki, and Alloy only\n"
	@printf "  make api          Start Postgres, Loki, Alloy, and API in Docker Compose\n"
	@printf "  make api-real     Start infra in Docker and API on host with YOLO inference\n"
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

api-real:
	$(COMPOSE) up -d postgres loki alloy
	-$(COMPOSE) stop api
	PERCEPTIONLAB_API_ADDR=$${PERCEPTIONLAB_API_ADDR:-127.0.0.1:$(API_PORT)} PERCEPTIONLAB_ENV=$${PERCEPTIONLAB_ENV:-local} PERCEPTIONLAB_PROJECT_ROOT=$${PERCEPTIONLAB_PROJECT_ROOT:-$(CURDIR)} PERCEPTIONLAB_DATA_ROOT=$${PERCEPTIONLAB_DATA_ROOT:-$(CURDIR)/datasets} PERCEPTIONLAB_SEED_DATASET_ROOT=$${PERCEPTIONLAB_SEED_DATASET_ROOT:-$(CURDIR)/datasets/seed} PERCEPTIONLAB_STORAGE_ROOT=$${PERCEPTIONLAB_STORAGE_ROOT:-$(CURDIR)/.perceptionlab/storage} PERCEPTIONLAB_ARTIFACT_ROOT=$${PERCEPTIONLAB_ARTIFACT_ROOT:-$(CURDIR)/.perceptionlab/artifacts} PERCEPTIONLAB_TMP_ROOT=$${PERCEPTIONLAB_TMP_ROOT:-$(CURDIR)/.perceptionlab/tmp} PERCEPTIONLAB_REPOSITORY_BACKEND=$${PERCEPTIONLAB_REPOSITORY_BACKEND:-postgres} PERCEPTIONLAB_MIGRATIONS_ROOT=$${PERCEPTIONLAB_MIGRATIONS_ROOT:-$(CURDIR)/api/migrations} PERCEPTIONLAB_DATABASE_URL=$${PERCEPTIONLAB_DATABASE_URL:-postgres://perceptionlab:perceptionlab@127.0.0.1:55432/perceptionlab} PERCEPTIONLAB_QUEUE_URL=$${PERCEPTIONLAB_QUEUE_URL:-postgres://perceptionlab:perceptionlab@127.0.0.1:55432/perceptionlab} PERCEPTIONLAB_OBJECT_STORAGE_BUCKET=$${PERCEPTIONLAB_OBJECT_STORAGE_BUCKET:-perceptionlab} PERCEPTIONLAB_INFERENCE_ENGINE=yolo_cli cargo run --manifest-path api/Cargo.toml -p perception_api

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

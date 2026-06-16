#!/usr/bin/env sh
set -eu

api_addr="${PERCEPTIONLAB_API_ADDR:-127.0.0.1:8080}"
api_base_url="${PERCEPTIONLAB_API_BASE_URL:-http://${api_addr}}"
log_file="${PERCEPTIONLAB_TMP_ROOT:-.perceptionlab/tmp}/fire-demo-api.log"

mkdir -p "$(dirname "$log_file")"

PERCEPTIONLAB_API_ADDR="$api_addr" cargo run --manifest-path api/Cargo.toml -p perception_api >"$log_file" 2>&1 &
api_pid="$!"

cleanup() {
  kill "$api_pid" >/dev/null 2>&1 || true
}
trap cleanup EXIT INT TERM

attempts=0
until curl -fsS "${api_base_url}/health" >/dev/null 2>&1; do
  attempts=$((attempts + 1))
  if [ "$attempts" -ge 60 ]; then
    printf '%s\n' "API did not become ready. See ${log_file}."
    exit 1
  fi
  sleep 1
done

PERCEPTIONLAB_API_BASE_URL="$api_base_url" node scripts/fire-demo-product.mjs

#!/usr/bin/env sh
set -eu

node scripts/validate-bdd-features.mjs
npm run bdd:dry-run

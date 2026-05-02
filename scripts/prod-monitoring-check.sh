#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "❌ $1"
  exit 1
}

info() {
  echo "ℹ️  $1"
}

success() {
  echo "✅ $1"
}

[ -f .env.production ] || fail "Missing .env.production"
[ -f docker-compose.monitoring.yml ] || fail "Missing docker-compose.monitoring.yml"

info "Validating compose monitoring config"
docker compose --env-file .env.production \
  -f docker-compose.prod.yml \
  -f docker-compose.monitoring.yml \
  config >/dev/null

info "Showing monitoring services"
docker compose --env-file .env.production \
  -f docker-compose.prod.yml \
  -f docker-compose.monitoring.yml \
  ps prometheus grafana loki promtail || true

success "Monitoring config looks valid"

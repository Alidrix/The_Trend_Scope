#!/usr/bin/env bash
set -euo pipefail

REQUIRE_MONITORING_RUNNING="${REQUIRE_MONITORING_RUNNING:-0}"

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

require_file() {
  local path="$1"
  [ -f "$path" ] || fail "Missing required file: $path"
}

compose() {
  docker compose --env-file .env.production \
    -f docker-compose.prod.yml \
    -f docker-compose.monitoring.yml \
    "$@"
}

check_config() {
  info "Checking required monitoring files"
  require_file .env.production
  require_file docker-compose.monitoring.yml
  require_file infra/prometheus/prometheus.yml
  require_file infra/prometheus/alerts.yml
  require_file infra/alertmanager/alertmanager.yml
  require_file infra/grafana/provisioning/datasources/datasources.yml
  require_file infra/grafana/provisioning/dashboards/dashboards.yml
  require_file infra/grafana/dashboards/trend-scope-overview.json
  require_file infra/promtail/promtail.yml

  info "Validating compose monitoring config"
  compose config >/dev/null
  success "Monitoring configuration is valid"
}

check_running_services() {
  info "REQUIRE_MONITORING_RUNNING=1, checking running monitoring services"

  compose ps prometheus alertmanager grafana loki promtail >/dev/null

  compose exec -T prometheus sh -c 'wget -qO- http://localhost:9090/-/ready >/dev/null'
  compose exec -T grafana sh -c 'wget -qO- http://localhost:3000/api/health >/dev/null'
  compose exec -T loki sh -c 'wget -qO- http://localhost:3100/ready >/dev/null'
  compose exec -T alertmanager sh -c 'wget -qO- http://localhost:9093/-/ready >/dev/null'

  success "Monitoring services readiness checks passed"
}

check_config

if [ "$REQUIRE_MONITORING_RUNNING" = "1" ]; then
  check_running_services
else
  info "REQUIRE_MONITORING_RUNNING=0, skipping runtime readiness checks"
fi

success "Monitoring checks completed"

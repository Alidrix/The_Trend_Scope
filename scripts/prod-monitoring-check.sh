#!/usr/bin/env bash
set -euo pipefail

REQUIRE_MONITORING_RUNNING="${REQUIRE_MONITORING_RUNNING:-0}"
CURL_IMAGE="${CURL_IMAGE:-curlimages/curl:8.10.1}"

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
  [ -f "$1" ] || fail "Missing required file: $1"
}

compose() {
  docker compose --env-file .env.production -f docker-compose.prod.yml -f docker-compose.monitoring.yml "$@"
}

curl_container() {
  local container="$1"
  local url="$2"

  docker run --rm --network "container:${container}" "$CURL_IMAGE" -fsS "$url" >/dev/null
}

check_required_files() {
  info "Checking required monitoring files"

  require_file .env.production
  require_file docker-compose.monitoring.yml
  require_file infra/prometheus/prometheus.yml
  require_file infra/prometheus/alerts.yml
  require_file infra/alertmanager/alertmanager.yml
  require_file infra/grafana/provisioning/datasources/datasources.yml
  require_file infra/grafana/provisioning/dashboards/dashboards.yml
  require_file infra/grafana/dashboards/trend-scope-overview.json
  require_file infra/grafana/dashboards/trend-scope-infra.json
  require_file infra/promtail/promtail.yml
  require_file infra/blackbox/blackbox.yml
}

check_compose_config() {
  info "Validating compose monitoring configuration"
  compose config >/dev/null
  success "Monitoring configuration is valid"
}

show_services() {
  info "Monitoring services status"
  compose ps prometheus alertmanager grafana loki promtail node-exporter cadvisor blackbox || true
}

check_runtime_readiness() {
  info "REQUIRE_MONITORING_RUNNING=1, checking running monitoring services"

  curl_container trend-scope-prometheus http://localhost:9090/-/ready
  curl_container trend-scope-grafana http://localhost:3000/api/health
  curl_container trend-scope-loki http://localhost:3100/ready
  curl_container trend-scope-alertmanager http://localhost:9093/-/ready
  curl_container trend-scope-blackbox http://localhost:9115/-/healthy
  curl_container trend-scope-node-exporter http://localhost:9100/metrics
  curl_container trend-scope-cadvisor http://localhost:8080/metrics

  success "Monitoring services readiness checks passed"
}

main() {
  check_required_files
  check_compose_config
  show_services

  if [ "$REQUIRE_MONITORING_RUNNING" = "1" ]; then
    check_runtime_readiness
  else
    info "REQUIRE_MONITORING_RUNNING=0, skipping runtime readiness checks"
  fi

  success "Monitoring checks completed"
}

main "$@"

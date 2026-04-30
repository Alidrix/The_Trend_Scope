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

require_env() {
  local key="$1"
  local value="${!key:-}"

  if [ -z "$value" ]; then
    fail "Missing $key"
  fi

  if [[ "$value" == *"replace"* || "$value" == *"change-me"* || "$value" == *"<"* || "$value" == *">"* ]]; then
    fail "Placeholder value for $key"
  fi
}

check_url() {
  local label="$1"
  local url="$2"

  info "Checking ${label}: ${url}"
  curl -fsS "$url" >/dev/null
  success "${label} reachable"
}

if [ ! -f .env.production ]; then
  fail "Missing .env.production"
fi

set -a
source .env.production
set +a

required=(
  APP_DOMAIN
  API_DOMAIN
  FRONTEND_ORIGIN
  DATABASE_URL
  REDIS_URL
  NATS_URL
  YOUTUBE_API_KEY
  STRIPE_SECRET_KEY
  STRIPE_WEBHOOK_SECRET
  STRIPE_PRICE_PRO_MONTHLY
  STRIPE_PRICE_STUDIO_MONTHLY
  CF_DNS_API_TOKEN
  ACME_EMAIL
  SECRET_KEY
)

for key in "${required[@]}"; do
  require_env "$key"
done

success "Environment variables look ready"

if [ "${SKIP_REMOTE_CHECKS:-0}" = "1" ]; then
  success "Remote checks skipped"
  success "Go-live check completed"
  exit 0
fi

check_url "Health" "https://${API_DOMAIN}/api/v1/health"
check_url "Ready" "https://${API_DOMAIN}/api/v1/ready"
check_url "Metrics" "https://${API_DOMAIN}/metrics"
if [ -n "${ADMIN_BEARER_TOKEN:-}" ]; then
  info "Checking Admin smoke endpoint"
  curl -fsS \
    -H "Authorization: Bearer ${ADMIN_BEARER_TOKEN}" \
    "https://${API_DOMAIN}/api/v1/admin/smoke" >/dev/null
  success "Admin smoke reachable"
else
  info "ADMIN_BEARER_TOKEN not provided; admin smoke check skipped"
fi

success "Go-live check completed"

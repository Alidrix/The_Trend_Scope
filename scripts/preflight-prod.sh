#!/usr/bin/env bash
set -euo pipefail

if [ ! -f .env.production ]; then
  echo "❌ Missing .env.production"
  exit 1
fi

set -a
source .env.production
set +a

required_vars=(
  APP_DOMAIN
  APP_WWW_DOMAIN
  API_DOMAIN
  TRAEFIK_DOMAIN
  ACME_EMAIL
  CF_DNS_API_TOKEN
  DATABASE_URL
  POSTGRES_PASSWORD
  SECRET_KEY
  YOUTUBE_API_KEY
  STRIPE_SECRET_KEY
  STRIPE_WEBHOOK_SECRET
  STRIPE_PRICE_PRO_MONTHLY
  STRIPE_PRICE_STUDIO_MONTHLY
)

for var in "${required_vars[@]}"; do
  value="${!var:-}"
  if [ -z "$value" ]; then
    echo "❌ Missing required variable: $var"
    exit 1
  fi
  if [[ "$value" == *"replace-with"* || "$value" == *"change-me"* || "$value" == *"<strong-password>"* ]]; then
    echo "❌ Placeholder value still used for: $var"
    exit 1
  fi
done

if [ ! -f infra/traefik/dynamic.yml ]; then
  echo "❌ Missing infra/traefik/dynamic.yml"
  exit 1
fi

if grep -q "REPLACE_WITH_BCRYPT_HASH\|replace-with-bcrypt-hash" infra/traefik/dynamic.yml; then
  echo "❌ Traefik dashboard auth is not configured"
  exit 1
fi

echo "✅ Production preflight passed"

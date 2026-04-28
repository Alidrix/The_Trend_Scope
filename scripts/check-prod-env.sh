#!/usr/bin/env bash
set -euo pipefail
required=(SECRET_KEY DATABASE_URL REDIS_URL NATS_URL CLICKHOUSE_URL STRIPE_SECRET_KEY STRIPE_WEBHOOK_SECRET)
for key in "${required[@]}"; do
  if ! grep -q "^${key}=" .env.production; then
    echo "missing $key"
    exit 1
  fi
done
echo "env production looks valid"

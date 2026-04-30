#!/usr/bin/env bash
set -euo pipefail
if [ ! -f .env.production ]; then echo "❌ Missing .env.production"; exit 1; fi
set -a; source .env.production; set +a
required=(APP_DOMAIN API_DOMAIN FRONTEND_ORIGIN DATABASE_URL REDIS_URL NATS_URL YOUTUBE_API_KEY STRIPE_SECRET_KEY STRIPE_WEBHOOK_SECRET STRIPE_PRICE_PRO_MONTHLY STRIPE_PRICE_STUDIO_MONTHLY CF_DNS_API_TOKEN ACME_EMAIL SECRET_KEY)
for key in "${required[@]}"; do value="${!key:-}"; if [ -z "$value" ]; then echo "❌ Missing $key"; exit 1; fi; if [[ "$value" == *"replace"* || "$value" == *"change-me"* || "$value" == *"<"* || "$value" == *">"* ]]; then echo "❌ Placeholder value for $key"; exit 1; fi; done
 echo "✅ Environment variables look ready"
if [ -n "${API_DOMAIN:-}" ]; then
curl -fsS "https://${API_DOMAIN}/api/v1/health"; echo
curl -fsS "https://${API_DOMAIN}/api/v1/ready"; echo
curl -fsS "https://${API_DOMAIN}/metrics" >/dev/null; echo "✅ Metrics endpoint reachable"
fi
echo "✅ Go-live check completed"

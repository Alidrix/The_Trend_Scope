#!/usr/bin/env bash
set -euo pipefail

RESULTS=()

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

run_check() {
  local label="$1"
  shift

  info "Running $label"
  if "$@"; then
    success "$label OK"
    RESULTS+=("✅ $label OK")
  else
    RESULTS+=("❌ $label FAILED")
    fail "$label failed"
  fi
}

run_check "prod-check" ./scripts/prod-check.sh
run_check "volumes check" ./scripts/prod-volumes-check.sh

if [ "${SKIP_BACKUP_VERIFY:-0}" = "1" ]; then
  echo "⚠️ Backup verification skipped"
  RESULTS+=("⚠️ Backup verification skipped")
else
  run_check "backup verify" ./scripts/prod-backup-verify.sh
fi

if [ "${SKIP_MONITORING_CHECK:-0}" = "1" ]; then
  echo "⚠️ Monitoring verification skipped"
  RESULTS+=("⚠️ Monitoring verification skipped")
else
  run_check "monitoring check" ./scripts/prod-monitoring-check.sh
  RESULTS+=("✅ monitoring check OK")
fi

if [ "${SKIP_ALERTING_TEST:-1}" = "1" ]; then
  echo "⚠️ Alerting test skipped"
  RESULTS+=("⚠️ Alerting test skipped")
else
  run_check "alerting test" ./scripts/prod-alerting-test.sh
fi

run_check "go-live check" ./scripts/go-live-check.sh

echo
info "Summary"
for line in "${RESULTS[@]}"; do
  echo "$line"
done
success "VPS Go/No-Go checks completed"

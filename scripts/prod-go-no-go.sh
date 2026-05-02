#!/usr/bin/env bash
set -euo pipefail
run_check(){ local label="$1"; shift; "$@"; echo "✅ $label OK"; }
run_check "prod-check" ./scripts/prod-check.sh
run_check "volumes check" ./scripts/prod-volumes-check.sh
if [ "${SKIP_BACKUP_VERIFY:-0}" = "1" ]; then
  echo "⚠️ Backup verification skipped"
else
  run_check "backup verify" ./scripts/prod-backup-verify.sh
fi
run_check "go-live check" ./scripts/go-live-check.sh
echo "✅ VPS Go/No-Go checks completed"

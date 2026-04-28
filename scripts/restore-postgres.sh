#!/usr/bin/env bash
set -euo pipefail
if [ $# -ne 1 ]; then
  echo "Usage: $0 <backup.sql>"
  exit 1
fi
docker compose exec -T postgres psql -U "${POSTGRES_USER:-viral}" "${POSTGRES_DB:-viral_radar}" < "$1"

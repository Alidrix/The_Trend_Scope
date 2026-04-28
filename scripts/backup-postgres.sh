#!/usr/bin/env bash
set -euo pipefail
mkdir -p backups/postgres
docker compose exec -T postgres pg_dump -U "${POSTGRES_USER:-viral}" "${POSTGRES_DB:-viral_radar}" > "backups/postgres/viral_radar_$(date +%Y%m%d_%H%M%S).sql"

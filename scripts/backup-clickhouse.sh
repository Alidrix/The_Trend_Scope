#!/usr/bin/env bash
set -euo pipefail
mkdir -p backups/clickhouse
docker compose exec -T clickhouse clickhouse-client --query "BACKUP DATABASE viral_analytics TO Disk('backups', 'viral_analytics_$(date +%Y%m%d_%H%M%S)')"

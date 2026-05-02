#!/usr/bin/env bash
set -euo pipefail

BACKUP_FILE="${1:-}"
CONTAINER_NAME="trendscope-restore-dry-run-$(date +%s)"
POSTGRES_IMAGE="${POSTGRES_IMAGE:-postgres:16-alpine}"
POSTGRES_USER="${POSTGRES_USER:-viral}"
POSTGRES_DB="${POSTGRES_DB:-viral_radar}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-dry-run-password}"

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

cleanup() {
  docker rm -f "$CONTAINER_NAME" >/dev/null 2>&1 || true
}

validate_backup_file() {
  [ -n "$BACKUP_FILE" ] || fail "Usage: ./scripts/prod-restore-dry-run.sh <backup-file.sql|backup-file.sql.gz>"
  [ -f "$BACKUP_FILE" ] || fail "Backup file not found: $BACKUP_FILE"

  case "$BACKUP_FILE" in
    *.sql|*.sql.gz) ;;
    *) fail "Unsupported backup format" ;;
  esac
}

verify_checksum() {
  if [ -f "${BACKUP_FILE}.sha256" ]; then
    info "Verifying checksum"
    sha256sum -c "${BACKUP_FILE}.sha256"
  else
    info "No checksum file found, skipping"
  fi
}

start_temp_postgres() {
  info "Starting temporary PostgreSQL container: $CONTAINER_NAME"
  docker run -d \
    --name "$CONTAINER_NAME" \
    -e POSTGRES_USER="$POSTGRES_USER" \
    -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
    -e POSTGRES_DB="$POSTGRES_DB" \
    "$POSTGRES_IMAGE" >/dev/null
}

wait_for_postgres() {
  for _ in $(seq 1 40); do
    if docker exec "$CONTAINER_NAME" pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" >/dev/null 2>&1; then
      return 0
    fi
    sleep 1
  done

  docker exec "$CONTAINER_NAME" pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" >/dev/null 2>&1 || fail "Temporary PostgreSQL did not become ready"
}

restore_backup() {
  info "Restoring dump in isolated container"
  if [[ "$BACKUP_FILE" == *.sql.gz ]]; then
    gzip -dc "$BACKUP_FILE" | docker exec -i "$CONTAINER_NAME" psql -U "$POSTGRES_USER" "$POSTGRES_DB" >/dev/null
  else
    docker exec -i "$CONTAINER_NAME" psql -U "$POSTGRES_USER" "$POSTGRES_DB" < "$BACKUP_FILE" >/dev/null
  fi
  success "Restore SQL completed"
}

run_best_effort_checks() {
  for table in users reports admin_audit_logs; do
    if docker exec "$CONTAINER_NAME" psql -U "$POSTGRES_USER" "$POSTGRES_DB" -tAc "SELECT COUNT(*) FROM $table;" >/dev/null 2>&1; then
      count="$(docker exec "$CONTAINER_NAME" psql -U "$POSTGRES_USER" "$POSTGRES_DB" -tAc "SELECT COUNT(*) FROM $table;" | xargs)"
      echo "✅ check $table count=$count"
    else
      echo "⚠️  table check skipped: $table"
    fi
  done
}

trap cleanup EXIT

validate_backup_file
verify_checksum
start_temp_postgres
wait_for_postgres
restore_backup
run_best_effort_checks
success "Dry-run restore finished safely"

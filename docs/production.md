# Production deployment guide

## Prérequis serveur
- Ubuntu 22.04+ / Debian 12+
- Docker + Docker Compose plugin
- DNS: `your-domain.com` (frontend) et `api.your-domain.com` (API)
- TLS certs (Let's Encrypt via reverse proxy recommandé)

## Bootstrap
```bash
cp .env.production.example .env.production
docker compose -f docker-compose.prod.yml pull
docker compose -f docker-compose.prod.yml build
docker compose -f docker-compose.prod.yml up -d
docker compose -f docker-compose.prod.yml ps
```

## Vérifications
```bash
curl -fsS https://api.your-domain.com/api/v1/health
curl -fsS https://api.your-domain.com/api/v1/ready
curl -fsS https://api.your-domain.com/metrics
```

## Backup / restore
```bash
./scripts/backup-postgres.sh
./scripts/backup-clickhouse.sh
./scripts/restore-postgres.sh backups/postgres/your_backup.sql
```

## Mise à jour
1. `git pull`
2. `docker compose -f docker-compose.prod.yml build`
3. `docker compose -f docker-compose.prod.yml up -d`

## Rollback
- Revenir au tag Git précédent puis relancer `up -d`.
- Restaurer PostgreSQL depuis le dernier dump validé.

## Monitoring
- `/api/v1/health`, `/api/v1/ready`, `/metrics`
- Logs: `docker compose -f docker-compose.prod.yml logs -f backend worker reverse-proxy`

## Sécurité
- N’exposer que 80/443 publiquement.
- Ne jamais exposer PostgreSQL, PgBouncer, Redis, NATS, ClickHouse TCP.
- Secrets forts obligatoires (`SECRET_KEY`, DB, ClickHouse, Stripe, SMTP).
- Headers HTTP de sécurité activés dans Nginx.

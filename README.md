# Viral Radar — SaaS Social Trend Intelligence (PostgreSQL Standard Infra)

Viral Radar est une base SaaS publique pour détecter des tendances vidéo (YouTube puis TikTok/Instagram), avec une architecture scalable orientée **50k inscrits / 10k actifs**.

## Architecture officielle (sans Supabase imposé)

Supabase n'est plus une dépendance produit officielle.
Le stack de référence est désormais:

- **PostgreSQL** (transactionnel: users, subscriptions, preferences, consentements)
- **PgBouncer** (pool de connexions)
- **Redis** (cache, rate limiting, sessions/locks)
- **NATS JetStream** (jobs asynchrones)
- **ClickHouse** (analytics massifs)
- **Backend Rust Axum** (API stateless)
- **Worker Rust** (scan/scheduling/scoring)
- **Frontend SvelteKit**
- **Stripe** (paiements)

## Démarrage local

```bash
cp .env.example .env
docker compose build --no-cache
docker compose up -d
docker compose ps
curl -fsS http://localhost:4443/api/v1/health
curl -fsS http://localhost:4443/api/v1/auth/status
```

Arrêt:

```bash
docker compose down -v
```

## Variables d'environnement

- `.env.example`: profile local-first (PostgreSQL/PgBouncer/Redis/NATS/ClickHouse locaux).
- `.env.production.example`: profile production (DB externe possible, `sslmode=require`).

⚠️ Aucune vraie clé ne doit être commitée.

## Sécurité et règles SaaS

- Clé YouTube **uniquement côté backend** (`YOUTUBE_API_KEY`) et jamais en `VITE_*`.
- Scans sensibles protégés en **admin-only** (`/api/v1/videos`, `/api/v1/videos/scan`).
- Le frontend lit les tendances calculées via `/api/v1/radar/daily`; il ne déclenche pas directement un scan pour les users standards.
- Paiements via Stripe: aucune donnée carte stockée localement.
- CORS restreint à `FRONTEND_ORIGIN`.

## Données métier et RGPD (base technique)

`db/migrations/init.sql` prépare:

- comptes (`users`, `user_profiles`, `user_preferences`)
- consentements (`consents`)
- traçabilité (`audit_logs`)
- abonnements (`subscriptions`)
- usage et quotas (`user_usage_daily`, `trend_views`)
- fonctionnalités utilisateur (`favorites`, `watchlists`)
- demandes RGPD (`data_export_requests`, `account_deletion_requests`)

## Analytics ClickHouse

`infra/clickhouse/init.sql` prépare:

- `trend_events`
- `trend_rankings_hourly`

L'API est conçue pour lire prioritairement cache/rankings, pas la donnée brute massive à chaque requête.

## CI/CD

Le workflow GitHub Actions lance:

- backend: fmt + clippy + tests
- frontend: check + build
- smoke docker: build, up, health, auth status, down

## Roadmap plateforme

1. YouTube (MVP)
2. TikTok (worker topics préparés)
3. Instagram (worker topics préparés)

## Plans

- Free
- Pro
- Studio

Les limites sont validées côté backend (pas uniquement côté frontend).

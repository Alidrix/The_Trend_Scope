# The Trend Scope

SaaS de détection de tendances vidéo (YouTube/TikTok) basé sur **PostgreSQL standard**.

> Supabase n'est plus une dépendance officielle du projet.

## Stack officielle

- Backend: Rust + Axum
- Frontend: SvelteKit
- Données: PostgreSQL + PgBouncer
- Cache: Redis
- Queue: NATS
- Analytics: ClickHouse
- Orchestration locale: Docker Compose

## Démarrage local

```bash
cp .env.example .env
docker compose build --no-cache
docker compose up -d
docker compose ps
curl -fsS http://localhost:4443/api/v1/health
curl -fsS http://localhost:4443/api/v1/auth/status
docker compose down -v
```

## Production

1. Partir de `.env.production.example`.
2. Renseigner secrets forts (`SECRET_KEY`, mots de passe DB, Stripe).
3. Déployer backend + worker + frontend avec reverse proxy TLS.
4. Autoriser uniquement `FRONTEND_ORIGIN` côté CORS.

## Sécurité API YouTube

- Utiliser uniquement `YOUTUBE_API_KEY` côté serveur.
- Ne jamais exposer la clé au frontend (`VITE_*`).
- Ne jamais logger la clé.

## RGPD minimal

- Pages légales: `/privacy`, `/terms`, `/cookies`.
- Endpoints privacy: `/api/v1/me`, `/api/v1/me/consents`, `/api/v1/me/data-export`, `/api/v1/me/delete-request`.
- Stockage de consentements et journaux d'audit sans secrets.

## Stripe (préparé)

Variables:

- `STRIPE_SECRET_KEY`
- `STRIPE_WEBHOOK_SECRET`
- `STRIPE_PRICE_PRO_MONTHLY`
- `STRIPE_PRICE_STUDIO_MONTHLY`

Routes préparées:

- `POST /api/v1/billing/checkout`
- `POST /api/v1/billing/portal`
- `POST /api/v1/billing/webhook`
- `GET /api/v1/billing/status`

Si Stripe n'est pas configuré, l'API retourne `billing is not configured yet`.

## Mode sombre / clair

- Thème basé sur variables CSS.
- Détection préférence système.
- Persistance du choix dans `localStorage`.
- Toggle global dans le layout.

## CI

Workflow GitHub Actions:

- `backend`: fmt + clippy + tests
- `frontend`: check + build
- `docker-smoke`: `docker compose build`, `up -d`, vérification `/health` et `/auth/status`

## Observabilité future

Dossier `infra/` prêt pour Prometheus/Grafana/Loki.

Métriques cibles:

- API latency
- erreurs API
- scans réussis/échoués
- jobs NATS
- cache hit Redis
- quotas YouTube
- taille ClickHouse
- connexions PgBouncer
- utilisateurs actifs

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="frontend/static/brand/trend-scope-logo-dark.png">
    <source media="(prefers-color-scheme: light)" srcset="frontend/static/brand/trend-scope-logo-light.png">
    <img src="frontend/static/brand/trend-scope-logo-light.png" alt="The Trend Scope — Create Success" width="420">
  </picture>
</p>

<h1 align="center">🚀 The Trend Scope</h1>

<p align="center">
  <strong>Repère les tendances avant les autres et crée du contenu au bon moment.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Status-SaaS%20MVP-8A2BFF?style=for-the-badge" alt="Status SaaS MVP" />
  <img src="https://img.shields.io/badge/Backend-Rust%20%2F%20Axum-111827?style=for-the-badge&logo=rust" alt="Backend Rust Axum" />
  <img src="https://img.shields.io/badge/Frontend-SvelteKit-FF3E00?style=for-the-badge&logo=svelte&logoColor=white" alt="Frontend SvelteKit" />
  <img src="https://img.shields.io/badge/Database-PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white" alt="PostgreSQL" />
  <img src="https://img.shields.io/badge/Cache-Redis-DC382D?style=for-the-badge&logo=redis&logoColor=white" alt="Redis" />
  <img src="https://img.shields.io/badge/Queue-NATS-27AAE1?style=for-the-badge" alt="NATS" />
  <img src="https://img.shields.io/badge/Analytics-ClickHouse-FFCC01?style=for-the-badge&logo=clickhouse&logoColor=black" alt="ClickHouse" />
  <img src="https://img.shields.io/badge/Infra-Docker%20Compose-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker Compose" />
</p>

---

## 🌈 Bannière produit

> **The Trend Scope** est une plateforme SaaS de veille virale pensée pour les créateurs, influenceurs, agences social media et marques.  
> L’objectif : détecter les tendances vidéo émergentes sur **YouTube**. TikTok et Instagram restent sur la roadmap : bientôt disponible.

<table>
  <tr>
    <td>🎯 <strong>Promesse</strong></td>
    <td>Transformer les signaux sociaux en opportunités de contenu claires et exploitables.</td>
  </tr>
  <tr>
    <td>⚡ <strong>Expérience</strong></td>
    <td>Connexion → Radar du jour → tendances utiles → upgrade Pro / Studio.</td>
  </tr>
  <tr>
    <td>🧠 <strong>Approche</strong></td>
    <td>Pas un dashboard technique : un outil simple, lisible, premium et orienté action.</td>
  </tr>
</table>

---

## 📚 Sommaire

- [🆕 Dernières évolutions](#-dernières-évolutions)
- [🏗️ Architecture officielle](#️-architecture-officielle)
- [🚀 Démarrage rapide](#-démarrage-rapide)
- [🧰 Commandes utiles](#-commandes-utiles)
- [🔐 Variables d’environnement](#-variables-denvironnement)
- [💳 Offres SaaS](#-offres-saas)
- [🧭 Navigation produit](#-navigation-produit)
- [🎨 Branding et thème](#-branding-et-thème)
- [🔒 Sécurité et RGPD](#-sécurité-et-rgpd)
- [🧠 Moteur de tendance](#-moteur-de-tendance)
- [🧩 API backend](#-api-backend)
- [🗄️ Données](#️-données)
- [📊 Observabilité](#-observabilité)
- [🧪 Qualité et CI](#-qualité-et-ci)
- [🛣️ Roadmap](#️-roadmap)

---

## 🆕 Dernières évolutions

| Statut | Évolution | Détail |
| --- | --- | --- |
| ✅ | **Supabase retiré** | Le projet utilise maintenant une stack PostgreSQL standard. |
| ✅ | **Infra locale complète** | PostgreSQL, PgBouncer, Redis, NATS, ClickHouse, API, Worker, Frontend. |
| ✅ | **API / Worker séparés** | Le Dockerfile compile désormais les binaires `/app/api` et `/app/worker`. |
| ✅ | **PgBouncer corrigé** | Auth SCRAM compatible PostgreSQL 16. |
| ✅ | **Mode sombre / clair** | Thème par variables CSS, préférence système et persistance locale. |
| ✅ | **RGPD préparé** | Pages légales, consentements, export et suppression de compte. |
| ✅ | **Stripe implémenté** | Stripe Checkout / Portal / Webhook implémentés, à valider avec de vraies clés Stripe. |
| ✅ | **Logos ajoutés** | Versions claire et sombre disponibles dans `frontend/static/brand/`. |

> 🔥 **Supabase n’est plus une dépendance officielle du projet.**  
> The Trend Scope repose maintenant sur une infrastructure SaaS standard et portable.

---

## ✅ État réel du projet

| Bloc | Statut |
| --- | --- |
| Auth / Register / Onboarding | ✅ Implémenté |
| Plans Free / Pro / Studio | ✅ Implémenté |
| Stripe Checkout | 🟡 Implémenté, à tester avec Stripe CLI |
| Stripe Portal | 🟡 Implémenté, à tester |
| Stripe Webhook | 🟡 Implémenté, idempotence à renforcer |
| Watchlist | ✅ Base CRUD |
| Alertes | ✅ Base CRUD Studio |
| Rapports | ✅ Base Studio |
| Admin panel | 🟡 UI prête, données à enrichir |
| TikTok | 🔜 Bientôt disponible |
| Déploiement production | 🔜 À créer |

---

## 🏗️ Architecture officielle

```txt
Utilisateur
   │
   ▼
Frontend SvelteKit ───────► Backend Rust / Axum
                                  │
                                  ├── PostgreSQL via PgBouncer  → données métier
                                  ├── Redis                     → cache, rate limiting, locks
                                  ├── NATS JetStream            → jobs, scans, scoring
                                  ├── ClickHouse                → analytics, vues/h, historiques
                                  └── Worker Rust               → scans et traitements asynchrones
```

| Brique | Technologie | Rôle |
| --- | --- | --- |
| 🌐 Frontend | SvelteKit / Vite | Landing, login, radar, pricing, dashboard abonné. |
| 🧩 API | Rust / Axum | Auth, plans, quotas, radar, billing, RGPD. |
| ⚙️ Worker | Rust | Scans, scoring, jobs NATS, préparation analytics. |
| 🗄️ Base métier | PostgreSQL 16 | Utilisateurs, plans, abonnements, consentements, favoris. |
| 🚦 Pool DB | PgBouncer | Pool de connexions PostgreSQL pour montée en charge. |
| ⚡ Cache | Redis | Cache radar, rate limit, verrous, sessions temporaires. |
| 📬 Queue | NATS JetStream | File de jobs pour scans et traitements asynchrones. |
| 📊 Analytics | ClickHouse | Historique massif, tendances, vues/h, métriques Studio. |
| 💳 Paiement | Stripe | Checkout, portail client, webhooks, abonnements. |
| 🐳 Infra locale | Docker Compose | Lancement complet sans service externe. |

---

## 🚀 Démarrage rapide

### 1️⃣ Préparer l’environnement

```bash
cp .env.example .env
```

### 2️⃣ Construire l’infra

```bash
docker compose build --no-cache
```

### 3️⃣ Démarrer tous les services

```bash
docker compose up -d
```

### 4️⃣ Vérifier l’état des conteneurs

```bash
docker compose ps
```

### 5️⃣ Vérifier l’API

```bash
curl -fsS http://localhost:4443/api/v1/health
curl -fsS http://localhost:4443/api/v1/auth/status
```

### 6️⃣ Accéder à la plateforme

| Service | URL |
| --- | --- |
| 🌐 Frontend | http://localhost:5173 |
| 🧩 Backend API | http://localhost:4443/api/v1 |
| 💚 Healthcheck | http://localhost:4443/api/v1/health |
| 📬 NATS Monitoring | http://localhost:8222 |
| 📊 ClickHouse HTTP | http://localhost:8123 |
| 🗄️ PostgreSQL via PgBouncer | localhost:6432 |

---

## 🧰 Commandes utiles

### Logs complets

```bash
docker compose logs -f
```

### Logs ciblés

```bash
docker compose logs -f backend
docker compose logs -f worker
docker compose logs -f postgres
docker compose logs -f pgbouncer
docker compose logs -f redis
docker compose logs -f nats
docker compose logs -f clickhouse
docker compose logs -f frontend
```

### Redémarrer un service

```bash
docker compose restart backend
docker compose restart worker
docker compose restart frontend
```

### Stopper l’infra

```bash
docker compose down
```

### Stopper et supprimer les volumes locaux

```bash
docker compose down -v
```

### Rebuild propre complet

```bash
docker compose down -v
docker compose build --no-cache
docker compose up -d
```

---

## 🔐 Variables d’environnement

Le projet est **local-first** : `.env.example` permet de lancer toute l’infra sans dépendance externe.

| Variable | Exemple local | Rôle |
| --- | --- | --- |
| `APP_ENV` | `local` | Environnement applicatif. |
| `APP_USERNAME` | `admin` | Compte seed admin local. |
| `APP_PASSWORD` | `change-me-with-a-strong-password` | Mot de passe seed. |
| `SECRET_KEY` | `openssl rand -hex 32` | Secret JWT. |
| `FRONTEND_ORIGIN` | `http://localhost:5173` | Origine CORS autorisée. |
| `VITE_API_BASE` | `http://localhost:4443/api/v1` | URL API côté frontend. |
| `DATABASE_URL` | `postgresql://viral:viral@pgbouncer:5432/viral_radar?sslmode=disable` | Connexion backend via PgBouncer. |
| `REDIS_URL` | `redis://redis:6379` | Cache et rate limiting. |
| `NATS_URL` | `nats://nats:4222` | Queue de jobs. |
| `CLICKHOUSE_URL` | `http://clickhouse:8123` | Analytics. |
| `YOUTUBE_API_KEY` | `your-server-youtube-api-key` | Clé YouTube côté serveur uniquement. |
| `SCAN_INTERVAL_MINUTES` | `30` | Intervalle worker. |

### Production

Utiliser :

```bash
cp .env.production.example .env
```

Puis renseigner :

```txt
SECRET_KEY
DATABASE_URL
CLICKHOUSE_PASSWORD
YOUTUBE_API_KEY
STRIPE_SECRET_KEY
STRIPE_WEBHOOK_SECRET
STRIPE_PRICE_PRO_MONTHLY
STRIPE_PRICE_STUDIO_MONTHLY
```

---

## 💳 Offres SaaS

| Plan | Prix | Cible | Limite | Fonctionnalités |
| --- | ---: | --- | --- | --- |
| 🟢 **Free** | **0 €** | Découverte | 3 tendances / jour | Radar limité, stats basiques. |
| 🔵 **Pro** | **10 €/mois** | Créateurs | Illimité | Filtres, favoris, vues/h, historique 7 jours. |
| 🟣 **Studio** | **18 €/mois** | Agences / avancé | Illimité | Alertes, rapports, exports, historique 90 jours, scores avancés. |

### Règles backend attendues

```txt
Free   → 3 tendances visibles / jour
Pro    → tendances illimitées + stats standards
Studio → stats avancées + alertes + rapports + exports
```

Le contrôle des limites doit toujours se faire côté backend.

---

## 🧭 Navigation produit

### 🌍 Espace public

| Route | Objectif |
| --- | --- |
| `/` | Landing page SaaS. |
| `/pricing` | Offres Free / Pro / Studio. |
| `/login` | Connexion. |
| `/register` | Création de compte. |
| `/privacy` | Politique de confidentialité. |
| `/terms` | Conditions d’utilisation. |
| `/cookies` | Politique cookies. |

### 🔒 Espace connecté

| Route | Objectif |
| --- | --- |
| `/radar` | Radar du jour, page principale. |
| `/dashboard` | Vue plateforme / compat MVP. |
| `/favorites` | Tendances sauvegardées. |
| `/subscription` | Plan, paiement, portail client. |
| `/settings` | Profil, préférences, RGPD. |
| `/settings/privacy` | Consentements et confidentialité. |
| `/settings/data` | Export / suppression des données. |

---

## 🎨 Branding et thème

Deux assets de marque sont disponibles :

| Asset | Usage |
| --- | --- |
| `frontend/static/brand/trend-scope-logo-light.png` | Logo pour fond clair. |
| `frontend/static/brand/trend-scope-logo-dark.png` | Logo pour fond sombre. |

### Mode sombre / clair

| Fonction | Statut |
| --- | --- |
| Détection préférence système | ✅ |
| Toggle manuel | ✅ |
| Persistance `localStorage` | ✅ |
| Variables CSS globales | ✅ |
| Logo clair / sombre | ✅ |

### Direction artistique

✅ À privilégier :

```txt
social analytics premium
clair
simple
creator-friendly
data lisible
business accessible
```

❌ À éviter :

```txt
robots
néons IA génériques
fond violet/bleu excessif
dashboard cyber
jargon technique
interface trop chargée
```

---

## 🔒 Sécurité et RGPD

### Clé YouTube

> 🚨 La clé YouTube ne doit jamais être exposée au frontend.

| Règle | Statut |
| --- | --- |
| `YOUTUBE_API_KEY` côté serveur uniquement | ✅ |
| Pas de clé dans `VITE_*` | ✅ |
| Pas de clé dans le README | ✅ |
| Pas de clé dans les logs | ✅ |
| Pas de scan déclenché par utilisateur non-admin | Objectif API |

### RGPD minimal

| Besoin | Implémentation prévue |
| --- | --- |
| Consentements | `consents` |
| Audit | `audit_logs` |
| Export des données | `data_export_requests` |
| Suppression de compte | `account_deletion_requests` |
| Pages légales | `/privacy`, `/terms`, `/cookies` |
| Paramètres confidentialité | `/settings/privacy`, `/settings/data` |

### Données financières

Aucune donnée bancaire brute ne doit être stockée localement.

Stockage autorisé :

```txt
stripe_customer_id
stripe_subscription_id
plan
status
current_period_start
current_period_end
cancel_at_period_end
```

---

## 🧠 Moteur de tendance

Le produit doit détecter ce qui **accélère maintenant**, pas seulement ce qui est déjà populaire.

| Métrique | Rôle |
| --- | --- |
| ⚡ `views_per_hour` | Vues moyennes gagnées par heure. |
| 🚀 `velocity_score` | Vitesse de progression récente. |
| 🔥 `trend_score` | Potentiel viral global. |
| 🕒 `freshness_score` | Fraîcheur de l’opportunité. |
| 🧊 `saturation_score` | Tendance déjà saturée ou non. |
| 💎 `opportunity_score` | Intérêt à créer maintenant. |
| 🏷️ `category_rank` | Rang dans une catégorie. |
| 🌍 `region_rank` | Rang dans une région. |
| 📡 `cross_platform_score` | Signal sur plusieurs plateformes. |

Exemple d’affichage :

```txt
Trend Score : 92/100
Opportunité : Très forte
Saturation : Faible
```

---

## 🧩 API backend

| Méthode | Route | Accès | Description |
| --- | --- | --- | --- |
| `GET` | `/api/v1/health` | Public | Santé API. |
| `GET` | `/api/v1/ready` | Public / Ops | Readiness infra. |
| `GET` | `/api/v1/auth/status` | Public | État auth. |
| `POST` | `/api/v1/auth/login` | Public | Connexion. |
| `POST` | `/api/v1/auth/register` | Public | Création compte. |
| `GET` | `/api/v1/plans` | Public | Plans Free / Pro / Studio. |
| `GET` | `/api/v1/radar/daily` | Auth | Radar avec limites de plan. |
| `GET` | `/api/v1/videos` | Admin | Liste brute vidéos. |
| `POST` | `/api/v1/videos/scan` | Admin | Scan plateforme côté serveur. |
| `GET` | `/api/v1/billing/status` | Auth | État abonnement. |
| `POST` | `/api/v1/billing/checkout` | Auth | Stripe Checkout. |
| `POST` | `/api/v1/billing/portal` | Auth | Portail client Stripe. |
| `POST` | `/api/v1/billing/webhook` | Stripe | Webhook Stripe. |
| `GET` | `/api/v1/me` | Auth | Profil utilisateur. |
| `PATCH` | `/api/v1/me` | Auth | Mise à jour profil. |
| `GET` | `/api/v1/me/consents` | Auth | Consentements. |
| `POST` | `/api/v1/me/consents` | Auth | Mise à jour consentements. |
| `POST` | `/api/v1/me/data-export` | Auth | Demande export RGPD. |
| `POST` | `/api/v1/me/delete-request` | Auth | Demande suppression compte. |

---

## 🗄️ Données

PostgreSQL stocke le transactionnel.

| Table | Rôle |
| --- | --- |
| `users` | Comptes, rôles, plans. |
| `user_profiles` | Profil utilisateur. |
| `user_preferences` | Onboarding et préférences. |
| `plans` | Free / Pro / Studio. |
| `subscriptions` | Abonnements Stripe. |
| `user_usage_daily` | Limites quotidiennes. |
| `trend_views` | Tendances consultées. |
| `favorites` | Favoris. |
| `watchlists` | Niches, mots-clés, plateformes suivies. |
| `consents` | Consentements RGPD. |
| `audit_logs` | Journal d’actions. |
| `data_export_requests` | Exports RGPD. |
| `account_deletion_requests` | Suppression compte. |

ClickHouse stocke l’analytics massif :

| Table | Rôle |
| --- | --- |
| `trend_events` | Événements de collecte et métriques. |
| `trend_rankings_hourly` | Classements horaires. |

---

## 📊 Observabilité

Dossier `infra/` préparé pour :

```txt
Prometheus
Grafana
Loki
```

Métriques cibles :

| Métrique | Pourquoi |
| --- | --- |
| Latence API | Surveiller l’expérience utilisateur. |
| Erreurs API | Détection incidents. |
| Jobs NATS | Suivi des traitements. |
| Cache hit Redis | Performance radar. |
| Connexions PgBouncer | Capacité DB. |
| Quotas YouTube | Prévenir blocage API. |
| Volume ClickHouse | Suivi analytics. |
| Utilisateurs actifs | Pilotage SaaS. |

---

## 🧪 Qualité et CI

Workflow GitHub Actions attendu :

| Job | Commandes |
| --- | --- |
| `backend` | `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` |
| `frontend` | `npm ci`, `npm run check`, `npm run build` |
| `docker-smoke` | `docker compose build`, `docker compose up -d`, curl `/health`, curl `/auth/status` |

### Tests locaux

```bash
cd backend
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

```bash
cd frontend
npm ci
npm run check
npm run build
```

```bash
cp .env.example .env
docker compose build --no-cache
docker compose up -d
curl -fsS http://localhost:4443/api/v1/health
curl -fsS http://localhost:4443/api/v1/auth/status
docker compose down -v
```

---

## 🛣️ Roadmap

| Phase | Objectif | Priorité |
| --- | --- | --- |
| 1️⃣ SaaS YouTube MVP | Radar, plans, quotas Free, dashboard clair | Haute |
| 2️⃣ Monétisation | Stripe Checkout, portail, webhooks | Haute |
| 3️⃣ Studio Intelligence | Alertes, rapports, exports, watchlists | Moyenne |
| 4️⃣ TikTok | Source TikTok, classement dédié | Moyenne |
| 5️⃣ Instagram | Reels, analytics visuels | Moyenne |
| 6️⃣ Scale | Multi-workers, monitoring, cache avancé | Continue |

---

## 🏁 Objectif final

<p align="center">
  <strong>Construire un SaaS social analytics simple, premium et monétisable, capable de détecter les tendances vidéo avant saturation.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Free-3%20tendances%20%2F%20jour-22C55E?style=for-the-badge" alt="Free plan" />
  <img src="https://img.shields.io/badge/Pro-10%E2%82%AC%20%2F%20mois-2563EB?style=for-the-badge" alt="Pro plan" />
  <img src="https://img.shields.io/badge/Studio-18%E2%82%AC%20%2F%20mois-7C3AED?style=for-the-badge" alt="Studio plan" />
</p>

---

## 🧱 Business pages SaaS (mise à niveau)

Cette base inclut désormais :

- Landing page publique SaaS (The Trend Scope + proposition de valeur).
- Pricing commercial complet avec fallback local si API indisponible.
- Subscription opérationnelle (status, checkout, portal, fallback Stripe non configuré).
- Pages checkout `success` / `cancel`.
- AppShell connecté (Topbar + Sidebar) pour Radar, Favoris, Watchlist, Alertes, Rapports et Settings.
- Settings RGPD (`/settings/profile`, `/settings/privacy`, `/settings/data`).
- Admin panel préparé (`/admin`, `/admin/users`, `/admin/sources`, `/admin/jobs`, `/admin/billing`, `/admin/system`).
- Routes backend business: favorites et admin.

### Stripe (état)

- Si variables Stripe absentes: `enabled=false` + message `billing is not configured yet`.
- Si configurées: endpoints checkout/portal renvoient un état prêt (`enabled=true`).

### Commandes de démarrage / tests

```bash
# Frontend
cd frontend
npm ci
npm run check
npm run build

# Backend
cd ../backend
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# Stack complète
cd ..
docker compose build
cp .env.example .env
docker compose up -d
curl -fsS http://localhost:4443/api/v1/health
curl -fsS http://localhost:4443/api/v1/auth/status
docker compose down -v
```

## Production & Monetization update

Stripe checkout/portal/webhook, user role+plan, dynamic radar filters, favorites API integration, watchlists, alerts, reports, forgot/reset password, readiness and metrics endpoints are now wired in the app.

## 🚀 Production avec Traefik + Cloudflare

La production officielle utilise:

- Cloudflare DNS / Proxy / WAF
- Cloudflare SSL/TLS Full (strict)
- Traefik v3 comme reverse proxy Docker
- Let’s Encrypt DNS-01 via Cloudflare
- Frontend sur `https://thetrendscope.com`
- API sur `https://api.thetrendscope.com`
- Dashboard Traefik protégé sur `https://traefik.thetrendscope.com`

```bash
cp .env.production.example .env.production
nano .env.production
chmod +x scripts/prod-*.sh
./scripts/prod-up.sh
./scripts/prod-check.sh
```

> ⚠️ Nginx n'est plus le reverse proxy production officiel.
>
> ⚠️ Ne jamais exposer PostgreSQL, Redis, NATS ou ClickHouse publiquement.
> Seul Traefik doit exposer les ports 80 et 443.

## 🏁 Avant lancement réel

Le moteur applicatif est prévu pour tourner sans secrets dans le repo. Avant lancement, ajouter dans `.env.production` :

| Secret | Rôle |
| --- | --- |
| `CF_DNS_API_TOKEN` | Certificats Let’s Encrypt DNS-01 via Cloudflare |
| `YOUTUBE_API_KEY` | Collecte YouTube côté serveur |
| `TIKTOK_API_KEY` | Future collecte TikTok |
| `INSTAGRAM_API_KEY` | Future collecte Instagram |
| `STRIPE_SECRET_KEY` | Paiements Stripe |
| `STRIPE_WEBHOOK_SECRET` | Webhooks Stripe |
| `SMTP_*` | Emails transactionnels |
| `SECRET_KEY` | JWT |
| `POSTGRES_PASSWORD` | Base PostgreSQL |

Commande de vérification :

```bash
./scripts/preflight-prod.sh
```

## ⚠️ Services internes

Ne jamais exposer publiquement :

- PostgreSQL
- PgBouncer
- Redis
- NATS
- ClickHouse

Seul Traefik expose `80` et `443`.


## État fonctionnel actuel

| Module | État |
| --- | --- |
| YouTube | Fonctionnel côté serveur |
| TikTok | Bientôt disponible |
| Instagram | Bientôt disponible |
| Radar | Fonctionnel avec filtre plateforme |
| Admin panel | Connecté aux vrais endpoints |
| Watchlists | Fonctionnelles, keywords / plateformes / régions |
| Alertes Web | Notifications in-app |
| Alertes Email | Fonctionnelles si SMTP configuré |
| Alertes Telegram | Fonctionnelles si TELEGRAM_BOT_TOKEN et chat ID configurés |
| Rapports JSON | Fonctionnels |
| Rapports CSV | Export local MVP |
| Rapports PDF | Préparé, à venir |
| Stripe | Implémenté, à tester live |
| Cloudflare/Traefik | Préparé production |


Note: `webhook_url` est conservé uniquement par compatibilité DB mais n'est pas utilisé dans le scope go-live.

## CI officielle

| Workflow | Rôle |
| --- | --- |
| `CI` | Backend, frontend, scripts, Docker smoke, prod compose et guards produit |

Un ancien workflow de diagnostic a été supprimé pour garder une seule CI officielle.

### Déclenchements

La CI se lance sur :

- `workflow_dispatch`
- `push` vers `main`, `codex/**`, `feature/**`, `fix/**`
- `pull_request` vers `main`

## Go-live : tests d’exploitation
1. Vérifier que `CI` se lance.
2. Déployer sur VPS.
4. Configurer `.env.production`.
5. Lancer `./scripts/preflight-prod.sh`.
6. Tester `/api/v1/health`.
7. Tester `/api/v1/ready`.
8. Depuis `/admin/ops`, tester SMTP.
9. Depuis `/admin/ops`, tester Telegram.
10. Créer une alerte web.
11. Vérifier notification in-app.
12. Créer un rapport CSV.
13. Télécharger le CSV.
14. Tester Stripe CLI.


### Admin go-live cockpit

Le cockpit admin expose :

- `/admin` : vue globale SaaS
- `/admin/ops` : tests SMTP, Telegram, YouTube et Stripe
- `/admin/system` : état runtime, services et intégrations
- `/admin/billing` : abonnements et MRR estimé
- `/admin/go-live` : checklist finale avant VPS

## Préproduction VPS

Avant ouverture publique, valider :

1. CI verte.
2. Docker smoke vert.
3. `bash -n` sur tous les scripts.
4. `.env.production` complet.
5. `./scripts/preflight-prod.sh`.
6. `SKIP_REMOTE_CHECKS=1 ./scripts/go-live-check.sh`.
7. `./scripts/prod-up.sh`.
8. `./scripts/prod-check.sh`.
9. `./scripts/go-live-check.sh`.
10. `/admin/system`.
11. `/admin/ops`.
12. `/admin/billing`.
13. `/admin/go-live`.
14. `/metrics`.

## Validation préproduction

La CI est le premier filtre obligatoire.

Avant déploiement VPS :
1. Vérifier que le workflow `CI` est vert.
2. Préparer `.env.production`.
3. Lancer `SKIP_REMOTE_CHECKS=1 ./scripts/go-live-check.sh`.
4. Déployer avec `./scripts/prod-up.sh`.
5. Lancer `./scripts/prod-check.sh`.
6. Lancer `./scripts/go-live-check.sh`.
7. Ouvrir `/admin/system`.
8. Ouvrir `/admin/ops`.
9. Lancer le smoke interne.
10. Tester YouTube, Stripe, SMTP et Telegram.
11. Vérifier `/admin/go-live`.


## Cockpit admin

Le cockpit admin est accessible aux comptes `admin`.

| Page | Usage |
| --- | --- |
| `/admin` | Vue globale SaaS |
| `/admin/system` | Runtime, services internes et intégrations |
| `/admin/billing` | Abonnements, MRR estimé et état Stripe |
| `/admin/ops` | Tests SMTP, Telegram, YouTube, Stripe et smoke interne |
| `/admin/go-live` | Checklist préproduction avant VPS |

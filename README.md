# 🎥 Viral Radar – YouTube & TikTok (WIP)

Un outil auto‑hébergé pour repérer et suivre les vidéos virales YouTube (TikTok à venir) avec stats, historique, filtres thématiques et notifications locales. Conçu pour un usage personnel sur port **4443** avec backend Axum + PostgreSQL et frontend SvelteKit.

## ✨ Points forts
- 🔐 Authentification par mot de passe (16+ caractères, support UTF‑8) avec JWT sécurisé
- 📈 Classement par vues/heure, badge **Short** ≤ 60s, filtres par catégorie
- 📝 Historique persistant, notes personnelles et rafraîchissement à la demande
- 🔔 Notifications locales (toasts + panneau) avec activation/désactivation
- 🐳 Déploiement Docker (backend, frontend, base PostgreSQL)

## 🧱 Architecture
```
/backend        → Axum + sqlx + JWT, API REST /api/v1
/frontend       → SvelteKit (pastel bleu/violet), login + dashboard + historique
/db/migrations  → Schéma PostgreSQL (vidéos, stats, notes)
Dockerfile      → Build multi-étapes (backend)
frontend/Dockerfile → Build/preview SvelteKit
Docker-compose.yml  → Orchestration backend/frontend/db (port 4443 exposé)
.env.example    → Variables d'environnement
```

## ⚙️ Pré-requis
- Docker & Docker Compose
- (Optionnel) Rust + Node 20 si vous voulez développer hors conteneur

<<<<<<< codex/generate-personal-video-tracking-project-zqushc
## ✅ Checklist « ça démarre du premier coup »
Avant `docker compose up -d`, vérifiez :
- `.env` est présent et bien rempli (`APP_USERNAME`, `APP_PASSWORD` ≥ 16 caractères, `APP_SECRET`, `YOUTUBE_API_KEY`, `DATABASE_URL`, `FRONTEND_ORIGIN`).
- Le port `4443` n’est pas occupé (backend) et `5173` libre (frontend exposé).
- Votre machine peut accéder à l’API YouTube (si le réseau bloque, le backend renverra un message d’erreur dans les logs).
- Le volume Docker `pgdata` est vierge ou cohérent : si vous changez le schéma, lancez `docker compose down -v` pour repartir proprement.
- Si vous développez hors Docker, installez les dépendances : `cargo build` dans `backend`, `npm install` dans `frontend`.

## 🔑 Configuration
1. Copier le modèle : `cp .env.example .env`
2. Définir les variables ci‑dessous (mêmes noms pour Docker et le développement local) :

| Variable | Description | Exemple |
| --- | --- | --- |
| `YOUTUBE_API_KEY` | Clé API YouTube v3 | `AIza...` |
| `APP_USERNAME` | Identifiant de connexion (UTF‑8) | `admin` |
| `APP_PASSWORD` | Mot de passe 16+ caractères (UTF‑8) | `m0tDeP@55€安全` |
| `APP_SECRET` | Secret JWT HMAC | `super-long-random-string` |
| `FRONTEND_ORIGIN` | Origine autorisée pour les cookies/JWT | `http://localhost:5173` |
| `DATABASE_URL` | Chaîne Postgres (par défaut `db` en Docker) | `postgres://postgres:postgres@db:5432/viral` |

💡 Si vous développez hors Docker, gardez les mêmes variables pour éviter les écarts entre environnements.
=======
## 🔑 Configuration
1. Copier le modèle : `cp .env.example .env`
2. Définir :
   - `YOUTUBE_API_KEY`
   - `AUTH_USERNAME` / `AUTH_PASSWORD` (16+ caractères, peut inclure caractères spéciaux/japonais)
   - `JWT_SECRET`
   - `DATABASE_URL` (par défaut vers le service `db` en Docker)
------> main

## 🚀 Déploiement rapide (Docker)
```bash
docker compose build
docker compose up -d
```
- Backend : http://localhost:4443/api/v1
- Frontend : http://localhost:5173 (cible l’API backend)
- Postgres : volume `pgdata`, migrations appliquées depuis `db/migrations`

### Vérifier / arrêter
```bash
docker compose ps
docker compose logs -f backend
# …
docker compose down        # stop
docker compose down -v     # stop + reset base
```

## 🧪 Tests
Backend (dans /backend) :
```bash
cargo test
```
Frontend (dans /frontend) :
```bash
npm install
npm run check
npm run test              # Playwright ou tests Svelte
```

## 🛠️ Développement local (sans Docker)
Backend :
```bash
cd backend
cp ../.env.example .env    # ou configurez vos variables
cargo run
```
Frontend :
```bash
cd frontend
npm install
npm run dev -- --host
```
Assurez-vous que `VITE_API_BASE` pointe vers votre backend (par ex. http://localhost:4443/api/v1).

## 🔮 Roadmap
- Intégration TikTok Trends
- Dashboard analytics avancé
- Export CSV/JSON
- Notifications Telegram/email
- Mode multi-comptes et thèmes light/dark

## 🤝 Contribution
Projet pensé pour un usage personnel ; issues et suggestions bienvenues. Merci de respecter le style pastel et l’auth unique lors des contributions.

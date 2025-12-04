# 🎥 Viral Radar – YouTube & TikTok (WIP)

Outil auto‑hébergé pour repérer et suivre les vidéos YouTube (TikTok à venir) via un backend Axum + Supabase/PostgreSQL et un frontend SvelteKit. Tout se lance en Docker avec `docker compose`.

## 🧱 Architecture
| Bloc | Détails |
| --- | --- |
| Backend | Axum + sqlx, JWT HMAC, connexion PostgreSQL hébergée sur Supabase (TLS forcé) |
| Backend | Axum + sqlx, JWT HMAC, connexion PostgreSQL hébergée sur Supabase |
| Frontend | SvelteKit (build + preview), consomme l’API `/api/v1` |
| Base | Tables `videos`, `video_stats`, `users` (script SQL dans `db/migrations/init.sql`) |
| Conteneurs | `backend` (port 4443), `frontend` (port 5173) |

## ⚙️ Configuration environnement
Copiez `.env.example` en `.env`, puis ajustez les valeurs si besoin :

| Variable | Rôle | Valeur par défaut dans `.env.example` |
| --- | --- | --- |
| `APP_USERNAME` | Identifiant du compte seed | `zakamon` |
| `APP_PASSWORD` | Mot de passe seed (≥10 caractères) | `Azerty1234$` |
| `SECRET_KEY` | Secret HMAC JWT (généré, voir ci‑dessous) | `08f0f56770086327107e33189b7e584d` |
| `YOUTUBE_API_KEY` | Clé API YouTube v3 | `AIzaSyBTs5Y2tNY2ZvEB5FJgp_ED2QXb3djvRik` |
| `SUPABASE_URL` | URL du projet Supabase | `https://ltxjjnzsphhprykuwwye.supabase.co` |
| `SUPABASE_ANON_KEY` | Clé publique Supabase (utilisation front) | `…AR4MHCGy…` |
| `SUPABASE_SERVICE_ROLE_KEY` | Clé service pour exécuter les migrations | `…MTSelIYv…` |
| `DATABASE_URL` | Chaîne Postgres Supabase (ajoutez `sslmode=require`) | `postgresql://postgres:<service_role>@db.ltxjjnzsphhprykuwwye.supabase.co:5432/postgres?sslmode=require` |
| `DATABASE_URL` | Chaîne Postgres Supabase | `postgresql://postgres:<service_role>@db.ltxjjnzsphhprykuwwye.supabase.co:5432/postgres` |
| `REGIONS` | Liste des régions à scanner | `FR,US,ES` |
| `THEMES` | Thèmes de recherche séparés par des virgules | `nourriture,voiture,business,drôle,influenceurs` |
| `FRONTEND_ORIGIN` | Origine autorisée côté frontend | `http://localhost:5173` |

### Générer un `SECRET_KEY`
Vous pouvez régénérer une clé HMAC aléatoire en hexadécimal (64 caractères) :

| Commande | Description |
| --- | --- |
| `openssl rand -hex 32` | Génère un secret compatible JWT à copier dans `SECRET_KEY` |

## 🗄️ Préparer Supabase
1. Ouvrez l’onglet **SQL Editor** dans Supabase et exécutez le contenu de `db/migrations/init.sql` pour créer les tables (le backend exécute aussi ce script au démarrage s’il manque du schéma).
2. Vérifiez que la base est accessible via l’URL `DATABASE_URL` (rôle service). Gardez ce DSN dans `.env` pour le backend ; ajoutez `sslmode=require` pour Supabase.
1. Ouvrez l’onglet **SQL Editor** dans Supabase et exécutez le contenu de `db/migrations/init.sql` pour créer les tables.
2. Vérifiez que la base est accessible via l’URL `DATABASE_URL` (rôle service). Gardez ce DSN dans `.env` pour le backend.

## 🚀 Lancer avec Docker (obligatoire)
```
docker compose build
docker compose up -d
```

| Service | URL | Notes |
| --- | --- | --- |
| Backend | http://localhost:4443/api/v1 | Auth + vidéos |
| Frontend | http://localhost:5173 | Tableau de bord, login |

Pour suivre les logs :
```
docker compose logs -f backend
```
Arrêter les services :
```
docker compose down
```

## 🔐 Authentification
- Si aucun utilisateur n’existe, le compte seed (`APP_USERNAME`/`APP_PASSWORD`) est créé automatiquement au démarrage.
- Les mots de passe doivent contenir au moins 10 caractères.

## 🧪 Tests
Backend (hors Docker, nécessite Rust) :
```
cargo test
```

Frontend (hors Docker, nécessite Node 20) :
```
npm install
npm run check
```

## 📚 Sources utilisées
| Sujet | Lien |
| --- | --- |
| SQL Supabase / Postgres distant | https://supabase.com/docs/guides/database/connecting-to-postgres |
| Génération de secrets avec OpenSSL | https://www.openssl.org/docs/manmaster/man1/openssl-rand.html |

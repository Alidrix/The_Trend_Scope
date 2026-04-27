# Viral Radar

Viral Radar est un dashboard auto-hébergé de veille virale YouTube (avec base PostgreSQL Supabase), backend Rust/Axum et frontend SvelteKit.

## Stack
- Backend: Rust, Axum, SQLx, PostgreSQL
- Frontend: SvelteKit, TypeScript, Vite
- Infra: Docker Compose

## Démarrage rapide
```bash
cp .env.example .env
docker compose build --no-cache
docker compose up -d
docker compose ps
```

Services:
- Frontend: http://localhost:5173
- Backend: http://localhost:4443/api/v1
- Health: http://localhost:4443/api/v1/health

## Variables d'environnement
Utilisez `.env.example` comme base (aucun secret réel).

Variables clés:
- `APP_USERNAME`, `APP_PASSWORD`, `SECRET_KEY`
- `DATABASE_URL` (avec `sslmode=require`)
- `YOUTUBE_API_KEY`
- `REGIONS`, `THEMES`
- `FRONTEND_ORIGIN`

## API
- `GET /api/v1/health`
- `GET /api/v1/auth/status`
- `POST /api/v1/auth/register`
- `POST /api/v1/auth/login`
- `GET /api/v1/videos` (auth)
- `POST /api/v1/videos` (auth, compat historique)
- `POST /api/v1/videos/scan` (auth, scan YouTube réel)
- `POST /api/v1/notes` (auth)

Réponse scan:
```json
{
  "message": "scan completed",
  "inserted": 12,
  "updated": 8,
  "total": 20
}
```

## Qualité locale
```bash
cd backend
cargo fmt --check
cargo clippy -- -D warnings
cargo test

cd ../frontend
npm ci
npm run check
npm run build

cd ..
docker compose build
```

## CI
Workflow GitHub Actions: `.github/workflows/ci.yml`
- job backend (fmt, clippy, test)
- job frontend (npm ci, check, build)
- job docker (`docker compose build`)

## Troubleshooting
- Si backend ne démarre pas: vérifier `DATABASE_URL` et accès réseau Supabase.
- Si scan vide: vérifier `YOUTUBE_API_KEY`, quotas API, `REGIONS` et `THEMES`.
- Si frontend ne joint pas l'API: vérifier `VITE_API_BASE` et `docker compose ps`.

## Roadmap courte
- Ajouter scan TikTok côté backend
- Ajouter pagination / tri avancé
- Alertes automatiques (webhook/email)

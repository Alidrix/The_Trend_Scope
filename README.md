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

## Procédure complète — Démarrer l'infrastructure et accéder à la plateforme

Cette procédure permet de démarrer toute l'infrastructure Docker et d'accéder à l'interface web Viral Radar.

### 1. Cloner le projet

```bash
git clone https://github.com/Alidrix/Youtube_Tiktok_WEB.git
cd Youtube_Tiktok_WEB
```

Si vous travaillez depuis la branche de développement de la PR :

```bash
git checkout codex/fix-docker-backend-build-issues
```

### 2. Préparer le fichier d'environnement

Créer le fichier `.env` à partir du modèle :

```bash
cp .env.example .env
```

Éditer ensuite le fichier :

```bash
nano .env
```

Variables minimales à vérifier avant le lancement :

| Variable | Rôle | Exemple attendu |
| --- | --- | --- |
| `APP_USERNAME` | Identifiant du compte initial | `admin` |
| `APP_PASSWORD` | Mot de passe du compte initial | mot de passe robuste |
| `SECRET_KEY` | Secret utilisé pour signer les JWT | résultat de `openssl rand -hex 32` |
| `DATABASE_URL` | Connexion PostgreSQL Supabase | doit contenir `sslmode=require` |
| `YOUTUBE_API_KEY` | Clé API YouTube Data API v3 | clé générée côté Google Cloud |
| `REGIONS` | Régions à scanner | `FR,US,ES` |
| `THEMES` | Thèmes à surveiller | `business,drole,voiture` |
| `FRONTEND_ORIGIN` | Origine autorisée du frontend | `http://localhost:5173` |

Générer un secret applicatif si besoin :

```bash
openssl rand -hex 32
```

> Ne jamais commiter le fichier `.env`. Il doit rester local à l'environnement de déploiement.

### 3. Construire les images Docker

```bash
docker compose build --no-cache
```

Cette commande construit :

| Service | Description |
| --- | --- |
| `backend` | API Rust/Axum exposée sur le port `4443` |
| `frontend` | Interface SvelteKit exposée sur le port `5173` |

### 4. Démarrer l'infrastructure

```bash
docker compose up -d
```

Vérifier l'état des conteneurs :

```bash
docker compose ps
```

État attendu :

```txt
backend    running / healthy
frontend   running
```

### 5. Vérifier que l'API répond

```bash
curl -i http://localhost:4443/api/v1/health
```

Réponse attendue :

```json
{
  "message": "ok"
}
```

Vérifier aussi l'état de l'authentification :

```bash
curl -i http://localhost:4443/api/v1/auth/status
```

### 6. Accéder à la plateforme

Ouvrir le navigateur sur :

```txt
http://localhost:5173
```

Puis :

1. ouvrir la page de connexion ;
2. se connecter avec les valeurs définies dans `.env` :
   - identifiant : `APP_USERNAME` ;
   - mot de passe : `APP_PASSWORD` ;
3. accéder au dashboard ;
4. cliquer sur **Scanner maintenant** pour lancer un scan YouTube ;
5. vérifier que des vidéos remontent dans le tableau de bord.

### 7. Consulter les logs

Logs complets :

```bash
docker compose logs -f
```

Logs backend uniquement :

```bash
docker compose logs -f backend
```

Logs frontend uniquement :

```bash
docker compose logs -f frontend
```

### 8. Redémarrer ou arrêter l'infrastructure

Redémarrer les services :

```bash
docker compose restart
```

Arrêter les services :

```bash
docker compose down
```

Rebuild complet après modification du code :

```bash
docker compose down
docker compose build --no-cache
docker compose up -d
```

### 9. Points de contrôle en cas de problème

| Problème | Vérification |
| --- | --- |
| Le backend ne démarre pas | vérifier `DATABASE_URL`, `sslmode=require` et les logs backend |
| Le frontend ne charge pas | vérifier que le port `5173` est disponible |
| Le dashboard est vide | vérifier `YOUTUBE_API_KEY`, les quotas API et cliquer sur **Scanner maintenant** |
| Le login échoue | vérifier `APP_USERNAME` et `APP_PASSWORD` dans `.env` |
| Le frontend ne contacte pas l'API | vérifier `VITE_API_BASE` dans `docker-compose.yml` |

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

# 🎥 Viral Radar — YouTube & TikTok Intelligence

![Status](https://img.shields.io/badge/status-WIP-orange?style=for-the-badge)
![Backend](https://img.shields.io/badge/backend-Rust%20%2F%20Axum-black?style=for-the-badge)
![Frontend](https://img.shields.io/badge/frontend-SvelteKit-ff3e00?style=for-the-badge)
![Database](https://img.shields.io/badge/database-Supabase%20%2F%20PostgreSQL-3ecf8e?style=for-the-badge)
![Docker](https://img.shields.io/badge/docker-ready-2496ed?style=for-the-badge)

**Viral Radar** est un outil auto-hébergé de veille et d’analyse des vidéos virales.  
L’objectif est de détecter les contenus YouTube à fort potentiel, de suivre leur évolution et de préparer progressivement l’intégration TikTok.

Le projet repose sur :

- un **backend Rust / Axum** pour l’API, l’authentification et la collecte de données ;
- un **frontend SvelteKit** pour le dashboard ;
- une base **Supabase / PostgreSQL** pour stocker les vidéos, statistiques et utilisateurs ;
- un lancement complet via **Docker Compose**.

> ⚠️ Projet en cours de stabilisation. Une refonte technique est prévue pour corriger le build Docker, sécuriser les variables d’environnement, structurer le backend et ajouter un vrai scan YouTube côté API.

---

## 📚 Sommaire

- [Architecture](#-architecture)
- [Structure du projet](#-structure-du-projet)
- [Prérequis](#-prérequis)
- [Configuration](#-configuration)
- [Lancement avec Docker](#-lancement-avec-docker)
- [Commandes utiles](#-commandes-utiles)
- [API actuelle](#-api-actuelle)
- [Sécurité](#-sécurité)
- [Dépannage](#-dépannage)
- [Roadmap technique](#-roadmap-technique)
- [Ressources](#-ressources)

---

## 🧱 Architecture

| Bloc | Technologie | Rôle |
| --- | --- | --- |
| Frontend | SvelteKit, Vite, TypeScript | Interface web, login, dashboard, notes |
| Backend | Rust, Axum, Tokio, SQLx | API REST, JWT, accès PostgreSQL, logique métier |
| Base de données | Supabase / PostgreSQL | Stockage des vidéos, statistiques et utilisateurs |
| Authentification | JWT HMAC + bcrypt | Connexion utilisateur et protection des routes |
| Conteneurisation | Docker Compose | Build et lancement du frontend/backend |
| API externe | YouTube Data API v3 | Récupération des vidéos et statistiques |

---

## 📁 Structure du projet

```txt
.
├── backend/
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
│       └── main.rs
├── db/
│   └── migrations/
│       └── init.sql
├── frontend/
│   ├── Dockerfile
│   ├── package.json
│   ├── svelte.config.js
│   └── src/
│       ├── lib/
│       └── routes/
├── docker-compose.yml
├── .env.example
├── .gitignore
└── README.md
```

Structure cible recommandée pour la prochaine refonte backend :

```txt
backend/src/
├── main.rs
├── config.rs
├── error.rs
├── state.rs
├── models/
│   ├── user.rs
│   └── video.rs
├── repositories/
│   ├── users.rs
│   └── videos.rs
├── routes/
│   ├── auth.rs
│   ├── health.rs
│   ├── notes.rs
│   └── videos.rs
└── services/
    ├── auth.rs
    └── youtube.rs
```

---

## ✅ Prérequis

Pour lancer le projet en local :

| Outil | Version recommandée |
| --- | --- |
| Docker | Version récente |
| Docker Compose | v2 recommandé |
| Node.js | 20+ si lancement frontend hors Docker |
| Rust | Stable si lancement backend hors Docker |
| Supabase | Projet PostgreSQL actif |
| YouTube Data API | Clé API valide |

---

## ⚙️ Configuration

Créer un fichier `.env` à la racine du projet :

```bash
cp .env.example .env
```

Exemple propre de configuration attendue :

```env
# Application
APP_USERNAME=admin
APP_PASSWORD=change-me-with-a-strong-password
SECRET_KEY=change-me-with-a-64-hex-secret

# YouTube
YOUTUBE_API_KEY=your-youtube-api-key

# Supabase / PostgreSQL
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_ANON_KEY=your-supabase-anon-key
SUPABASE_SERVICE_ROLE_KEY=your-supabase-service-role-key
DATABASE_URL=postgresql://postgres:<password>@db.<project>.supabase.co:5432/postgres?sslmode=require

# Scan configuration
REGIONS=FR,US,ES
THEMES=nourriture,voiture,business,drole,influenceurs

# Frontend
FRONTEND_ORIGIN=http://localhost:5173
```

Générer un secret JWT robuste :

```bash
openssl rand -hex 32
```

> ⚠️ Ne jamais publier de vraie clé API, clé Supabase ou chaîne `DATABASE_URL` dans GitHub.

---

## 🗄️ Préparer Supabase

La migration SQL se trouve ici :

```txt
db/migrations/init.sql
```

Elle crée les tables principales :

| Table | Rôle |
| --- | --- |
| `users` | Stockage des utilisateurs et hash de mot de passe |
| `videos` | Stockage des vidéos détectées |
| `video_stats` | Historique des statistiques par vidéo |

Pour initialiser manuellement la base :

1. ouvrir le **SQL Editor** dans Supabase ;
2. copier le contenu de `db/migrations/init.sql` ;
3. exécuter le script ;
4. vérifier que `DATABASE_URL` contient bien `sslmode=require`.

---

## 🚀 Lancement avec Docker

Build complet :

```bash
docker compose build --no-cache
```

Lancement :

```bash
docker compose up -d
```

Vérifier les conteneurs :

```bash
docker compose ps
```

Accès aux services :

| Service | URL |
| --- | --- |
| Frontend | http://localhost:5173 |
| Backend healthcheck | http://localhost:4443/api/v1/health |
| API backend | http://localhost:4443/api/v1 |

Arrêter le projet :

```bash
docker compose down
```

---

## 🧰 Commandes utiles

### Logs

```bash
docker compose logs -f
```

```bash
docker compose logs -f backend
```

```bash
docker compose logs -f frontend
```

### Rebuild ciblé

```bash
docker compose build backend --no-cache
```

```bash
docker compose build frontend --no-cache
```

### Relancer un service

```bash
docker compose restart backend
```

```bash
docker compose restart frontend
```

### Tester l’API backend

```bash
curl -i http://localhost:4443/api/v1/health
```

```bash
curl -i http://localhost:4443/api/v1/auth/status
```

### Tests hors Docker

Backend :

```bash
cd backend
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Frontend :

```bash
cd frontend
npm ci
npm run check
npm run build
```

---

## 🔌 API actuelle

| Méthode | Route | Description | Auth requise |
| --- | --- | --- | --- |
| `GET` | `/api/v1/health` | Vérifie que l’API répond | Non |
| `GET` | `/api/v1/auth/status` | Vérifie l’état de configuration | Non |
| `POST` | `/api/v1/auth/register` | Crée le premier utilisateur | Non, uniquement si aucun utilisateur |
| `POST` | `/api/v1/auth/login` | Connexion utilisateur | Non |
| `GET` | `/api/v1/videos` | Liste les vidéos stockées | Oui |
| `POST` | `/api/v1/videos` | Insère ou met à jour des vidéos | Oui |
| `POST` | `/api/v1/notes` | Met à jour les notes d’une vidéo | Oui |

Route cible à ajouter :

```http
POST /api/v1/videos/scan
```

Objectif : appeler réellement l’API YouTube, calculer les vues par heure et alimenter la base.

---

## 🔐 Sécurité

Points importants :

- ne jamais commiter `.env` ;
- ne jamais mettre de vraies clés dans `.env.example` ;
- régénérer immédiatement toute clé exposée publiquement ;
- garder `DATABASE_URL` avec `sslmode=require` pour Supabase ;
- préférer à terme un cookie `HttpOnly`, `Secure`, `SameSite=Lax` au stockage JWT dans `localStorage` ;
- ajouter du rate limiting sur les routes d’authentification ;
- ajouter des healthchecks Docker ;
- ajouter un scan de secrets et dépendances dans la CI.

---

## 🛠️ Dépannage

### Le backend ne build pas

Symptôme possible :

```txt
couldn't read ../../db/migrations/init.sql
```

Cause probable : le dossier `db/` n’est pas copié dans l’image Docker backend.

Correction attendue : construire le backend depuis le contexte racine ou copier explicitement `db/migrations/init.sql` dans l’image.

---

### Connexion Supabase impossible

Vérifier :

```bash
grep DATABASE_URL .env
```

La valeur doit contenir :

```txt
sslmode=require
```

---

### Le frontend affiche une erreur API

Vérifier que le backend répond :

```bash
curl -i http://localhost:4443/api/v1/health
```

Vérifier les logs :

```bash
docker compose logs -f backend
```

---

### Le dashboard reste vide

État actuel : le bouton de rafraîchissement ne déclenche pas encore un vrai scan YouTube.  
La route cible à développer est :

```http
POST /api/v1/videos/scan
```

---

## 🧭 Roadmap technique

### Priorité 1 — Stabilisation

- [ ] Corriger le build Docker backend avec accès à `db/migrations/init.sql`.
- [ ] Nettoyer `.env.example` pour supprimer les secrets réels.
- [ ] Supprimer les doublons de variables d’environnement.
- [ ] Corriger `normalize_database_url()` pour utiliser l’URL normalisée.
- [ ] Ajouter des healthchecks Docker.

### Priorité 2 — Fonctionnel métier

- [ ] Ajouter un service `youtube.rs`.
- [ ] Ajouter une route `POST /api/v1/videos/scan`.
- [ ] Récupérer les vidéos par région et thème.
- [ ] Calculer `views_per_hour`.
- [ ] Stocker l’historique dans `video_stats`.
- [ ] Ajouter des filtres API : région, thème, shorts, période.

### Priorité 3 — Qualité projet

- [ ] Modulariser le backend.
- [ ] Ajouter des tests unitaires et d’intégration.
- [ ] Ajouter GitHub Actions.
- [ ] Ajouter `cargo fmt`, `clippy`, `cargo test`, `npm run check`, `npm run build`.
- [ ] Ajouter Dependabot et secret scanning.

### Priorité 4 — Refonte UI/UX

- [ ] Créer un vrai design system.
- [ ] Ajouter une sidebar dashboard.
- [ ] Ajouter des cartes vidéo avec thumbnails.
- [ ] Ajouter des KPI : vidéos détectées, vues/h moyennes, shorts détectés.
- [ ] Ajouter des graphiques de tendance.
- [ ] Ajouter dark/light mode.
- [ ] Ajouter skeleton loaders et empty states propres.

---

## 🎨 Direction graphique cible

Style recommandé :

- dashboard sombre premium ;
- accents YouTube rouge, TikTok cyan/rose ;
- cartes arrondies avec hiérarchie nette ;
- sidebar fixe ;
- topbar avec état API et bouton scan ;
- animations légères ;
- responsive desktop/tablette/mobile.

Objectif : passer d’un prototype simple à une interface de veille sociale crédible et professionnelle.

---

## 📚 Ressources

| Sujet | Lien |
| --- | --- |
| Axum | https://docs.rs/axum/latest/axum/ |
| SQLx | https://docs.rs/sqlx/latest/sqlx/ |
| SvelteKit | https://kit.svelte.dev/docs |
| Supabase PostgreSQL | https://supabase.com/docs/guides/database |
| YouTube Data API v3 | https://developers.google.com/youtube/v3 |
| Docker Compose | https://docs.docker.com/compose/ |
| GitHub Actions | https://docs.github.com/actions |

---

## ⚠️ Note importante

Ce projet manipule des clés API et une base distante.  
Avant tout déploiement public, vérifier :

- la suppression des secrets du dépôt ;
- la régénération des clés exposées ;
- la configuration CORS ;
- la robustesse de l’authentification ;
- la présence de logs et healthchecks ;
- la protection HTTPS derrière un reverse proxy.

# Viral Radar

Viral Radar est une plateforme de veille virale destinée à détecter les tendances émergentes sur YouTube, puis à terme sur TikTok et Instagram.

L'objectif produit est de rendre accessible, via une interface centralisée et un abonnement payant, les contenus qui performent le mieux dans le monde : vidéos en forte croissance, vues par heure, catégories en vogue, signaux faibles et opportunités de visibilité pour créateurs, influenceurs, agences et marques.

## Stack
- Backend: Rust, Axum, SQLx, PostgreSQL
- Frontend: SvelteKit, TypeScript, Vite
- Infra: Docker Compose
- Base: PostgreSQL / Supabase
- Source actuelle: YouTube Data API v3

## Vision produit

Viral Radar doit évoluer vers une plateforme SaaS publique permettant aux utilisateurs abonnés d'accéder rapidement aux tendances vidéo les plus prometteuses.

Objectifs métier :

- détecter les vidéos YouTube qui progressent le plus vite ;
- calculer des métriques fortes comme les vues par heure, la vélocité et le potentiel viral ;
- identifier les catégories et domaines en vogue ;
- aider les influenceurs, créateurs de contenu, agences social media et marques à s'inspirer des tendances émergentes ;
- centraliser à terme YouTube, TikTok et Instagram dans un seul radar de tendances ;
- monétiser l'accès à la donnée via un système d'abonnement.

Le produit ne doit pas seulement afficher les vidéos populaires déjà connues. Il doit surtout faire remonter les contenus en accélération, c'est-à-dire les tendances qui commencent à émerger avant qu'elles soient saturées.

## Modèle SaaS cible

Architecture cible recommandée pour une exposition publique :

| Bloc | Rôle |
| --- | --- |
| Frontend public | Landing page, présentation de l'offre, inscription, login |
| Dashboard abonné | Accès aux tendances, filtres, statistiques et exports |
| Backend API privé | Agrégation, scoring, authentification, quotas utilisateur |
| Worker de scan | Collecte planifiée des données YouTube/TikTok/Instagram |
| Base de données | Stockage vidéos, stats, catégories, signaux et utilisateurs |
| Paiement | Abonnements mensuels/annuels, gestion des accès |
| Admin | Supervision quotas API, utilisateurs, plans et logs |

Le frontend public pourra être exposé à tous, mais les données avancées doivent rester derrière authentification et abonnement.

## Gestion de la clé API YouTube

Pour un vrai modèle SaaS, le choix recommandé est le suivant :

### Option recommandée — Clé API propriétaire côté serveur

La plateforme utilise une ou plusieurs clés YouTube contrôlées par l'éditeur de Viral Radar. Les clients n'ajoutent pas leur propre clé API.

Avantages :

- expérience utilisateur plus simple ;
- aucun paramétrage technique demandé au client ;
- meilleure maîtrise de la qualité des données ;
- possibilité de créer un cache global partagé entre tous les abonnés ;
- centralisation des quotas, logs et optimisations côté serveur.

Contraintes :

- surveiller les quotas YouTube ;
- éviter les scans inutiles ;
- mettre en cache les résultats ;
- dédupliquer les appels API ;
- prévoir plusieurs niveaux de fréquence de scan selon l'offre commerciale ;
- ne jamais exposer la clé au frontend.

### Option alternative — Clé API fournie par le client

Chaque client ajoute sa propre clé YouTube API dans son espace personnel.

Avantages :

- les quotas sont portés par le client ;
- utile pour des offres entreprise ou self-hosted.

Inconvénients :

- complexifie l'onboarding ;
- demande des connaissances Google Cloud ;
- augmente le support client ;
- moins adapté à une offre SaaS grand public.

### Décision produit recommandée

Pour une plateforme publique à abonnement, utiliser une clé API serveur gérée par Viral Radar. L'option clé client pourra être ajoutée plus tard pour un plan avancé, agence, entreprise ou self-hosted.

## Moteur de détection des tendances

Le moteur de tendance doit évoluer au-delà d'un simple classement par vues.

Métriques recommandées :

| Métrique | Description |
| --- | --- |
| `views_per_hour` | Nombre moyen de vues gagnées par heure depuis la publication |
| `velocity_score` | Score d'accélération basé sur la progression récente |
| `trend_score` | Score global combinant vues, âge, catégorie et croissance |
| `freshness_score` | Priorise les contenus récents encore exploitables |
| `category_rank` | Classement par catégorie ou domaine |
| `region_rank` | Classement par pays ou zone géographique |
| `shorts_ratio` | Part des Shorts dans les tendances détectées |
| `channel_momentum` | Détection des chaînes en forte progression |

Objectif : identifier non seulement ce qui est déjà viral, mais surtout ce qui est en train de le devenir.

## Utilisateurs cibles

| Segment | Besoin |
| --- | --- |
| Influenceurs | Trouver des idées de contenus à fort potentiel |
| Créateurs YouTube/TikTok | Comprendre les formats et sujets qui performent |
| Agences social media | Préparer des recommandations clients basées sur la donnée |
| Marques | Surveiller les tendances de marché et opportunités éditoriales |
| Growth marketers | Identifier rapidement des angles de contenu exploitables |
| Veilleurs / analystes | Suivre les mouvements culturels ou commerciaux émergents |

## Fonctionnalités SaaS à prévoir

Fonctionnalités publiques :

- landing page marketing ;
- page tarifs ;
- inscription ;
- connexion ;
- démonstration limitée ;
- aperçu de quelques tendances gratuites.

Fonctionnalités abonnés :

- dashboard complet ;
- classement mondial ;
- filtres par pays, catégorie, thème, durée, format ;
- statistiques vues par heure ;
- détection des tendances émergentes ;
- historique de progression ;
- favoris ;
- notes privées ;
- exports CSV/PDF ;
- alertes personnalisées ;
- recommandations de niches et catégories.

Fonctionnalités admin :

- gestion des utilisateurs ;
- gestion des plans d'abonnement ;
- suivi des quotas API ;
- supervision des jobs de scan ;
- logs techniques ;
- état des sources YouTube/TikTok/Instagram ;
- désactivation temporaire d'une source en cas de quota ou incident.

## Roadmap produit long terme

### Phase 1 — Stabilisation YouTube

- stabiliser Docker et CI ;
- fiabiliser le scan YouTube ;
- calculer les vues par heure ;
- ajouter les catégories, régions et thumbnails ;
- créer un premier scoring de tendance ;
- améliorer le dashboard.

### Phase 2 — SaaS public

- ajouter une landing page publique ;
- ajouter inscription et gestion utilisateur ;
- ajouter plans d'abonnement ;
- connecter un prestataire de paiement ;
- protéger les routes premium ;
- créer un dashboard abonné ;
- ajouter une limite d'accès selon le plan.

### Phase 3 — Intelligence de tendance

- ajouter un moteur de scoring avancé ;
- historiser la croissance des vidéos ;
- détecter les signaux faibles ;
- ajouter des alertes personnalisées ;
- créer des recommandations par niche ;
- ajouter des rapports automatiques.

### Phase 4 — TikTok

- ajouter une source TikTok ;
- normaliser les données TikTok dans le même modèle que YouTube ;
- créer un classement TikTok ;
- comparer les tendances YouTube et TikTok ;
- détecter les tendances cross-platform.

### Phase 5 — Instagram

- ajouter une source Instagram ;
- intégrer Reels et contenus courts ;
- créer des statistiques par compte, catégorie et format ;
- centraliser YouTube, TikTok et Instagram dans un seul radar.

### Phase 6 — Plateforme complète

- API publique ou API partenaire ;
- webhooks ;
- exports avancés ;
- espace agence ;
- multi-utilisateurs par organisation ;
- dashboard marque blanche ;
- système de recommandations assistées par IA.

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

## Exposition publique et prérequis production

Avant d'exposer la plateforme publiquement, prévoir au minimum :

- reverse proxy HTTPS devant le frontend et le backend ;
- domaine dédié ;
- cookies sécurisés ou stratégie d'authentification renforcée ;
- stockage serveur des clés API ;
- aucun secret dans le navigateur ;
- rate limiting sur les routes sensibles ;
- système d'abonnement et contrôle d'accès ;
- monitoring des quotas YouTube ;
- logs applicatifs centralisés ;
- sauvegardes PostgreSQL ;
- politique de cache pour réduire les appels API ;
- supervision des workers de scan.

La clé YouTube doit rester côté serveur. Elle ne doit jamais être exposée dans le code frontend, les réponses API publiques ou le navigateur.

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
- Stabiliser le score de tendance YouTube
- Ajouter un worker de scan planifié
- Ajouter un système d'abonnement
- Ajouter scan TikTok côté backend
- Ajouter scan Instagram côté backend
- Ajouter pagination / tri avancé
- Alertes automatiques (webhook/email)

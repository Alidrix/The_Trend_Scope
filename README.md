# 🚀 Viral Radar — SaaS public de veille virale

<p align="center">
  <img src="https://img.shields.io/badge/Status-SaaS%20MVP-FF6B00?style=for-the-badge" alt="Status SaaS MVP" />
  <img src="https://img.shields.io/badge/Backend-Rust%20%2F%20Axum-111827?style=for-the-badge&logo=rust" alt="Backend Rust Axum" />
  <img src="https://img.shields.io/badge/Frontend-SvelteKit-FF3E00?style=for-the-badge&logo=svelte&logoColor=white" alt="Frontend SvelteKit" />
  <img src="https://img.shields.io/badge/Database-PostgreSQL%20%2F%20Supabase-3ECF8E?style=for-the-badge&logo=supabase&logoColor=white" alt="Database Supabase PostgreSQL" />
  <img src="https://img.shields.io/badge/Infra-Docker%20Compose-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker Compose" />
</p>

<p align="center">
  <strong>Repère les tendances avant les autres et crée du contenu au bon moment.</strong>
</p>

<p align="center">
  Viral Radar est une plateforme SaaS simple, premium et monétisable pour aider les créateurs, influenceurs, agences social media et marques à détecter les tendances vidéo émergentes avant saturation.
</p>

---

## 📚 Sommaire

- [🎯 Vision produit](#-vision-produit)
- [✨ Positionnement](#-positionnement)
- [💳 Offres SaaS](#-offres-saas)
- [🧭 Navigation produit](#-navigation-produit)
- [🔐 Sécurité API YouTube](#-sécurité-api-youtube)
- [🧠 Moteur de tendance](#-moteur-de-tendance)
- [🧩 Backend API](#-backend-api)
- [🗄️ Données et tables](#️-données-et-tables)
- [🛣️ Roadmap](#️-roadmap)
- [⚙️ Lancement local](#️-lancement-local)

---

## 🎯 Vision produit

> **Viral Radar aide les créateurs, influenceurs et agences à repérer les contenus qui commencent à exploser sur YouTube, TikTok et Instagram, avant qu’ils soient saturés.**

L’objectif est de transformer la donnée brute des plateformes sociales en **opportunités claires et actionnables**.

<table>
  <tr>
    <td><strong>🎥 YouTube</strong></td>
    <td>Détection des vidéos qui gagnent rapidement en vues/heure.</td>
  </tr>
  <tr>
    <td><strong>🎵 TikTok</strong></td>
    <td>Roadmap prévue pour suivre les formats courts émergents.</td>
  </tr>
  <tr>
    <td><strong>📸 Instagram</strong></td>
    <td>Roadmap prévue pour intégrer les Reels et tendances visuelles.</td>
  </tr>
  <tr>
    <td><strong>📈 Social Analytics</strong></td>
    <td>Scores simples pour comprendre rapidement ce qui monte.</td>
  </tr>
</table>

---

## ✨ Positionnement

Viral Radar doit rester **simple, lisible et orienté créateur**.

### ✅ À privilégier

| Axe | Intention |
| --- | --- |
| 🎯 Clair | L’utilisateur comprend immédiatement quoi regarder. |
| ⚡ Rapide | Connexion → Radar du jour → tendances utiles. |
| 💼 Premium | Interface SaaS propre, crédible et monétisable. |
| 📊 Data lisible | Peu de jargon, scores compréhensibles. |
| 🎬 Creator-friendly | Pensé pour créateurs, influenceurs et agences. |

### ❌ À éviter

| À éviter | Pourquoi |
| --- | --- |
| 🤖 Robots / wording trop IA | Le produit ne doit pas faire outil IA générique. |
| 🧪 Dashboard laboratoire | L’utilisateur final n’est pas un data scientist. |
| 🌌 Néons bleu/violet excessifs | Effet SaaS IA générique trop vu. |
| 🧱 Interface trop chargée | Le client doit savoir quoi faire en 5 secondes. |
| 🛡️ DA cyber | Le produit vise le social media, pas la cybersécurité. |

---

## 💳 Offres SaaS

La gamme retenue repose sur **3 plans simples**.

| Plan | Prix | Cible | Accès clé | Positionnement |
| --- | ---: | --- | --- | --- |
| 🟢 **Free** | **0 €** | Découverte | 3 tendances/jour | Tester la valeur du radar |
| 🔵 **Pro** | **10 €/mois** | Créateurs | Illimité + filtres + favoris | Offre recommandée |
| 🟣 **Studio** | **18 €/mois** | Agences / avancé | Stats avancées + rapports | Décision et analyse |

<p align="center">
  <img src="https://img.shields.io/badge/Free-3%20tendances%20%2F%20jour-22C55E?style=flat-square" alt="Free" />
  <img src="https://img.shields.io/badge/Pro-10%E2%82%AC%20%2F%20mois-2563EB?style=flat-square" alt="Pro" />
  <img src="https://img.shields.io/badge/Studio-18%E2%82%AC%20%2F%20mois-7C3AED?style=flat-square" alt="Studio" />
</p>

### 🟢 Free — Découverte

- 3 tendances visibles par jour.
- À terme : 1 YouTube + 1 TikTok + 1 Instagram.
- Données basiques uniquement.
- Pas d’historique avancé.
- Pas d’exports.
- Pas d’alertes.

Message de limite recommandé :

```txt
Tu as consulté tes 3 tendances gratuites du jour.
Passe en Pro pour débloquer toutes les tendances.
```

### 🔵 Pro — 10 €/mois

- Tendances illimitées.
- Filtres par plateforme, pays, catégorie et format.
- Score tendance.
- Vues par heure.
- Favoris.
- Notes privées.
- Historique court, par exemple 7 jours.

> Le plan **Pro** est l’offre principale à mettre en avant.

### 🟣 Studio — 18 €/mois

- Historique 30 à 90 jours.
- Courbes d’évolution.
- Alertes personnalisées.
- Rapports hebdomadaires.
- Exports CSV/PDF.
- Tendances cross-platform.
- Score d’opportunité.
- Score de saturation.
- Top créateurs / chaînes en accélération.
- Watchlist de niches.

> **Promesse Studio :** Ne regarde pas seulement ce qui buzz. Comprends pourquoi ça buzz et quoi publier ensuite.

---

## 🧭 Navigation produit

L’utilisateur doit arriver directement sur le **Radar du jour**, pas sur une page technique.

### 🌍 Espace public

| Route | Objectif |
| --- | --- |
| `/` | Landing page SaaS |
| `/pricing` | Tarifs Free / Pro / Studio |
| `/login` | Connexion / inscription |

### 🔒 Espace connecté

| Route | Objectif |
| --- | --- |
| `/radar` | Radar du jour, arrivée par défaut |
| `/dashboard` | Vue plateforme YouTube MVP |
| `/favorites` | Favoris, préparé |
| `/subscription` | Abonnement, préparé |
| `/alerts` | Alertes Studio, préparé |
| `/reports` | Rapports Studio, préparé |

### 🧑‍💻 Parcours utilisateur cible

```txt
Landing → Inscription / Connexion → Radar du jour → Limite Free → Upgrade Pro / Studio
```

---

## 🔐 Sécurité API YouTube

> 🚨 **Règle absolue : la clé API YouTube ne doit jamais être exposée au frontend.**

La clé YouTube est uniquement côté backend :

- ✅ stockée en variable serveur `YOUTUBE_API_KEY` ;
- ✅ utilisée seulement par l’API backend ;
- ✅ protégée dans `.env` ou secret manager ;
- ❌ jamais dans `VITE_*` ;
- ❌ jamais dans le code frontend ;
- ❌ jamais dans les réponses API ;
- ❌ jamais dans les logs ;
- ❌ jamais dans le README ;
- ❌ jamais dans `.env.example`.

### 🔑 Modèle recommandé

| Modèle | Recommandation | Usage |
| --- | --- | --- |
| ✅ Clé serveur propriétaire | Recommandé | SaaS public standard |
| ⚠️ Clé fournie par le client | Option future | Enterprise / self-hosted |

Pour le SaaS public, Viral Radar doit utiliser **une ou plusieurs clés API YouTube côté serveur**, contrôlées par la plateforme.

---

## 🧠 Moteur de tendance

Le moteur ne doit pas uniquement classer par nombre total de vues. Il doit détecter ce qui **accélère maintenant**.

| Métrique | Description |
| --- | --- |
| ⚡ `views_per_hour` | Nombre moyen de vues gagnées par heure. |
| 🚀 `velocity_score` | Vitesse de progression récente. |
| 🔥 `trend_score` | Potentiel viral global. |
| 🕒 `freshness_score` | Fraîcheur de l’opportunité. |
| 🧊 `saturation_score` | Niveau de saturation de la tendance. |
| 💎 `opportunity_score` | Intérêt à créer du contenu maintenant. |
| 🏷️ `category_rank` | Position dans une catégorie. |
| 🌍 `region_rank` | Position dans un pays ou une région. |
| 📡 `cross_platform_score` | Force d’une tendance sur plusieurs plateformes. |

Exemple d’affichage utilisateur :

```txt
Trend Score : 92/100
Opportunité : Très forte
Saturation : Faible
```

---

## 🧩 Backend API

### ✅ Routes MVP SaaS

| Méthode | Route | Description |
| --- | --- | --- |
| `GET` | `/api/v1/plans` | Liste des offres Free / Pro / Studio. |
| `GET` | `/api/v1/radar/daily` | Radar du jour avec quota selon plan. |
| `GET` | `/api/v1/billing/status` | État de préparation Stripe. |
| `GET` | `/api/v1/health` | Vérification de l’état API. |
| `GET` | `/api/v1/auth/status` | État de l’authentification. |
| `POST` | `/api/v1/auth/login` | Connexion utilisateur. |
| `POST` | `/api/v1/videos/scan` | Scan YouTube côté serveur. |

### 🔒 Contrôle Free côté backend

Le contrôle des limites doit être appliqué côté API, pas seulement dans l’interface.

```txt
user_plan = free | pro | studio
free.daily_trend_limit = 3
pro.daily_trend_limit = unlimited
studio.daily_trend_limit = unlimited
```

---

## 🗄️ Données et tables

Le schéma SaaS doit permettre la gestion des plans, abonnements, limites, favoris et futures fonctionnalités Studio.

| Table / Champ | Rôle |
| --- | --- |
| `users.plan` | Plan actif : `free`, `pro`, `studio`. |
| `subscriptions` | Suivi des abonnements. |
| `user_usage_daily` | Limites quotidiennes du plan Free. |
| `trend_views` | Historique des tendances consultées. |
| `favorites` | Tendances sauvegardées. |
| `watchlists` | Niches, mots-clés ou catégories suivies. |
| `alerts` | Alertes Studio. |
| `reports` | Rapports Studio. |
| `user_preferences` | Onboarding et personnalisation. |

---

## 🎨 Direction artistique

Le rendu doit être **premium, social analytics et créateur-friendly**.

### Palette d’intention

| Plateforme | Accent recommandé |
| --- | --- |
| ▶️ YouTube | Rouge sobre |
| 🎵 TikTok | Cyan / rose contrôlé |
| 📸 Instagram | Dégradé léger |
| 📊 SaaS | Gris doux, blanc, noir profond |

### Inspirations d’esprit

| Produit | Pourquoi |
| --- | --- |
| Spotify for Artists | Analytics créateur clair. |
| YouTube Studio | Référence naturelle pour les créateurs. |
| Stripe Dashboard | SaaS premium et lisible. |
| Linear | Navigation propre et moderne. |
| Notion | Simplicité et lisibilité. |
| Metricool / Later | Positionnement social media. |
| HypeAuditor | Influence marketing et analytics. |

---

## 🛣️ Roadmap

| Phase | Objectif | Statut |
| --- | --- | --- |
| 1️⃣ YouTube SaaS MVP | Plans + limite Free + Radar du jour | En cours |
| 2️⃣ Monétisation | Stripe checkout + portail + webhooks | À venir |
| 3️⃣ Studio Intelligence | Alertes, rapports, scores avancés | À venir |
| 4️⃣ TikTok | Source TikTok + classement dédié | Roadmap |
| 5️⃣ Instagram | Reels + tendances visuelles | Roadmap |
| 6️⃣ Plateforme avancée | API partenaire, agence, multi-users | Roadmap |

---

## ⚙️ Lancement local

### 1️⃣ Préparer l’environnement

```bash
cp .env.example .env
```

### 2️⃣ Construire les images

```bash
docker compose build
```

### 3️⃣ Démarrer les services

```bash
docker compose up -d
```

### 4️⃣ Vérifier les services

```bash
docker compose ps
```

| Service | URL |
| --- | --- |
| 🌐 Frontend | http://localhost:5173 |
| 🧩 Backend API | http://localhost:4443/api/v1 |
| 💚 Healthcheck | http://localhost:4443/api/v1/health |

---

## 🧪 Qualité locale

### Backend

```bash
cd backend
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

### Frontend

```bash
cd frontend
npm ci
npm run check
npm run build
```

### Docker

```bash
docker compose build
```

---

## 🛠️ Troubleshooting

| Problème | Vérification |
| --- | --- |
| 🚫 Backend KO | Vérifier `DATABASE_URL`, `sslmode=require` et les logs backend. |
| 🧱 Frontend KO | Vérifier que le port `5173` est disponible. |
| 📭 Dashboard vide | Vérifier `YOUTUBE_API_KEY`, quotas API et bouton **Scanner maintenant**. |
| 🔐 Login KO | Vérifier `APP_USERNAME` et `APP_PASSWORD`. |
| 🌐 API inaccessible | Vérifier `VITE_API_BASE` et `docker compose ps`. |

---

## 🏁 Objectif final

<p align="center">
  <strong>Construire un SaaS social analytics simple, premium et monétisable pour détecter les tendances vidéo avant saturation.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Free-3%20tendances%20%2F%20jour-22C55E?style=for-the-badge" alt="Free plan" />
  <img src="https://img.shields.io/badge/Pro-10%E2%82%AC%20%2F%20mois-2563EB?style=for-the-badge" alt="Pro plan" />
  <img src="https://img.shields.io/badge/Studio-18%E2%82%AC%20%2F%20mois-7C3AED?style=for-the-badge" alt="Studio plan" />
</p>

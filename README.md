# Viral Radar — SaaS public de veille virale

Viral Radar devient un SaaS simple, premium et monétisable pour aider créateurs, influenceurs, agences et marques à repérer les tendances vidéo émergentes avant saturation.

**Promesse produit:** _Repère les tendances avant les autres et crée du contenu au bon moment._

## Positionnement

- Pas un labo data science.
- Pas un outil IA générique.
- Oui à une interface claire orientée social analytics.
- Oui à un parcours SaaS simple: landing → login/signup → Radar du jour → upgrade.

## Plans

| Plan | Prix | Cible | Accès clé |
| --- | --- | --- | --- |
| Free | 0 € | Découverte | 3 tendances/jour, stats basiques |
| Pro | 10 €/mois | Créateurs | tendances illimitées, filtres, favoris, historique 7 jours |
| Studio | 18 €/mois | Agences / avancé | historique 90 jours, alertes, rapports, exports |

Le plan **Pro** est l'offre recommandée.

## Sécurité API YouTube (règle absolue)

La clé YouTube est uniquement côté backend:

- stockée en variable serveur (`YOUTUBE_API_KEY`),
- jamais exposée dans le frontend,
- jamais dans `VITE_*`,
- jamais dans les réponses API,
- jamais dans les logs.

## Navigation produit (MVP)

### Public
- `/` landing
- `/pricing` tarifs
- `/login` connexion/inscription

### Connecté
- `/radar` Radar du jour (arrivée par défaut)
- `/dashboard` vue plateforme YouTube (MVP)
- `/favorites` favoris (préparé)
- `/subscription` abonnement (préparé)
- `/alerts` alertes Studio (préparé)
- `/reports` rapports Studio (préparé)

## Backend API (MVP SaaS)

- `GET /api/v1/plans` liste des offres Free/Pro/Studio.
- `GET /api/v1/radar/daily` radar du jour avec quota plan.
- `GET /api/v1/billing/status` état de préparation Stripe.
- Auth JWT + contrôle limite côté backend pour Free.

## Données et tables

Le schéma inclut maintenant:

- `users.plan` (`free|pro|studio`),
- `subscriptions`,
- `user_usage_daily`,
- `trend_views`,
- `favorites`, `watchlists`, `alerts`, `reports`, `user_preferences`.

## Roadmap

1. **YouTube SaaS MVP**: plans + limite Free + Radar du jour.
2. **Monétisation**: Stripe checkout + portail + webhooks.
3. **Studio intelligence**: alertes, rapports, scores avancés.
4. **Multi-plateforme**: TikTok puis Instagram.

## Lancement local

```bash
cp .env.example .env
docker compose build
docker compose up -d
```

- Frontend: http://localhost:5173
- Backend: http://localhost:4443/api/v1
- Health: http://localhost:4443/api/v1/health

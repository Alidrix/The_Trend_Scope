# Production deployment guide (Traefik + Cloudflare)

> Nginx n'est plus le reverse proxy officiel en production. La stack officielle utilise Traefik + Cloudflare.

## 1. Pré-requis serveur
- Ubuntu 22.04+ / Debian 12+
- Docker Engine + Docker Compose plugin
- Ports publics ouverts: `80/tcp` et `443/tcp` uniquement
- Domaine Cloudflare: `thetrendscope.com`

## 2. DNS Cloudflare
Créer les enregistrements:

| Type | Nom | Cible | Proxy |
| --- | --- | --- | --- |
| A | thetrendscope.com | IP_SERVEUR | Proxied |
| CNAME | www | thetrendscope.com | Proxied |
| CNAME | api | thetrendscope.com | Proxied |
| CNAME | traefik | thetrendscope.com | Proxied |

## 3. Token API Cloudflare
Créer un token Cloudflare minimal:
- Zone → DNS → Edit
- Zone → Zone → Read
- Zone ciblée uniquement: `thetrendscope.com`

Dans `.env.production`:
```env
CF_DNS_API_TOKEN=...
```

## 4. Variables `.env.production`
```bash
cp .env.production.example .env.production
nano .env.production
```
Renseigner au minimum `ACME_EMAIL`, `CF_DNS_API_TOKEN`, `TRAEFIK_DASHBOARD_AUTH`, secrets DB et SMTP/Stripe.

## 5. Démarrage Traefik
```bash
chmod +x scripts/prod-*.sh
./scripts/prod-up.sh
```

## 6. Vérification certificats
```bash
docker compose --env-file .env.production -f docker-compose.prod.yml logs -f traefik
```
Vérifier la création de `infra/traefik/acme/acme.json` (permissions `600`).

## 7. Mode SSL Full (strict)
Dans Cloudflare → SSL/TLS:
1. Mode `Full (strict)`
2. Activer `Always Use HTTPS`
3. Activer `Automatic HTTPS Rewrites` (optionnel)

## 8. Vérifications API
```bash
./scripts/prod-check.sh
```

## 9. Logs Traefik
```bash
./scripts/prod-logs.sh traefik
```

## 10. Rollback
1. Revenir au commit/tag précédent.
2. `./scripts/prod-up.sh`
3. Restaurer backups si nécessaire.

## 11. Backup / Restore
```bash
./scripts/backup-postgres.sh
./scripts/backup-clickhouse.sh
./scripts/restore-postgres.sh backups/postgres/your_backup.sql
```

## 12. Sécurité
- N'exposer publiquement que Traefik (ports `80` et `443`).
- Ne jamais exposer PostgreSQL, PgBouncer, Redis, NATS ou ClickHouse.
- Dashboard Traefik protégé par Basic Auth.

## 13. Dépannage
### Erreur Cloudflare 525
Cause probable:
- Traefik ne répond pas en HTTPS
- certificat pas encore généré
- port 443 fermé

### Erreur Cloudflare 526
Cause probable:
- Cloudflare Full strict activé
- certificat origin invalide ou pas encore généré
- hostname non couvert par le certificat

### Certificat Let’s Encrypt non généré
Vérifier:
- `CF_DNS_API_TOKEN`
- permissions DNS Edit
- logs Traefik
- propagation DNS
- `acme.json` en `chmod 600`

## Let’s Encrypt staging
Fichier fourni: `infra/traefik/traefik.staging.yml` (avec `caServer` staging). Utiliser ce fichier pour tests afin d'éviter le rate-limit.

## Préflight production

```bash
cp infra/traefik/dynamic.example.yml infra/traefik/dynamic.yml
htpasswd -nbB admin 'strong-password'
nano infra/traefik/dynamic.yml
./scripts/preflight-prod.sh
```

Ne jamais déployer le dashboard Traefik sans Basic Auth.

En production Docker, l’API passe par PgBouncer pour limiter le nombre de connexions PostgreSQL. PostgreSQL n’est jamais exposé publiquement.

## Telegram alerts

1. Créer un bot avec BotFather.
2. Récupérer le token du bot.
3. Ajouter le bot dans le groupe ou canal cible si nécessaire.
4. Récupérer le chat ID.
5. Renseigner `.env.production` :

```env
TELEGRAM_BOT_TOKEN=...
TELEGRAM_DEFAULT_CHAT_ID=...
```

Créer une alerte Studio avec le canal `telegram`.

Discord et Slack ne sont pas inclus dans le scope go-live.

## CI de préproduction

Avant déploiement VPS, vérifier que le workflow `CI` est vert.

La CI couvre :

1. Backend Rust : format, clippy, tests.
2. Frontend SvelteKit : install, check, build.
3. Scripts shell : validation `bash -n`.
4. Docker smoke local.
5. Docker Compose production config.
6. Guards produit : alertes, admin, go-live, absence de workflow de diagnostic historique.

## Go-live : tests d’exploitation
1. Vérifier `CI`.
2. Déployer et configurer `.env.production`.
3. Exécuter `./scripts/preflight-prod.sh`.
4. Vérifier `/api/v1/health` et `/api/v1/ready`.
5. Tester SMTP/Telegram depuis `/admin/ops`.
6. Vérifier alerte web + notification in-app.
7. Générer et télécharger un rapport CSV.
8. Tester Stripe CLI.


### Admin go-live cockpit

Le cockpit admin expose :

- `/admin` : vue globale SaaS
- `/admin/ops` : tests SMTP, Telegram, YouTube et Stripe
- `/admin/system` : état runtime, services et intégrations
- `/admin/billing` : abonnements et MRR estimé
- `/admin/go-live` : checklist finale avant VPS


Après déploiement, ouvrir dans cet ordre :

1. `/admin/system`
2. `/admin/ops`
3. `/admin/billing`
4. `/admin/go-live`
5. `/metrics`

## Validation finale préproduction

### Étape 1 — validation locale

```bash
cp .env.example .env
docker compose build --no-cache
docker compose up -d postgres pgbouncer redis nats clickhouse
docker compose up -d backend
curl -fsS http://localhost:4443/api/v1/health
curl -fsS http://localhost:4443/api/v1/ready
curl -fsS http://localhost:4443/metrics
docker compose up -d worker frontend
docker compose down -v
```

### Étape 2 — validation VPS sans appels distants

```bash
SKIP_REMOTE_CHECKS=1 ./scripts/go-live-check.sh
```

### Étape 3 — validation VPS avec HTTPS

```bash
./scripts/go-live-check.sh
```

### Étape 4 — validation admin

Ouvrir :

- /admin/system
- /admin/ops
- /admin/billing
- /admin/go-live
- /metrics

## Vérification admin smoke avec token

`go-live-check.sh` peut vérifier l’endpoint admin smoke si un token admin est fourni :

```bash
ADMIN_BEARER_TOKEN="..." ./scripts/go-live-check.sh
```

Le token n’est jamais affiché dans les logs.

Sans token, le check admin est ignoré.


## Validation opérateur depuis le cockpit admin

Après déploiement VPS :

1. Ouvrir `/admin/system`.
2. Vérifier PostgreSQL, Redis, NATS, ClickHouse.
3. Ouvrir `/admin/ops`.
4. Lancer le smoke interne.
5. Tester YouTube.
6. Tester Stripe.
7. Tester SMTP.
8. Tester Telegram.
9. Vérifier les logs email.
10. Vérifier les exports.
11. Ouvrir `/admin/go-live`.
12. Corriger tous les items bloquants.


## Sauvegardes production

### Sauvegarder PostgreSQL

```bash
./scripts/prod-backup.sh
```

### Restaurer PostgreSQL

```bash
./scripts/prod-restore.sh backups/postgres/postgres-YYYYMMDD-HHMMSS.sql
```

### Sauvegarder les exports

```bash
./scripts/prod-backup-exports.sh
```

### Vérifier volumes et espace Docker

```bash
./scripts/prod-volumes-check.sh
```

## Audit admin

Les actions sensibles du cockpit admin sont journalisées dans `admin_audit_logs`.

Page disponible : `/admin/audit`

Ne jamais stocker de secrets dans l’audit.

## Runbook restauration PostgreSQL

1. Prévenir l’équipe.
2. Vérifier le fichier backup.
3. Vérifier le checksum `.sha256`.
4. Lancer un backup de sécurité.
5. Restaurer avec `./scripts/prod-restore.sh`.
6. Vérifier `/api/v1/health`.
7. Vérifier `/api/v1/ready`.
8. Vérifier `/admin/system`.
9. Vérifier `/admin/audit`.
10. Vérifier les derniers rapports / exports.

## Planification automatique des sauvegardes

1. Déployer le projet dans `/opt/the-trend-scope`.
2. Vérifier que `.env.production` est renseigné.
3. Copier les unités systemd.
4. Activer les timers.
5. Vérifier les timers.
6. Vérifier les logs.
7. Tester une restauration sur environnement de préproduction.

Commandes :

```bash
sudo cp infra/systemd/*.service /etc/systemd/system/
sudo cp infra/systemd/*.timer /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now trendscope-postgres-backup.timer
sudo systemctl enable --now trendscope-exports-backup.timer
sudo systemctl enable --now trendscope-backup-verify.timer
systemctl list-timers | grep trendscope
```

## Vérification des backups

```bash
./scripts/prod-backup-verify.sh
```

Le script vérifie :

- la présence de backups PostgreSQL ;
- l’intégrité gzip ;
- les checksums SHA256 ;
- la fraîcheur du dernier backup ;
- la taille totale du dossier de backups.


## Dashboard backups admin

La page `/admin/backups` permet de vérifier en lecture seule :

- dernier backup PostgreSQL ;
- dernier backup exports ;
- présence des checksums ;
- fraîcheur des backups ;
- rétention configurée ;
- commandes opérateur utiles.

## Monitoring production optionnel

Pour la configuration complète Prometheus / Alertmanager / Grafana / Loki, voir [`docs/monitoring.md`](monitoring.md).

### Mode strict monitoring

```bash
REQUIRE_MONITORING_RUNNING=1 ./scripts/prod-monitoring-check.sh
```

Premier VPS sans monitoring démarré

```bash
SKIP_MONITORING_CHECK=1 ./scripts/prod-go-no-go.sh
```

## Go / No-Go VPS

Commande opérateur finale :

```bash
./scripts/prod-go-no-go.sh
```

Pour un premier déploiement sans backup existant :

```bash
SKIP_BACKUP_VERIFY=1 ./scripts/prod-go-no-go.sh
```


## Go / No-Go préproduction

Avant exposition publique :

1. CI GitHub verte.
2. `docker compose --env-file .env.production -f docker-compose.prod.yml config` OK.
3. `./scripts/prod-check.sh` OK.
4. `./scripts/prod-volumes-check.sh` OK.
5. `./scripts/prod-backup.sh` exécuté au moins une fois.
6. `./scripts/prod-backup-verify.sh` OK.
7. `/admin/system` OK.
8. `/admin/ops` smoke OK.
9. `/admin/backups` sans erreur bloquante.
10. `/admin/go-live` sans item bloquant en error.

## Test de restauration hors production

```bash
./scripts/prod-restore-dry-run.sh backups/postgres/postgres-YYYYMMDD-HHMMSS.sql.gz
```

Ce test restaure le dump dans un conteneur PostgreSQL temporaire et ne touche jamais à la base de production.

## Runbook incident production

1. Confirmer l'incident et geler les changements.
2. Vérifier l'état API (`/health`, `/ready`) et journaux (`./scripts/prod-logs.sh`).
3. Vérifier le dernier backup (`./scripts/prod-backup-verify.sh`).
4. Exécuter un dry-run (`./scripts/prod-restore-dry-run.sh <backup>`).
5. Si restauration validée, appliquer `./scripts/prod-restore.sh <backup>` selon procédure approuvée.
6. Vérifier `/admin/system`, `/admin/backups`, `/admin/go-live`.

## Runbook incident

### Backend indisponible

1. Vérifier `/api/v1/health`.
2. Vérifier `/api/v1/ready`.
3. Vérifier `/admin/system`.
4. Consulter les logs backend.
5. Vérifier PostgreSQL / Redis / NATS.
6. Lancer `./scripts/prod-go-no-go.sh`.

### Backups absents ou anciens

1. Ouvrir `/admin/backups`.
2. Lancer `./scripts/prod-backup.sh`.
3. Lancer `./scripts/prod-backup-verify.sh`.
4. Vérifier les timers systemd.

### Emails échoués

1. Ouvrir `/admin/ops`.
2. Vérifier SMTP.
3. Vérifier les logs email.
4. Vérifier les variables SMTP dans `.env.production`.

### Rapports en échec

1. Vérifier `/admin/incidents`.
2. Vérifier les logs worker.
3. Vérifier PostgreSQL / exports.
4. Relancer un rapport de test.

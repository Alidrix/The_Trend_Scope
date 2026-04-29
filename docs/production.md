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

# Monitoring The Trend Scope

## Objectif

La stack monitoring est optionnelle mais recommandée en préproduction.

Elle contient :

- Prometheus
- Alertmanager
- Grafana
- Loki
- Promtail

## Démarrage

```bash
docker compose --env-file .env.production \
  -f docker-compose.prod.yml \
  -f docker-compose.monitoring.yml \
  up -d prometheus alertmanager grafana loki promtail
```

## Vérification config

```bash
./scripts/prod-monitoring-check.sh
```

## Vérification stricte des services

```bash
REQUIRE_MONITORING_RUNNING=1 ./scripts/prod-monitoring-check.sh
```

## Grafana

URL :

`https://grafana.<domain>`

Variables :

- `GRAFANA_DOMAIN=`
- `GRAFANA_ADMIN_USER=`
- `GRAFANA_ADMIN_PASSWORD=`

Grafana est publié derrière Traefik avec Basic Auth.

## Prometheus

Prometheus reste interne et ne doit pas être exposé publiquement.

## Alertmanager

Alertmanager reste interne.

Par défaut, son receiver est neutre. Brancher les notifications réelles plus tard, sans commiter de secrets.

## Loki / Promtail

Promtail collecte les logs Docker et les envoie vers Loki.

## Dashboards

Dashboard provisionné :

- Trend Scope Overview

## Métriques applicatives principales

Lister les métriques exposées par `/metrics` :

- `trend_scope_users_total`
- `trend_scope_users_plan_free`
- `trend_scope_users_plan_pro`
- `trend_scope_users_plan_studio`
- `trend_scope_reports_pending`
- `trend_scope_reports_completed_total`
- `trend_scope_reports_failed_total`
- `trend_scope_alert_deliveries_sent_total`
- `trend_scope_alert_deliveries_failed_total`
- `trend_scope_alert_deliveries_skipped_total`
- `trend_scope_notifications_unread_total`
- `trend_scope_email_logs_sent_total`
- `trend_scope_email_logs_failed_total`
- `trend_scope_email_logs_skipped_total`

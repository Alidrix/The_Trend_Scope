# The Trend Scope — systemd timers

Ces unités permettent de planifier automatiquement :
- backup PostgreSQL ;
- backup des exports locaux ;
- vérification des backups.

## Installation

Depuis le dossier du projet, en supposant un déploiement dans `/opt/the-trend-scope` :

```bash
sudo cp infra/systemd/*.service /etc/systemd/system/
sudo cp infra/systemd/*.timer /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now trendscope-postgres-backup.timer
sudo systemctl enable --now trendscope-exports-backup.timer
sudo systemctl enable --now trendscope-backup-verify.timer
```

Vérifier les timers

```bash
systemctl list-timers | grep trendscope
```

Voir les logs

```bash
journalctl -u trendscope-postgres-backup.service -n 100 --no-pager
journalctl -u trendscope-exports-backup.service -n 100 --no-pager
journalctl -u trendscope-backup-verify.service -n 100 --no-pager
```


## Dashboard backups admin

La page `/admin/backups` permet de vérifier en lecture seule :

- dernier backup PostgreSQL ;
- dernier backup exports ;
- présence des checksums ;
- fraîcheur des backups ;
- rétention configurée ;
- commandes opérateur utiles.

## Go / No-Go VPS

Commande opérateur finale :

```bash
./scripts/prod-go-no-go.sh
```

Pour un premier déploiement sans backup existant :

```bash
SKIP_BACKUP_VERIFY=1 ./scripts/prod-go-no-go.sh
```

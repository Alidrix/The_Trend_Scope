<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import AdminToolbar from '$lib/components/AdminToolbar.svelte';
  import AdminSection from '$lib/components/AdminSection.svelte';
  import AdminStatCard from '$lib/components/AdminStatCard.svelte';
  import AdminStatusList from '$lib/components/AdminStatusList.svelte';
  import DataTable from '$lib/components/DataTable.svelte';
  import StatusBadge from '$lib/components/StatusBadge.svelte';
  import { fetchAdminBackupsStatus, type AdminBackupsStatus, type AdminBackupStatusItem } from '$lib/api';
  import { getErrorMessage } from '$lib/errors';
  import { currentUser } from '$lib/stores/user';

  let loading = true;
  let error = '';
  let data: AdminBackupsStatus | null = null;

  const formatBytes = (value?: number | null) => {
    if (!value) return '—';
    const units = ['B', 'KB', 'MB', 'GB'];
    let size = value;
    let unit = 0;
    while (size >= 1024 && unit < units.length - 1) {
      size /= 1024;
      unit += 1;
    }
    return `${size.toFixed(unit === 0 ? 0 : 1)} ${units[unit]}`;
  };

  const formatAge = (seconds?: number | null) => {
    if (seconds === null || seconds === undefined) return '—';
    if (seconds < 60) return `${seconds}s`;
    if (seconds < 3600) return `${Math.round(seconds / 60)}min`;
    if (seconds < 86400) return `${Math.round(seconds / 3600)}h`;
    return `${Math.round(seconds / 86400)}j`;
  };

  const backupRows = (item?: AdminBackupStatusItem | null) =>
    item
      ? [
          { key: 'Directory', value: item.directory },
          { key: 'Latest file', value: item.latest_file ?? '—' },
          { key: 'Age', value: formatAge(item.latest_age_seconds) },
          { key: 'Size', value: formatBytes(item.latest_size_bytes) },
          { key: 'Checksum file', value: item.checksum_file ?? '—' },
          { key: 'Checksum present', value: item.checksum_present ? 'Oui' : 'Non' },
          { key: 'Status', value: item.status }
        ]
      : [];

  const detailColumns = [
    { key: 'key', label: 'Champ' },
    { key: 'value', label: 'Valeur' }
  ];

  const load = async () => {
    loading = true;
    error = '';
    try {
      data = await fetchAdminBackupsStatus();
    } catch (err: unknown) {
      error = getErrorMessage(err, 'Failed to load backup status');
    } finally {
      loading = false;
    }
  };

  onMount(load);
</script>

<AppShell>
  {#if $currentUser?.role !== 'admin'}
    <p>Accès restreint</p>
  {:else}
    <PageHeader title="Admin backups" subtitle="Supervision read-only des sauvegardes" />
    <AdminToolbar {loading} {error}>
      <button type="button" disabled={loading} on:click={load}>Refresh</button>
    </AdminToolbar>

    <AdminSection title="Résumé backups">
      <div class="stats-grid">
        <AdminStatCard label="PostgreSQL status" value={data?.postgres.status ?? 'unknown'} status={data?.postgres.status ?? 'warning'} />
        <AdminStatCard label="Exports status" value={data?.exports.status ?? 'unknown'} status={data?.exports.status ?? 'warning'} />
        <AdminStatCard label="Backup retention days" value={data?.retention.backup_retention_days ?? '—'} />
        <AdminStatCard label="Audit retention days" value={data?.retention.audit_retention_days ?? '—'} />
      </div>
    </AdminSection>

    <AdminSection title="PostgreSQL backup">
      {#if data?.postgres}
        <div class="section-header"><StatusBadge status={data.postgres.status} /></div>
        <DataTable columns={detailColumns} rows={backupRows(data.postgres)} />
      {:else}
        <p>Aucune donnée PostgreSQL backup.</p>
      {/if}
    </AdminSection>

    <AdminSection title="Exports backup">
      {#if data?.exports}
        <div class="section-header"><StatusBadge status={data.exports.status} /></div>
        <DataTable columns={detailColumns} rows={backupRows(data.exports)} />
      {:else}
        <p>Aucune donnée exports backup.</p>
      {/if}
    </AdminSection>

    <AdminSection title="Rétention">
      <AdminStatusList
        items={[
          { label: 'BACKUP_RETENTION_DAYS', status: 'configured', hint: String(data?.retention.backup_retention_days ?? '—') },
          { label: 'AUDIT_RETENTION_DAYS', status: 'configured', hint: String(data?.retention.audit_retention_days ?? '—') },
          { label: 'MAX_BACKUP_AGE_HOURS', status: 'configured', hint: String(data?.retention.max_backup_age_hours ?? '—') }
        ]}
      />
    </AdminSection>

    <AdminSection title="Avertissements">
      {#if data?.warnings?.length}
        <ul>
          {#each data.warnings as warning}
            <li>{warning}</li>
          {/each}
        </ul>
      {:else}
        <p>Aucun avertissement backup.</p>
      {/if}
    </AdminSection>

    <AdminSection title="Commandes opérateur">
      <ul class="commands">
        <li><code>./scripts/prod-backup.sh</code></li>
        <li><code>./scripts/prod-backup-exports.sh</code></li>
        <li><code>./scripts/prod-backup-verify.sh</code></li>
        <li><code>./scripts/prod-restore.sh backups/postgres/postgres-YYYYMMDD-HHMMSS.sql.gz</code></li>
        <li><code>./scripts/prod-restore-dry-run.sh backups/postgres/postgres-YYYYMMDD-HHMMSS.sql.gz</code></li>
        <li><code>./scripts/prod-go-no-go.sh</code></li>
      </ul>
    </AdminSection>
  {/if}
</AppShell>

<style>
  .stats-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 0.75rem; }
  .section-header { margin-bottom: 0.75rem; }
  .commands { display: grid; gap: 0.4rem; }
</style>

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
  import { currentUser } from '$lib/stores/user';
  import { fetchAdminIncidentsStatus, type AdminIncidentsStatus } from '$lib/api';
  import { getErrorMessage } from '$lib/errors';

  let loading = true;
  let error = '';
  let data: AdminIncidentsStatus | null = null;

  const load = async () => {
    loading = true;
    error = '';
    try { data = await fetchAdminIncidentsStatus(); } catch (err: unknown) { error = getErrorMessage(err, 'Failed to load incidents status'); } finally { loading = false; }
  };

  const counterRows = () => data ? [
    { key: 'reports_failed', value: data.counters.reports_failed },
    { key: 'reports_pending', value: data.counters.reports_pending },
    { key: 'emails_failed', value: data.counters.emails_failed },
    { key: 'notifications_unread', value: data.counters.notifications_unread }
  ] : [];
  const columns = [{ key: 'key', label: 'Counter' }, { key: 'value', label: 'Value' }];

  onMount(load);
</script>

<AppShell>
  {#if $currentUser?.role !== 'admin'}<p>Accès restreint</p>{:else}
    <PageHeader title="Admin incidents" subtitle="Pilotage read-only des incidents courants" />
    <AdminToolbar {loading} {error}><button type="button" on:click={load} disabled={loading}>Refresh</button></AdminToolbar>

    <AdminSection title="Résumé incident">
      <div class="stats-grid">
        <AdminStatCard label="Global status" value={data?.status ?? 'unknown'} status={data?.status ?? 'warning'} />
        <AdminStatCard label="Failed reports" value={data?.counters.reports_failed ?? '—'} status={(data?.counters.reports_failed ?? 0) > 0 ? 'error' : 'ok'} />
        <AdminStatCard label="Failed emails" value={data?.counters.emails_failed ?? '—'} status={(data?.counters.emails_failed ?? 0) > 0 ? 'error' : 'ok'} />
        <AdminStatCard label="Pending reports" value={data?.counters.reports_pending ?? '—'} status={(data?.counters.reports_pending ?? 0) > 20 ? 'warning' : 'ok'} />
        <AdminStatCard label="Unread notifications" value={data?.counters.notifications_unread ?? '—'} status={(data?.counters.notifications_unread ?? 0) > 100 ? 'warning' : 'ok'} />
      </div>
    </AdminSection>

    <AdminSection title="Checks">
      {#if data}<div class="section-header"><StatusBadge status={data.status} /></div>{/if}
      <AdminStatusList items={Object.entries(data?.checks ?? {}).map(([label, status]) => ({ label, status }))} />
    </AdminSection>

    <AdminSection title="Compteurs"><DataTable {columns} rows={counterRows()} /></AdminSection>

    <AdminSection title="Actions recommandées">
      <ul>{#each data?.runbook.recommended_actions ?? [] as action}<li>{action}</li>{/each}</ul>
      <p>Runbook: <code>{data?.runbook.docs ?? 'docs/production.md'}</code></p>
    </AdminSection>

    <AdminSection title="Commandes opérateur">
      <ul class="commands">
        <li><code>./scripts/prod-go-no-go.sh</code></li><li><code>./scripts/prod-check.sh</code></li><li><code>./scripts/prod-volumes-check.sh</code></li><li><code>./scripts/prod-backup-verify.sh</code></li><li><code>docker compose --env-file .env.production -f docker-compose.prod.yml ps</code></li><li><code>docker compose --env-file .env.production -f docker-compose.prod.yml logs --tail=100 backend worker</code></li>
      </ul>
    </AdminSection>
  {/if}
</AppShell>

<style>.stats-grid { display:grid; grid-template-columns:repeat(auto-fit,minmax(180px,1fr)); gap:.75rem; } .section-header { margin-bottom:.75rem; } .commands { display:grid; gap:.4rem; }</style>

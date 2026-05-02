<script lang="ts">
  import { onMount } from 'svelte';

  import { fetchAdminMonitoringStatus, type AdminMonitoringStatus } from '$lib/api';
  import AdminSection from '$lib/components/AdminSection.svelte';
  import AdminStatCard from '$lib/components/AdminStatCard.svelte';
  import AdminStatusList from '$lib/components/AdminStatusList.svelte';
  import AdminToolbar from '$lib/components/AdminToolbar.svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import DataTable from '$lib/components/DataTable.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import { getErrorMessage } from '$lib/errors';
  import { currentUser } from '$lib/stores/user';

  let loading = true;
  let error = '';
  let data: AdminMonitoringStatus | null = null;

  const serviceCards: Array<{ label: string; key: keyof NonNullable<AdminMonitoringStatus['services']> }> = [
    { label: 'Prometheus', key: 'prometheus' },
    { label: 'Grafana', key: 'grafana' },
    { label: 'Loki', key: 'loki' },
    { label: 'Alertmanager', key: 'alertmanager' },
    { label: 'Blackbox', key: 'blackbox' },
    { label: 'Node Exporter', key: 'node_exporter' },
    { label: 'cAdvisor', key: 'cadvisor' }
  ];

  const columns = [
    { key: 'service', label: 'Service' },
    { key: 'url', label: 'URL' }
  ];

  const load = async () => {
    loading = true;
    error = '';

    try {
      data = await fetchAdminMonitoringStatus();
    } catch (err: unknown) {
      error = getErrorMessage(err, 'Failed to load monitoring status');
    } finally {
      loading = false;
    }
  };

  const endpointRows = () =>
    Object.entries(data?.endpoints ?? {}).map(([service, url]) => ({
      service,
      url
    }));

  onMount(load);
</script>

<AppShell>
  {#if $currentUser?.role !== 'admin'}
    <p>Accès restreint</p>
  {:else}
    <PageHeader title="Admin monitoring" subtitle="Observabilité runtime read-only" />

    <AdminToolbar {loading} {error}>
      <button type="button" on:click={load} disabled={loading}>Refresh</button>
    </AdminToolbar>

    <AdminSection title="Résumé monitoring">
      <div class="stats-grid">
        <AdminStatCard
          label="Global status"
          value={data?.status ?? 'unknown'}
          status={data?.status ?? 'warning'}
        />

        {#each serviceCards as service}
          <AdminStatCard
            label={service.label}
            value={data?.services?.[service.key] ?? 'unknown'}
            status={data?.services?.[service.key] ?? 'warning'}
          />
        {/each}
      </div>
    </AdminSection>

    <AdminSection title="Services">
      <AdminStatusList
        items={Object.entries(data?.services ?? {}).map(([label, status]) => ({
          label,
          status
        }))}
      />
    </AdminSection>

    <AdminSection title="Endpoints">
      <DataTable {columns} rows={endpointRows()} />
    </AdminSection>

    <AdminSection title="Commandes opérateur">
      <ul>
        <li><code>./scripts/prod-monitoring-check.sh</code></li>
      </ul>
    </AdminSection>

    <AdminSection title="Mode strict">
      <ul>
        <li><code>REQUIRE_MONITORING_RUNNING=1 ./scripts/prod-monitoring-check.sh</code></li>
      </ul>
    </AdminSection>

    <AdminSection title="Alerting test">
      <ul>
        <li><code>./scripts/prod-alerting-test.sh</code></li>
        <li><code>REQUIRE_MONITORING_RUNNING=1 ./scripts/prod-alerting-test.sh</code></li>
      </ul>
    </AdminSection>

    <AdminSection title="Documentation">
      <p><code>docs/monitoring.md</code></p>
      <p><code>docker compose --env-file .env.production -f docker-compose.prod.yml -f docker-compose.monitoring.yml ps</code></p>
      <p><code>SKIP_MONITORING_CHECK=1 ./scripts/prod-go-no-go.sh</code></p>
    </AdminSection>
  {/if}
</AppShell>

<style>
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 0.75rem;
  }
</style>

<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import AdminSection from '$lib/components/AdminSection.svelte';
  import AdminStatCard from '$lib/components/AdminStatCard.svelte';
  import AdminStatusList from '$lib/components/AdminStatusList.svelte';
  import AdminToolbar from '$lib/components/AdminToolbar.svelte';
  import { currentUser } from '$lib/stores/user';
  import { fetchAdminSystem, type AdminSystem } from '$lib/api';
  import { getErrorMessage } from '$lib/errors';

  let d: AdminSystem | null = null;
  let loading = false;
  let error = '';

  const load = async () => {
    loading = true;
    error = '';
    try { d = await fetchAdminSystem(); } catch (err: unknown) { error = getErrorMessage(err, 'Erreur'); } finally { loading = false; }
  };

  $: serviceItems = [
    { label: 'PostgreSQL', status: d?.services?.postgres ?? 'error' },
    { label: 'Redis', status: d?.services?.redis ?? 'error' },
    { label: 'NATS', status: d?.services?.nats ?? 'configured', hint: 'NATS peut être affiché comme "configured" lorsqu’un check non destructif actif n’est pas encore exécuté.' },
    { label: 'ClickHouse', status: d?.services?.clickhouse ?? 'not_configured' }
  ];
  $: integrationItems = [
    { label: 'YouTube', status: d?.integrations?.youtube ?? 'not_configured' },
    { label: 'Stripe', status: d?.integrations?.stripe ?? 'not_configured' },
    { label: 'SMTP', status: d?.integrations?.smtp ?? 'not_configured' },
    { label: 'Telegram', status: d?.integrations?.telegram ?? 'not_configured' },
    { label: 'Cloudflare', status: d?.integrations?.cloudflare ?? 'not_configured' }
  ];
  $: storageItems = [{ label: 'S3 / MinIO', status: d?.storage?.s3 ?? 'not_configured' }];

  onMount(load);
</script>

<AppShell>
  {#if $currentUser?.role !== 'admin'}<p>Accès restreint</p>{:else}
    <PageHeader title="Admin System" subtitle="Runtime, services, intégrations" />
    <AdminToolbar {loading} {error}><button type="button" disabled={loading} on:click={load}>Refresh</button></AdminToolbar>
    {#if d}
      <AdminSection title="Runtime"><div class="grid"><AdminStatCard label="Environment" value={d.runtime?.env ?? '-'} /><AdminStatCard label="Frontend origin" value={d.runtime?.frontend_origin ?? '-'} /></div></AdminSection>
      <AdminSection title="Services internes"><AdminStatusList items={serviceItems} /></AdminSection>
      <AdminSection title="Intégrations"><AdminStatusList items={integrationItems} /></AdminSection>
      <AdminSection title="Stockage"><p>Local exports dir: {d.storage?.local_exports_dir || '-'}</p><AdminStatusList items={storageItems} /></AdminSection>
    {/if}
  {/if}
</AppShell>

<style>.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:.75rem}</style>

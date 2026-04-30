<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import AdminStatCard from '$lib/components/AdminStatCard.svelte';
  import AdminSection from '$lib/components/AdminSection.svelte';
  import AdminStatusList from '$lib/components/AdminStatusList.svelte';
  import AdminToolbar from '$lib/components/AdminToolbar.svelte';
  import { currentUser } from '$lib/stores/user';
  import { fetchAdminOverview, type AdminOverview } from '$lib/api';
  import { getErrorMessage } from '$lib/errors';

  let data: AdminOverview | null = null;
  let loading = false;
  let error = '';

  const load = async () => {
    loading = true;
    error = '';
    try {
      data = await fetchAdminOverview();
    } catch (err: unknown) {
      error = getErrorMessage(err, 'Erreur');
    } finally {
      loading = false;
    }
  };

  $: sourceItems = [
    { label: 'YouTube', status: data?.sources?.youtube ?? 'unknown' },
    { label: 'TikTok', status: data?.sources?.tiktok ?? 'unknown' },
    { label: 'Instagram', status: data?.sources?.instagram ?? 'unknown' }
  ];

  onMount(load);
</script>

<AppShell>
  {#if $currentUser?.role !== 'admin'}
    <p>Accès restreint</p>
  {:else}
    <PageHeader title="Admin Overview" subtitle="Cockpit SaaS" />

    <AdminToolbar {loading} {error}>
      <button type="button" disabled={loading} on:click={load}>Refresh</button>
    </AdminToolbar>

    {#if data}
      <AdminSection title="Utilisateurs">
        <div class="grid">
          <AdminStatCard label="Total" value={String(data.users?.total ?? 0)} />
          <AdminStatCard label="Verified" value={String(data.users?.verified ?? 0)} />
          <AdminStatCard label="Admins" value={String(data.users?.admins ?? 0)} />
        </div>
      </AdminSection>

      <AdminSection title="Plans">
        <div class="grid">
          <AdminStatCard label="Free" value={String(data.plans?.free ?? 0)} />
          <AdminStatCard label="Pro" value={String(data.plans?.pro ?? 0)} />
          <AdminStatCard label="Studio" value={String(data.plans?.studio ?? 0)} />
        </div>
      </AdminSection>

      <AdminSection title="Abonnements">
        <div class="grid">
          <AdminStatCard label="Total" value={String(data.subscriptions?.total ?? 0)} />
          <AdminStatCard label="Active" value={String(data.subscriptions?.active ?? 0)} />
          <AdminStatCard label="Inactive" value={String(data.subscriptions?.inactive ?? 0)} />
        </div>
      </AdminSection>

      <AdminSection title="Alertes">
        <div class="grid">
          <AdminStatCard label="Rules enabled" value={String(data.alerts?.rules_enabled ?? 0)} />
          <AdminStatCard label="Sent 24h" value={String(data.alerts?.deliveries_sent_24h ?? 0)} />
          <AdminStatCard label="Failed 24h" value={String(data.alerts?.deliveries_failed_24h ?? 0)} />
          <AdminStatCard label="Skipped 24h" value={String(data.alerts?.deliveries_skipped_24h ?? 0)} />
        </div>
      </AdminSection>

      <AdminSection title="Rapports">
        <div class="grid">
          <AdminStatCard label="Pending" value={String(data.reports?.pending ?? 0)} />
          <AdminStatCard label="Completed 24h" value={String(data.reports?.completed_24h ?? 0)} />
          <AdminStatCard label="Failed 24h" value={String(data.reports?.failed_24h ?? 0)} />
        </div>
      </AdminSection>

      <AdminSection title="Notifications">
        <div class="grid">
          <AdminStatCard label="Total" value={String(data.notifications?.total ?? 0)} />
          <AdminStatCard label="Unread" value={String(data.notifications?.unread ?? 0)} />
        </div>
      </AdminSection>

      <AdminSection title="Emails">
        <div class="grid">
          <AdminStatCard label="Sent 24h" value={String(data.emails?.sent_24h ?? 0)} />
          <AdminStatCard label="Failed 24h" value={String(data.emails?.failed_24h ?? 0)} />
          <AdminStatCard label="Skipped 24h" value={String(data.emails?.skipped_24h ?? 0)} />
        </div>
      </AdminSection>

      <AdminSection title="Sources">
        <AdminStatusList items={sourceItems} />
      </AdminSection>
    {/if}
  {/if}
</AppShell>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(170px, 1fr));
    gap: 0.75rem;
  }
</style>

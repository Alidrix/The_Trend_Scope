<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import AdminSection from '$lib/components/AdminSection.svelte';
  import AdminStatCard from '$lib/components/AdminStatCard.svelte';
  import AdminToolbar from '$lib/components/AdminToolbar.svelte';
  import { currentUser } from '$lib/stores/user';
  import { fetchAdminGoLiveChecklist, type GoLiveChecklistResponse } from '$lib/api';
  import { getErrorMessage } from '$lib/errors';
  import GoLiveChecklist from '$lib/components/GoLiveChecklist.svelte';

  let d: GoLiveChecklistResponse = { items: [] };
  let loading = false;
  let error = '';

  const load = async () => {
    loading = true;
    error = '';
    try { d = await fetchAdminGoLiveChecklist(); } catch (err: unknown) { error = getErrorMessage(err, 'Erreur'); } finally { loading = false; }
  };

  $: total = d.items.length;
  $: ok = d.items.filter((i) => i.status === 'ok').length;
  $: warning = d.items.filter((i) => i.status === 'warning').length;
  $: errorCount = d.items.filter((i) => i.status === 'error').length;
  $: manual = d.items.filter((i) => i.status === 'manual').length;
  $: blockingRemaining = d.items.filter((i) => i.blocking && i.status !== 'ok').length;
  $: optional = d.items.filter((i) => !i.blocking).length;

  onMount(load);
</script>

<AppShell>
  {#if $currentUser?.role !== 'admin'}<p>Accès restreint</p>{:else}
    <PageHeader title="Go-live checklist" subtitle="Préparation VPS" />
    <AdminToolbar {loading} {error}><button type="button" disabled={loading} on:click={load}>Refresh checklist</button></AdminToolbar>

    <AdminSection title="Summary"><div class="grid"><AdminStatCard label="Total items" value={String(total)} /><AdminStatCard label="OK" value={String(ok)} /><AdminStatCard label="Warning" value={String(warning)} /><AdminStatCard label="Error" value={String(errorCount)} /><AdminStatCard label="Manual" value={String(manual)} /><AdminStatCard label="Blocking remaining" value={String(blockingRemaining)} /><AdminStatCard label="Optional items" value={String(optional)} /></div></AdminSection>

    <AdminSection title="Décision">
      {#if blockingRemaining === 0}
        <p><strong>Prêt pour préproduction</strong></p>
      {:else}
        <p><strong>Actions bloquantes restantes</strong>: {blockingRemaining}</p>
      {/if}
    </AdminSection>

    <a href="https://github.com/Alidrix/The_Trend_Scope/blob/main/docs/production.md" target="_blank" rel="noreferrer">docs/production.md</a>

    <AdminSection title="Checklist"><GoLiveChecklist items={d.items} /></AdminSection>
  {/if}
</AppShell>

<style>.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(170px,1fr));gap:.75rem}</style>

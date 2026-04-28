<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import { fetchReports, generateReport } from '$lib/api';
  import { currentUser } from '$lib/stores/user';
  let reports: any[] = [];
  let error = '';

  async function load() {
    try { const res = await fetchReports(); reports = res.reports || []; } catch (e) { error = (e as Error).message; }
  }
  async function create() { await generateReport({ title: 'Weekly report' }); await load(); }
  onMount(load);
</script>
<AppShell>
  <h1>Rapports</h1>
  {#if $currentUser?.plan !== 'studio'}
    <p>Les rapports avancés sont inclus dans Studio. <a href="/subscription">Découvrir Studio</a></p>
  {:else}
    <button on:click={create}>Générer un rapport</button>
    {#if error}<p>{error}</p>{/if}
    {#each reports as report}<p>{report.title} — {report.status}</p>{/each}
  {/if}
</AppShell>

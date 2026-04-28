<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import { createAlert, deleteAlert, fetchAlerts } from '$lib/api';
  import { currentUser } from '$lib/stores/user';
  let alerts: any[] = [];
  let error = '';

  async function load() {
    try { const res = await fetchAlerts(); alerts = res.alerts || []; } catch (e) { error = (e as Error).message; }
  }
  async function create() { await createAlert({ name: 'Alerte VPH', platform: 'youtube', min_views_per_hour: 10000 }); await load(); }
  async function remove(id: string) { await deleteAlert(id); await load(); }
  onMount(load);
</script>
<AppShell>
  <h1>Alertes</h1>
  {#if $currentUser?.plan !== 'studio'}
    <p>Les alertes personnalisées sont incluses dans Studio. <a href="/subscription">Passer Studio</a></p>
  {:else}
    <button on:click={create}>Créer une alerte</button>
    {#if error}<p>{error}</p>{/if}
    {#each alerts as alert}<p>{alert.name} <button on:click={() => remove(alert.id)}>Supprimer</button></p>{/each}
  {/if}
</AppShell>

<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import { createWatchlist, deleteWatchlist, fetchWatchlists } from '$lib/api';
  import { currentUser } from '$lib/stores/user';
  let items: any[] = [];
  let name = '';
  let error = '';

  async function load() {
    try { const res = await fetchWatchlists(); items = res.watchlists || []; } catch (e) { error = (e as Error).message; }
  }
  async function create() { await createWatchlist({ name, keywords: [], categories: [], platforms: ['youtube'], regions: ['FR'] }); name=''; await load(); }
  async function remove(id: string) { await deleteWatchlist(id); await load(); }
  onMount(load);
</script>
<AppShell>
  <h1>Watchlist</h1>
  {#if $currentUser?.plan === 'free'}
    <p>Watchlists disponibles avec Pro et Studio. <a href="/subscription">Passer Pro</a></p>
  {:else}
    <input bind:value={name} placeholder="Nom de la watchlist"/><button on:click={create}>Créer</button>
    {#if error}<p>{error}</p>{/if}
    {#each items as item}<p>{item.name} <button on:click={() => remove(item.id)}>Supprimer</button></p>{/each}
  {/if}
</AppShell>

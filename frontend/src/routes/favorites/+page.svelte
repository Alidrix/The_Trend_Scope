<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import TrendCard from '$lib/components/TrendCard.svelte';
  import { deleteFavorite, fetchFavorites } from '$lib/api';

  let favorites: any[] = [];
  let loading = true;
  let error = '';

  async function loadFavorites() {
    loading = true;
    error = '';
    try {
      const res = await fetchFavorites();
      favorites = res.favorites || [];
    } catch (e) {
      error = (e as Error).message;
    } finally {
      loading = false;
    }
  }

  async function removeFavorite(item: any) {
    await deleteFavorite(item.platform, item.trend_id);
    await loadFavorites();
  }

  onMount(loadFavorites);
</script>

<AppShell>
  <PageHeader title="Favoris" subtitle="Retrouvez vos tendances sauvegardées." />
  {#if loading}
    <p>Chargement...</p>
  {:else if error}
    <p>{error}</p>
  {:else if favorites.length === 0}
    <EmptyState title="Vous n’avez pas encore sauvegardé de tendance." message="Ajoutez des tendances depuis votre Radar du jour pour les retrouver ici." />
  {:else}
    <div class="grid">
      {#each favorites as trend}
        <div>
          <TrendCard {trend} isFavorite={true} />
          <button on:click={() => removeFavorite(trend)}>Supprimer</button>
        </div>
      {/each}
    </div>
  {/if}
</AppShell>

<style>
  .grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(280px,1fr));gap:.9rem}
</style>

<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { token } from '$lib/stores/auth';
  import { currentUser } from '$lib/stores/user';
  import { fetchDailyRadar, scanVideos } from '$lib/api';
  import AppShell from '$lib/components/AppShell.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import KpiCard from '$lib/components/KpiCard.svelte';
  import TrendCard from '$lib/components/TrendCard.svelte';
  import LockedTrendCard from '$lib/components/LockedTrendCard.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';

  let loading = true;
  let data: any = { trends: [], plan: 'free', kpis: {} };
  let error: string | null = null;
  let filters = { platform: 'youtube', region: 'FR', category: 'business', format: '' };

  async function loadRadar() {
    loading = true;
    error = null;
    try {
      data = await fetchDailyRadar(filters);
    } catch (e) {
      error = (e as Error).message;
    } finally {
      loading = false;
    }
  }

  function updateFilter(key: keyof typeof filters, value: string) {
    filters = { ...filters, [key]: value };
    const query = new URLSearchParams(filters as Record<string, string>);
    goto(`/radar?${query.toString()}`, { replaceState: true, noScroll: true });
    loadRadar();
  }

  onMount(async () => {
    token.subscribe((value) => { if (!value) goto('/login'); })();
    const q = $page.url.searchParams;
    filters = {
      platform: q.get('platform') || 'youtube',
      region: q.get('region') || 'FR',
      category: q.get('category') || 'business',
      format: q.get('format') || ''
    };
    await loadRadar();
  });
</script>

<AppShell>
  <PageHeader title="Radar du jour" subtitle="Aujourd’hui, les tendances qui accélèrent le plus." />

  <div class="filters">
    <select bind:value={filters.platform} on:change={(e) => updateFilter('platform', (e.target as HTMLSelectElement).value)}><option value="youtube">YouTube</option><option value="tiktok">TikTok</option></select>
    <input placeholder="Region" bind:value={filters.region} on:change={(e) => updateFilter('region', (e.target as HTMLInputElement).value)} />
    <input placeholder="Catégorie" bind:value={filters.category} on:change={(e) => updateFilter('category', (e.target as HTMLInputElement).value)} />
  </div>

  <div class="kpis">
    <KpiCard label="Tendances détectées" value={String(data.kpis?.trends_detected || 0)} />
    <KpiCard label="Vues/h moyennes" value={String(data.kpis?.average_views_per_hour || 0)} />
    <KpiCard label="Niches en hausse" value={String(data.kpis?.rising_niches || 0)} />
    <KpiCard label="Opportunités fortes" value={String(data.kpis?.strong_opportunities || 0)} />
  </div>

  {#if loading}
    <p>Chargement du radar...</p>
  {:else if error}
    <p class="error">{error}</p>
  {:else if !data.trends?.length}
    <EmptyState title="Aucune tendance disponible pour le moment." message="Les workers préparent les prochains signaux." />
    {#if $currentUser?.role === 'admin'}<button on:click={scanVideos}>Lancer un scan</button>{/if}
  {:else}
    <div class="grid">
      {#each data.trends as trend}
        <TrendCard {trend} />
      {/each}
      {#if data.plan === 'free'}
        <LockedTrendCard /><LockedTrendCard />
      {/if}
    </div>
    {#if data.upgrade_required}<p class="upgrade">Plan Free limité. Passez Pro ou Studio.</p>{/if}
  {/if}
</AppShell>

<style>
  .kpis{display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:.7rem;margin:1rem 0}
  .grid { display:grid;grid-template-columns:repeat(auto-fill,minmax(280px,1fr));gap:.9rem;margin-top:.8rem }
  .error{color:var(--youtube)}
  .filters{display:flex;gap:.5rem}
  .upgrade{margin-top:1rem;color:var(--primary)}
</style>

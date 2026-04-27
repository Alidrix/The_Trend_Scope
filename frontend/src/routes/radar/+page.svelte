<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { token } from '$lib/stores/auth';
  import { fetchDailyRadar } from '$lib/api';
  import TrendCard from '$lib/components/TrendCard.svelte';
  import LockedTrendCard from '$lib/components/LockedTrendCard.svelte';
  import UpgradeModal from '$lib/components/UpgradeModal.svelte';

  let loading = true;
  let data: any = { trends: [] };
  let error: string | null = null;

  onMount(async () => {
    token.subscribe((value) => {
      if (!value) goto('/login');
    })();
    try {
      data = await fetchDailyRadar();
    } catch (e) {
      error = (e as Error).message;
    } finally {
      loading = false;
    }
  });
</script>

<section class="wrap">
  <header>
    <h1>Radar du jour</h1>
    <p>Aujourd’hui, les tendances qui accélèrent le plus.</p>
    <div class="tabs"><span class="active">YouTube</span><span>TikTok (Preview)</span><span>Instagram (Preview)</span></div>
  </header>

  {#if loading}
    <p>Chargement du radar...</p>
  {:else if error}
    <p class="error">{error}</p>
  {:else}
    <p class="plan">Plan actuel: <strong>{data.plan}</strong> · Restant aujourd’hui: {data.remaining_today ?? 'illimité'}</p>
    <div class="grid">
      {#each data.trends as trend}
        <TrendCard {trend} />
      {/each}
      {#if data.plan === 'free'}
        <LockedTrendCard />
      {/if}
    </div>
  {/if}
</section>

<UpgradeModal visible={data.upgrade_required} message={data.message || ''} />

<style>
  .wrap { max-width: 1100px; margin: 0 auto; padding: 2rem 1rem 4rem; color: #121826; }
  h1 { margin: 0; }
  .tabs { display: flex; gap: .5rem; margin: 1rem 0; }
  .tabs span { padding: .4rem .7rem; border-radius: 999px; background: #eef1f7; color: #445069; font-size: .85rem; }
  .tabs .active { background: #ffe9ec; color: #b01a28; }
  .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(290px, 1fr)); gap: .9rem; }
  .plan { color: #47526a; }
  .error { color: #b10027; }
</style>

<script lang="ts">
  import TrendScore from './TrendScore.svelte';
  import { addFavorite, deleteFavorite } from '$lib/api';
  export let trend: any;
  export let isFavorite = false;
  const score = Math.min(99, Math.max(55, Math.round((trend.views_per_hour || 0) / 350)));

  async function toggleFavorite() {
    if (isFavorite) {
      await deleteFavorite(trend.platform || 'youtube', trend.youtube_id || trend.trend_id);
      isFavorite = false;
      return;
    }
    await addFavorite(trend.platform || 'youtube', trend.youtube_id || trend.trend_id);
    isFavorite = true;
  }
</script>

<article class="card">
  <img src={trend.thumbnail_url || 'https://placehold.co/480x270?text=Trend'} alt={trend.title} />
  <div class="body">
    <div class="row">
      <p class="platform">{(trend.platform || 'YouTube')}</p>
      <button class="fav" on:click={toggleFavorite}>{isFavorite ? '★' : '☆'}</button>
    </div>
    <h3>{trend.title}</h3>
    <p class="meta">{trend.category} • {trend.region || 'Global'}</p>
    <div class="stats">
      <TrendScore {score} />
      <span>{Math.round(trend.views_per_hour || 0).toLocaleString()} vues/h</span>
    </div>
  </div>
</article>

<style>
  .card { background: var(--surface); border: 1px solid var(--border); border-radius: 16px; overflow: hidden; }
  img { width: 100%; aspect-ratio: 16/9; object-fit: cover; background: var(--surface-soft); }
  .body { padding: 0.9rem; }
  .platform { font-size: 0.78rem; color: var(--youtube); font-weight: 700; margin: 0 0 .4rem; }
  h3 { margin: 0; font-size: 1rem; color: var(--text); }
  .meta { color: var(--muted); font-size: .85rem; margin: .4rem 0 .5rem; }
  .stats,.row { display: flex; justify-content: space-between; align-items:center; font-size: .82rem; color: var(--text); }
  .fav{border:none;background:transparent;color:var(--primary);font-size:1.1rem;cursor:pointer}
</style>

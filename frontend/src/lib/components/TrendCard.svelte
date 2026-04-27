<script lang="ts">
  export let trend: any;
</script>

<article class="card">
  <img src={trend.thumbnail_url || 'https://placehold.co/480x270?text=Trend'} alt={trend.title} />
  <div class="body">
    <p class="platform">YouTube</p>
    <h3>{trend.title}</h3>
    <p class="meta">{trend.category} • {trend.region || 'Global'}</p>
    <div class="stats">
      <span>Trend score <strong>{Math.min(99, Math.max(55, Math.round((trend.views_per_hour || 0) / 350)))} / 100</strong></span>
      <span>{Math.round(trend.views_per_hour || 0).toLocaleString()} vues/h</span>
    </div>
  </div>
</article>

<style>
  .card { background: white; border: 1px solid #e6e8ef; border-radius: 16px; overflow: hidden; }
  img { width: 100%; aspect-ratio: 16/9; object-fit: cover; background: #f5f6fa; }
  .body { padding: 0.9rem; }
  .platform { font-size: 0.78rem; color: #bd2b36; font-weight: 700; margin: 0 0 .4rem; }
  h3 { margin: 0; font-size: 1rem; color: #121826; }
  .meta { color: #5f6779; font-size: .85rem; margin: .4rem 0 .5rem; }
  .stats { display: flex; justify-content: space-between; font-size: .82rem; color: #30374a; }
</style>

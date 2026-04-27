<script lang="ts">
  import StatusBadge from './StatusBadge.svelte';

  export let video: any;
  export let noteDraft = '';
  export let onDraftChange: (note: string) => void;
  export let onSave: (note: string) => void;
</script>

<article class="card">
  <img src={video.thumbnail_url || 'https://placehold.co/640x360/111827/f9fafb?text=Viral+Radar'} alt={video.title} />
  <div class="body">
    <div class="title-row">
      <h3>{video.title}</h3>
      {#if video.is_short}
        <StatusBadge label="Short" tone="danger" />
      {/if}
    </div>
    <p class="meta">{video.channel_title || 'Chaîne inconnue'} • {video.region || 'N/A'} • {video.category}</p>
    <p class="stats">{video.views_per_hour} vues/h • {video.duration_seconds}s</p>
    <p class="date">Publié: {new Date(video.published_at).toLocaleString()}</p>
    <textarea value={noteDraft} on:input={(e) => onDraftChange((e.target as HTMLTextAreaElement).value)} placeholder="Ajouter une note"></textarea>
    <div class="actions">
      <a href={video.url || `https://youtube.com/watch?v=${video.youtube_id}`} target="_blank" rel="noreferrer">Ouvrir YouTube</a>
      <button on:click={() => onSave(noteDraft)}>Sauver</button>
    </div>
  </div>
</article>

<style>
  .card { background: #101827; border-radius: var(--radius-card); overflow: hidden; border: 1px solid #26354d; }
  img { width: 100%; aspect-ratio: 16/9; object-fit: cover; display: block; }
  .body { padding: 0.9rem; }
  .title-row { display: flex; justify-content: space-between; gap: .5rem; align-items: flex-start; }
  h3 { color: var(--color-text); margin: 0; font-size: 1rem; }
  .meta,.date { color: var(--color-muted); font-size: .86rem; }
  .stats { color: #d8e4f9; font-weight: 700; }
  textarea { width: 100%; min-height: 72px; border-radius: 12px; background: #0b111d; color: #d6e1f1; border: 1px solid #2a3952; padding: .6rem; }
  .actions { display: flex; justify-content: space-between; margin-top: .6rem; }
  a { color: var(--color-tiktok-cyan); }
  button { background: linear-gradient(120deg, var(--color-youtube), var(--color-tiktok-pink)); color: white; border: none; border-radius: 10px; padding: .45rem .7rem; }
</style>

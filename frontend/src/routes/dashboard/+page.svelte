<script lang="ts">
  import { onMount } from 'svelte';
  import { fetchVideos, refreshVideos, saveNote } from '$lib/api';
  import { notificationsEnabled, pushNotification, notifications } from '$lib/stores/notifications';
  import { token } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  type Video = {
    id: string;
    youtube_id: string;
    title: string;
    category: string;
    views_per_hour: number;
    duration_seconds: number;
    published_at: string;
    is_short: boolean;
    notes?: string | null;
  };

  let videos: Video[] = [];
  let filtered: Video[] = [];
  let category = 'Toutes';
  let loading = true;
  let noteDraft: Record<string, string> = {};
  let alertMessage: string | null = null;

  function ensureAuth() {
    const unsub = token.subscribe((value) => {
      if (!value) {
        goto('/login');
      }
    });
    unsub();
  }

  onMount(async () => {
    ensureAuth();
    await loadVideos();
  });

  async function loadVideos() {
    loading = true;
    try {
      const response = await fetchVideos();
      videos = response.videos;
      filtered = videos;
    } catch (err) {
      alertMessage = (err as Error).message;
    } finally {
      loading = false;
    }
  }

  function applyFilter() {
    filtered = category === 'Toutes' ? videos : videos.filter((v) => v.category === category);
  }

  async function triggerRefresh() {
    const payload = videos.map((v) => ({
      youtube_id: v.youtube_id,
      title: v.title,
      category: v.category,
      views_per_hour: v.views_per_hour,
      duration_seconds: v.duration_seconds,
      published_at: v.published_at
    }));
    await refreshVideos(payload);
    pushNotification({ id: crypto.randomUUID(), title: 'Rafraîchi', body: 'Les vidéos ont été actualisées', level: 'info' });
    await loadVideos();
  }

  async function saveNotes(video: Video) {
    await saveNote(video.id, noteDraft[video.id] ?? video.notes ?? '');
    pushNotification({ id: crypto.randomUUID(), title: 'Note sauvegardée', body: video.title, level: 'success' });
  }
</script>

<section class="topbar">
  <div>
    <h1>Dashboard</h1>
    <p>Classement par vues/h, badge short, refresh manuel.</p>
  </div>
  <div class="actions">
    <button on:click={triggerRefresh}>Rafraîchir</button>
    <label class="toggle">
      <input type="checkbox" bind:checked={$notificationsEnabled} />
      Notifications
    </label>
  </div>
</section>

<section class="filters">
  <label>Filtrer par catégorie</label>
  <select bind:value={category} on:change={applyFilter}>
    <option>Toutes</option>
    {#each Array.from(new Set(videos.map((v) => v.category))) as cat}
      <option>{cat}</option>
    {/each}
  </select>
</section>

{#if alertMessage}
  <p class="error">{alertMessage}</p>
{/if}

<section class="grid">
  {#if loading}
    <p>Chargement...</p>
  {:else if filtered.length === 0}
    <p>Aucune vidéo pour le moment</p>
  {:else}
    {#each filtered as video}
      <article class="card">
        <header>
          <div>
            <p class="category">{video.category}</p>
            <h2>{video.title}</h2>
          </div>
          {#if video.is_short}<span class="badge">Short</span>{/if}
        </header>
        <p class="stat">{video.views_per_hour} vues / heure</p>
        <p class="meta">Durée: {video.duration_seconds}s • Publié: {new Date(video.published_at).toLocaleString()}</p>
        <textarea placeholder="Notes personnelles" bind:value={noteDraft[video.id]}>{video.notes}</textarea>
        <div class="card-actions">
          <a class="link" href={`https://youtube.com/watch?v=${video.youtube_id}`} target="_blank" rel="noreferrer">Ouvrir</a>
          <button on:click={() => saveNotes(video)}>Sauver</button>
        </div>
      </article>
    {/each}
  {/if}
</section>

<section class="panel">
  <h3>Notifications locales</h3>
  <p>Toasts in-app et via l'API Notification du navigateur.</p>
  <div class="toast-feed">
    {#each $notifications as item}
      <div class="toast {item.level}">
        <strong>{item.title}</strong>
        <p>{item.body}</p>
      </div>
    {/each}
  </div>
</section>

<style>
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 2rem 3rem 1rem;
  }
  .actions {
    display: flex;
    gap: 1rem;
    align-items: center;
  }
  button {
    background: linear-gradient(135deg, #5f6bff, #9c6bff);
    color: white;
    border: none;
    padding: 0.8rem 1.1rem;
    border-radius: 12px;
    font-weight: 700;
    cursor: pointer;
  }
  .toggle {
    display: flex;
    gap: 0.35rem;
    align-items: center;
    padding: 0.6rem 0.9rem;
    border-radius: 12px;
    background: rgba(95, 107, 255, 0.12);
  }
  .filters {
    padding: 0 3rem;
    display: flex;
    gap: 1rem;
    align-items: center;
  }
  select {
    padding: 0.65rem 0.9rem;
    border-radius: 12px;
    border: 1px solid rgba(0, 0, 0, 0.08);
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 1.2rem;
    padding: 1.5rem 3rem;
  }
  .card {
    background: #ffffff;
    border-radius: 16px;
    padding: 1.25rem;
    box-shadow: 0 12px 32px rgba(42, 52, 94, 0.08);
    border: 1px solid rgba(95, 107, 255, 0.08);
  }
  header {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
  }
  .badge {
    background: rgba(95, 107, 255, 0.18);
    color: #4a52d6;
    padding: 0.35rem 0.65rem;
    border-radius: 12px;
    font-weight: 700;
  }
  .category {
    color: #7c4cff;
    margin: 0;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.4px;
  }
  h2 {
    margin: 0.2rem 0 0;
    color: #1f1c3a;
  }
  .stat {
    font-weight: 800;
    color: #1f2a44;
  }
  .meta {
    color: #4b4f6f;
  }
  textarea {
    width: 100%;
    min-height: 90px;
    border-radius: 12px;
    border: 1px solid rgba(0, 0, 0, 0.06);
    padding: 0.8rem;
    resize: vertical;
    background: #f7f8ff;
  }
  .card-actions {
    margin-top: 0.75rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .link {
    color: #5f6bff;
    font-weight: 700;
  }
  .panel {
    padding: 1rem 3rem 3rem;
  }
  .toast-feed {
    display: grid;
    gap: 0.5rem;
  }
  .toast {
    padding: 0.8rem 1rem;
    border-radius: 12px;
    background: #fff;
    box-shadow: 0 10px 24px rgba(42, 52, 94, 0.08);
  }
  .toast.success {
    border-left: 4px solid #53c68c;
  }
  .toast.info {
    border-left: 4px solid #5f6bff;
  }
  .error {
    color: #e76f51;
    padding: 0 3rem;
  }
</style>

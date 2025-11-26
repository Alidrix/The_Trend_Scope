<script lang="ts">
  import { onMount } from 'svelte';
  import { fetchVideos } from '$lib/api';

  type Video = {
    id: string;
    title: string;
    views_per_hour: number;
    published_at: string;
  };

  let videos: Video[] = [];
  let loading = true;

  onMount(async () => {
    const result = await fetchVideos();
    videos = result.videos;
    loading = false;
  });
</script>

<section class="history">
  <div>
    <h1>Historique</h1>
    <p>Évolution des vues/h. Export CSV/JSON prévu.</p>
  </div>
  {#if loading}
    <p>Chargement...</p>
  {:else}
    <table>
      <thead>
        <tr>
          <th>Vidéo</th>
          <th>Vues/h</th>
          <th>Publié</th>
        </tr>
      </thead>
      <tbody>
        {#each videos as video}
          <tr>
            <td>{video.title}</td>
            <td>{video.views_per_hour}</td>
            <td>{new Date(video.published_at).toLocaleString()}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</section>

<style>
  .history {
    padding: 2rem 3rem;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    background: white;
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 10px 20px rgba(42, 52, 94, 0.08);
  }
  th, td {
    padding: 0.9rem 1rem;
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
    text-align: left;
  }
  th {
    background: rgba(95, 107, 255, 0.12);
    color: #4a52d6;
  }
</style>

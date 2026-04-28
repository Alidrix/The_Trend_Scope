<script lang="ts">
  import { onMount } from 'svelte';
  import { fetchVideos, scanVideos } from '$lib/api';
  import { token } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  let videos: any[] = [];
  let loading = true;
  let scanning = false;

  onMount(async () => {
    token.subscribe((value) => {
      if (!value) goto('/login');
    })();
    await load();
  });

  async function load() {
    loading = true;
    try {
      const data = await fetchVideos();
      videos = data.videos || [];
    } finally {
      loading = false;
    }
  }

  async function scan() {
    scanning = true;
    await scanVideos();
    await load();
    scanning = false;
  }
</script>

<div class="shell">
  <aside>
    <h1>Viral Radar</h1>
    <nav>
      <a href="/radar">Radar</a>
      <a class="active" href="/dashboard">Plateformes</a>
      <a href="/favorites">Favoris</a>
      <a href="/subscription">Abonnement</a>
      <a href="/pricing">Tarifs</a>
    </nav>
  </aside>
  <main>
    <h2>Plateforme YouTube (MVP)</h2>
    <button on:click={scan} disabled={scanning}>{scanning ? 'Scan...' : 'Scanner maintenant'}</button>
    {#if loading}<p>Chargement...</p>{:else}<p>{videos.length} vidéos collectées.</p>{/if}
  </main>
</div>

<style>
  .shell { display: grid; grid-template-columns: 220px 1fr; min-height: 100vh; }
  aside { border-right: 1px solid #e3e8f3; padding: 1rem; background: var(--surface); }
  h1 { font-size: 1.1rem; }
  nav { display: grid; gap: .4rem; }
  nav a { padding: .55rem .7rem; border-radius: 10px; color: #334155; }
  nav a.active, nav a:hover { background: #eef2fa; }
  main { padding: 1.4rem; }
  button { background: var(--youtube); color: white; border: none; border-radius: 10px; padding: .55rem .8rem; }
</style>

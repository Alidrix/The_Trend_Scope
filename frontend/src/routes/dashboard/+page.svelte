<script lang="ts">
  import { onMount } from 'svelte';
  import { fetchVideos, saveNote, scanVideos } from '$lib/api';
  import { notificationsEnabled, notifications, pushNotification } from '$lib/stores/notifications';
  import { token } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import MetricCard from '$lib/components/MetricCard.svelte';
  import VideoCard from '$lib/components/VideoCard.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import StatusBadge from '$lib/components/StatusBadge.svelte';

  let videos: any[] = [];
  let filtered: any[] = [];
  let loading = true;
  let scanning = false;
  let alertMessage: string | null = null;
  let region = 'Toutes';
  let category = 'Toutes';
  let kind = 'Tous';
  let noteDraft: Record<string, string> = {};
  let lastUpdated = '-';

  onMount(async () => {
    token.subscribe((value) => {
      if (!value) goto('/login');
    })();
    await loadVideos();
  });

  async function loadVideos() {
    loading = true;
    alertMessage = null;
    try {
      const response = await fetchVideos();
      videos = response.videos || [];
      filtered = videos;
      lastUpdated = new Date().toLocaleTimeString();
      applyFilter();
    } catch (err) {
      alertMessage = (err as Error).message;
    } finally {
      loading = false;
    }
  }

  function applyFilter() {
    filtered = videos.filter((v) => {
      const byRegion = region === 'Toutes' || (v.region || 'N/A') === region;
      const byCategory = category === 'Toutes' || v.category === category;
      const byKind = kind === 'Tous' || (kind === 'Shorts' ? v.is_short : !v.is_short);
      return byRegion && byCategory && byKind;
    });
  }

  async function triggerScan() {
    scanning = true;
    try {
      const result = await scanVideos();
      pushNotification({ id: crypto.randomUUID(), title: 'Scan terminé', body: `${result.total} vidéos traitées`, level: 'success' });
      await loadVideos();
    } catch (err) {
      pushNotification({ id: crypto.randomUUID(), title: 'Erreur scan', body: (err as Error).message, level: 'warning' });
      alertMessage = (err as Error).message;
    } finally {
      scanning = false;
    }
  }

  async function saveNotes(video: any, value: string) {
    await saveNote(video.id, value || video.notes || '');
    pushNotification({ id: crypto.randomUUID(), title: 'Note sauvegardée', body: video.title, level: 'info' });
  }

  $: totalVideos = videos.length;
  $: avgVph = totalVideos ? Math.round(videos.reduce((acc, v) => acc + (v.views_per_hour || 0), 0) / totalVideos) : 0;
  $: shortsCount = videos.filter((v) => v.is_short).length;
  $: regions = ['Toutes', ...Array.from(new Set(videos.map((v) => v.region || 'N/A')))];
  $: categories = ['Toutes', ...Array.from(new Set(videos.map((v) => v.category)))];
</script>

<div class="dashboard-shell">
  <aside class="sidebar">
    <h1>Viral Radar</h1>
    <nav>
      <a class="active" href="/dashboard">Dashboard</a>
      <a href="/history">Tendances</a>
      <a href="/dashboard">Alertes</a>
      <a href="/login">Paramètres</a>
    </nav>
  </aside>

  <main>
    <section class="topbar">
      <div>
        <h2>Dashboard</h2>
        <p>Veille virale YouTube/TikTok</p>
      </div>
      <div class="top-actions">
        <StatusBadge label="API Online" tone="success" />
        <button disabled={scanning} on:click={triggerScan}>{scanning ? 'Scan en cours...' : 'Scanner maintenant'}</button>
        <label class="toggle"><input type="checkbox" bind:checked={$notificationsEnabled} /> Notifications</label>
      </div>
    </section>

    <section class="kpis">
      <MetricCard title="Vidéos détectées" value={String(totalVideos)} subtitle="Base active" />
      <MetricCard title="Moyenne vues/h" value={String(avgVph)} subtitle="Signal viral" />
      <MetricCard title="Shorts détectés" value={String(shortsCount)} subtitle="<= 60 secondes" />
      <MetricCard title="Dernière mise à jour" value={lastUpdated} subtitle="Heure locale" />
    </section>

    <section class="filters">
      <select bind:value={region} on:change={applyFilter}>{#each regions as item}<option>{item}</option>{/each}</select>
      <select bind:value={category} on:change={applyFilter}>{#each categories as item}<option>{item}</option>{/each}</select>
      <select bind:value={kind} on:change={applyFilter}><option>Tous</option><option>Shorts</option><option>Longs</option></select>
    </section>

    {#if alertMessage}<p class="error">{alertMessage}</p>{/if}

    {#if loading}
      <div class="loading">Chargement du dashboard…</div>
    {:else if filtered.length === 0}
      <EmptyState title="Aucune vidéo disponible" message="Cliquez sur Scanner maintenant pour lancer un vrai scan YouTube." />
    {:else}
      <section class="grid">
        {#each filtered as video}
          <VideoCard
            video={video}
            noteDraft={noteDraft[video.id] ?? video.notes ?? ''}
            onDraftChange={(value) => (noteDraft[video.id] = value)}
            onSave={(value) => saveNotes(video, value)}
          />
        {/each}
      </section>
    {/if}

    <section class="toast-feed">
      {#each $notifications as item}
        <div class="toast {item.level}"><strong>{item.title}</strong><p>{item.body}</p></div>
      {/each}
    </section>
  </main>
</div>

<style>
  .dashboard-shell { display: grid; grid-template-columns: 240px 1fr; min-height: 100vh; }
  .sidebar { background: #0b101b; border-right: 1px solid #202d42; padding: 1.4rem; position: sticky; top: 0; height: 100vh; }
  .sidebar h1 { color: var(--color-text); }
  nav { display: grid; gap: .4rem; }
  nav a { padding: .65rem .8rem; border-radius: 10px; color: #9fb0cc; }
  nav a.active, nav a:hover { background: #162338; color: white; }
  main { padding: 1.2rem 1.4rem 2rem; }
  .topbar { display: flex; justify-content: space-between; gap: 1rem; align-items: center; }
  h2 { color: var(--color-text); margin: 0; }
  .topbar p { color: var(--color-muted); margin: 0.3rem 0 0; }
  .top-actions { display: flex; align-items: center; gap: .6rem; }
  button { background: linear-gradient(110deg, var(--color-youtube), var(--color-tiktok-pink)); color: white; border: none; border-radius: 12px; padding: .65rem .95rem; font-weight: 700; }
  button:disabled { opacity: .65; cursor: progress; }
  .toggle { color: var(--color-muted); display: flex; gap: .35rem; align-items: center; }
  .kpis { display: grid; grid-template-columns: repeat(4, minmax(0,1fr)); gap: .8rem; margin: 1rem 0; }
  .filters { display: flex; gap: .6rem; margin-bottom: 1rem; }
  select { background: #101827; color: #d8e1f0; border: 1px solid #2d3a51; border-radius: 10px; padding: .55rem .75rem; }
  .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); gap: .9rem; }
  .toast-feed { display: grid; gap: .5rem; margin-top: 1rem; }
  .toast { background: #0f1726; border: 1px solid #2c3c55; border-radius: 10px; padding: .7rem .9rem; color: #dce7fa; }
  .error { color: #ff8a8a; }
  .loading { color: var(--color-muted); padding: 1rem; }
  @media (max-width: 900px) { .dashboard-shell { grid-template-columns: 1fr; } .sidebar { position: static; height: auto; } .kpis { grid-template-columns: repeat(2, minmax(0,1fr)); } }
</style>

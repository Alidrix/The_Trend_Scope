<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import { fetchReports, generateReport } from '$lib/api';
  import { currentUser } from '$lib/stores/user';
  let reports:any[]=[]; let error=''; let loading=false;
  let title='Trend report'; let period_start=''; let period_end=''; let platforms=['youtube']; let categories:string[]=[]; let format='json';
  async function load(){loading=true; try{const r=await fetchReports(); reports=r.reports||[];}catch(e){error=(e as Error).message;}finally{loading=false;}}
  async function create(){await generateReport({title,period_start,period_end,platforms,categories,format}); await load();}
  onMount(load);
</script>
<AppShell><h1>Rapports</h1>
{#if $currentUser?.plan !== 'studio'}<p>Rapports réservés Studio. <a href="/subscription">Découvrir Studio</a></p>
{:else}
<input bind:value={title} placeholder="Titre"/><input type="date" bind:value={period_start}/><input type="date" bind:value={period_end}/>
<select bind:value={format}><option value="json">json</option><option value="csv">csv</option><option value="pdf">pdf</option></select>
<button on:click={create}>Générer</button>
{#if format!=='json'}<p>Export CSV/PDF préparé, génération fichier à venir.</p>{/if}
{#if loading}<p>Loading...</p>{/if}{#if error}<p>{error}</p>{/if}
{#each reports as r}<p>{r.title} — {r.status}</p>{/each}
{/if}</AppShell>

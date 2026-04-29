<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import { createAlert, deleteAlert, fetchAlerts } from '$lib/api';
  import { currentUser } from '$lib/stores/user';
  let alerts:any[]=[]; let loading=false; let error='';
  let name=''; let platform='youtube'; let region='FR'; let category='business'; let keyword=''; let min_views_per_hour=10000; let min_trend_score=70; let channel='web';
  async function load(){loading=true;try{const r=await fetchAlerts();alerts=r.alerts||[];}catch(e){error=(e as Error).message;}finally{loading=false;}}
  async function create(){await createAlert({name,platform,region,category,keyword,min_views_per_hour,min_trend_score,channel}); await load();}
  async function remove(id:string){await deleteAlert(id); await load();}
  onMount(load);
</script>
<AppShell><h1>Alertes</h1>
{#if $currentUser?.plan !== 'studio'}<p>Alertes réservées Studio. <a href="/subscription">Passer Studio</a></p>
{:else}
<input bind:value={name} placeholder="Nom alerte"/><input bind:value={keyword} placeholder="Mot-clé"/>
<select bind:value={platform}><option>youtube</option><option>tiktok</option><option>instagram</option></select>
<select bind:value={channel}><option>web</option><option>email</option><option>discord</option><option>slack</option></select>
<input type="number" bind:value={min_views_per_hour}/><input type="number" bind:value={min_trend_score}/>
<button on:click={create}>Créer</button>
{#if channel==='discord' || channel==='slack'}<p>Canal préparé, envoi à connecter prochainement.</p>{/if}
{#if loading}<p>Loading...</p>{/if}{#if error}<p>{error}</p>{/if}
{#each alerts as a}<p>{a.name} ({a.channel}) <button on:click={()=>remove(a.id)}>Supprimer</button></p>{/each}
{/if}</AppShell>

<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import { createWatchlist, deleteWatchlist, fetchWatchlists } from '$lib/api';
  import { currentUser } from '$lib/stores/user';
  const platforms = ['youtube', 'tiktok', 'instagram'];
  const regions = ['FR', 'US', 'ES', 'GLOBAL'];
  const categories = ['business','gaming','tech','lifestyle','finance','sport','food','travel','music','humor','beauty','education','automotive'];
  let items: any[] = []; let name=''; let keywordInput=''; let selectedPlatforms:string[]=[]; let selectedRegions:string[]=[]; let selectedCategories:string[]=[]; let error=''; let loading=false;
  const toggle=(arr:string[],v:string)=>arr.includes(v)?arr.filter(x=>x!==v):[...arr,v];
  async function load(){loading=true; error=''; try{const r=await fetchWatchlists(); items=r.watchlists||[];}catch(e){error=(e as Error).message;}finally{loading=false;}}
  async function create(){ const keywords=keywordInput.split(',').map((k)=>k.trim()).filter(Boolean); await createWatchlist({name,keywords,categories:selectedCategories,platforms:selectedPlatforms,regions:selectedRegions}); name=''; keywordInput=''; await load();}
  async function remove(id:string){await deleteWatchlist(id); await load();}
  onMount(load);
</script>
<AppShell><h1>Watchlist</h1>
{#if $currentUser?.plan === 'free'}<p>Watchlists premium. <a href="/subscription">Passer Pro</a></p>
{:else}
<input bind:value={name} placeholder="Nom de la watchlist"/><input bind:value={keywordInput} placeholder="ia, business, fitness"/>
<p>Plateformes: {#each platforms as p}<button on:click={() => selectedPlatforms=toggle(selectedPlatforms,p)}>{p}</button>{/each}</p>
<p>Régions: {#each regions as r}<button on:click={() => selectedRegions=toggle(selectedRegions,r)}>{r}</button>{/each}</p>
<p>Catégories: {#each categories as c}<button on:click={() => selectedCategories=toggle(selectedCategories,c)}>{c}</button>{/each}</p>
<button on:click={create}>Créer</button>
{#if loading}<p>Loading...</p>{/if}{#if error}<p>{error}</p>{/if}
{#if !loading && items.length===0}<p>Aucune watchlist.</p>{/if}
{#each items as item}<p>{item.name} <button on:click={() => remove(item.id)}>Supprimer</button></p>{/each}
{/if}</AppShell>

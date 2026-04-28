<script lang="ts">
  import { currentUser } from '$lib/stores/user';

  $: isAdmin = $currentUser?.role === 'admin';
  const links = [
    ['Radar', '/radar'],
    ['Favoris', '/favorites'],
    ['Watchlist', '/watchlist'],
    ['Alertes', '/alerts'],
    ['Rapports', '/reports'],
    ['Abonnement', '/subscription'],
    ['Paramètres', '/settings']
  ];
</script>

<aside>
  {#each links as link}
    <a href={link[1]}>{link[0]}</a>
  {/each}
  {#if isAdmin}
    <a href="/admin">Admin</a>
  {/if}
</aside>

<style>
  aside { width:220px; display:flex; flex-direction:column; gap:.35rem; padding:1rem; border-right:1px solid var(--border); background:var(--surface); }
  a { padding:.55rem .7rem; border-radius:10px; color:var(--text); }
  a:hover { background:var(--surface-soft); }
</style>

<script lang="ts">
  import PlanBadge from './PlanBadge.svelte';
  import UserMenu from './UserMenu.svelte';
  import { onMount } from 'svelte';
  import { currentUser } from '$lib/stores/user';
  import { fetchUnreadNotificationsCount } from '$lib/api';
  let unread = 0;
  onMount(async()=>{try{const r:any=await fetchUnreadNotificationsCount();unread=r.count||0;}catch{}});
</script>

<div class="topbar">
  <a class="logo" href="/radar">The Trend Scope</a>
  <input placeholder="Rechercher une niche ou un sujet" />
  <PlanBadge plan={$currentUser?.plan || 'free'} />
  <a href="/notifications">🔔 {unread}</a>
  <UserMenu />
</div>

<style>
  .topbar { display:grid; grid-template-columns:auto 1fr auto auto; gap:.75rem; align-items:center; padding:.8rem 1rem; border-bottom:1px solid var(--border); background:var(--surface); }
  .logo { font-weight:800; }
  input { border:1px solid var(--border); background:var(--bg); color:var(--text); border-radius:10px; padding:.55rem .7rem; }
</style>

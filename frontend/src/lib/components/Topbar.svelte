<script lang="ts">
  import PlanBadge from './PlanBadge.svelte';
  import UserMenu from './UserMenu.svelte';
  import { currentUser } from '$lib/stores/user';
</script>

<div class="topbar">
  <a class="logo" href="/radar">The Trend Scope</a>
  <input placeholder="Rechercher une niche ou un sujet" />
  <PlanBadge plan={$currentUser?.plan || 'free'} />
  <UserMenu />
</div>

<style>
  .topbar { display:grid; grid-template-columns:auto 1fr auto auto; gap:.75rem; align-items:center; padding:.8rem 1rem; border-bottom:1px solid var(--border); background:var(--surface); }
  .logo { font-weight:800; }
  input { border:1px solid var(--border); background:var(--bg); color:var(--text); border-radius:10px; padding:.55rem .7rem; }
</style>

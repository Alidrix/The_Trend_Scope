<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import Sidebar from './Sidebar.svelte';
  import Topbar from './Topbar.svelte';
  import { token } from '$lib/stores/auth';
  import { currentUser, loadCurrentUser } from '$lib/stores/user';

  onMount(async () => {
    if (get(token)) await loadCurrentUser();
  });
</script>

<div class="shell">
  <Topbar />
  {#if $currentUser && $currentUser.email_verified === false}
    <div class="verify-banner">Votre email n'est pas vérifié. Vérifiez votre boîte mail pour activer pleinement le compte.</div>
  {/if}
  <div class="main">
    <Sidebar />
    <section class="content"><slot /></section>
  </div>
</div>

<style>
  .shell { min-height:100vh; background:var(--bg); color:var(--text); }
  .verify-banner { background:#fef3c7; color:#92400e; padding:.65rem 1rem; border-bottom:1px solid #fde68a; }
  .main { display:flex; min-height:calc(100vh - 60px); }
  .content { flex:1; padding:1rem; }
</style>

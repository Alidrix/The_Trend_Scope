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
  <div class="main">
    <Sidebar />
    <section class="content"><slot /></section>
  </div>
</div>

<style>
  .shell { min-height:100vh; background:var(--bg); color:var(--text); }
  .main { display:flex; min-height:calc(100vh - 60px); }
  .content { flex:1; padding:1rem; }
</style>

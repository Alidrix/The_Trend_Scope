<script lang="ts">
  import { currentUser, clearCurrentUser } from '$lib/stores/user';
  import { token } from '$lib/stores/auth';

  function logout() {
    token.set(null);
    clearCurrentUser();
    window.location.href = '/login';
  }
</script>

<div class="user">
  <span>{$currentUser?.display_name || $currentUser?.username || 'Compte'}</span>
  <a href="/settings/profile">Mon profil</a>
  <button on:click={logout}>Déconnexion</button>
</div>

<style>
  .user { display:flex; gap:.6rem; align-items:center; }
  a, span { color:var(--muted); font-size:.9rem; }
  button { border:1px solid var(--border); background:var(--surface-soft); color:var(--text); border-radius:8px; }
</style>

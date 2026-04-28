<script lang="ts">
  import AppShell from '$lib/components/AppShell.svelte';
  import { currentUser } from '$lib/stores/user';
</script>
<AppShell>
{#if !$currentUser}
  <h1>Connexion requise</h1>
  <p>Veuillez vous connecter pour accéder à cette page.</p>
{:else if $currentUser.role !== 'admin'}
  <h1>Accès restreint</h1>
  <p>L’espace admin est réservé aux administrateurs.</p>
{:else}
  <h1>Admin</h1>
  <p>Endpoints admin accessibles.</p>
{/if}
</AppShell>

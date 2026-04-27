<script lang="ts">
  import { goto } from '$app/navigation';
  import { authStatus, login, register } from '$lib/api';
  import { token } from '$lib/stores/auth';
  import { pushNotification } from '$lib/stores/notifications';
  import { onMount } from 'svelte';
  import StatusBadge from '$lib/components/StatusBadge.svelte';

  let username = '';
  let password = '';
  let registerUsername = '';
  let registerPassword = '';
  let error: string | null = null;
  let loading = false;
  let needsSetup = false;
  let hasApiKey = false;
  let apiOnline = false;

  onMount(async () => {
    try {
      const status = await authStatus();
      needsSetup = status.needs_setup;
      hasApiKey = status.has_api_key;
      apiOnline = true;
    } catch {
      apiOnline = false;
    }
  });

  async function handleLogin(event: Event) {
    event.preventDefault();
    loading = true;
    error = null;
    try {
      const result = await login(username, password);
      token.set(result.token);
      pushNotification({ id: crypto.randomUUID(), title: 'Connecté', body: 'Bienvenue sur le dashboard', level: 'success' });
      goto('/dashboard');
    } catch (err) {
      error = (err as Error).message;
    } finally {
      loading = false;
    }
  }

  async function handleRegister(event: Event) {
    event.preventDefault();
    loading = true;
    error = null;
    try {
      await register(registerUsername, registerPassword);
      pushNotification({ id: crypto.randomUUID(), title: 'Compte créé', body: 'Vous pouvez maintenant vous connecter', level: 'success' });
      needsSetup = false;
      username = registerUsername;
      password = registerPassword;
    } catch (err) {
      error = (err as Error).message;
    } finally {
      loading = false;
    }
  }
</script>

<section class="login">
  <div class="status-row">
    <StatusBadge label={apiOnline ? 'API online' : 'API hors ligne'} tone={apiOnline ? 'success' : 'warning'} />
    <StatusBadge label={hasApiKey ? 'Clé YouTube détectée' : 'Clé YouTube absente'} tone={hasApiKey ? 'success' : 'warning'} />
  </div>

  <div class="grid">
    <div class="card">
      <h1>Connexion</h1>
      <p>Accédez au dashboard privé Viral Radar.</p>
      <form on:submit|preventDefault={handleLogin}>
        <input required bind:value={username} placeholder="Identifiant" />
        <input required type="password" bind:value={password} placeholder="Mot de passe" minlength="10" />
        {#if error && !needsSetup}<p class="error">{error}</p>{/if}
        <button disabled={loading || needsSetup}>{loading ? 'Connexion...' : needsSetup ? 'Initialisez d’abord' : 'Se connecter'}</button>
      </form>
    </div>

    <div class="card muted">
      <h2>Initialisation</h2>
      <p>Créez le premier compte administrateur (une seule fois).</p>
      <form on:submit|preventDefault={handleRegister}>
        <input required bind:value={registerUsername} placeholder="Nouvel identifiant" />
        <input required type="password" bind:value={registerPassword} placeholder="Mot de passe fort" minlength="10" />
        {#if error && needsSetup}<p class="error">{error}</p>{/if}
        <button disabled={loading || !needsSetup}>{loading ? 'Création...' : needsSetup ? 'Créer le compte' : 'Compte déjà créé'}</button>
      </form>
    </div>
  </div>
</section>

<style>
  .login { min-height: 100vh; padding: 2rem; background: radial-gradient(circle at 15% 10%, #17233a 0%, #0b1019 45%, #070a12 100%); }
  .status-row { display: flex; gap: .6rem; justify-content: center; margin-bottom: 1rem; }
  .grid { max-width: 980px; margin: 0 auto; display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 1rem; }
  .card { backdrop-filter: blur(8px); background: rgba(17, 24, 39, .78); border: 1px solid #2d3a51; border-radius: var(--radius-card); padding: 1.5rem; }
  .muted { background: rgba(17, 24, 39, .6); }
  h1,h2 { color: var(--color-text); margin-top: 0; }
  p { color: var(--color-muted); }
  form { display: grid; gap: .65rem; }
  input { background: #0c1320; border: 1px solid #2a3952; color: #dbe7fb; padding: .72rem .82rem; border-radius: 10px; }
  button { background: linear-gradient(120deg, var(--color-youtube), var(--color-tiktok-pink)); color: white; border: none; border-radius: 10px; padding: .7rem; font-weight: 700; }
  .error { color: #ff8b8b; }
</style>

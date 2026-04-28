<script lang="ts">
  import { goto } from '$app/navigation';
  import { authStatus, login, register } from '$lib/api';
  import { token } from '$lib/stores/auth';
  import { pushNotification } from '$lib/stores/notifications';
  import { onMount } from 'svelte';
  import StatusBadge from '$lib/components/StatusBadge.svelte';
  import BrandLogo from '$lib/components/BrandLogo.svelte';
  import AuthCard from '$lib/components/AuthCard.svelte';

  let username = '';
  let password = '';
  let registerUsername = '';
  let registerPassword = '';
  let error: string | null = null;
  let setupError: string | null = null;
  let loading = false;
  let setupLoading = false;
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
      pushNotification({ id: crypto.randomUUID(), title: 'Connecté', body: 'Bienvenue sur votre Radar du jour', level: 'success' });
      goto('/radar');
    } catch (err) {
      error = (err as Error).message;
    } finally {
      loading = false;
    }
  }

  async function handleRegister(event: Event) {
    event.preventDefault();
    setupLoading = true;
    setupError = null;
    try {
      await register(registerUsername, registerPassword);
      pushNotification({ id: crypto.randomUUID(), title: 'Initialisation terminée', body: 'Compte administrateur créé', level: 'success' });
      needsSetup = false;
      username = registerUsername;
      password = registerPassword;
    } catch (err) {
      setupError = (err as Error).message;
    } finally {
      setupLoading = false;
    }
  }
</script>

<section class="login-page">
  <div class="status-row">
    <StatusBadge label={apiOnline ? 'API online' : 'API hors ligne'} tone={apiOnline ? 'success' : 'warning'} />
    <StatusBadge label={hasApiKey ? 'Clé YouTube détectée' : 'Clé YouTube absente'} tone={hasApiKey ? 'success' : 'warning'} />
  </div>

  <div class="grid">
    <aside class="intro">
      <BrandLogo size="lg" showTagline={true} />
      <h1>Connectez-vous à votre radar de tendances</h1>
      <p>Repérez les tendances avant les autres et créez du contenu au bon moment.</p>
      <ul>
        <li>Détecte les tendances qui accélèrent</li>
        <li>Compare YouTube, TikTok et Instagram</li>
        <li>Crée du contenu au bon moment</li>
      </ul>
    </aside>

    <AuthCard>
      <h2>Connexion</h2>
      <form on:submit|preventDefault={handleLogin}>
        <label>
          Email ou identifiant
          <input required bind:value={username} placeholder="votre@email.com" autocomplete="username" />
        </label>
        <label>
          Mot de passe
          <input required type="password" bind:value={password} placeholder="Votre mot de passe" minlength="10" autocomplete="current-password" />
        </label>
        {#if error}<p class="error">{error}</p>{/if}
        <button disabled={loading}>{loading ? 'Connexion...' : 'Se connecter'}</button>
      </form>

      <div class="links">
        <a href="/forgot-password">Mot de passe oublié ?</a>
        <a href="/register">Pas encore de compte ? Créer un compte</a>
        <a href="/pricing">Voir les offres</a>
      </div>
    </AuthCard>
  </div>

  {#if needsSetup}
    <div class="setup-note">
      <h3>Initialisation administrateur requise</h3>
      <p>Créez le premier compte administrateur pour finaliser l'installation.</p>
      <form on:submit|preventDefault={handleRegister}>
        <input required bind:value={registerUsername} placeholder="Identifiant admin" />
        <input required type="password" bind:value={registerPassword} placeholder="Mot de passe fort" minlength="10" />
        {#if setupError}<p class="error">{setupError}</p>{/if}
        <button disabled={setupLoading}>{setupLoading ? 'Création...' : 'Créer le premier compte administrateur'}</button>
      </form>
    </div>
  {/if}
</section>

<style>
  .login-page {
    min-height: calc(100vh - 56px);
    padding: 1rem 1rem 2.5rem;
    background: radial-gradient(circle at 10% 0%, color-mix(in srgb, var(--primary) 12%, transparent), transparent 33%);
  }

  .status-row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.6rem;
    justify-content: center;
    margin-bottom: 1rem;
  }

  .grid {
    max-width: 1060px;
    margin: 0 auto;
    display: grid;
    grid-template-columns: 1.1fr 0.9fr;
    gap: 1.25rem;
    align-items: center;
  }

  .intro h1 { margin: 1rem 0 0.6rem; font-size: clamp(1.5rem, 3vw, 2.2rem); }
  .intro p { margin: 0; color: var(--muted); max-width: 56ch; }
  .intro ul { margin: 1.25rem 0 0; padding-left: 1.2rem; display: grid; gap: 0.6rem; color: var(--text); }

  h2 { margin: 0 0 1rem; }

  form { display: grid; gap: 0.8rem; }
  label { display: grid; gap: 0.35rem; font-weight: 600; color: var(--text); }
  input {
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
    border-radius: 12px;
    padding: 0.75rem 0.85rem;
    font: inherit;
  }

  button {
    border: none;
    border-radius: 12px;
    background: var(--primary);
    color: #fff;
    font-weight: 700;
    padding: 0.72rem 0.85rem;
    cursor: pointer;
  }

  button:disabled { opacity: 0.75; cursor: not-allowed; }

  .links {
    margin-top: 1rem;
    display: grid;
    gap: 0.45rem;
  }

  .links a { color: var(--muted); font-weight: 500; }
  .links a:hover { color: var(--primary); }

  .setup-note {
    margin: 1rem auto 0;
    max-width: 1060px;
    border: 1px dashed var(--border);
    border-radius: var(--radius-card);
    background: color-mix(in srgb, var(--surface-soft), transparent 15%);
    padding: 1rem;
  }

  .setup-note h3 { margin: 0 0 0.3rem; }
  .setup-note p { margin: 0 0 0.9rem; color: var(--muted); }
  .error { margin: 0; color: #d14343; }

  @media (max-width: 900px) {
    .grid { grid-template-columns: 1fr; }
  }
</style>

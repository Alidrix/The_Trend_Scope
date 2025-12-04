<script lang="ts">
  import { goto } from '$app/navigation';
  import { authStatus, login, register } from '$lib/api';
  import { token } from '$lib/stores/auth';
  import { pushNotification } from '$lib/stores/notifications';
  import { onMount } from 'svelte';

  let username = '';
  let password = '';
  let registerUsername = '';
  let registerPassword = '';
  let error: string | null = null;
  let loading = false;
  let needsSetup = false;
  let hasApiKey = false;

  onMount(async () => {
    try {
      const status = await authStatus();
      needsSetup = status.needs_setup;
      hasApiKey = status.has_api_key;
    } catch (err) {
      console.error(err);
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
      pushNotification({
        id: crypto.randomUUID(),
        title: 'Compte créé',
        body: 'Vous pouvez maintenant vous connecter',
        level: 'success'
      });
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
  <div class="grid">
    <div class="card">
      <p class="badge">Sécurité</p>
      <h1>Connexion</h1>
      <p class="lede">Authentification unique, mot de passe robuste (16+ caractères).</p>
      <form on:submit|preventDefault={handleLogin} aria-label="Connexion">
        <label for="login-username">Identifiant</label>
        <input id="login-username" required bind:value={username} placeholder="admin" />

        <label for="login-password">Mot de passe</label>
        <input
          id="login-password"
          required
          type="password"
          bind:value={password}
          placeholder="••••••••"
          minlength="16"
        />

        {#if error && !needsSetup}
          <p class="error">{error}</p>
        {/if}
        <button class:loading={loading} disabled={loading || needsSetup} type="submit">
          {loading ? 'Connexion...' : needsSetup ? 'Configurez d’abord' : 'Entrer'}
        </button>
      </form>
    </div>

    <div class="card secondary">
      <p class="badge ghost">Initialisation</p>
      <h2>Créer le compte privé</h2>
      <p class="lede">Un seul utilisateur. Mot de passe 16+ caractères, stocké chiffré en base.</p>
      <form on:submit|preventDefault={handleRegister} aria-label="Création de compte">
        <label for="register-username">Identifiant</label>
        <input id="register-username" required bind:value={registerUsername} placeholder="mon_compte" />

        <label for="register-password">Mot de passe</label>
        <input
          id="register-password"
          required
          type="password"
          bind:value={registerPassword}
          placeholder="mot de passe solide"
          minlength="16"
        />

        {#if error && needsSetup}
          <p class="error">{error}</p>
        {/if}
        <button class:loading={loading} disabled={loading || !needsSetup} type="submit">
          {loading ? 'Création...' : needsSetup ? 'Créer le compte' : 'Compte déjà créé'}
        </button>
      </form>
      <p class="hint">
        {#if hasApiKey}
          🔑 Clé YouTube détectée via <code>YOUTUBE_API_KEY</code> dans votre <code>.env</code>.
        {:else}
          ⚠️ Ajoutez votre <code>YOUTUBE_API_KEY</code> dans le fichier <code>.env</code> avant de rafraîchir les vidéos.
        {/if}
      </p>
    </div>
  </div>
</section>

<style>
  .login {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.5rem;
    width: 100%;
    max-width: 960px;
  }
  .card {
    background: white;
    padding: 2.4rem;
    border-radius: 16px;
    box-shadow: 0 20px 40px rgba(42, 52, 94, 0.12);
    border: 1px solid rgba(95, 107, 255, 0.08);
  }
  .card.secondary {
    background: linear-gradient(135deg, rgba(95, 107, 255, 0.06), rgba(156, 107, 255, 0.08));
    border: 1px solid rgba(95, 107, 255, 0.15);
  }
  h1 {
    margin: 0.5rem 0;
  }
  h2 {
    margin: 0.5rem 0 0.25rem;
  }
  .badge {
    padding: 0.35rem 0.75rem;
    border-radius: 999px;
    background: rgba(156, 107, 255, 0.12);
    color: #7c4cff;
    font-weight: 700;
    letter-spacing: 0.2px;
  }
  .badge.ghost {
    background: rgba(95, 107, 255, 0.12);
    color: #4750c8;
  }
  .lede {
    color: #4b4f6f;
  }
  form {
    display: grid;
    gap: 0.75rem;
    margin-top: 1rem;
  }
  label {
    font-weight: 600;
  }
  input {
    padding: 0.85rem 1rem;
    border-radius: 12px;
    border: 1px solid rgba(95, 107, 255, 0.16);
    font-size: 1rem;
  }
  button {
    background: linear-gradient(135deg, #5f6bff, #9c6bff);
    color: white;
    border: none;
    padding: 0.9rem 1.1rem;
    border-radius: 12px;
    font-weight: 700;
    cursor: pointer;
    transition: transform 0.1s ease, box-shadow 0.1s ease;
  }
  button.loading {
    opacity: 0.8;
    cursor: progress;
  }
  .error {
    color: #c0392b;
  }
  .hint {
    margin-top: 1rem;
    color: #4b4f6f;
  }
</style>

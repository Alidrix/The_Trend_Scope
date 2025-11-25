<script lang="ts">
  import { goto } from '$app/navigation';
  import { login } from '$lib/api';
  import { token } from '$lib/stores/auth';
  import { pushNotification } from '$lib/stores/notifications';

  let username = '';
  let password = '';
  let error: string | null = null;
  let loading = false;

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
</script>

<section class="login">
  <div class="card">
    <p class="badge">Sécurité</p>
    <h1>Connexion</h1>
    <p class="lede">Authentification unique, mot de passe robuste (16+ caractères).</p>
    <form on:submit|preventDefault={handleLogin}>
      <label>Identifiant</label>
      <input required bind:value={username} placeholder="admin" />
      <label>Mot de passe</label>
      <input required type="password" bind:value={password} placeholder="••••••••" minlength="16" />
      {#if error}
        <p class="error">{error}</p>
      {/if}
      <button class:loading={loading} disabled={loading} type="submit">{loading ? 'Connexion...' : 'Entrer'}</button>
    </form>
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
  .card {
    background: white;
    padding: 2.4rem;
    border-radius: 16px;
    box-shadow: 0 20px 40px rgba(42, 52, 94, 0.12);
    max-width: 420px;
    width: 100%;
    border: 1px solid rgba(95, 107, 255, 0.08);
  }
  h1 {
    margin: 0.5rem 0;
  }
  .badge {
    padding: 0.35rem 0.75rem;
    border-radius: 999px;
    background: rgba(156, 107, 255, 0.12);
    color: #7c4cff;
    font-weight: 700;
    letter-spacing: 0.2px;
  }
  .lede {
    color: #4b4f6f;
  }
  form {
    display: grid;
    gap: 0.75rem;
    margin-top: 1.5rem;
  }
  input {
    padding: 0.85rem 1rem;
    border-radius: 12px;
    border: 1px solid rgba(0, 0, 0, 0.05);
    background: #f7f8ff;
    font-size: 1rem;
  }
  label {
    font-weight: 700;
  }
  .error {
    color: #e76f51;
    margin: 0;
  }
  button {
    background: linear-gradient(135deg, #5f6bff, #9c6bff);
    color: white;
    border: none;
    padding: 0.9rem 1.2rem;
    border-radius: 12px;
    font-weight: 800;
    cursor: pointer;
    transition: transform 120ms ease;
  }
  button.loading {
    opacity: 0.7;
    cursor: wait;
  }
  button:hover {
    transform: translateY(-1px);
  }
</style>

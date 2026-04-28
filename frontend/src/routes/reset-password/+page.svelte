<script lang="ts">
  import { page } from '$app/stores';
  import { resetPassword } from '$lib/api';

  let password = '';
  let done = false;
  let error = '';

  async function submit() {
    error = '';
    const token = $page.url.searchParams.get('token') || '';
    try {
      await resetPassword(token, password);
      done = true;
    } catch (e) {
      error = (e as Error).message;
    }
  }
</script>

<section>
  <h1>Nouveau mot de passe</h1>
  <input type="password" bind:value={password} placeholder="Nouveau mot de passe" />
  <button on:click={submit}>Mettre à jour</button>
  {#if done}<p>Mot de passe mis à jour.</p>{/if}
  {#if error}<p>{error}</p>{/if}
</section>

<script lang="ts">
  import { forgotPassword } from '$lib/api';
  let email = '';
  let submitted = false;
  let error = '';

  async function submit(event: Event) {
    event.preventDefault();
    error = '';
    try {
      await forgotPassword(email);
      submitted = true;
    } catch (e) {
      error = (e as Error).message;
    }
  }
</script>

<section class="forgot-page">
  <div class="card">
    <h1>Réinitialisation du mot de passe</h1>
    <p>Entrez votre email. Si un compte existe, vous recevrez un lien de réinitialisation.</p>

    <form on:submit|preventDefault={submit}>
      <label>Email<input required type="email" bind:value={email} /></label>
      <button type="submit">Envoyer</button>
    </form>

    {#if submitted}
      <p class="notice">Si un compte existe, un e-mail de réinitialisation vient d’être envoyé.</p>
    {/if}
    {#if error}<p>{error}</p>{/if}

    <a href="/login">Retour à la connexion</a>
  </div>
</section>

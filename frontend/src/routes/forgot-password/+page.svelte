<script lang="ts">
  let email = '';
  let submitted = false;

  function submit(event: Event) {
    event.preventDefault();
    submitted = true;
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
      <p class="notice">Fonctionnalité bientôt disponible. Votre demande a bien été prise en compte.</p>
    {/if}

    <a href="/login">Retour à la connexion</a>
  </div>
</section>

<style>
  .forgot-page { min-height: calc(100vh - 56px); padding: 1rem; display: grid; place-items: center; }
  .card { max-width: 520px; background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius-card); padding: 1.2rem; display: grid; gap: 0.8rem; }
  h1, p { margin: 0; }
  p { color: var(--muted); }
  form { display: grid; gap: 0.6rem; }
  label { display: grid; gap: 0.35rem; }
  input { border: 1px solid var(--border); border-radius: 12px; padding: 0.72rem 0.85rem; background: var(--surface); color: var(--text); }
  button { border: none; border-radius: 12px; background: var(--primary); color: white; padding: 0.72rem 0.85rem; font-weight: 700; }
  .notice { color: var(--text); background: var(--primary-soft); border-radius: 10px; padding: 0.65rem 0.75rem; }
  a { color: var(--primary); font-weight: 700; }
</style>

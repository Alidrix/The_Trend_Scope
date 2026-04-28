<script lang="ts">
  import { page } from '$app/stores';
  import { verifyEmail } from '$lib/api';

  let message = 'Vérification en cours...';

  $: token = $page.url.searchParams.get('token') || '';
  $: if (token) {
    verifyEmail(token)
      .then(() => (message = 'Email vérifié avec succès. Vous pouvez continuer.'))
      .catch(() => (message = 'Lien invalide ou expiré.'));
  }
</script>

<section>
  <h1>Vérification email</h1>
  <p>{message}</p>
</section>

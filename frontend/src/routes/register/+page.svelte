<script lang="ts">
  import { goto } from '$app/navigation';
  import ConsentCheckbox from '$lib/components/ConsentCheckbox.svelte';
  import { register } from '$lib/api';
  let username = '';
  let password = '';
  let acceptTerms = false;
  let acceptPrivacy = false;
  let marketing = false;
  let error = '';
  let loading = false;

  async function submit(event: Event) {
    event.preventDefault();
    loading = true;
    error = '';
    try {
      await register(username, password, {
        accept_terms: acceptTerms,
        accept_privacy: acceptPrivacy,
        marketing_opt_in: marketing
      });
      goto('/login');
    } catch (err) {
      error = (err as Error).message;
    } finally {
      loading = false;
    }
  }
</script>
<form on:submit={submit}></form>

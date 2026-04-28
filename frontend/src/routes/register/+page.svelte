<script lang="ts">
  import { goto } from '$app/navigation';
  import AuthCard from '$lib/components/AuthCard.svelte';
  import BrandLogo from '$lib/components/BrandLogo.svelte';
  import ConsentCheckbox from '$lib/components/ConsentCheckbox.svelte';
  import ProfileTypeSelect from '$lib/components/ProfileTypeSelect.svelte';
  import { register } from '$lib/api';

  let email = '';
  let password = '';
  let displayName = '';
  let country = 'France';
  let profileType = 'creator';
  let acceptTerms = false;
  let acceptPrivacy = false;
  let marketingOptIn = false;
  let loading = false;
  let error = '';

  function validate() {
    if (!email.trim()) return 'L’email est obligatoire.';
    if (password.length < 10) return 'Le mot de passe doit contenir au moins 10 caractères.';
    if (!acceptTerms) return 'Vous devez accepter les CGU.';
    if (!acceptPrivacy) return 'Vous devez accepter la politique de confidentialité.';
    return '';
  }

  async function submit(event: Event) {
    event.preventDefault();
    error = validate();
    if (error) return;

    loading = true;
    try {
      await register({
        email,
        password,
        display_name: displayName || undefined,
        country: country || undefined,
        profile_type: profileType,
        accept_terms: acceptTerms,
        accept_privacy: acceptPrivacy,
        marketing_opt_in: marketingOptIn
      });
      goto('/onboarding');
    } catch (err) {
      error = (err as Error).message;
    } finally {
      loading = false;
    }
  }
</script>

<section class="register-page">
  <div class="wrapper">
    <BrandLogo size="md" showTagline={true} />
    <h1>Créez votre compte gratuit</h1>
    <p>Commencez avec 3 tendances gratuites par jour.</p>

    <AuthCard>
      <h2>Création de compte</h2>
      <form on:submit|preventDefault={submit}>
        <label>Email<input type="email" bind:value={email} required autocomplete="email" /></label>
        <label>Mot de passe<input type="password" bind:value={password} minlength="10" required autocomplete="new-password" /></label>
        <label>Nom affiché / prénom<input bind:value={displayName} placeholder="Ex: Alex" /></label>
        <label>Pays<input bind:value={country} placeholder="France" /></label>
        <label>Type de profil<ProfileTypeSelect bind:value={profileType} /></label>

        <div class="consents">
          <ConsentCheckbox label="J’accepte les CGU" required={true} bind:checked={acceptTerms} />
          <ConsentCheckbox label="J’accepte la politique de confidentialité" required={true} bind:checked={acceptPrivacy} />
          <ConsentCheckbox label="J’accepte de recevoir des conseils produit" bind:checked={marketingOptIn} />
        </div>

        {#if error}<p class="error">{error}</p>{/if}
        <button disabled={loading}>{loading ? 'Création en cours...' : 'Créer mon compte gratuit'}</button>
      </form>
      <p class="links">Déjà inscrit ? <a href="/login">Se connecter</a></p>
    </AuthCard>
  </div>
</section>

<style>
  .register-page { min-height: calc(100vh - 56px); padding: 1rem 1rem 2.5rem; }
  .wrapper { max-width: 620px; margin: 0 auto; display: grid; gap: 1rem; }
  h1 { margin: 0.4rem 0 0; }
  p { margin: 0; color: var(--muted); }
  h2 { margin: 0 0 1rem; }

  form { display: grid; gap: 0.8rem; }
  label { display: grid; gap: 0.35rem; font-weight: 600; }
  input {
    border: 1px solid var(--border);
    border-radius: 12px;
    background: var(--surface);
    color: var(--text);
    padding: 0.72rem 0.85rem;
    font: inherit;
  }

  .consents { margin-top: 0.35rem; display: grid; gap: 0.35rem; }

  button {
    border: none;
    border-radius: 12px;
    background: var(--primary);
    color: white;
    font-weight: 700;
    padding: 0.75rem 0.9rem;
  }

  .error { color: #d14343; margin: 0.2rem 0 0; }
  .links { margin-top: 0.8rem; }
  .links a { color: var(--primary); font-weight: 700; }
</style>

<script lang="ts">
  import { goto } from '$app/navigation';
  import { pushNotification } from '$lib/stores/notifications';
  import AuthCard from '$lib/components/AuthCard.svelte';
  import BrandLogo from '$lib/components/BrandLogo.svelte';
  import CategorySelector from '$lib/components/CategorySelector.svelte';
  import OnboardingStep from '$lib/components/OnboardingStep.svelte';
  import PlatformSelector from '$lib/components/PlatformSelector.svelte';
  import { saveOnboarding } from '$lib/api';

  const goals = [
    { label: 'Trouver des idées de vidéos', value: 'find_content_ideas' },
    { label: 'Surveiller une niche', value: 'monitor_niche' },
    { label: 'Suivre les tendances mondiales', value: 'track_global_trends' },
    { label: 'Trouver des opportunités TikTok / Reels / Shorts', value: 'grow_social_accounts' },
    { label: 'Faire de la veille pour une marque', value: 'support_brand_strategy' },
    { label: 'Créer des rapports pour des clients', value: 'agency_reporting' }
  ];

  const regions = [
    { label: 'France', value: 'FR' },
    { label: 'Europe', value: 'EU' },
    { label: 'États-Unis', value: 'US' },
    { label: 'Monde', value: 'GLOBAL' }
  ];

  let step = 1;
  let primaryGoal = goals[0].value;
  let platforms: string[] = ['YouTube'];
  let categories: string[] = ['Tech'];
  let selectedRegions: string[] = ['FR'];
  let loading = false;

  function toggleRegion(value: string) {
    selectedRegions = selectedRegions.includes(value)
      ? selectedRegions.filter((item) => item !== value)
      : [...selectedRegions, value];
  }

  async function finish() {
    loading = true;
    try {
      await saveOnboarding({
        primary_goal: primaryGoal,
        platforms,
        categories,
        regions: selectedRegions
      });
    } catch {
      pushNotification({
        id: crypto.randomUUID(),
        title: 'Préférences sauvegardées localement',
        body: 'La synchronisation serveur sera réessayée plus tard.',
        level: 'warning'
      });
    } finally {
      loading = false;
      goto('/radar');
    }
  }
</script>

<section class="onboarding-page">
  <div class="wrapper">
    <BrandLogo size="md" showTagline={true} />
    <h1>Personnalisez votre Radar du jour</h1>

    <AuthCard>
      {#if step === 1}
        <OnboardingStep index={1} total={4} title="Quel est votre objectif principal ?" description="Nous adaptons vos tendances prioritaires selon votre usage.">
          <div class="options">
            {#each goals as goal}
              <label class="radio"><input type="radio" name="goal" bind:group={primaryGoal} value={goal.value} />{goal.label}</label>
            {/each}
          </div>
        </OnboardingStep>
      {:else if step === 2}
        <OnboardingStep index={2} total={4} title="Quelles plateformes voulez-vous suivre ?" description="Sélectionnez une ou plusieurs plateformes.">
          <PlatformSelector bind:selected={platforms} />
        </OnboardingStep>
      {:else if step === 3}
        <OnboardingStep index={3} total={4} title="Quelles catégories vous intéressent ?" description="Affinez votre flux selon vos sujets.">
          <CategorySelector bind:selected={categories} />
        </OnboardingStep>
      {:else}
        <OnboardingStep index={4} total={4} title="Quelles régions voulez-vous surveiller ?" description="Choisissez les zones à suivre dans votre radar.">
          <div class="options">
            {#each regions as region}
              <button type="button" class:selected={selectedRegions.includes(region.value)} on:click={() => toggleRegion(region.value)}>{region.label}</button>
            {/each}
          </div>
        </OnboardingStep>
      {/if}

      <div class="actions">
        <button type="button" on:click={() => (step = Math.max(1, step - 1))} disabled={step === 1}>Retour</button>
        {#if step < 4}
          <button type="button" class="primary" on:click={() => (step = Math.min(4, step + 1))}>Suivant</button>
        {:else}
          <button type="button" class="primary" on:click={finish} disabled={loading}>{loading ? 'Finalisation...' : 'Accéder à mon Radar du jour'}</button>
        {/if}
      </div>
    </AuthCard>
  </div>
</section>

<style>
  .onboarding-page { min-height: calc(100vh - 56px); padding: 1rem 1rem 2.5rem; }
  .wrapper { max-width: 720px; margin: 0 auto; display: grid; gap: 1rem; }
  h1 { margin: 0.2rem 0 0; }

  .options { display: grid; gap: 0.6rem; }
  .radio {
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 0.65rem 0.75rem;
    display: flex;
    gap: 0.6rem;
    align-items: flex-start;
  }

  .options button {
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
    border-radius: 999px;
    padding: 0.45rem 0.8rem;
  }

  .options button.selected { border-color: var(--primary); background: var(--primary-soft); }

  .actions { display: flex; justify-content: space-between; gap: 0.6rem; margin-top: 1.25rem; }
  .actions button {
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
    border-radius: 12px;
    padding: 0.62rem 0.85rem;
    font-weight: 700;
  }

  .actions .primary {
    border-color: transparent;
    background: var(--primary);
    color: white;
  }
</style>

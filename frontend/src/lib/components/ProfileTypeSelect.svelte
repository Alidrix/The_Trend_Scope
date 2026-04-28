<script lang="ts">
  export let value = 'creator';

  const options = [
    { value: 'creator', label: 'Créateur de contenu' },
    { value: 'influencer', label: 'Influenceur' },
    { value: 'agency', label: 'Agence social media' },
    { value: 'brand', label: 'Marque' },
    { value: 'freelance', label: 'Freelance' },
    { value: 'student', label: 'Étudiant' },
    { value: 'other', label: 'Autre' }
  ];
</script>

<select bind:value>
  {#each options as option}
    <option value={option.value}>{option.label}</option>
  {/each}
</select>

<style>
  select {
    width: 100%;
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--text);
    border-radius: 12px;
    padding: 0.75rem 0.85rem;
    font: inherit;
  }
</style>

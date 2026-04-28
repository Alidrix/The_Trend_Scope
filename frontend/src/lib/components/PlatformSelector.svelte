<script lang="ts">
  export let selected: string[] = [];

  const platforms = ['YouTube', 'TikTok', 'Instagram'];

  function toggle(platform: string) {
    selected = selected.includes(platform)
      ? selected.filter((item) => item !== platform)
      : [...selected, platform];
  }
</script>

<div class="chips">
  {#each platforms as platform}
    <button type="button" class:active={selected.includes(platform)} on:click={() => toggle(platform)}>{platform}</button>
  {/each}
</div>

<style>
  .chips { display: flex; flex-wrap: wrap; gap: 0.5rem; }
  button {
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
    border-radius: 999px;
    padding: 0.45rem 0.85rem;
    font-weight: 600;
    cursor: pointer;
  }
  .active { border-color: var(--primary); background: var(--primary-soft); }
</style>

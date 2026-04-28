<script lang="ts">
  export let selected: string[] = [];

  const categories = ['Business', 'Gaming', 'Tech', 'Lifestyle', 'Sport', 'Food', 'Travel', 'Music', 'Humor', 'Finance', 'Beauty', 'Education', 'Automotive'];

  function toggle(category: string) {
    selected = selected.includes(category)
      ? selected.filter((item) => item !== category)
      : [...selected, category];
  }
</script>

<div class="chips">
  {#each categories as category}
    <button type="button" class:active={selected.includes(category)} on:click={() => toggle(category)}>{category}</button>
  {/each}
</div>

<style>
  .chips { display: flex; flex-wrap: wrap; gap: 0.45rem; }
  button {
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
    border-radius: 999px;
    padding: 0.4rem 0.75rem;
    cursor: pointer;
  }
  .active { border-color: var(--primary); background: var(--primary-soft); }
</style>

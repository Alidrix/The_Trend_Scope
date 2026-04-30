<script lang="ts">
  import StatusBadge from './StatusBadge.svelte';

  type StatusItem = {
    label: string;
    status: string;
    hint?: string;
  };

  export let items: StatusItem[] = [];
</script>

<div class="status-list">
  {#each items as item}
    <div class="status-row">
      <span class="label">{item.label}</span>
      <StatusBadge status={item.status} />
      {#if item.hint}
        <small>{item.hint}</small>
      {/if}
    </div>
  {/each}
</div>

<style>
  .status-list {
    display: grid;
    gap: 0.6rem;
  }

  .status-row {
    display: grid;
    grid-template-columns: minmax(140px, 1fr) auto;
    gap: 0.6rem 1rem;
    align-items: center;
  }

  .label {
    font-weight: 500;
  }

  small {
    grid-column: 1 / -1;
    color: var(--muted);
  }
</style>

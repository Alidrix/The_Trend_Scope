<script lang="ts">
  export let label = '';
  export let required = false;
  export let checked = false;
</script>

<label class="consent">
  <input type="checkbox" bind:checked required={required} />
  <span>{label}</span>
</label>

<style>
  .consent { display: flex; gap: 0.5rem; color: var(--muted); }
</style>

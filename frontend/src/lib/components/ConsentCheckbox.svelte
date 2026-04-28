<script lang="ts">
  export let label = '';
  export let required = false;
  export let checked = false;
  export let id = `consent-${Math.random().toString(36).slice(2)}`;
</script>

<label class="consent" for={id}>
  <input id={id} type="checkbox" bind:checked required={required} aria-required={required} />
  <span>{label}{#if required}<strong> *</strong>{/if}</span>
</label>

<style>
  .consent {
    display: flex;
    align-items: flex-start;
    gap: 0.55rem;
    color: var(--muted);
    font-size: 0.94rem;
    line-height: 1.4;
  }

  input {
    margin-top: 0.15rem;
  }

  strong {
    color: var(--text);
  }
</style>

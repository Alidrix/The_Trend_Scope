<script lang="ts">
  export let title = '';
  export let description = '';
  export let index = 1;
  export let total = 4;
</script>

<section class="step">
  <p class="meta">Étape {index} / {total}</p>
  <h2>{title}</h2>
  <p class="description">{description}</p>
  <slot />
</section>

<style>
  .step { display: grid; gap: 0.9rem; }
  .meta { margin: 0; color: var(--primary); font-weight: 700; font-size: 0.85rem; }
  h2 { margin: 0; font-size: 1.3rem; }
  .description { margin: 0; color: var(--muted); }
</style>

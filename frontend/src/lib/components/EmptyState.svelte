<script lang="ts">
  export let title = 'Aucune donnée';
  export let message = 'Lancez un scan pour afficher des vidéos.';
</script>

<div class="empty">
  <h3>{title}</h3>
  <p>{message}</p>
</div>

<style>
  .empty { border: 1px dashed #2d3a52; border-radius: var(--radius-card); padding: 2rem; text-align: center; background: #0e1420; }
  h3 { color: var(--color-text); margin-bottom: 0.5rem; }
  p { color: var(--color-muted); margin: 0; }
</style>

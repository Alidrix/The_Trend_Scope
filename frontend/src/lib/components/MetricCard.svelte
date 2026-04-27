<script lang="ts">
  export let title = '';
  export let value = '';
  export let subtitle = '';
</script>

<article class="metric">
  <p>{title}</p>
  <h3>{value}</h3>
  <small>{subtitle}</small>
</article>

<style>
  .metric { background: linear-gradient(160deg, #151b29, #101621); border: 1px solid #243044; border-radius: var(--radius-card); padding: 1rem; }
  p { color: var(--color-muted); margin: 0; }
  h3 { margin: 0.4rem 0; color: var(--color-text); font-size: 1.5rem; }
  small { color: #7f93b3; }
</style>

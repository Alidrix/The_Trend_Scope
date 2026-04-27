<script lang="ts">
  import '$lib/styles/tokens.css';
</script>

<svelte:head>
  <style>
    :global(body) {
      margin: 0;
      font-family: 'Inter', system-ui, -apple-system, sans-serif;
      background: radial-gradient(circle at 10% 20%, #111827, #090b13 60%);
      color: var(--color-text);
    }
    :global(*) { box-sizing: border-box; }
    :global(a) {
      color: var(--color-tiktok-cyan);
      text-decoration: none;
    }
  </style>
</svelte:head>

<slot />

<script lang="ts">
  import '$lib/styles/tokens.css';
</script>

<svelte:head>
  <style>
    :global(body) {
      margin: 0;
      font-family: 'Inter', system-ui, -apple-system, sans-serif;
      background: #f5f7fb;
      color: #101828;
    }
    :global(*) { box-sizing: border-box; }
    :global(a) {
      color: #0f172a;
      text-decoration: none;
    }
  </style>
</svelte:head>

<slot />

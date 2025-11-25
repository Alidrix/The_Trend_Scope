<svelte:head>
  <style>
    :global(body) {
      margin: 0;
      font-family: 'Inter', system-ui, -apple-system, sans-serif;
      background: radial-gradient(circle at 10% 20%, #dce8ff, #f2e6ff 60%);
      color: #1f2a44;
    }
    :global(a) {
      color: #5f6bff;
      text-decoration: none;
    }
  </style>
</svelte:head>

<slot />

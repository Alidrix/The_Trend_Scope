<script lang="ts">
  import { onMount } from 'svelte';
  import '$lib/styles/tokens.css';
  import ThemeToggle from '$lib/components/ThemeToggle.svelte';
  import { applyTheme, theme } from '$lib/stores/theme';

  onMount(() => {
    const unsubscribe = theme.subscribe((value) => applyTheme(value));
    return unsubscribe;
  });
</script>

<svelte:head>
  <style>
    :global(html) {
      background: var(--bg);
      color: var(--text);
    }
    :global(body) {
      margin: 0;
      font-family: 'Inter', system-ui, -apple-system, sans-serif;
      background: var(--bg);
      color: var(--text);
    }
    :global(*) { box-sizing: border-box; }
    :global(a) {
      color: var(--text);
      text-decoration: none;
    }
  </style>
</svelte:head>

<div class="theme-bar">
  <ThemeToggle />
</div>
<slot />

<style>
  .theme-bar {
    display: flex;
    justify-content: flex-end;
    padding: 0.75rem 1rem 0;
  }
</style>

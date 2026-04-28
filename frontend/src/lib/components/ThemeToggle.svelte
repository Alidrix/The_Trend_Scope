<script lang="ts">
  import { theme, toggleTheme } from '$lib/stores/theme';

  function onToggle() {
    theme.update((current) => toggleTheme(current));
  }
</script>

<button type="button" class="theme-toggle" on:click={onToggle}>
  {#if $theme === 'dark'}☀️ Clair{:else}🌙 Sombre{/if}
</button>

<style>
  .theme-toggle {
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
    border-radius: 10px;
    padding: 0.5rem 0.75rem;
    cursor: pointer;
  }
</style>

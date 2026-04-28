<script lang="ts">
  import { theme, toggleTheme } from '$lib/stores/theme';

  function onToggle() {
    theme.update((current) => toggleTheme(current));
  }
</script>

<button type="button" class="theme-toggle" on:click={onToggle} aria-label="Basculer le thème">
  <span class="icon" aria-hidden="true">{$theme === 'dark' ? '☀️' : '🌙'}</span>
  <span>{$theme === 'dark' ? 'Mode clair' : 'Mode sombre'}</span>
</button>

<style>
  .theme-toggle {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    border: 1px solid var(--border);
    background: color-mix(in srgb, var(--surface), transparent 8%);
    color: var(--text);
    border-radius: 999px;
    padding: 0.45rem 0.75rem;
    cursor: pointer;
    font-size: 0.86rem;
    font-weight: 600;
  }

  .icon {
    line-height: 1;
  }
</style>

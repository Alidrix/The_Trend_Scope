<script lang="ts">
  import { theme } from '$lib/stores/theme';

  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let showTagline = false;

  const sizeMap = {
    sm: 120,
    md: 168,
    lg: 220
  } as const;

  $: width = sizeMap[size] ?? sizeMap.md;
  $: fallbackSrc = $theme === 'dark' ? '/brand/trend-scope-logo-dark.png' : '/brand/trend-scope-logo-light.png';
</script>

<div class="brand-logo">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="/brand/trend-scope-logo-dark.png" />
    <img src={fallbackSrc} alt="The Trend Scope — Create Success" width={width} loading="eager" decoding="async" />
  </picture>
  {#if showTagline}
    <p>Create Success</p>
  {/if}
</div>

<style>
  .brand-logo {
    display: inline-flex;
    flex-direction: column;
    gap: 0.35rem;
    align-items: flex-start;
  }

  img {
    display: block;
    height: auto;
    max-width: 100%;
  }

  p {
    margin: 0;
    color: var(--muted);
    font-size: 0.9rem;
    font-weight: 500;
    letter-spacing: 0.01em;
  }
</style>

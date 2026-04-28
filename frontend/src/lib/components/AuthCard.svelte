<div class="auth-card">
  <slot />
</div>

<style>
  .auth-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-card);
    padding: clamp(1.1rem, 2vw, 1.75rem);
    box-shadow: 0 12px 40px rgba(15, 23, 42, 0.08);
  }
</style>

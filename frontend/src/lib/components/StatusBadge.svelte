<script lang="ts">
  export let label = '';
  export let tone: 'success' | 'warning' | 'danger' | 'neutral' = 'neutral';
</script>

<span class={`badge ${tone}`}>{label}</span>

<style>
  .badge { padding: 0.3rem 0.7rem; border-radius: 999px; font-size: 0.78rem; font-weight: 700; }
  .neutral { background: #253145; color: #c5d2e8; }
  .success { background: rgba(37, 244, 238, 0.18); color: var(--color-tiktok-cyan); }
  .warning { background: rgba(255, 153, 0, 0.2); color: #ffc46b; }
  .danger { background: rgba(254, 44, 85, 0.18); color: var(--color-tiktok-pink); }
</style>

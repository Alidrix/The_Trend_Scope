<script lang="ts">
  export let status = 'pending';
  const labels: Record<string,string> = {not_configured:'Non configuré',configured_but_not_implemented:'Configuré (non implémenté)'};
  $: tone = status;
</script>
<span class={`badge ${tone}`}>{labels[status] ?? status}</span>
<style>
.badge{padding:.2rem .6rem;border-radius:999px;font-size:.75rem;border:1px solid var(--border);background:var(--surface);color:var(--text)}
.ok,.configured,.active,.completed,.sent,.studio,.pro{background:color-mix(in srgb,var(--primary) 14%, transparent)}
.error,.failed,.not_configured,.inactive{background:color-mix(in srgb,#b33 14%, transparent)}
.pending,.logged,.configured_but_not_implemented,.free{background:color-mix(in srgb,#888 14%, transparent)}
</style>

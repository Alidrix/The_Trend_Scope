<script lang="ts">
  export let status = 'pending';
  export let label: string | undefined = undefined;
  export let tone: string | undefined = undefined;
  const labels: Record<string, string> = {
    ok: 'OK', error: 'Erreur', warning: 'Attention', manual: 'Manuel', configured: 'Configuré', not_configured: 'Non configuré',
    coming_soon: 'Bientôt', configured_but_not_implemented: 'Configuré (non implémenté)', active: 'Actif', inactive: 'Inactif',
    pending: 'En attente', completed: 'Terminé', failed: 'Échec', sent: 'Envoyé', skipped: 'Ignoré', logged: 'Journalisé', free: 'Free', pro: 'Pro', studio: 'Studio'
  };
</script>
<span class={`badge ${tone ?? status}`}>{label ?? labels[status] ?? status}</span>
<style>
.badge{padding:.2rem .6rem;border-radius:999px;font-size:.75rem;border:1px solid var(--border);background:var(--surface);color:var(--text)}
.ok,.configured,.active,.completed,.sent,.studio,.pro{background:color-mix(in srgb,var(--primary) 14%, transparent)}
.error,.failed,.not_configured,.inactive{background:color-mix(in srgb,#b33 14%, transparent)}
.pending,.configured_but_not_implemented,.free,.warning,.manual,.coming_soon,.skipped,.logged{background:color-mix(in srgb,#888 14%, transparent)}
</style>

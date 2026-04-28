<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import { createCheckout, fetchBillingStatus, openBillingPortal } from '$lib/api';
  import { currentUser } from '$lib/stores/user';

  let status: any = { provider: 'stripe', enabled: false, message: 'Chargement...' };

  onMount(async () => {
    try { status = await fetchBillingStatus(); } catch { status = { enabled: false, message: 'Le paiement Stripe n’est pas encore configuré sur cet environnement.' }; }
  });

  async function checkout(plan: 'pro' | 'studio') {
    const res = await createCheckout(plan);
    if (res.checkout_url) window.location.href = res.checkout_url;
  }

  async function portal() {
    const res = await openBillingPortal();
    if (res.url) window.location.href = res.url;
  }
</script>

<AppShell>
  <PageHeader title="Abonnement" subtitle="Gérez votre offre, vos paiements et votre montée en puissance." />
  <section class="card">
    <p><strong>Plan actuel :</strong> {$currentUser?.plan?.toUpperCase() || 'FREE'}</p>
    <p><strong>État abonnement :</strong> {status.subscription_status || 'inactive'}</p>
    <p><strong>Renouvellement :</strong> {status.current_period_end ? new Date(status.current_period_end).toLocaleDateString() : '—'}</p>
    <div class="actions">
      <button on:click={() => checkout('pro')}>Passer Pro — 10 €/mois</button>
      <button on:click={() => checkout('studio')}>Choisir Studio — 18 €/mois</button>
      <button class="ghost" on:click={portal}>Ouvrir le portail client</button>
    </div>
    {#if !status.enabled}<p class="muted">Stripe non configuré : {status.message}</p>{/if}
  </section>
</AppShell>

<style>
  .card{background:var(--surface);border:1px solid var(--border);border-radius:16px;padding:1rem;margin-top:1rem}
  .actions{display:flex;gap:.6rem;flex-wrap:wrap}
  button{background:var(--primary);color:var(--surface);border:none;border-radius:10px;padding:.6rem .8rem;cursor:pointer}
  .ghost{background:var(--surface-soft);color:var(--text)}
  .muted{color:var(--muted)}
</style>

<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import AdminSection from '$lib/components/AdminSection.svelte';
  import AdminStatCard from '$lib/components/AdminStatCard.svelte';
  import AdminStatusList from '$lib/components/AdminStatusList.svelte';
  import AdminToolbar from '$lib/components/AdminToolbar.svelte';
  import { currentUser } from '$lib/stores/user';
  import { fetchAdminBilling, type AdminBilling } from '$lib/api';
  import { getErrorMessage } from '$lib/errors';

  let d: AdminBilling | null = null;
  let error = '';
  let loading = false;

  const formatEuros = (cents: number) =>
    `${(cents / 100).toLocaleString('fr-FR', { minimumFractionDigits: 2, maximumFractionDigits: 2 })} €`;

  const load = async () => {
    loading = true; error = '';
    try { d = await fetchAdminBilling(); } catch (err: unknown) { error = getErrorMessage(err, 'Erreur'); } finally { loading = false; }
  };

  $: stripeItems = [
    { label: 'Configured', status: d?.stripe?.configured ? 'configured' : 'not_configured' },
    { label: 'Webhook configured', status: d?.stripe?.webhook_configured ? 'configured' : 'not_configured' },
    { label: 'Price Pro configured', status: d?.stripe?.price_pro_configured ? 'configured' : 'not_configured' },
    { label: 'Price Studio configured', status: d?.stripe?.price_studio_configured ? 'configured' : 'not_configured' }
  ];

  onMount(load);
</script>
<AppShell>{#if $currentUser?.role !== 'admin'}<p>Accès restreint</p>{:else}
  <PageHeader title="Admin Billing" subtitle="Abonnements et Stripe" />
  <AdminToolbar {loading} {error}><button type="button" disabled={loading} on:click={load}>Refresh</button></AdminToolbar>
  {#if d}
    <AdminSection title="Abonnements"><div class="grid"><AdminStatCard label="Total" value={String(d.subscriptions?.total ?? 0)} /><AdminStatCard label="Active" value={String(d.subscriptions?.active ?? 0)} /><AdminStatCard label="Inactive" value={String(d.subscriptions?.inactive ?? 0)} /><AdminStatCard label="Pro" value={String(d.subscriptions?.pro ?? 0)} /><AdminStatCard label="Studio" value={String(d.subscriptions?.studio ?? 0)} /></div></AdminSection>
    <AdminSection title="MRR"><div class="grid"><AdminStatCard label="MRR estimé" value={formatEuros(d.mrr?.estimate_cents ?? 0)} /><AdminStatCard label="Currency" value={d.mrr?.currency ?? 'EUR'} /><AdminStatCard label="Pro unit price" value={formatEuros(d.mrr?.pro_unit_cents ?? 0)} /><AdminStatCard label="Studio unit price" value={formatEuros(d.mrr?.studio_unit_cents ?? 0)} /></div><p>MRR estimé à partir des plans internes. Ce n’est pas une source comptable officielle Stripe.</p></AdminSection>
    <AdminSection title="Stripe"><AdminStatusList items={stripeItems} /></AdminSection>
  {/if}
{/if}</AppShell>
<style>.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(170px,1fr));gap:.75rem}</style>

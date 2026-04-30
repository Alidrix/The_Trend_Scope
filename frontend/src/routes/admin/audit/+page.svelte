<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import AdminToolbar from '$lib/components/AdminToolbar.svelte';
  import AdminSection from '$lib/components/AdminSection.svelte';
  import DataTable from '$lib/components/DataTable.svelte';
  import { fetchAdminAuditLogs, type AdminAuditLog } from '$lib/api';
  import { getErrorMessage } from '$lib/errors';
  import { currentUser } from '$lib/stores/user';

  let loading = true;
  let error = '';
  let logs: AdminAuditLog[] = [];

  const load = async () => {
    loading = true;
    error = '';
    try {
      const res = await fetchAdminAuditLogs();
      logs = res.logs ?? [];
    } catch (err: unknown) {
      error = getErrorMessage(err, 'Failed to load audit logs');
    } finally {
      loading = false;
    }
  };

  const columns = [
    { key: 'created_at', label: 'Date', type: 'date' as const },
    { key: 'admin_username', label: 'Admin' },
    { key: 'action', label: 'Action', type: 'status' as const },
    { key: 'target', label: 'Target' },
    { key: 'status', label: 'Status', type: 'status' as const }
  ];

  onMount(load);
</script>

<AppShell>
  {#if $currentUser?.role !== 'admin'}
    <p>Accès restreint</p>
  {:else}
    <PageHeader title="Admin Audit" subtitle="Dernières actions administrateur" />

    <AdminToolbar {loading} {error}>
      <button type="button" disabled={loading} on:click={load}>Refresh</button>
    </AdminToolbar>

    <AdminSection title="Audit admin">
      {#if loading}
        <p>Chargement…</p>
      {:else if logs.length === 0}
        <p>Aucun log d’audit pour le moment.</p>
      {:else}
        <DataTable {columns} rows={logs} />
        <p>Metadata disponible côté API, non affichée par défaut.</p>
      {/if}
    </AdminSection>
  {/if}
</AppShell>

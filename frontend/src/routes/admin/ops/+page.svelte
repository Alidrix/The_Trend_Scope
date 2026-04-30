<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/AppShell.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import AdminSection from '$lib/components/AdminSection.svelte';
  import AdminStatCard from '$lib/components/AdminStatCard.svelte';
  import DataTable from '$lib/components/DataTable.svelte';
  import StatusBadge from '$lib/components/StatusBadge.svelte';
  import { currentUser } from '$lib/stores/user';
  import {
    fetchAdminEmailLogs,
    fetchAdminExports,
    fetchAdminNotifications,
    fetchAdminSmoke,
    fetchAdminSystem,
    testAdminSmtp,
    testAdminStripe,
    testAdminTelegram,
    testAdminYoutube,
    type AdminEmailLog,
    type AdminExport,
    type AdminNotificationSnapshot,
    type AdminSmoke,
    type AdminSystem,
    type AdminTestResult
  } from '$lib/api';

  let system: AdminSystem | null = null;
  let logs: AdminEmailLog[] = [];
  let notifs: AdminNotificationSnapshot | null = null;
  let exportsData: AdminExport[] = [];
  let smoke: AdminSmoke | null = null;

  let smtpTo = '';
  let chatId = '';
  let loading: Record<string, boolean> = {};
  let latest: Record<string, string> = {};
  let history: { key: string; ok: boolean; message: string; date: string }[] = [];
  let loadingGlobal = false;
  let errorGlobal = '';

  $: smokeRows = smoke
    ? Object.entries(smoke.checks || {}).map(([check, status]) => ({
        check,
        status,
        blocking: smoke.blocking?.[check] ? 'yes' : 'no'
      }))
    : [];

  $: blockingFailed = smoke
    ? Object.entries(smoke.checks || {}).filter(([check, status]) => {
        const isBlocking = smoke?.blocking?.[check];
        return isBlocking && !['ok', 'configured'].includes(String(status));
      }).length
    : 0;

  const load = async () => {
    loadingGlobal = true;
    errorGlobal = '';
    try {
      system = await fetchAdminSystem();
      logs = (await fetchAdminEmailLogs()).logs || [];
      notifs = await fetchAdminNotifications();
      exportsData = (await fetchAdminExports()).exports || [];
    } catch (e: any) {
      errorGlobal = e.message || 'Erreur';
    } finally {
      loadingGlobal = false;
    }
  };

  const record = (key: string, ok: boolean, message: string) => {
    latest[key] = message;
    history = [{ key, ok, message, date: new Date().toISOString() }, ...history].slice(0, 20);
  };

  const run = async (key: 'smtp' | 'telegram' | 'youtube' | 'stripe') => {
    if (key === 'smtp' && !smtpTo.trim()) {
      record(key, false, 'Renseigne un email destinataire avant de tester SMTP.');
      return;
    }

    loading[key] = true;
    try {
      let response: AdminTestResult;
      if (key === 'smtp') response = await testAdminSmtp({ to: smtpTo });
      else if (key === 'telegram') response = await testAdminTelegram({ chat_id: chatId || undefined });
      else if (key === 'youtube') response = await testAdminYoutube();
      else response = await testAdminStripe();

      record(key, Boolean(response.ok ?? response.sent), response.message ?? response.reason ?? 'ok');
    } catch (e: any) {
      record(key, false, e.message || 'error');
    } finally {
      loading[key] = false;
    }
  };

  const runSmoke = async () => {
    smoke = await fetchAdminSmoke();
  };

  onMount(load);
</script>

<AppShell>
  {#if $currentUser?.role !== 'admin'}
    <p>Accès restreint</p>
  {:else}
    <PageHeader title="Admin ops" subtitle="Cockpit d'exploitation" />
    <button on:click={load}>Refresh</button>

    {#if loadingGlobal}<p>Chargement global…</p>{/if}
    {#if errorGlobal}<p>{errorGlobal}</p>{/if}

    <AdminSection title="Tests d'intégration">
      <div class="g">
        <AdminStatCard
          label="SMTP"
          value={system?.integrations?.smtp ?? 'not_configured'}
          status={system?.integrations?.smtp ?? 'not_configured'}
          hint={latest.smtp ?? ''}
        />
        <input bind:value={smtpTo} placeholder="Destinataire test" />
        <button disabled={loading.smtp} on:click={() => run('smtp')}>Tester SMTP</button>

        <AdminStatCard
          label="Telegram"
          value={system?.integrations?.telegram ?? 'not_configured'}
          status={system?.integrations?.telegram ?? 'not_configured'}
          hint={latest.telegram ?? ''}
        />
        <input bind:value={chatId} placeholder="Chat ID optionnel" />
        <button disabled={loading.telegram} on:click={() => run('telegram')}>Tester Telegram</button>

        <AdminStatCard
          label="YouTube API"
          value={system?.integrations?.youtube ?? 'not_configured'}
          status={system?.integrations?.youtube ?? 'not_configured'}
          hint={latest.youtube ?? ''}
        />
        <button disabled={loading.youtube} on:click={() => run('youtube')}>Tester YouTube</button>

        <AdminStatCard
          label="Stripe"
          value={system?.integrations?.stripe ?? 'not_configured'}
          status={system?.integrations?.stripe ?? 'not_configured'}
          hint={latest.stripe ?? ''}
        />
        <button disabled={loading.stripe} on:click={() => run('stripe')}>Tester Stripe</button>
      </div>
    </AdminSection>

    <AdminSection title="Smoke interne">
      <button on:click={runSmoke}>Lancer smoke</button>
      {#if smoke}
        <p>Smoke global : <StatusBadge status={smoke.ok ? 'ok' : 'error'} /></p>
        <p>Blocking failed : {blockingFailed}</p>
        <DataTable
          columns={[
            { key: 'check', label: 'Check' },
            { key: 'status', label: 'Status', type: 'status' },
            { key: 'blocking', label: 'Blocking' }
          ]}
          rows={smokeRows}
        />
      {/if}
    </AdminSection>

    <AdminSection title="Résultats des tests">
      <DataTable
        columns={[
          { key: 'key', label: 'Test' },
          { key: 'ok', label: 'OK' },
          { key: 'message', label: 'Message' },
          { key: 'date', label: 'Date', type: 'date' }
        ]}
        rows={history}
      />
    </AdminSection>

    <AdminSection title="Logs email">
      <DataTable
        columns={[
          { key: 'recipient', label: 'Recipient' },
          { key: 'subject', label: 'Subject' },
          { key: 'status', label: 'Status', type: 'status' },
          { key: 'error_message', label: 'Error' },
          { key: 'created_at', label: 'Date', type: 'date' }
        ]}
        rows={logs}
      />
    </AdminSection>

    <AdminSection title="Notifications">
      <div class="g">
        <AdminStatCard label="Total" value={notifs?.total ?? 0} />
        <AdminStatCard label="Unread" value={notifs?.unread ?? 0} />
      </div>
      {#if notifs?.latest?.length}
        <DataTable
          columns={[
            { key: 'title', label: 'Title' },
            { key: 'body', label: 'Body' },
            { key: 'type', label: 'Type', type: 'status' },
            { key: 'created_at', label: 'Date', type: 'date' }
          ]}
          rows={notifs.latest}
        />
      {/if}
    </AdminSection>

    <AdminSection title="Exports">
      <DataTable
        columns={[
          { key: 'title', label: 'Title' },
          { key: 'format', label: 'Format' },
          { key: 'file_url', label: 'File URL', type: 'url' },
          { key: 'created_at', label: 'Created at', type: 'date' }
        ]}
        rows={exportsData}
      />
    </AdminSection>
  {/if}
</AppShell>

<style>
  .g {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 0.6rem;
  }
</style>

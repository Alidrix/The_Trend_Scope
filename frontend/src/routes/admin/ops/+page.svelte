<script lang="ts">
  import { onMount } from 'svelte';
  import { fetchAdminEmailLogs, fetchAdminNotifications, fetchAdminExports, fetchAdminSystem, testAdminSmtp, testAdminTelegram } from '$lib/api';
  let system:any={}, logs:any[]=[], notifications:any, exports:any[]=[];
  let smtpTo='admin@example.com'; let chatId='';
  async function load(){ system=await fetchAdminSystem(); logs=(await fetchAdminEmailLogs()).logs ?? []; notifications=await fetchAdminNotifications(); exports=(await fetchAdminExports()).exports ?? []; }
  onMount(load);
</script>
<h1>Admin Ops</h1>
<p>SMTP: {system.smtp_configured ? 'configured' : 'not configured'}</p>
<button on:click={async()=>{await testAdminSmtp({to:smtpTo}); await load();}}>Test SMTP</button>
<p>Telegram: {system.telegram_configured ? 'configured' : 'not configured'}</p>
<button on:click={async()=>{await testAdminTelegram({chat_id:chatId || undefined});}}>Test Telegram</button>
<h2>Email logs</h2>{#each logs as l}<div>{l.recipient} - {l.status}</div>{/each}
<h2>Notifications</h2><div>Total: {notifications?.total} / Unread: {notifications?.unread}</div>
<h2>Exports</h2>{#each exports as e}<div>{e.title} - {e.file_url}</div>{/each}

<script lang="ts">
import { onMount } from 'svelte';
import AppShell from '$lib/components/AppShell.svelte';
import PageHeader from '$lib/components/PageHeader.svelte';
import AdminSection from '$lib/components/AdminSection.svelte';
import AdminStatCard from '$lib/components/AdminStatCard.svelte';
import { currentUser } from '$lib/stores/user';
import { fetchAdminSystem, type AdminSystem } from '$lib/api';
import StatusBadge from '$lib/components/StatusBadge.svelte';
let d: AdminSystem | null = null; let loading=false; let error='';
const load=async()=>{loading=true;error='';try{d=await fetchAdminSystem();}catch(e:any){error=e.message||'Erreur';}finally{loading=false;}};
onMount(load);
</script>
<AppShell>{#if $currentUser?.role!=='admin'}<p>Accès restreint</p>{:else}<PageHeader title="Admin System" subtitle="Runtime, services, intégrations"/><button on:click={load}>Refresh</button>{#if loading}<p>Chargement…</p>{/if}{#if error}<p>{error}</p>{/if}{#if d}<AdminSection title="Runtime"><div class="g"><AdminStatCard label="Environment" value={d?.runtime?.env??'-'}/><AdminStatCard label="Frontend origin" value={d?.runtime?.frontend_origin??'-'}/></div></AdminSection><AdminSection title="Services internes"><p>PostgreSQL <StatusBadge status={d?.services?.postgres||'error'}/></p><p>Redis <StatusBadge status={d?.services?.redis||'error'}/></p><p>NATS <StatusBadge status={d?.services?.nats||'configured'}/></p><p>ClickHouse <StatusBadge status={d?.services?.clickhouse||'not_configured'}/></p><small>NATS peut être affiché comme "configured" lorsqu’un check non destructif actif n’est pas encore exécuté.</small></AdminSection><AdminSection title="Intégrations"><p>YouTube <StatusBadge status={d?.integrations?.youtube||'not_configured'}/></p><p>Stripe <StatusBadge status={d?.integrations?.stripe||'not_configured'}/></p><p>SMTP <StatusBadge status={d?.integrations?.smtp||'not_configured'}/></p><p>Telegram <StatusBadge status={d?.integrations?.telegram||'not_configured'}/></p><p>Cloudflare <StatusBadge status={d?.integrations?.cloudflare||'not_configured'}/></p></AdminSection><AdminSection title="Stockage"><p>Local exports dir: {d?.storage?.local_exports_dir || '-'}</p><p>S3 / MinIO <StatusBadge status={d?.storage?.s3||'not_configured'}/></p></AdminSection>{/if}{/if}</AppShell>
<style>.g{display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:.6rem}</style>

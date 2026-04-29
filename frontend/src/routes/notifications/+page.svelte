<script lang="ts">
import {onMount} from 'svelte';
import {fetchNotifications,fetchUnreadNotificationsCount,markAllNotificationsRead,markNotificationRead} from '$lib/api';
import NotificationCard from '$lib/components/NotificationCard.svelte';
let loading=true,error='',items:any[]=[],unread=0;
async function load(){loading=true;error='';try{const [a,b]=await Promise.all([fetchNotifications(),fetchUnreadNotificationsCount()]);items=a.notifications||[];unread=b.count||0;}catch(e:any){error=e.message}finally{loading=false}}
onMount(load);
</script>
<h1>Notifications</h1><p>Non lues: {unread}</p><button on:click={async()=>{await markAllNotificationsRead();await load();}}>Tout marquer comme lu</button>
{#if loading}<p>Loading...</p>{:else if error}<p>{error}</p>{:else if items.length===0}<p>Aucune notification.</p>{:else}{#each items as n}<NotificationCard notification={n} onRead={async()=>{await markNotificationRead(n.id);await load();}} />{/each}{/if}

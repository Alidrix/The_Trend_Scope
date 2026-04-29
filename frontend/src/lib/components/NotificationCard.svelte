<script lang="ts">export let notification:any;export let onRead=()=>{};</script>
<div class="card"><h3>{notification.title}</h3><p>{notification.body}</p>{#if !notification.read_at}<button on:click={onRead}>Marquer comme lu</button>{/if}</div>

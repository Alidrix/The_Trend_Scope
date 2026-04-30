<script lang="ts">export let items:any[]=[];</script>
{#each items as item}<div><b>{item.label}</b> — {item.status} {item.blocking ? '(blocking)' : '(optional)'}</div>{/each}

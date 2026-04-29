<script lang="ts">import StatusBadge from './StatusBadge.svelte'; import { buildApiUrl } from '$lib/api'; export let item:any;</script>
<article><h3>{item.title}</h3><StatusBadge status={item.status} /><p>{item.format} · {(item.platforms||[]).join(',')}</p>
{#if item.format==='json'}<p>Résumé généré</p>{:else if item.format==='csv' && item.file_url}<a href={buildApiUrl(item.file_url)}>Télécharger CSV</a>{:else if item.format==='csv'}<p>CSV en préparation</p>{:else if item.format==='pdf'}<p>PDF bientôt disponible</p>{/if}
</article>

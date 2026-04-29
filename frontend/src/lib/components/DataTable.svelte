<script lang="ts">export let columns=[]; export let rows=[]; export let emptyMessage='Aucune donnée disponible.';</script>
{#if !rows.length}<p>{emptyMessage}</p>{:else}
<table><thead><tr>{#each columns as c}<th>{c.label}</th>{/each}</tr></thead><tbody>{#each rows as r}<tr>{#each columns as c}<td>{r[c.key] ?? '—'}</td>{/each}</tr>{/each}</tbody></table>{/if}
<style>table{width:100%;border-collapse:collapse}th,td{padding:.55rem;border-bottom:1px solid var(--border);text-align:left}th{color:var(--muted)}</style>

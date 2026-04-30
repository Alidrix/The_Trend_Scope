<script lang="ts">
  import StatusBadge from './StatusBadge.svelte';

  type Column = {
    key: string;
    label: string;
    type?: 'text' | 'status' | 'date' | 'url' | 'number' | 'boolean';
  };

  export let columns: Column[] = [];
  export let rows: Record<string, unknown>[] = [];
  export let emptyMessage = 'Aucune donnée disponible.';

  const isEmpty = (value: unknown) =>
    value === null || value === undefined || value === '';

  const formatDate = (value: unknown) => {
    if (isEmpty(value)) return '—';
    const date = new Date(String(value));
    return Number.isNaN(date.getTime()) ? String(value) : date.toLocaleString();
  };

  const formatValue = (value: unknown, type: Column['type'] = 'text') => {
    if (isEmpty(value)) return '—';

    if (type === 'date') return formatDate(value);
    if (type === 'number') return Number(value).toLocaleString();
    if (type === 'boolean') return value ? 'Oui' : 'Non';

    return String(value);
  };
</script>

{#if rows.length === 0}
  <p>{emptyMessage}</p>
{:else}
  <div class="table-wrap">
    <table>
      <thead>
        <tr>
          {#each columns as column}
            <th>{column.label}</th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each rows as row}
          <tr>
            {#each columns as column}
              <td>
                {#if column.type === 'status'}
                  <StatusBadge status={String(row[column.key] ?? 'pending')} />
                {:else if column.type === 'url' && !isEmpty(row[column.key])}
                  <a href={String(row[column.key])} target="_blank" rel="noreferrer">
                    {String(row[column.key])}
                  </a>
                {:else}
                  {formatValue(row[column.key], column.type)}
                {/if}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
{/if}

<style>
  .table-wrap {
    overflow: auto;
  }

  table {
    width: 100%;
    min-width: 700px;
    border-collapse: collapse;
  }

  th,
  td {
    padding: 0.55rem;
    border-bottom: 1px solid var(--border);
    text-align: left;
  }

  th {
    color: var(--muted);
  }
</style>

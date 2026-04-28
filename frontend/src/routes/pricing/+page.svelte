<script lang="ts">
  import { onMount } from 'svelte';
  import { fetchPlans } from '$lib/api';

  let plans: any[] = [];
  onMount(async () => {
    plans = await fetchPlans();
  });
</script>

<section class="pricing">
  <h1>Tarifs</h1>
  <p>Repère les tendances avant les autres et crée du contenu au bon moment.</p>
  <div class="cards">
    {#each plans as plan}
      <article class:recommended={plan.recommended}>
        {#if plan.recommended}<span class="badge">Le plus populaire</span>{/if}
        <h2>{plan.tier}</h2>
        <p class="price">{plan.price_eur_monthly} €/mois</p>
        <ul>
          <li>{plan.limits.daily_trend_limit || 'Tendances illimitées'} tendances/jour</li>
          <li>Historique {plan.limits.history_days} jour(s)</li>
          <li>Alertes: {plan.limits.alerts_enabled ? 'Oui' : 'Non'}</li>
          <li>Rapports: {plan.limits.reports_enabled ? 'Oui' : 'Non'}</li>
        </ul>
      </article>
    {/each}
  </div>
</section>

<style>
  .pricing { max-width: 1050px; margin: 0 auto; padding: 2rem 1rem; color: var(--text); }
  .cards { display: grid; grid-template-columns: repeat(auto-fit, minmax(230px, 1fr)); gap: 1rem; margin-top: 1.2rem; }
  article { background: var(--surface); border: 1px solid #e8ebf2; border-radius: 16px; padding: 1rem; position: relative; }
  .recommended { border-color: #f43f5e; box-shadow: 0 8px 24px rgba(244,63,94,.15); }
  .badge { position: absolute; top: -10px; right: 12px; background: #f43f5e; color: var(--surface); padding: .25rem .55rem; border-radius: 999px; font-size: .74rem; }
  .price { font-size: 1.4rem; font-weight: 700; }
</style>

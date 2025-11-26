<script lang="ts">
  import { onMount } from 'svelte';
  import { requestPermission } from '$lib/stores/notifications';

  onMount(() => {
    requestPermission();
  });
</script>

<section class="hero">
  <div>
    <p class="badge">YouTube / TikTok scout</p>
    <h1>Surveillez les vidéos virales</h1>
    <p class="lede">Dashboard personnel, alertes locales et suivi horaire. Connectez-vous pour commencer.</p>
    <div class="actions">
      <a class="button" href="/login">Se connecter</a>
      <a class="ghost" href="/dashboard">Accéder au dashboard</a>
    </div>
  </div>
</section>

<style>
  .hero {
    min-height: 100vh;
    display: grid;
    place-items: center;
    padding: 3rem;
  }
  .badge {
    display: inline-block;
    padding: 0.4rem 0.8rem;
    border-radius: 999px;
    background: rgba(95, 107, 255, 0.12);
    color: #4a52d6;
    font-weight: 600;
    letter-spacing: 0.2px;
  }
  h1 {
    font-size: clamp(2.5rem, 4vw, 3.8rem);
    margin: 1rem 0 0.5rem;
    color: #1f1c3a;
  }
  .lede {
    max-width: 520px;
    color: #3f4168;
    font-size: 1.1rem;
  }
  .actions {
    margin-top: 1.5rem;
    display: flex;
    gap: 1rem;
  }
  .button {
    background: linear-gradient(135deg, #5f6bff, #9c6bff);
    color: white;
    padding: 0.85rem 1.6rem;
    border-radius: 12px;
    font-weight: 700;
    box-shadow: 0 8px 24px rgba(95, 107, 255, 0.25);
  }
  .ghost {
    padding: 0.85rem 1.2rem;
    border-radius: 12px;
    border: 1px solid rgba(95, 107, 255, 0.35);
    color: #4a52d6;
    font-weight: 700;
  }
</style>

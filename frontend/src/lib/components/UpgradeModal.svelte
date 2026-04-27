<script lang="ts">
  export let visible = false;
  export let message = '';
</script>

{#if visible}
  <div class="backdrop">
    <div class="modal">
      <h3>Limite gratuite atteinte</h3>
      <p>{message}</p>
      <a href="/pricing">Passer en Pro</a>
    </div>
  </div>
{/if}

<style>
  .backdrop { position: fixed; inset: 0; background: rgba(12, 18, 30, .45); display: grid; place-items: center; }
  .modal { background: white; border-radius: 16px; padding: 1.2rem; width: min(460px, 92vw); box-shadow: 0 18px 50px rgba(15, 23, 42, .2); }
  h3 { margin: 0; color: #121826; }
  p { color: #55607a; }
  a { display: inline-block; margin-top: .5rem; background: #e73141; color: #fff; padding: .6rem .9rem; border-radius: 10px; }
</style>

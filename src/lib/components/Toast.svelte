<script lang="ts">
  export let message = '';
  export let type: 'error' | 'success' = 'error';
  export let visible = false;

  $: if (visible) {
    setTimeout(() => visible = false, 3000);
  }
</script>

<div class="toast toast-center toast-middle" class:hidden={!visible}>
  <div class="alert {type === 'error' ? 'alert-error' : 'alert-success'} shadow-lg">
    <span>
      {#if type === 'error'}
        ❌
      {:else}
        ✅
      {/if}
      {message}
    </span>
  </div>
</div>
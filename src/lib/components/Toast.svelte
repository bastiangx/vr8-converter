<script lang="ts">
  export let message = "";
  export let type: "error" | "success" = "error";
  export let visible = false;
</script>

{#if visible}
  <div class="toast toast-start">
    <div
      class="alert {type === 'error'
        ? 'alert-error'
        : 'alert-success'} shadow-md"
    >
      <span>{message}</span>
    </div>
  </div>
{/if}

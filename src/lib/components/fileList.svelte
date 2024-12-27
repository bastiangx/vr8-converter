<script lang="ts">
  export let files: string[];
  export let onCancel: () => void;
  export let onConvert: () => void;
</script>

<div class="card bg-base-200 shadow-xl w-full max-w-lg mx-auto">
  <div class="card-body">
    <h3 class="card-title text-lg">Selected Files</h3>

    <ul class="menu bg-base-100 rounded-box p-4">
      {#each files as file}
        <li class="menu-item tracking-tight">{file.split('/').pop()}</li>
      {/each}
    </ul>

    <div class="card-actions justify-end mt-4">
      <button class="btn btn-ghost" on:click={onCancel}>Cancel</button>
      <button class="btn btn-wide btn-primary" on:click={onConvert}>Convert</button>
    </div>
  </div>
</div>
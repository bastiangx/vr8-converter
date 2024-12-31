<script lang="ts">
  export let files: string[];
  export let onCancel: () => void;
  export let onConvert: () => void;
  export let output_dir: string | null = null;
  export let onPickDir: () => void;
  export let onAddMore: () => Promise<void | string[]>;

  function removeFile(index: number) {
    files = files.filter((_, i) => i !== index);
  }

  $: displayFiles = files.slice(-4);
  $: remainingCount = Math.max(0, files.length - 4);
  $: shortPath = output_dir
    ? output_dir.split("/").slice(-2).join("/")
    : "No output directory selected";
</script>

<div class="card bg-base-200 shadow-xl w-full max-w-lg mx-auto">
  <div class="card-body">
    <h3 class="card-title text-lg">Selected Files ({files.length})</h3>

    <div class="flex justify-between items-center gap-2 mb-2">
      <span class="hidden lg:text-sm truncate tracking-tight">{shortPath}</span>
      <button class="btn w-full lg:btn-wide btn-secondary btn-outline" on:click={onPickDir}>
        Select Output
      </button>
    </div>

    <ul class="menu bg-base-100 rounded-box p-2">
      {#each displayFiles as file, index}
        <li class="menu-item">
          <div class="flex w-full justify-between items-center gap-1">
            <span class="truncate tracking-tight">{file.split("/").pop()}</span>
            <button
              class="btn btn-ghost btn-sm btn-circle flex-shrink-0"
              on:click={() => removeFile(index)}
            >
              ✕
            </button>
          </div>
        </li>
      {/each}
      {#if remainingCount > 0}
        <li class="menu-item text-gray-500">
          + {remainingCount} more files
        </li>
      {/if}
    </ul>

    <div class="flex justify-between mt-4">
      <button class="btn btn-ghost" on:click={onAddMore}>
        Add More Files
      </button>
      <div class="flex gap-2">
        <button class="btn btn-ghost" on:click={onCancel}>Cancel</button>
        <button
          class="btn btn-secondary"
          on:click={() => onConvert()}
          disabled={files.length === 0}
        >
          Convert
        </button>
      </div>
    </div>
  </div>
</div>

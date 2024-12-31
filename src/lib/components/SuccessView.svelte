<script lang="ts">
  export let duration: number;
  export let outputDir: string;
  export let onConvertMore: () => void;
  import { open } from "@tauri-apps/plugin-shell";

  $: formattedDuration = (duration / 1000).toFixed(2);
  $: shortPath = outputDir.split("/").slice(-2).join("/");

  const openFolder = () => {
    open(outputDir);
  };
</script>

<div class="card bg-base-200 shadow-xl w-full max-w-lg mx-auto text-center">
  <div class="card-body">
    <h3 class="card-title text-2xl justify-center">Conversion Complete! 🎉</h3>
    <p class="text-lg">⏱️ Time taken: {formattedDuration}s</p>
    <p class="text-lg">📁 folder: {shortPath}</p>

    <div class="card-actions justify-center mt-4 gap-4">
      <button class="btn btn-outline" on:click={onConvertMore}>
        Convert More
      </button>
      <button class="btn btn-wide btn-secondary" on:click={openFolder}>
        Open Folder
      </button>
    </div>
  </div>
</div>

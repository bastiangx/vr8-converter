<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import Toast from "$lib/components/Toast.svelte";
  import SuccessView from "$lib/components/SuccessView.svelte";
  import FileList from "$lib/components/fileList.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let view: "select" | "confirm" | "success" = "select";
  let converting = false;

  let selectedFiles: string[] = [];
  let selectedOutputDir: string | null = null;

  let result: {
    files_converted: number;
    duration_ms: number;
    output_dir: string;
  } | null = null;

  let toastMessage = "";
  let toastType: "error" | "success" = "error";
  let toastVisible = false;

  let progress = 0;

  function showToast(message: string, type: "error" | "success" = "error") {
    toastMessage = message;
    toastType = type;
    toastVisible = true;
    setTimeout(() => {
      toastVisible = false;
    }, 3000);
  }

  async function handleClick() {
    const filePaths = await open({
      multiple: true,
      filters: [{ name: "VR8 Files", extensions: ["VR8"] }],
    });

    if (!filePaths) {
      return;
    }

    const newFiles = Array.isArray(filePaths) ? filePaths : [filePaths];
    const uniqueFiles = newFiles.filter(
      (file) => !selectedFiles.includes(file),
    );

    if (uniqueFiles.length === 0) {
      // showToast('No new files selected', 'error');
      return;
    }

    if (uniqueFiles.length < newFiles.length) {
      showToast("Some files were already selected", "error");
    }

    selectedFiles = [...selectedFiles, ...uniqueFiles];
    if (selectedFiles.length > 0) {
      view = "confirm";
    }
  }

  async function pickOutputDir() {
    try {
      const output_dir = await open({
        directory: true,
        multiple: false,
      });

      if (output_dir) {
        selectedOutputDir = output_dir;
        // showToast('Output directory selected', 'success');
      } else {
        showToast("Output directory selection cancelled", "error");
      }
    } catch (e) {
      const errMsg = `Error picking output directory: ${e}`;
      console.error(errMsg);
    }
  }

  async function handleConvert() {
    if (!selectedOutputDir) {
      showToast("Please select an output folder", "error");
      return;
    }
    converting = true;
    try {
      result = await invoke("convert_files", {
        paths: selectedFiles,
        outputDir: selectedOutputDir,
      });
      view = "success";
    } catch (e) {
      console.error(e);
    } finally {
      converting = false;
    }
  }

  onMount(() => {
    let unlisten: () => void;
    listen("conversion_progress", (event) => {
      progress = event.payload as number;
    }).then((unlistenFn) => {
      unlisten = unlistenFn;
    });

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  });
</script>

<main class="flex justify-center w-full">
  {#if view === "select"}
    <div
      role="button"
      tabindex="0"
      class="
      card bg-base-200 w-full max-w-xl h-64 flex items-center
      justify-center cursor-pointer border-4 border-dashed
      border-base-300 hover:border-secondary transition-colors
      "
      on:click={handleClick}
      on:keydown={(e) => e.key === "Enter" && handleClick()}
    >
      <div class="text-center">
        {#if converting}
          <span class="loading loading-spinner loading-lg"></span>
          <p class="mt-2">Converting...</p>
        {:else}
          <p class="text-lg">Click here to choose your file</p>
        {/if}
      </div>
    </div>
  {:else if view === "confirm"}
    <div class="card bg-base-200 shadow-xl w-full max-w-lg mx-auto">
      {#if converting}
        <div class="card-body items-center text-center">
          <h3 class="card-title mb-4">Converting files</h3>
          <progress
            class="progress progress-secondary w-56"
            value={progress}
            max="100"
          ></progress>
          <p class="mt-2 text-sm">This might take a few seconds</p>
        </div>
      {:else}
        <FileList
          bind:files={selectedFiles}
          output_dir={selectedOutputDir}
          onPickDir={pickOutputDir}
          onCancel={() => (view = "select")}
          onConvert={handleConvert}
          onAddMore={handleClick}
        />
      {/if}
    </div>
  {:else if view === "success" && result}
    <SuccessView
      duration={result.duration_ms}
      outputDir={result.output_dir}
      onConvertMore={() => {
        view = "select";
        result = null;
        selectedFiles = [];
        selectedOutputDir = null;
      }}
    />
  {/if}
</main>

<Toast message={toastMessage} type={toastType} visible={toastVisible} />

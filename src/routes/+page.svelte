<script lang="ts">
 import { invoke } from "@tauri-apps/api/core";
 import { open } from "@tauri-apps/plugin-dialog";
 import Toast from "$lib/components/Toast.svelte";
 import SuccessView from "$lib/components/SuccessView.svelte";
 import FileList from "$lib/components/fileList.svelte";

 let view: 'select' | 'confirm' | 'success' = 'select';
 let isDragging = false;
 let converting = false;
 let selectedFiles: (string[] | string)[] = [];
 let result: { files_converted: number; duration_ms: number; output_dir: string } | null = null;

 let toastMessage = '';
 let toastType: 'error' | 'success' = 'error';
 let toastVisible = false;

 function showToast(message: string, type: 'error' | 'success' = 'error') {
   toastMessage = message;
   toastType = type;
   toastVisible = true;
 }

 async function handleDrop(event: DragEvent) {
   isDragging = false;
   const items = event.dataTransfer?.items;
   if (!items) return;

   converting = true;
   try {
     const item = items[0];
     const entry = item.webkitGetAsEntry();
     
     if (entry?.isDirectory) {
       const dirPath = await open({ directory: true });
       if (dirPath) {
         selectedFiles = [dirPath];
         view = 'confirm';
       }
     } else {
       const filePath = await open({ 
         filters: [{ name: 'VR8 Files', extensions: ['VR8'] }] 
       });
       if (filePath) {
         selectedFiles = [filePath];
         view = 'confirm';
       }
     }
   } catch (e) {
     console.error(e);
     showToast(`Selection failed: ${e}`, 'error');
   } finally {
     converting = false;
   }
 }

 async function handleClick() {
   try {
     const filePath = await open({
       multiple: false,
       filters: [{ name: 'VR8 Files', extensions: ['VR8'] }]
     });
     
     if (filePath) {
       selectedFiles = [filePath];
       view = 'confirm';
     }
   } catch(e) {
     showToast('Selection failed: ' + e, 'error');
   }
 }

 async function handleConvert() {
   converting = true;
   try {
     result = await invoke('convert_files', { 
       path: selectedFiles[0],
       isDir: selectedFiles[0].includes('wav-files')
     });
     view = 'success';
   } catch(e) {
     showToast('Conversion failed: ' + e, 'error');
   } finally {
     converting = false;
   }
 }
</script>


<main class="container mx-auto px-4 flex h-[calc(100vh-12rem)] items-center justify-center">
    {#if view === 'select'}
        <div
                role="button"
                tabindex="0"
                class="card bg-base-200 w-full max-w-xl h-64 flex items-center justify-center cursor-pointer border-4 border-dashed border-base-300 hover:border-primary transition-colors"
                class:border-secondary={isDragging}
                class:border-accent={converting}
                on:dragover|preventDefault={() => isDragging = true}
                on:dragleave|preventDefault={() => isDragging = false}
                on:drop|preventDefault={handleDrop}
                on:click={handleClick}
                on:keydown={(e) => e.key === 'Enter' && handleClick()}
        >
            <div class="text-center">
                {#if converting}
                    <span class="loading loading-spinner loading-lg"></span>
                    <p class="mt-2">Converting...</p>
                {:else}
                    <p class="text-lg">Drop VR8 files or folder here</p>
                    <p class="text-sm text-base-content/60">or click to select</p>
                {/if}
            </div>
        </div>
    {:else if view === 'confirm'}
        <FileList
                files={selectedFiles}
                onCancel={() => view = 'select'}
                onConvert={handleConvert}
        />
    {:else if view === 'success' && result}
        <SuccessView
                duration={result.duration_ms}
                outputDir={result.output_dir}
                onConvertMore={() => {
        view = 'select';
        result = null;
      }}
        />
    {/if}
</main>

<Toast message={toastMessage} type={toastType} visible={toastVisible} />


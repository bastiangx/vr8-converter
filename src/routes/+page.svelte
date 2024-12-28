<script lang="ts">
 import { invoke } from "@tauri-apps/api/core";
 import { open } from "@tauri-apps/plugin-dialog";
 import Toast from "$lib/components/Toast.svelte";
 import SuccessView from "$lib/components/SuccessView.svelte";
 import FileList from "$lib/components/fileList.svelte";


 let view: 'select' | 'confirm' | 'success' = 'select';
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

     setTimeout(() => {
       toastVisible = false;
     }, 3000);
 }

 async function handleClick() {
   try {
     const filePaths = await open({
       multiple: true, // 👈 Enable multiple
       filters: [{ name: 'VR8 Files', extensions: ['VR8'] }]
     });
     
     if (filePaths && filePaths.length > 0) {
       selectedFiles = filePaths;
       showToast(`${filePaths.length} files selected`, 'success');
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
       paths: selectedFiles,
       isDir: false
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
                class:border-accent={converting}
                on:click={handleClick}
                on:keydown={(e) => e.key === 'Enter' && handleClick()}
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

 <!-- utility, don't delete -->
<div class="hidden">
<Toast message={toastMessage} type={toastType} visible={toastVisible} />
    </div>


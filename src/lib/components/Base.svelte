<script lang="ts">
    import Footer from "$lib/components/footer.svelte";
    import ThemeToggle from "./ThemeToggle.svelte";
    import { open } from '@tauri-apps/plugin-shell'

  function openLink(url: string) {
    open(url);
  }
</script>

<div class="flex flex-col">
    <header class="navbar px-4">
        <div class="navbar-start">
            <h1 class="animate-enter text-xl font-bold tracking-tight">VR8 Converter</h1>
        </div>
        <div class="navbar-end">
            <button class="btn tracking-tight btn-ghost btn-sm mr-12"
              aria-label="button"
              on:click={() => openLink('https://github.com/bastiangx')}
              > GitHub
            </button>
        </div>
        <ThemeToggle />
    </header>

    <main class="flex-1 container mx-auto px-2">
        <slot />
    </main>

    <Footer />
</div>

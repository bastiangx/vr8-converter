<script lang="ts">
    import ThemeToggle from "./ThemeToggle.svelte";
    import { open } from "@tauri-apps/plugin-shell";

    function openLink(url: string) {
        open(url);
    }
</script>

<div class="w-full">
    <div class="container mx-auto px-4 max-w-5x1">
        <header class="navbar">
            <div class="navbar-start">
                <h1 class="animate-enter text-xl font-bold tracking-tight">
                    VR8 Converter
                </h1>
            </div>
            <div class="navbar-end">
                <button
                    class="btn tracking-tight btn-ghost btn-sm mr-2"
                    aria-label="button"
                    on:click={() => openLink("https://github.com/bastiangx")}
                >
                    GitHub
                </button>
            </div>
            <ThemeToggle />
        </header>
    </div>
</div>

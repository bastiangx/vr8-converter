<script lang="ts">
  import { open } from "@tauri-apps/plugin-shell";
  import kofi from "$lib/assets/kofi.png";

  function openLink(url: string) {
    open(url);
  }
</script>

<footer class="footer py-2 px-8 w-full">
  <nav class="container mx-auto flex justify-end">
    <button
      aria-label="Kofi support button"
      class="btn bg-base-100 hover:bg-base-300 border-none gap-2 transition-transform hover:scale-105 duration-200"
      on:click={() => openLink("https://ko-fi.com/bastiangx")}
    >
      <img src={kofi} alt="Ko-fi" class="w-24 h-6 lg:w-26 lg:h-8" />
    </button>
  </nav>
</footer>

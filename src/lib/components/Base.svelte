<script lang="ts">
  import Footer from "$lib/components/footer.svelte";
  import Navbar from "./Navbar.svelte";
</script>

<div class="min-h-screen flex flex-col">
  <Navbar />
  <main class="flex-1 container mx-auto px-4 max-w-5x1">
    <div class=" flex justify-center w-full">
      <div class="w-full">
        <slot />
      </div>
    </div>
  </main>
  <Footer />
</div>

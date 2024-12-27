<script lang="ts">
  let theme = 'dark';

  function toggleTheme() {
    theme = theme === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', theme);
  }
</script>

<button
        class="btn btn-circle btn-ghost fixed right-2 text-xl"
        on:click={toggleTheme}
        aria-label="Toggle theme"
>
  {#if theme === 'dark'}
    🌙
  {:else}
    ☀️
  {/if}
</button>
<script lang="ts">
  import ThemeToggle from "./ThemeToggle.svelte";
  import { open } from "@tauri-apps/plugin-shell";

  function openLink(url: string) {
    open(url);
  }
</script>

<div class="w-full">
  <div class="container mx-auto px-4 max-w-5xl">
    <header class="navbar">
      <div class="navbar-start">
        <h1 class="animate-enter text-xl font-bold tracking-tight">
          VR8 Converter
        </h1>
      </div>

      <div class="navbar-end hidden lg:flex gap-2">
        <button
          class="btn tracking-tight btn-ghost btn-sm"
          aria-label="button"
          on:click={() => openLink("https://github.com/bastiangx")}
        >
          GitHub
        </button>
        <ThemeToggle />
      </div>

      <div class="navbar-end lg:hidden">
        <div class="dropdown dropdown-end">
          <button
            tabindex="0"
            class="btn btn-ghost btn-circle"
            aria-label="Open Menu"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 6h16M4 12h16M4 18h7"
              />
            </svg>
          </button>
          <ul
            class="dropdown-content menu p-4 shadow
            bg-base-200 rounded-box z-50 min-w-fit whitespace-nowrap
            border border-1 border-secondary
            "
          >
            <li class="w-full">
              <button
                class="btn btn-ghost btn-sm"
                on:click={() => openLink("https://github.com/bastiangx")}
              >
                GitHub
              </button>
            </li>
            <li class="w-full flex justify-center">
              <ThemeToggle />
            </li>
          </ul>
        </div>
      </div>
    </header>
  </div>
</div>

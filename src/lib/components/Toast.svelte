// Toast.svelte
<script lang="ts">
  export let message = '';
  export let type: 'error' | 'success' = 'error';
  export let visible = false;
</script>

<div 
  class="fixed bottom-8 right-8 max-w-sm p-4 rounded-lg shadow-lg transition-all duration-300 backdrop-blur-sm {visible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-20'}"
  class:bg-rose-900={type === 'error'} 
  class:bg-pine-900={type === 'success'}
  style="background-color: rgba(var(--{type === 'error' ? 'love' : 'pine'}), 0.1);"
>
  <div class="flex items-center gap-2 text-sm">
    <span>{type === 'error' ? '❌' : '✨'}</span>
    <span>{message}</span>
        <span> herrlo</span>
  </div>
</div>

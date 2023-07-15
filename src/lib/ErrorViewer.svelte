<script>
    import { listen } from '@tauri-apps/api/event'
    import { onMount } from 'svelte';

    onMount(async () => {
        await listen('error', (event) => {
            alert(event.payload.message)
        })
    })
  </script>
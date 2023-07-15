<script>
  import SatViewer from './lib/SatViewer.svelte'
  import { emit } from '@tauri-apps/api/event'
  import { onMount } from 'svelte';
  import Serial from './lib/Serial.svelte';
  import ErrorViewer from './lib/ErrorViewer.svelte';

  onMount(async () => {
    await emit('page-load', '');
  })
</script>

<main class="container">
  <h1>Interestelar</h1>
  <ErrorViewer />
  <SatViewer />
  <Serial />
</main>

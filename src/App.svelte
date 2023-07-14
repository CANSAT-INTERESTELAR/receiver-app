<script>
  import SatViewer from './lib/SatViewer.svelte'
  import { emit } from '@tauri-apps/api/event'
  import { onMount } from 'svelte';
    import Serial from './lib/Serial.svelte';

  onMount(async () => {
    emit('page-load', '');
  })
</script>

<main class="container">
  <h1>Interestelar</h1>
  <SatViewer />
  <Serial />
</main>

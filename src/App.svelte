<script>
  import SatViewer from './lib/SatViewer.svelte';
  import Serial from './lib/Serial.svelte';
  import ErrorViewer from './lib/ErrorViewer.svelte';
  import DataViewer from './lib/DataViewer.svelte';
  import Map from './lib/Map.svelte';
</script>

<main class="container">
  <h1>Interestelar</h1>
  <ErrorViewer />
  <SatViewer />
  <DataViewer />
  <Serial />
  <Map />
</main>

<script>
  import SatViewer from './lib/SatViewer.svelte';
  import Serial from './lib/Serial.svelte';
  import ErrorViewer from './lib/ErrorViewer.svelte';
  import Map from './lib/Map.svelte';
  import Pressure from './lib/Pressure.svelte';
  import Height from './lib/Height.svelte';
  import Temperature from './lib/Temperature.svelte';
  import Humidity from './lib/Humidity.svelte';
  import Velocity from './lib/Velocity.svelte';
  import GasPresence from './lib/GasPresence.svelte';
  import Alive from './lib/Alive.svelte';
</script>

<div class="columns is-multiline">
  <div class="column is-three-quarters padded">
    <div class="columns">
      <div class="column">
        <div class="container has-background-white-ter">
          <h1 class="title label">Temperatura:</h1>
          <Temperature />
        </div>
      </div>
      <div class="column">
        <div class="container mid-cont has-background-white-ter">
          <h1 class="title label">Velocidad:</h1>
          <Velocity />
        </div>
        <div class="container mid-cont has-background-white-ter">
          <h1 class="title label">Satélite vivo:</h1>
          <Alive />
        </div>
      </div>
      <div class="column">
        <div class="container has-background-white-ter">
          <h1 class="title label">Presión:</h1>
          <Pressure />
        </div>
      </div>
    </div>

    <div class="column is-three-quarters padded">
      <div class="columns">
        <div class="column">
          <div class="container has-background-white-ter">
            <h1 class="title label">Posición:</h1>
            <Map />
          </div>
        </div>
        <div class="column">
          <div class="container has-background-white-ter">
            <h1 class="title label">Humedad:</h1>
            <Humidity />
          </div>
        </div>
        <div class="column">
          <div class="container has-background-white-ter">
            <h1 class="title label">Presencia de gases:</h1>
            <GasPresence />
          </div>
        </div>
      </div>
    </div>
  </div>

  <div id="right-panel" class="column is-one-quarter padded">
    <div class="columns is-multiline">
      <div class="column">
        <Serial />
      </div>
      <div class="column has-background-white-ter container" >
        <h1 class="title label">Altitud:</h1>
        <Height />
      </div>
      <div class="column container">
        <h1 class="title label">Orientación:</h1>
        <SatViewer />
      </div>
    </div>
  </div>
</div>

<ErrorViewer />

<style>
  .title {
    font-size: larger;
  }

  #right-panel {
    border-left: 2px solid black;
    height: 102vh;
  }

  .container {
    width: min-content;
    padding: 5px;
  }

  .mid-cont {
    width: auto;
    text-align: center;
  }

  .padded {
    padding: 20px;
  }
</style>
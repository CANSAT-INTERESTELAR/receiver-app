<script>
    import { onMount } from 'svelte';
    import { listen, emit } from '@tauri-apps/api/event'
    import Select from 'svelte-select';
    import { Button } from '@svelteuidev/core';
    import { latestHeightByPressure, latestRX } from './stores.js';

    let value;
    let availablePorts = ["None"];

    onMount(async () => {
        await listen('available-ports', (event) => {
            parseAvailablePorts(event.payload.message);
        })

        await listen('rx', (event) => {
            latestRX.set(JSON.parse(event.payload.sat_data));
            latestHeightByPressure.set(JSON.parse(event.payload.height_p));
        })

        function parseAvailablePorts(ports) {
            availablePorts = ports.split(",");
        }
    });

    function connect() {
        let port = value?.label;
        if (!port) {
            return;
        }

        emit('connect', port);
    }
</script>

<div class="row">
    <Select items={availablePorts} bind:value />
    <Button color="dark" radius="md" size="md" on:click={connect}>
        Conectar
    </Button>
</div>
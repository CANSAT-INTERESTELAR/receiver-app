<script>
    import { onMount } from 'svelte';
    import { listen } from '@tauri-apps/api/event'
    import { invoke } from '@tauri-apps/api';
    import Select from 'svelte-select';
    import { Button } from '@svelteuidev/core';
    import { latestHeightByPressure, latestRX } from './stores.js';

    let value;
    let availablePorts = ["None"];

    onMount(async () => {
        await listen('available-ports', (event) => {
            console.log(event.payload.message);
            parseAvailablePorts(event.payload.message);
        });

        await listen('rx', (event) => {
            latestRX.set(JSON.parse(event.payload.sat_data));
            latestHeightByPressure.set(JSON.parse(event.payload.height_p));
        });

        await invoke('pageload');

        function parseAvailablePorts(ports) {
            availablePorts = ports.split(",");
        }
    });

    function connect() {
        let port = value?.label;
        if (!port) {
            return;
        }

        invoke('connect', { port: port });
    }
</script>

<div class="row">
    <Select items={availablePorts} bind:value />
    <Button color="dark" radius="md" size="md" on:click={connect}>
        Conectar
    </Button>
</div>
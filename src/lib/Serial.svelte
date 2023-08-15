<script>
    import { onMount } from 'svelte';
    import { listen } from '@tauri-apps/api/event'
    import { invoke } from '@tauri-apps/api';
    import Select from 'svelte-select';
    import { Button } from '@svelteuidev/core';
    import { latestSatRX, totalRX } from './stores.js';

    let value;
    let availablePorts = ["None"];

    onMount(async () => {
        await listen('available-ports', (event) => {
            console.log(event.payload.message);
            parseAvailablePorts(event.payload.message);
        });

        await listen('rx', (event) => {
            if (JSON.parse(event.payload.sat_data).pressure == 0) {
                return;
            }
            
            latestSatRX.set(JSON.parse(event.payload.sat_data));

            let total = $totalRX;
            total.data[event.payload.timestamp] = event.payload;
            totalRX.set(total);
        });

        await invoke('updateports');

        function parseAvailablePorts(ports) {
            availablePorts = ports.split(",");
        }
    });

    async function updatePorts() {
        await invoke('updateports');
    }

    function connect() {
        let port = value?.label;
        if (!port) {
            return;
        }

        invoke('connect', { port: port });
    }
</script>

<div class="row">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="full-width" on:click={updatePorts}>
        <Select items={availablePorts} bind:value placeholder="Puerto serial"/>
    </div>
    <Button color="dark" radius="sm" style="height: 40px" on:click={connect}>
        Conectar
    </Button>
</div>

<style>
    .full-width {
        width: 100%;
    }

    .row {
        display: flex;
        align-items: center;
    }

    :global(input) {
        cursor: pointer !important;
    }


</style>
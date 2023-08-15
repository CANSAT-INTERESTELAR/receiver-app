<script>
    import { onMount } from 'svelte';
    import { latestSatRX } from './stores';

    var lastRXTime = Date.now();
    var alive = false;

    onMount(() => {
        latestSatRX.subscribe(rx => {
            lastRXTime = Date.now();
        });

        setInterval(() => {
            var deltaT = Math.abs(lastRXTime - Date.now());
            if (deltaT > 300) {
                alive = false;
            } else {
                alive = true;
            }
        }, 10);
    });
</script>

<div class="center">
    {#if alive}
        <div class="circle c-green"></div>
    {:else}
        <div class="circle c-red"></div>
    {/if}
</div>

<style>
    .center {
        display: flex;
        justify-content: center;
    }

    .circle {
        border-radius: 50%;
        width: 50px;
        height: 50px;
    }

    .c-green {
        background-color: green;
    }
    
    .c-red {
        background-color: red;
    }
</style>
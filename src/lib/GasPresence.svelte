<script>
    import { onMount } from 'svelte';
    import { latestSatRX } from './stores';

    var hadMQ135 = false;
    var hadMQ4 = false;

    var currentMQ135 = false;
    var currentMQ4 = false;

    onMount(() => {
        latestSatRX.subscribe(rx => {
            if (rx.mq4) {
                currentMQ4 = true;
                hadMQ4 = true;
            } else {
                currentMQ4 = false;
            }

            if (rx.mq135) {
                currentMQ135 = true;
                hadMQ135 = true;
            } else {
                currentMQ135 = false;
            }
        });
    });
</script>

<div class="center">
    <div class="aa">
        <p>MQ4:</p>
        {#if currentMQ4}
            <div class="circle c-green"></div>
        {:else}
            <div class="circle c-red"></div>
        {/if}
        {#if hadMQ4}
            <div class="circle big c-green"></div>
        {:else}
            <div class="circle big c-red"></div>
        {/if}
    </div>
    <div class="aa">
        <p>MQ135:</p>
        {#if currentMQ135}
            <div class="circle c-green"></div>
        {:else}
            <div class="circle c-red"></div>
        {/if}
        {#if hadMQ135}
            <div class="circle big c-green"></div>
        {:else}
            <div class="circle big c-red"></div>
        {/if}
    </div>
</div>

<style>
    .center {
        display: flex;
        justify-content: center;
        flex-wrap: wrap;
    }

    .aa {
        display: flex;
        align-items: center;
        padding: 0.75rem;
    }

    .aa > p {
        padding-right: 1rem;
    }

    .circle {
        border-radius: 50%;
        width: 30px;
        height: 30px;
        margin: 0.25rem;
    }

    .big {
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
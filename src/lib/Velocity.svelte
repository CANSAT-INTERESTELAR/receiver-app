<script>
    import { onMount } from 'svelte';
    import { latestSatRX } from './stores';

    var velocity = 0;

    onMount(() => {
        latestSatRX.subscribe(rx => {
            velocity = rx.speed;
            if (rx.speed == undefined) {
                velocity = 0;
            }
        });
    });
</script>

<p>
    {velocity}m/s
</p>

<style>
    p {
        font-size: larger;
    }
</style>
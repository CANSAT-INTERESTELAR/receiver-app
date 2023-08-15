<script>
    import { LineChart, FixedScaleAxis } from 'chartist';
    import { onMount } from 'svelte';
    import 'chartist/dist/index.css';
    import { totalRX } from './stores';

    onMount(() => {
        totalRX.subscribe(rx => {
            let graphData = [];

            for (const time in rx.data) {
                let satData = JSON.parse(rx.data[time].sat_data);
                graphData.push({ x: new Date(time), y: satData.humidity });
            }

            new LineChart(
            '#chart-hum',
            {
                series: [graphData]
            },
            {
                showArea: true,
                showPoint: false,
                lineSmooth: true,
                axisX: {
                    type: FixedScaleAxis,
                    divisor: 5,
                    labelInterpolationFnc: value =>
                        new Date(value).toLocaleString(undefined, {
                            minute: '2-digit',
                            second: 'numeric'
                        })
                }
            }
        );
        });
    });
</script>

<div id="chart-hum" style="width: 300px; height: 156px"></div>
<script>
    import { LineChart, FixedScaleAxis } from 'chartist';
    import { onMount } from 'svelte';
    import 'chartist/dist/index.css';
    import { totalRX } from './stores';

    onMount(() => {
        totalRX.subscribe(rx => {
            let graphData1 = [];

            for (const time in rx.data) {
                let satData = JSON.parse(rx.data[time].sat_data);
                graphData1.push({ x: new Date(time), y: satData.pressure });
            }

            new LineChart(
            '#chart-pressure',
            {
                series: [graphData1],
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

<div id="chart-pressure" style="width: 350px; height: 156px"></div>
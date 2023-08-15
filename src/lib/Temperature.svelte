<script>
    import { LineChart, FixedScaleAxis } from 'chartist';
    import { onMount } from 'svelte';
    import 'chartist/dist/index.css';
    import { totalRX } from './stores';

    onMount(() => {
        totalRX.subscribe(rx => {
            let graphData2 = [];
            let graphData3 = [];

            for (const time in rx.data) {
                let satData = JSON.parse(rx.data[time].sat_data);
                graphData2.push({ x: new Date(time), y: satData.dht_temperature });
                graphData3.push({ x: new Date(time), y: satData.bmp_temperature });
            }

            new LineChart(
            '#chart-temp',
            {
                series: [graphData2, graphData3]
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

<div id="chart-temp" style="width: 350px; height: 156px"></div>
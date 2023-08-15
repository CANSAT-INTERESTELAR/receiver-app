<script>
    import { LineChart, FixedScaleAxis } from 'chartist';
    import { onMount } from 'svelte';
    import 'chartist/dist/index.css';
    import { totalRX } from './stores';

    onMount(() => {
        totalRX.subscribe(rx => {
            let graphData1 = [];
            let graphData2 = [];

            for (const time in rx.data) {
                let satData = JSON.parse(rx.data[time].sat_data);
                graphData1.push({ x: new Date(time), y: satData.altitude });
                graphData2.push({ x: new Date(time), y: rx.data[time].height_p });
            }

            new LineChart(
            '#chart-height',
            {
                series: [graphData1, graphData2],
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

<div id="chart-height" style="width: 280px; height: 120px"></div>
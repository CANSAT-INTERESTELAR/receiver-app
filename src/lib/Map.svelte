<script>
    import leaflet from 'leaflet';
    import { onMount } from 'svelte';
    import { latestSatRX } from './stores';

    var positionList = [];

    onMount(() => {
        var map = leaflet.map("map").setView([-38.95161, -68.0591], 12);
        leaflet.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '© OpenStreetMap'
        }).addTo(map);

        function updatePosition(latitude, longitude) {
            positionList.push([latitude, longitude]);
            var polyline = leaflet.polyline(positionList, {color: 'red'}).addTo(map);
            map.setView([latitude, longitude], 14);
        }

        latestSatRX.subscribe(rx => {
            if (rx.latitude == undefined && rx.longitude == undefined) {
                return;
            }

            if (rx.latitude == 0 && rx.longitude == 0) {
                return;
            }

            updatePosition(rx.latitude, rx.longitude);
        });
    });
</script>

<link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY=" crossorigin=""/>

<div id="map"></div>

<style>
    #map {
        width: 400px;
        height: 300px;
    }
</style>
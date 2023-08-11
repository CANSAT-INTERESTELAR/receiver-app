<script>
    import leaflet from 'leaflet';
    import { onMount } from 'svelte';
    import { currentPosition } from './stores';

    var positionList = [];

    onMount(() => {
        var map = leaflet.map("map").setView([0, 0], 16);
        leaflet.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '© OpenStreetMap'
        }).addTo(map);

        function updatePosition(latitude, longitude) {
            positionList.push([latitude, longitude]);
            var polyline = leaflet.polyline(positionList, {color: 'red'}).addTo(map);
            map.fitBounds(polyline.getBounds());
            map.setView([latitude, longitude], 16);
        }

        currentPosition.subscribe(pos => {
            if (pos.lat == 0 && pos.long == 0) {
                return;
            }

            updatePosition(pos.lat, pos.long);
            console.log(positionList);
        });
    });
</script>

<link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY=" crossorigin=""/>

<div id="map"></div>

<style>
    #map {
        width: 600px;
        height: 400px;
    }
</style>
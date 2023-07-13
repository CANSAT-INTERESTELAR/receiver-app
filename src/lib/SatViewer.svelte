<script>
    import { onMount } from 'svelte';
    import { listen } from '@tauri-apps/api/event';
    import * as THREE from 'three';
    import { latestRX } from './stores';

    let lastRXW;
    let lastRXX;
    let lastRXY;
    let lastRXZ;

    onMount(async () => {
        const container = document.getElementById('threejs-container');
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0xf6f6f6);
        const renderer = new THREE.WebGLRenderer();
        renderer.setSize(400, 300);
        container.appendChild(renderer.domElement);
        var satelliteGroup = new THREE.Group();
        const camera = new THREE.PerspectiveCamera(35, 4/3, 0.1, 1000);

        // Set the camera position for an isometric view
        camera.position.set(2, 2, 2);
        camera.lookAt(scene.position);

        // Adjust the camera's projection to create an isometric view
        const viewSize = 4;
        const aspectRatio = window.innerWidth / window.innerHeight;
        camera.top = viewSize;
        camera.bottom = -viewSize;
        camera.left = -viewSize * aspectRatio;
        camera.right = viewSize * aspectRatio;

        const boxGeometry = new THREE.BoxGeometry(0.5, 0.5, 1);

        // Create an array of materials for each face
        const materials = [
            new THREE.MeshBasicMaterial({ color: 0xff0000 }),   // Front face color (red)
            new THREE.MeshBasicMaterial({ color: 0x00ff00 }),   // Back face color (green)
            new THREE.MeshBasicMaterial({ color: 0x0000ff }),   // Right face color (blue)
            new THREE.MeshBasicMaterial({ color: 0xffff00 }),   // Left face color (yellow)
            new THREE.MeshBasicMaterial({ color: 0xff00ff }),   // Top face color (magenta)
            new THREE.MeshBasicMaterial({ color: 0x00ffff })    // Bottom face color (cyan)
        ];

        // Create a material index array to assign materials to faces
        const materialIndices = [0, 1, 2, 3, 4, 5];
        boxGeometry.groups.forEach(group => {
            group.materialIndex = materialIndices[group.materialIndex];
        });

        // Create a mesh using the box geometry and materials
        const boxMesh = new THREE.Mesh(boxGeometry, materials);

        const container3D = new THREE.Object3D();
        container3D.add(boxMesh);
        scene.add(container3D);

        function updateRotation(w, x, y, z) {
            container3D.quaternion.fromArray([-w, y, z, x]);
        }

        function animate() {
            requestAnimationFrame( animate );
            renderer.render( scene, camera );
        }

        latestRX.subscribe(rx => {
            lastRXW = rx.w;
            lastRXX = rx.x;
            lastRXY = rx.y;
            lastRXZ = rx.z;
            updateRotation(lastRXW, lastRXX, lastRXY, lastRXZ);
        });

        updateRotation(1, 0, 0, 0);
        animate();
    });
</script>

<div id="threejs-container"></div>
DEBUG CUATERNIÓN: W: {lastRXW}, X: {lastRXX}, Y: {lastRXY}, Z: {lastRXZ}
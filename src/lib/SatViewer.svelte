<script>
    import { onMount } from 'svelte';
    import * as THREE from 'three';
    import { latestSatRX } from './stores';

    onMount(async () => {
        const container = document.getElementById('threejs-container');
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0xf6f6f6);
        const renderer = new THREE.WebGLRenderer();
        renderer.setSize(400, 300);
        container.appendChild(renderer.domElement);
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
            if (w == 2 || x == 2 || y == 2 || z == 2) {
                return;
            }
            container3D.quaternion.fromArray([-w, y, z, x]);
        }

        function animate() {
            requestAnimationFrame( animate );
            renderer.render( scene, camera );
        }

        latestSatRX.subscribe(rx => {
            updateRotation(rx.w, rx.x, rx.y, rx.z);
        });

        updateRotation(1, 0, 0, 0);
        animate();
    });
</script>

<div id="threejs-container"></div>
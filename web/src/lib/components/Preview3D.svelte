<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { treeStore } from '$lib/stores/tree';
  import { editorStore } from '$lib/stores/editor';

  let container: HTMLDivElement;
  let renderer: any;
  let scene: any;
  let camera: any;
  let controls: any;
  let treeMesh: any;
  let animationId: number;

  onMount(async () => {
    const THREE = await import('three');
    const { OrbitControls } = await import('three/examples/jsm/controls/OrbitControls.js');

    // Scene setup
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x0f0f1a);

    // Camera
    camera = new THREE.PerspectiveCamera(
      60,
      container.clientWidth / container.clientHeight,
      0.1,
      1000
    );
    camera.position.set(5, 5, 10);

    // Renderer
    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(container.clientWidth, container.clientHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.shadowMap.enabled = true;
    container.appendChild(renderer.domElement);

    // Controls
    controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.05;

    // Lighting
    const ambientLight = new THREE.AmbientLight(0x404040, 0.5);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 1);
    directionalLight.position.set(10, 20, 10);
    directionalLight.castShadow = true;
    scene.add(directionalLight);

    // Ground plane
    const groundGeometry = new THREE.PlaneGeometry(50, 50);
    const groundMaterial = new THREE.MeshStandardMaterial({
      color: 0x1a1a2e,
      roughness: 0.8
    });
    const ground = new THREE.Mesh(groundGeometry, groundMaterial);
    ground.rotation.x = -Math.PI / 2;
    ground.receiveShadow = true;
    scene.add(ground);

    // Grid helper
    const gridHelper = new THREE.GridHelper(50, 50, 0x2a2a4a, 0x1a1a2e);
    scene.add(gridHelper);

    // Animation loop
    function animate() {
      animationId = requestAnimationFrame(animate);
      controls.update();
      renderer.render(scene, camera);
    }
    animate();

    // Handle resize
    const resizeObserver = new ResizeObserver(() => {
      camera.aspect = container.clientWidth / container.clientHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(container.clientWidth, container.clientHeight);
    });
    resizeObserver.observe(container);

    return () => {
      resizeObserver.disconnect();
    };
  });

  onDestroy(() => {
    if (animationId) {
      cancelAnimationFrame(animationId);
    }
    if (renderer) {
      renderer.dispose();
    }
  });

  // React to mesh data changes
  $: if ($treeStore.meshData && scene) {
    updateTreeMesh($treeStore.meshData);
  }

  async function updateTreeMesh(meshData: any) {
    const THREE = await import('three');

    // Remove old mesh
    if (treeMesh) {
      scene.remove(treeMesh);
      treeMesh.geometry.dispose();
      treeMesh.material.dispose();
    }

    // Create new mesh from WASM data
    // TODO: Parse mesh data from WASM and create BufferGeometry
    // For now, create a placeholder cylinder as tree trunk
    const geometry = new THREE.CylinderGeometry(0.3, 0.5, 5, 8);
    const material = new THREE.MeshStandardMaterial({
      color: 0x8b4513,
      roughness: 0.9,
      wireframe: $editorStore.showWireframe
    });

    treeMesh = new THREE.Mesh(geometry, material);
    treeMesh.position.y = 2.5;
    treeMesh.castShadow = true;
    scene.add(treeMesh);
  }

  // React to wireframe toggle
  $: if (treeMesh && treeMesh.material) {
    treeMesh.material.wireframe = $editorStore.showWireframe;
  }
</script>

<div class="preview-container" bind:this={container}>
  {#if $treeStore.loading}
    <div class="overlay">
      <div class="spinner"></div>
      <span>Generating...</span>
    </div>
  {/if}

  {#if $treeStore.error}
    <div class="error">
      <span>Error: {$treeStore.error}</span>
    </div>
  {/if}
</div>

<style>
  .preview-container {
    width: 100%;
    height: 100%;
    position: relative;
  }

  .overlay {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    color: var(--text-secondary);
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .error {
    position: absolute;
    bottom: 1rem;
    left: 50%;
    transform: translateX(-50%);
    background: rgba(220, 38, 38, 0.9);
    padding: 0.5rem 1rem;
    border-radius: 4px;
    color: white;
  }
</style>

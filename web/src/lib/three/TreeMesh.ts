/**
 * Three.js mesh builder for tree geometry
 *
 * Converts mesh data from the Grove WASM module into
 * Three.js BufferGeometry for rendering.
 */

import type { MeshData } from '$lib/grove/wasm';

/**
 * Create a Three.js BufferGeometry from WASM mesh data
 * @param THREE - Three.js module
 * @param meshData - Mesh data from Grove generator
 * @returns BufferGeometry ready for rendering
 */
export function createTreeGeometry(THREE: any, meshData: MeshData): any {
  const geometry = new THREE.BufferGeometry();

  // Set position attribute
  geometry.setAttribute(
    'position',
    new THREE.BufferAttribute(meshData.vertices, 3)
  );

  // Set normal attribute
  geometry.setAttribute(
    'normal',
    new THREE.BufferAttribute(meshData.normals, 3)
  );

  // Set UV attribute
  geometry.setAttribute(
    'uv',
    new THREE.BufferAttribute(meshData.uvs, 2)
  );

  // Set indices
  geometry.setIndex(new THREE.BufferAttribute(meshData.indices, 1));

  // Compute bounding box and sphere for frustum culling
  geometry.computeBoundingBox();
  geometry.computeBoundingSphere();

  return geometry;
}

/**
 * Create materials for tree rendering
 */
export function createTreeMaterials(THREE: any) {
  // Bark material
  const barkMaterial = new THREE.MeshStandardMaterial({
    color: 0x5c4033,
    roughness: 0.9,
    metalness: 0.0,
    side: THREE.DoubleSide
  });

  // Leaf material
  const leafMaterial = new THREE.MeshStandardMaterial({
    color: 0x228b22,
    roughness: 0.6,
    metalness: 0.0,
    side: THREE.DoubleSide,
    transparent: true,
    alphaTest: 0.5
  });

  // Wireframe material for debug view
  const wireframeMaterial = new THREE.MeshBasicMaterial({
    color: 0x4ade80,
    wireframe: true
  });

  return {
    bark: barkMaterial,
    leaf: leafMaterial,
    wireframe: wireframeMaterial
  };
}

/**
 * Create a tree mesh group with trunk, branches, and leaves
 */
export function createTreeMesh(
  THREE: any,
  meshData: MeshData,
  options: {
    wireframe?: boolean;
    showNormals?: boolean;
  } = {}
): any {
  const group = new THREE.Group();
  group.name = 'tree';

  const geometry = createTreeGeometry(THREE, meshData);
  const materials = createTreeMaterials(THREE);

  // Main mesh
  const material = options.wireframe ? materials.wireframe : materials.bark;
  const mesh = new THREE.Mesh(geometry, material);
  mesh.castShadow = true;
  mesh.receiveShadow = true;
  group.add(mesh);

  // Normal helper for debugging
  if (options.showNormals) {
    // Would need to import VertexNormalsHelper from three/examples
    // const helper = new VertexNormalsHelper(mesh, 0.1, 0xff0000);
    // group.add(helper);
  }

  return group;
}

/**
 * Update an existing tree mesh with new geometry
 */
export function updateTreeMesh(
  THREE: any,
  existingMesh: any,
  newMeshData: MeshData
): void {
  // Dispose old geometry
  if (existingMesh.geometry) {
    existingMesh.geometry.dispose();
  }

  // Create and assign new geometry
  existingMesh.geometry = createTreeGeometry(THREE, newMeshData);
}

/**
 * Dispose of tree mesh and its resources
 */
export function disposeTreeMesh(mesh: any): void {
  if (!mesh) return;

  // Dispose geometry
  if (mesh.geometry) {
    mesh.geometry.dispose();
  }

  // Dispose materials
  if (mesh.material) {
    if (Array.isArray(mesh.material)) {
      mesh.material.forEach((mat: any) => mat.dispose());
    } else {
      mesh.material.dispose();
    }
  }

  // Recursively dispose children
  if (mesh.children) {
    mesh.children.forEach((child: any) => disposeTreeMesh(child));
  }
}

/**
 * Convert engine LOD output into stack viewport descriptors.
 *
 * One MeshDescriptor per material range (bark/leaves); all descriptors of a
 * LOD share the same position/normal buffers and differ only by their index
 * slice, so no vertex data is duplicated.
 */

import type { MeshDescriptor } from '@jethac/tools-frontend-stack/viewports';
import type { LodMesh } from './engine';

export const MATERIAL_COLORS = {
  bark: '#6d4c2f',
  leaves: '#3f7a2e',
} as const;

export function lodToDescriptors(
  lod: LodMesh,
  revision: number,
  layers: { bark: boolean; leaves: boolean },
): MeshDescriptor[] {
  const positions = Float32Array.from(lod.vertices.positions);
  const normals = Float32Array.from(lod.vertices.normals);
  const indices = Uint32Array.from(lod.indices);

  const ranges =
    lod.submeshes.length > 0
      ? lod.submeshes
      : [{ index_start: 0, index_count: indices.length, material: 'bark' as const }];

  const descriptors: MeshDescriptor[] = [];
  for (const [i, range] of ranges.entries()) {
    if (!layers[range.material]) continue;
    descriptors.push({
      entityId: `${lod.name}-${range.material}-${i}`,
      revision,
      positions,
      normals,
      indices: indices.slice(range.index_start, range.index_start + range.index_count),
      color: MATERIAL_COLORS[range.material],
    });
  }
  return descriptors;
}

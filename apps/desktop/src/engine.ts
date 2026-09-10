/**
 * Engine boundary for the Grove workbench.
 *
 * The grove-wasm WebAssembly build is the same grove-core code that powers the
 * CLI and FFI crates, compiled for the webview. Species documents cross the
 * boundary as TOML text (authoritative) or as the serde JSON projection
 * produced by `toJson()` / consumed by `fromJson()`.
 */

import init, { GroveGenerator } from './wasm/grove_wasm.js';

export interface VertexData {
  positions: number[];
  normals: number[];
  uvs: number[];
  uv2s: number[];
  colors: number[];
}

export interface SubmeshRange {
  index_start: number;
  index_count: number;
  material: 'bark' | 'leaves';
}

export interface LodMesh {
  name: string;
  vertices: VertexData;
  indices: number[];
  submeshes: SubmeshRange[];
  vertex_count: number;
  triangle_count: number;
}

export interface TreeStats {
  stem_count: number;
  leaf_count: number;
  bounds_min: [number, number, number];
  bounds_max: [number, number, number];
}

export interface GltfParts {
  gltf: Uint8Array;
  bin: Uint8Array;
}

let moduleReady: Promise<unknown> | null = null;

type WasmInput = Parameters<typeof init>[0];

/**
 * Initialize the WASM module once. `input` overrides where the module bytes
 * come from (tests pass a Buffer; the browser resolves `grove_wasm_bg.wasm`
 * next to the bundle via import.meta.url).
 */
export function loadEngine(input?: WasmInput): Promise<unknown> {
  moduleReady ??= Promise.resolve(init(input ? { module_or_path: input } : undefined));
  return moduleReady;
}

/**
 * A live generator bound to one species document. Recreate it whenever the
 * species changes; seeds are cheap and per-call.
 */
export class Generator {
  private constructor(private readonly inner: GroveGenerator) {}

  static fromToml(toml: string): Generator {
    return new Generator(new GroveGenerator(toml));
  }

  static fromJson(json: unknown): Generator {
    return new Generator(GroveGenerator.fromJson(json));
  }

  get name(): string {
    return this.inner.name;
  }

  /** Species as a mutable plain object (full serde projection). */
  toJson(): unknown {
    return this.inner.toJson();
  }

  /** Species serialized back to TOML. */
  toToml(): string {
    return this.inner.toToml();
  }

  /** Generate all LOD levels for `seed`. */
  generate(seed: number): LodMesh[] {
    const out = this.inner.generate(BigInt(seed)) as { lods: LodMesh[] };
    return out.lods;
  }

  /** Stem/leaf/bounds stats without mesh output. */
  stats(seed: number): TreeStats {
    return this.inner.get_stats(BigInt(seed)) as TreeStats;
  }

  /** Single-file binary glTF for `seed`. */
  exportGlb(seed: number): Uint8Array {
    return this.inner.export_glb(BigInt(seed));
  }

  /** Separate .gltf JSON + .bin parts. `binName` becomes the buffer URI. */
  exportGltf(seed: number, binName: string): GltfParts {
    return this.inner.exportGltf(BigInt(seed), binName) as GltfParts;
  }

  free(): void {
    this.inner.free();
  }
}

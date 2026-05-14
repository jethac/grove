import { writable, derived } from 'svelte/store';

interface TreeState {
  species: string;
  seed: number;
  meshData: any | null;
  loading: boolean;
  error: string | null;
}

function createTreeStore() {
  const { subscribe, set, update } = writable<TreeState>({
    species: '',
    seed: 12345,
    meshData: null,
    loading: false,
    error: null
  });

  let generator: any = null;
  let wasmModule: any = null;

  return {
    subscribe,

    async init() {
      try {
        // Dynamic import of WASM
        wasmModule = await import('grove-wasm');
        await wasmModule.default();
        console.log('WASM loaded');
      } catch (e) {
        console.error('Failed to load WASM:', e);
        update(s => ({ ...s, error: 'Failed to load WASM module' }));
      }
    },

    async loadSpecies(toml: string) {
      if (!wasmModule) return;

      update(s => ({ ...s, loading: true, error: null }));

      try {
        generator = new wasmModule.GroveGenerator(toml);
        update(s => ({ ...s, species: generator.name, loading: false }));
        await this.regenerate();
      } catch (e: any) {
        update(s => ({ ...s, loading: false, error: e.toString() }));
      }
    },

    async regenerate() {
      if (!generator) return;

      update(s => ({ ...s, loading: true }));

      try {
        const state = await new Promise<TreeState>(resolve => {
          subscribe(s => resolve(s))();
        });

        const meshData = generator.generate(BigInt(state.seed));
        update(s => ({ ...s, meshData, loading: false }));
      } catch (e: any) {
        update(s => ({ ...s, loading: false, error: e.toString() }));
      }
    },

    setSeed(seed: number) {
      update(s => ({ ...s, seed }));
    },

    randomizeSeed() {
      update(s => ({ ...s, seed: Math.floor(Math.random() * 2147483647) }));
    }
  };
}

export const treeStore = createTreeStore();

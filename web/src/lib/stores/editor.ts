import { writable } from 'svelte/store';

interface EditorState {
  selectedNode: string | null;
  showWireframe: boolean;
  showNormals: boolean;
  autoRotate: boolean;
  autoRotateSpeed: number;
  currentLod: number;
  autoRegenerate: boolean;
}

function createEditorStore() {
  const { subscribe, set, update } = writable<EditorState>({
    selectedNode: null,
    showWireframe: false,
    showNormals: false,
    autoRotate: false,
    autoRotateSpeed: 1.0,
    currentLod: 0,
    autoRegenerate: true
  });

  return {
    subscribe,
    selectNode: (id: string | null) => update(s => ({ ...s, selectedNode: id })),
    toggleWireframe: () => update(s => ({ ...s, showWireframe: !s.showWireframe })),
    toggleNormals: () => update(s => ({ ...s, showNormals: !s.showNormals })),
    toggleAutoRotate: () => update(s => ({ ...s, autoRotate: !s.autoRotate })),
    setAutoRotateSpeed: (speed: number) => update(s => ({ ...s, autoRotateSpeed: speed })),
    setLod: (lod: number) => update(s => ({ ...s, currentLod: lod })),
    toggleAutoRegenerate: () => update(s => ({ ...s, autoRegenerate: !s.autoRegenerate })),
    reset: () => set({
      selectedNode: null,
      showWireframe: false,
      showNormals: false,
      autoRotate: false,
      autoRotateSpeed: 1.0,
      currentLod: 0,
      autoRegenerate: true
    })
  };
}

export const editorStore = createEditorStore();

import { writable } from 'svelte/store';

interface EditorState {
  selectedNode: string | null;
  showWireframe: boolean;
  showNormals: boolean;
  currentLod: number;
  autoRegenerate: boolean;
}

function createEditorStore() {
  const { subscribe, set, update } = writable<EditorState>({
    selectedNode: null,
    showWireframe: false,
    showNormals: false,
    currentLod: 0,
    autoRegenerate: true
  });

  return {
    subscribe,
    selectNode: (id: string | null) => update(s => ({ ...s, selectedNode: id })),
    toggleWireframe: () => update(s => ({ ...s, showWireframe: !s.showWireframe })),
    toggleNormals: () => update(s => ({ ...s, showNormals: !s.showNormals })),
    setLod: (lod: number) => update(s => ({ ...s, currentLod: lod })),
    toggleAutoRegenerate: () => update(s => ({ ...s, autoRegenerate: !s.autoRegenerate }))
  };
}

export const editorStore = createEditorStore();

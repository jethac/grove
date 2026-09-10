import { Dice5, Eye, EyeOff } from 'lucide-react';
import type { WorkbenchState } from '../model';

export function ObjectsPanel({
  state,
  onSelectLod,
  onSelectVariant,
  onNewVariant,
  onToggleLayer,
}: {
  state: WorkbenchState;
  onSelectLod: (index: number) => void;
  onSelectVariant: (seed: number) => void;
  onNewVariant: () => void;
  onToggleLayer: (layer: 'bark' | 'leaves') => void;
}) {
  return (
    <div className="grove-panel-content">
      <div className="grove-section-title">
        LOD LEVELS <span>{state.lods?.length ?? 0}</span>
      </div>
      {!state.lods?.length && <p className="grove-subtle">No meshes generated.</p>}
      <div className="grove-object-list" role="radiogroup" aria-label="Preview LOD level">
        {state.lods?.map((lod, index) => (
          <button
            key={lod.name}
            type="button"
            role="radio"
            aria-checked={state.selectedLod === index}
            className="grove-object-row"
            onClick={() => onSelectLod(index)}
          >
            <span>{lod.name}</span>
            <small>
              {lod.triangle_count.toLocaleString()} tris · {lod.vertex_count.toLocaleString()} verts
            </small>
          </button>
        ))}
      </div>

      <div className="grove-section-title">LAYERS</div>
      <div className="grove-layer-row">
        {(['bark', 'leaves'] as const).map((layer) => (
          <button
            key={layer}
            type="button"
            aria-pressed={state.layers[layer]}
            onClick={() => onToggleLayer(layer)}
            title={`Toggle ${layer}`}
          >
            {state.layers[layer] ? <Eye size={14} /> : <EyeOff size={14} />} {layer}
          </button>
        ))}
      </div>

      <div className="grove-section-title">
        VARIANTS <span>{state.variants.length}</span>
      </div>
      <div className="grove-variant-grid" role="radiogroup" aria-label="Variant seed">
        {state.variants.map((seed) => (
          <button
            key={seed}
            type="button"
            role="radio"
            aria-checked={state.seed === seed}
            className="grove-variant"
            onClick={() => onSelectVariant(seed)}
          >
            {seed}
          </button>
        ))}
        <button type="button" className="grove-variant grove-variant-new" onClick={onNewVariant} title="Generate next seed">
          <Dice5 size={14} aria-hidden="true" />
        </button>
      </div>
    </div>
  );
}

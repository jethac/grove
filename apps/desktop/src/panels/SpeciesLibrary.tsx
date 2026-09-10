import { FileUp, Leaf } from 'lucide-react';
import { PRESETS } from '../presets';
import type { WorkbenchState } from '../model';

export function SpeciesLibrary({
  state,
  onLoadPreset,
  onImport,
}: {
  state: WorkbenchState;
  onLoadPreset: (id: string) => void;
  onImport: () => void;
}) {
  const active = state.label.startsWith('Preset · ') ? state.label.slice(9) : null;
  return (
    <div className="grove-panel-content">
      <div className="grove-eyebrow">SPECIES</div>
      <div className="grove-preset-list">
        {PRESETS.map((preset) => (
          <button
            key={preset.id}
            type="button"
            className="grove-preset"
            aria-pressed={active === preset.label}
            onClick={() => onLoadPreset(preset.id)}
          >
            <Leaf size={14} aria-hidden="true" />
            <span>{preset.label}</span>
            <small>{preset.id}.toml</small>
          </button>
        ))}
      </div>
      <div className="grove-action-stack">
        <button type="button" onClick={onImport}>
          <FileUp size={14} aria-hidden="true" /> Import TOML…
        </button>
      </div>
      <div className="grove-section-title">DOCUMENT</div>
      <p className="grove-subtle">
        {state.label}
        {state.json?.species.scientific ? ` · ${state.json.species.scientific}` : ''}
      </p>
      <p className="grove-subtle">
        Parameters, TOML source and the generated mesh stay in sync. Seed {state.seed} produces this
        variant; every variant is reproducible from the document plus its seed.
      </p>
    </div>
  );
}

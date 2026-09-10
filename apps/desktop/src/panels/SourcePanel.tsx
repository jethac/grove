import { useEffect, useRef } from 'react';
import { Check, Undo2 } from 'lucide-react';
import type { WorkbenchState } from '../model';

export function SourcePanel({
  state,
  onEdit,
  onApply,
  onRevert,
}: {
  state: WorkbenchState;
  onEdit: (text: string) => void;
  onApply: () => void;
  onRevert: () => void;
}) {
  const areaRef = useRef<HTMLTextAreaElement | null>(null);
  // If params were edited elsewhere while the source is clean, keep it in sync.
  useEffect(() => {
    if (!state.sourceDirty && areaRef.current && areaRef.current.value !== state.sourceDraft) {
      areaRef.current.value = state.sourceDraft;
    }
  }, [state.sourceDraft, state.sourceDirty]);

  return (
    <div className="grove-source">
      <div className="grove-source-toolbar">
        <span className="grove-subtle">
          species.toml {state.sourceDirty ? '· modified' : '· applied'}
        </span>
        <div className="grove-source-actions">
          <button type="button" disabled={!state.sourceDirty} onClick={onRevert} title="Revert to applied source">
            <Undo2 size={13} aria-hidden="true" /> Revert
          </button>
          <button
            type="button"
            disabled={!state.sourceDirty}
            onClick={onApply}
            title="Parse TOML and regenerate"
          >
            <Check size={13} aria-hidden="true" /> Apply
          </button>
        </div>
      </div>
      <textarea
        ref={areaRef}
        className="grove-source-editor"
        defaultValue={state.sourceDraft}
        spellCheck={false}
        aria-label="Species TOML source"
        onChange={(event) => onEdit(event.target.value)}
      />
      {state.sourceError ? (
        <p className="grove-error" role="alert">
          {state.sourceError}
        </p>
      ) : null}
    </div>
  );
}

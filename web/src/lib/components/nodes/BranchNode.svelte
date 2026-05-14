<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';

  type $$Props = NodeProps;

  export let id: $$Props['id'];
  export let data: $$Props['data'] = {
    level: 1,
    count: 5,
    length: 3.0,
    angle: 45,
    rotation: 137.5,
    gravity: 0.2
  };

  function updateField(field: string, value: number) {
    data = { ...data, [field]: value };
  }
</script>

<div class="branch-node">
  <div class="node-header">
    <span class="node-title">Branch L{data.level ?? 1}</span>
  </div>

  <Handle type="target" position={Position.Left} id="in" />

  <div class="node-content">
    <div class="field">
      <label for="branch-level-{id}">Level</label>
      <input
        id="branch-level-{id}"
        type="number"
        step="1"
        min="1"
        max="5"
        value={data.level ?? 1}
        on:input={(e) => updateField('level', parseInt(e.currentTarget.value))}
      />
    </div>

    <div class="field">
      <label for="branch-count-{id}">Count</label>
      <input
        id="branch-count-{id}"
        type="number"
        step="1"
        min="1"
        max="20"
        value={data.count ?? 5}
        on:input={(e) => updateField('count', parseInt(e.currentTarget.value))}
      />
    </div>

    <div class="field">
      <label for="branch-length-{id}">Length</label>
      <input
        id="branch-length-{id}"
        type="number"
        step="0.1"
        min="0.1"
        value={data.length ?? 3.0}
        on:input={(e) => updateField('length', parseFloat(e.currentTarget.value))}
      />
    </div>

    <div class="field">
      <label for="branch-angle-{id}">Angle</label>
      <input
        id="branch-angle-{id}"
        type="number"
        step="1"
        min="0"
        max="90"
        value={data.angle ?? 45}
        on:input={(e) => updateField('angle', parseFloat(e.currentTarget.value))}
      />
    </div>

    <div class="field">
      <label for="branch-rotation-{id}">Rotation</label>
      <input
        id="branch-rotation-{id}"
        type="number"
        step="0.5"
        min="0"
        max="360"
        value={data.rotation ?? 137.5}
        on:input={(e) => updateField('rotation', parseFloat(e.currentTarget.value))}
      />
    </div>

    <div class="field">
      <label for="branch-gravity-{id}">Gravity</label>
      <input
        id="branch-gravity-{id}"
        type="number"
        step="0.05"
        min="-1"
        max="1"
        value={data.gravity ?? 0.2}
        on:input={(e) => updateField('gravity', parseFloat(e.currentTarget.value))}
      />
    </div>
  </div>

  <Handle type="source" position={Position.Right} id="out" />
</div>

<style>
  .branch-node {
    background: var(--node-bg, #1e1e3f);
    border: 2px solid var(--border, #2a2a4a);
    border-radius: 8px;
    min-width: 160px;
    font-size: 12px;
  }

  .node-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: linear-gradient(135deg, #4a5a2d 0%, #353d1a 100%);
    border-radius: 6px 6px 0 0;
    border-bottom: 1px solid var(--border, #2a2a4a);
  }

  .node-title {
    font-weight: 600;
    color: var(--text-primary, #eaeaea);
  }

  .node-content {
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .field {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
  }

  .field label {
    font-size: 10px;
    color: var(--text-secondary, #a0a0a0);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    flex-shrink: 0;
  }

  .field input {
    background: var(--bg-tertiary, #0f0f1a);
    border: 1px solid var(--border, #2a2a4a);
    border-radius: 4px;
    padding: 4px 6px;
    color: var(--text-primary, #eaeaea);
    font-size: 11px;
    width: 70px;
    text-align: right;
  }

  .field input:focus {
    outline: none;
    border-color: var(--accent, #4ade80);
  }
</style>

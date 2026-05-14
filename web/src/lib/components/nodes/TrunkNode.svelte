<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';

  type $$Props = NodeProps;

  export let id: $$Props['id'];
  export let data: $$Props['data'] = {
    height: 8.0,
    radius: 0.4,
    taper: 0.7,
    curve: 0.1,
    segments: 12
  };

  function updateField(field: string, value: number) {
    data = { ...data, [field]: value };
  }
</script>

<div class="trunk-node">
  <div class="node-header">
    <span class="node-title">Trunk</span>
  </div>

  <Handle type="target" position={Position.Left} id="in" />

  <div class="node-content">
    <div class="field">
      <label for="trunk-height-{id}">Height</label>
      <input
        id="trunk-height-{id}"
        type="number"
        step="0.1"
        min="0.1"
        value={data.height ?? 8.0}
        on:input={(e) => updateField('height', parseFloat(e.currentTarget.value))}
      />
    </div>

    <div class="field">
      <label for="trunk-radius-{id}">Radius</label>
      <input
        id="trunk-radius-{id}"
        type="number"
        step="0.05"
        min="0.05"
        value={data.radius ?? 0.4}
        on:input={(e) => updateField('radius', parseFloat(e.currentTarget.value))}
      />
    </div>

    <div class="field">
      <label for="trunk-taper-{id}">Taper</label>
      <input
        id="trunk-taper-{id}"
        type="number"
        step="0.05"
        min="0"
        max="1"
        value={data.taper ?? 0.7}
        on:input={(e) => updateField('taper', parseFloat(e.currentTarget.value))}
      />
    </div>

    <div class="field">
      <label for="trunk-curve-{id}">Curve</label>
      <input
        id="trunk-curve-{id}"
        type="number"
        step="0.01"
        min="0"
        max="1"
        value={data.curve ?? 0.1}
        on:input={(e) => updateField('curve', parseFloat(e.currentTarget.value))}
      />
    </div>

    <div class="field">
      <label for="trunk-segments-{id}">Segments</label>
      <input
        id="trunk-segments-{id}"
        type="number"
        step="1"
        min="3"
        max="32"
        value={data.segments ?? 12}
        on:input={(e) => updateField('segments', parseInt(e.currentTarget.value))}
      />
    </div>
  </div>

  <Handle type="source" position={Position.Right} id="out" />
</div>

<style>
  .trunk-node {
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
    background: linear-gradient(135deg, #5a4a2d 0%, #3d351a 100%);
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

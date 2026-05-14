<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';

  type $$Props = NodeProps;

  export let id: $$Props['id'];
  export let data: $$Props['data'] = {
    shape: 'spherical',
    offset: 0.7
  };

  const shapeOptions = [
    { value: 'spherical', label: 'Spherical' },
    { value: 'conical', label: 'Conical' },
    { value: 'cylindrical', label: 'Cylindrical' },
    { value: 'hemispherical', label: 'Hemispherical' },
    { value: 'umbrella', label: 'Umbrella' },
    { value: 'columnar', label: 'Columnar' }
  ];

  function updateField(field: string, value: string | number) {
    data = { ...data, [field]: value };
  }
</script>

<div class="crown-node">
  <div class="node-header">
    <span class="node-title">Crown</span>
  </div>

  <Handle type="target" position={Position.Left} id="in" />

  <div class="node-content">
    <div class="field">
      <label for="crown-shape-{id}">Shape</label>
      <select
        id="crown-shape-{id}"
        value={data.shape ?? 'spherical'}
        on:change={(e) => updateField('shape', e.currentTarget.value)}
      >
        {#each shapeOptions as option}
          <option value={option.value}>{option.label}</option>
        {/each}
      </select>
    </div>

    <div class="field">
      <label for="crown-offset-{id}">Offset</label>
      <input
        id="crown-offset-{id}"
        type="number"
        step="0.05"
        min="0"
        max="1"
        value={data.offset ?? 0.7}
        on:input={(e) => updateField('offset', parseFloat(e.currentTarget.value))}
      />
    </div>
  </div>

  <Handle type="source" position={Position.Right} id="out" />
</div>

<style>
  .crown-node {
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
    background: linear-gradient(135deg, #2d5a4a 0%, #1a3d35 100%);
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

  .field input,
  .field select {
    background: var(--bg-tertiary, #0f0f1a);
    border: 1px solid var(--border, #2a2a4a);
    border-radius: 4px;
    padding: 4px 6px;
    color: var(--text-primary, #eaeaea);
    font-size: 11px;
    width: 90px;
  }

  .field input {
    text-align: right;
    width: 70px;
  }

  .field input:focus,
  .field select:focus {
    outline: none;
    border-color: var(--accent, #4ade80);
  }
</style>

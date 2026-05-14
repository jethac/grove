<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';

  type $$Props = NodeProps;

  export let id: $$Props['id'];
  export let data: $$Props['data'] = {
    count: 1000,
    size: 0.15,
    geometry: 'quad'
  };

  const geometryOptions = [
    { value: 'quad', label: 'Quad' },
    { value: 'cross', label: 'Cross' },
    { value: 'star', label: 'Star' },
    { value: 'diamond', label: 'Diamond' },
    { value: 'mesh', label: 'Mesh' }
  ];

  function updateField(field: string, value: string | number) {
    data = { ...data, [field]: value };
  }
</script>

<div class="leaves-node">
  <div class="node-header">
    <span class="node-title">Leaves</span>
  </div>

  <Handle type="target" position={Position.Left} id="in" />

  <div class="node-content">
    <div class="field">
      <label for="leaves-count-{id}">Count</label>
      <input
        id="leaves-count-{id}"
        type="number"
        step="100"
        min="0"
        max="10000"
        value={data.count ?? 1000}
        on:input={(e) => updateField('count', parseInt(e.currentTarget.value))}
      />
    </div>

    <div class="field">
      <label for="leaves-size-{id}">Size</label>
      <input
        id="leaves-size-{id}"
        type="number"
        step="0.01"
        min="0.01"
        max="1"
        value={data.size ?? 0.15}
        on:input={(e) => updateField('size', parseFloat(e.currentTarget.value))}
      />
    </div>

    <div class="field">
      <label for="leaves-geometry-{id}">Geometry</label>
      <select
        id="leaves-geometry-{id}"
        value={data.geometry ?? 'quad'}
        on:change={(e) => updateField('geometry', e.currentTarget.value)}
      >
        {#each geometryOptions as option}
          <option value={option.value}>{option.label}</option>
        {/each}
      </select>
    </div>
  </div>

  <Handle type="source" position={Position.Right} id="out" />
</div>

<style>
  .leaves-node {
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
    background: linear-gradient(135deg, #3d5a2d 0%, #2a3d1a 100%);
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

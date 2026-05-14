<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';

  type $$Props = NodeProps;

  export let id: $$Props['id'];
  export let data: $$Props['data'] = { name: 'Oak' };

  function handleNameChange(event: Event) {
    const target = event.target as HTMLInputElement;
    data = { ...data, name: target.value };
  }
</script>

<div class="species-node">
  <div class="node-header">
    <span class="node-icon">&#x1F333;</span>
    <span class="node-title">Species</span>
  </div>

  <div class="node-content">
    <div class="field">
      <label for="species-name-{id}">Name</label>
      <input
        id="species-name-{id}"
        type="text"
        value={data.name || 'Oak'}
        on:input={handleNameChange}
        placeholder="Species name"
      />
    </div>
  </div>

  <Handle type="source" position={Position.Right} id="out" />
</div>

<style>
  .species-node {
    background: var(--node-bg, #1e1e3f);
    border: 2px solid var(--border, #2a2a4a);
    border-radius: 8px;
    min-width: 180px;
    font-size: 12px;
  }

  .node-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: linear-gradient(135deg, #2d5a27 0%, #1a3d1a 100%);
    border-radius: 6px 6px 0 0;
    border-bottom: 1px solid var(--border, #2a2a4a);
  }

  .node-icon {
    font-size: 14px;
  }

  .node-title {
    font-weight: 600;
    color: var(--text-primary, #eaeaea);
  }

  .node-content {
    padding: 12px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .field label {
    font-size: 10px;
    color: var(--text-secondary, #a0a0a0);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .field input {
    background: var(--bg-tertiary, #0f0f1a);
    border: 1px solid var(--border, #2a2a4a);
    border-radius: 4px;
    padding: 6px 8px;
    color: var(--text-primary, #eaeaea);
    font-size: 12px;
  }

  .field input:focus {
    outline: none;
    border-color: var(--accent, #4ade80);
  }
</style>

<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';
  import { createEventDispatcher } from 'svelte';

  type $$Props = NodeProps;

  export let id: $$Props['id'];
  export let data: $$Props['data'] = {};

  const dispatch = createEventDispatcher<{
    generate: void;
    export: void;
  }>();

  function handleGenerate() {
    dispatch('generate');
    if (data.onGenerate) {
      data.onGenerate();
    }
  }

  function handleExport() {
    dispatch('export');
    if (data.onExport) {
      data.onExport();
    }
  }
</script>

<div class="output-node">
  <div class="node-header">
    <span class="node-title">Output</span>
  </div>

  <Handle type="target" position={Position.Left} id="in" />

  <div class="node-content">
    <button class="generate-btn" on:click={handleGenerate}>
      Generate Tree
    </button>

    <button class="export-btn" on:click={handleExport}>
      Export TOML
    </button>
  </div>
</div>

<style>
  .output-node {
    background: var(--node-bg, #1e1e3f);
    border: 2px solid var(--border, #2a2a4a);
    border-radius: 8px;
    min-width: 140px;
    font-size: 12px;
  }

  .node-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: linear-gradient(135deg, #4a2d5a 0%, #351a3d 100%);
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

  .generate-btn,
  .export-btn {
    background: var(--bg-tertiary, #0f0f1a);
    border: 1px solid var(--border, #2a2a4a);
    border-radius: 4px;
    padding: 8px 12px;
    color: var(--text-primary, #eaeaea);
    font-size: 11px;
    cursor: pointer;
    transition: all 0.2s ease;
    text-align: center;
  }

  .generate-btn {
    background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
    border-color: #22c55e;
    font-weight: 600;
  }

  .generate-btn:hover {
    background: linear-gradient(135deg, #4ade80 0%, #22c55e 100%);
    transform: translateY(-1px);
  }

  .export-btn:hover {
    border-color: var(--accent, #4ade80);
    background: var(--bg-secondary, #16213e);
  }
</style>

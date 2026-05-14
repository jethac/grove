<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  const nodeTypes = [
    { type: 'trunk', label: 'Trunk', icon: '|', description: 'Main trunk structure' },
    { type: 'branch', label: 'Branch', icon: 'Y', description: 'Branching pattern' },
    { type: 'twig', label: 'Twig', icon: '/', description: 'Small twigs' },
    { type: 'leaf', label: 'Leaf', icon: '@', description: 'Leaf clusters' },
    { type: 'bark', label: 'Bark', icon: '#', description: 'Bark texture' },
    { type: 'wind', label: 'Wind', icon: '~', description: 'Wind animation' }
  ];

  function addNode(type: string, label: string) {
    dispatch('addNode', { type, label });
  }
</script>

<div class="palette">
  <div class="palette-header">
    <span>Nodes</span>
  </div>

  <div class="palette-items">
    {#each nodeTypes as node}
      <button
        class="palette-item"
        on:click={() => addNode(node.type, node.label)}
        title={node.description}
      >
        <span class="icon">{node.icon}</span>
        <span class="label">{node.label}</span>
      </button>
    {/each}
  </div>
</div>

<style>
  .palette {
    border-bottom: 1px solid var(--border);
  }

  .palette-header {
    padding: 0.5rem 1rem;
    font-size: 0.75rem;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .palette-items {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
    padding: 0 0.5rem 0.5rem;
  }

  .palette-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.125rem;
    padding: 0.375rem 0.5rem;
    min-width: 50px;
    background: var(--node-bg);
    border: 1px solid var(--border);
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .palette-item:hover {
    border-color: var(--accent);
    background: var(--bg-tertiary);
  }

  .icon {
    font-family: monospace;
    font-size: 1rem;
    font-weight: bold;
    color: var(--accent);
  }

  .label {
    font-size: 0.625rem;
    color: var(--text-secondary);
  }
</style>

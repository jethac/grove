<script lang="ts">
  import { editorStore } from '$lib/stores/editor';
  import NodePalette from './NodePalette.svelte';

  // Node graph state
  let nodes: Array<{
    id: string;
    type: string;
    label: string;
    x: number;
    y: number;
  }> = [];

  let connections: Array<{
    from: string;
    to: string;
    fromPort: string;
    toPort: string;
  }> = [];

  let selectedNodeId: string | null = null;
  let draggingNode: string | null = null;
  let offset = { x: 0, y: 0 };

  function selectNode(id: string) {
    selectedNodeId = id;
    editorStore.selectNode(id);
  }

  function startDrag(event: MouseEvent, nodeId: string) {
    draggingNode = nodeId;
    const node = nodes.find(n => n.id === nodeId);
    if (node) {
      offset = {
        x: event.clientX - node.x,
        y: event.clientY - node.y
      };
    }
  }

  function onDrag(event: MouseEvent) {
    if (!draggingNode) return;

    const nodeIndex = nodes.findIndex(n => n.id === draggingNode);
    if (nodeIndex !== -1) {
      nodes[nodeIndex].x = event.clientX - offset.x;
      nodes[nodeIndex].y = event.clientY - offset.y;
      nodes = [...nodes];
    }
  }

  function stopDrag() {
    draggingNode = null;
  }

  function addNode(type: string, label: string) {
    const id = `node-${Date.now()}`;
    nodes = [...nodes, {
      id,
      type,
      label,
      x: 100 + Math.random() * 100,
      y: 100 + Math.random() * 100
    }];
  }
</script>

<div class="node-editor" on:mousemove={onDrag} on:mouseup={stopDrag} role="application">
  <div class="header">
    <h3>Node Graph</h3>
  </div>

  <NodePalette on:addNode={(e) => addNode(e.detail.type, e.detail.label)} />

  <div class="canvas">
    <svg class="connections">
      {#each connections as conn}
        <!-- TODO: Draw bezier curves for connections -->
      {/each}
    </svg>

    {#each nodes as node (node.id)}
      <div
        class="node"
        class:selected={selectedNodeId === node.id}
        style="left: {node.x}px; top: {node.y}px"
        on:mousedown={(e) => startDrag(e, node.id)}
        on:click={() => selectNode(node.id)}
        role="button"
        tabindex="0"
      >
        <div class="node-header">{node.label}</div>
        <div class="node-body">
          <div class="port input"></div>
          <div class="port output"></div>
        </div>
      </div>
    {/each}

    {#if nodes.length === 0}
      <div class="empty-state">
        <p>Drag nodes from the palette above</p>
        <p>to build your tree</p>
      </div>
    {/if}
  </div>
</div>

<style>
  .node-editor {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .header {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--border);
  }

  .header h3 {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .canvas {
    flex: 1;
    position: relative;
    overflow: hidden;
    background:
      radial-gradient(circle, var(--border) 1px, transparent 1px);
    background-size: 20px 20px;
  }

  .connections {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }

  .node {
    position: absolute;
    min-width: 150px;
    background: var(--node-bg);
    border: 2px solid var(--border);
    border-radius: 8px;
    cursor: move;
    user-select: none;
  }

  .node.selected {
    border-color: var(--node-selected);
  }

  .node-header {
    padding: 0.5rem 0.75rem;
    background: var(--bg-tertiary);
    border-radius: 6px 6px 0 0;
    font-size: 0.8125rem;
    font-weight: 500;
  }

  .node-body {
    padding: 0.5rem;
    display: flex;
    justify-content: space-between;
  }

  .port {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--border);
    cursor: pointer;
  }

  .port:hover {
    background: var(--accent);
  }

  .port.input {
    margin-left: -6px;
  }

  .port.output {
    margin-right: -6px;
  }

  .empty-state {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
    color: var(--text-secondary);
    font-size: 0.875rem;
  }
</style>

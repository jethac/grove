<script lang="ts">
  import { writable } from 'svelte/store';
  import {
    SvelteFlow,
    Controls,
    Background,
    BackgroundVariant,
    MiniMap
  } from '@xyflow/svelte';
  import type { Node, Edge, NodeTypes, Connection } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';

  import { treeStore } from '$lib/stores/tree';
  import { editorStore } from '$lib/stores/editor';

  import SpeciesNode from './nodes/SpeciesNode.svelte';
  import TrunkNode from './nodes/TrunkNode.svelte';
  import BranchNode from './nodes/BranchNode.svelte';
  import CrownNode from './nodes/CrownNode.svelte';
  import LeavesNode from './nodes/LeavesNode.svelte';
  import OutputNode from './nodes/OutputNode.svelte';

  // Node types registration
  const nodeTypes: NodeTypes = {
    species: SpeciesNode,
    trunk: TrunkNode,
    branch: BranchNode,
    crown: CrownNode,
    leaves: LeavesNode,
    output: OutputNode
  };

  // Collect all node data and generate tree
  function handleGenerate() {
    const config = collectConfig();
    const toml = configToToml(config);
    treeStore.loadSpecies(toml);
  }

  // Export configuration as TOML
  function handleExportToml() {
    const config = collectConfig();
    const toml = configToToml(config);

    // Create download
    const blob = new Blob([toml], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${config.species?.name || 'tree'}.toml`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  // Default nodes for initial layout
  const initialNodes: Node[] = [
    {
      id: 'species-1',
      type: 'species',
      position: { x: 50, y: 150 },
      data: { name: 'Oak' }
    },
    {
      id: 'trunk-1',
      type: 'trunk',
      position: { x: 280, y: 50 },
      data: {
        height: 8.0,
        radius: 0.4,
        taper: 0.7,
        curve: 0.1,
        segments: 12
      }
    },
    {
      id: 'branch-1',
      type: 'branch',
      position: { x: 500, y: 20 },
      data: {
        level: 1,
        count: 5,
        length: 3.0,
        angle: 45,
        rotation: 137.5,
        gravity: 0.2
      }
    },
    {
      id: 'branch-2',
      type: 'branch',
      position: { x: 500, y: 250 },
      data: {
        level: 2,
        count: 4,
        length: 1.5,
        angle: 40,
        rotation: 137.5,
        gravity: 0.3
      }
    },
    {
      id: 'crown-1',
      type: 'crown',
      position: { x: 720, y: 80 },
      data: {
        shape: 'spherical',
        offset: 0.7
      }
    },
    {
      id: 'leaves-1',
      type: 'leaves',
      position: { x: 720, y: 220 },
      data: {
        count: 1000,
        size: 0.15,
        geometry: 'quad'
      }
    },
    {
      id: 'output-1',
      type: 'output',
      position: { x: 940, y: 150 },
      data: {
        onGenerate: handleGenerate,
        onExport: handleExportToml
      }
    }
  ];

  // Default edges connecting nodes
  const initialEdges: Edge[] = [
    { id: 'e-species-trunk', source: 'species-1', target: 'trunk-1', sourceHandle: 'out', targetHandle: 'in' },
    { id: 'e-trunk-branch1', source: 'trunk-1', target: 'branch-1', sourceHandle: 'out', targetHandle: 'in' },
    { id: 'e-branch1-branch2', source: 'branch-1', target: 'branch-2', sourceHandle: 'out', targetHandle: 'in' },
    { id: 'e-branch1-crown', source: 'branch-1', target: 'crown-1', sourceHandle: 'out', targetHandle: 'in' },
    { id: 'e-branch2-leaves', source: 'branch-2', target: 'leaves-1', sourceHandle: 'out', targetHandle: 'in' },
    { id: 'e-crown-output', source: 'crown-1', target: 'output-1', sourceHandle: 'out', targetHandle: 'in' },
    { id: 'e-leaves-output', source: 'leaves-1', target: 'output-1', sourceHandle: 'out', targetHandle: 'in' }
  ];

  // Reactive stores for nodes and edges
  const nodes = writable<Node[]>(initialNodes);
  const edges = writable<Edge[]>(initialEdges);

  // Handle node selection
  function handleNodeClick(event: CustomEvent<{ node: Node }>) {
    const { node } = event.detail;
    editorStore.selectNode(node.id);
  }

  // Handle pane click to deselect
  function handlePaneClick() {
    editorStore.selectNode(null);
  }

  // Handle new connections
  function handleConnect(event: CustomEvent<{ connection: Connection }>) {
    const { connection } = event.detail;
    if (connection.source && connection.target) {
      const newEdge: Edge = {
        id: `e-${connection.source}-${connection.target}`,
        source: connection.source,
        target: connection.target,
        sourceHandle: connection.sourceHandle || undefined,
        targetHandle: connection.targetHandle || undefined
      };
      edges.update(e => [...e, newEdge]);
    }
  }

  // Handle edge deletion
  function handleEdgesDelete(event: CustomEvent<{ edges: Edge[] }>) {
    const deletedIds = new Set(event.detail.edges.map(e => e.id));
    edges.update(e => e.filter(edge => !deletedIds.has(edge.id)));
  }

  // Handle nodes deletion
  function handleNodesDelete(event: CustomEvent<{ nodes: Node[] }>) {
    const deletedIds = new Set(event.detail.nodes.map(n => n.id));
    nodes.update(n => n.filter(node => !deletedIds.has(node.id)));
    // Also remove connected edges
    edges.update(e => e.filter(edge =>
      !deletedIds.has(edge.source) && !deletedIds.has(edge.target)
    ));
  }

  // Collect configuration from all nodes
  function collectConfig(): Record<string, any> {
    const currentNodes = $nodes;
    const config: Record<string, any> = {};

    for (const node of currentNodes) {
      switch (node.type) {
        case 'species':
          config.species = { name: node.data.name || 'Unnamed' };
          break;
        case 'trunk':
          config.trunk = {
            height: node.data.height ?? 8.0,
            radius: node.data.radius ?? 0.4,
            taper: node.data.taper ?? 0.7,
            curve: node.data.curve ?? 0.1,
            segments: node.data.segments ?? 12
          };
          break;
        case 'branch':
          if (!config.branches) config.branches = [];
          config.branches.push({
            level: node.data.level ?? 1,
            count: node.data.count ?? 5,
            length: node.data.length ?? 3.0,
            angle: node.data.angle ?? 45,
            rotation: node.data.rotation ?? 137.5,
            gravity: node.data.gravity ?? 0.2
          });
          break;
        case 'crown':
          config.crown = {
            shape: node.data.shape || 'spherical',
            offset: node.data.offset ?? 0.7
          };
          break;
        case 'leaves':
          config.leaves = {
            count: node.data.count ?? 1000,
            size: node.data.size ?? 0.15,
            geometry: node.data.geometry || 'quad'
          };
          break;
      }
    }

    // Sort branches by level
    if (config.branches) {
      config.branches.sort((a: any, b: any) => a.level - b.level);
    }

    return config;
  }

  // Convert configuration object to TOML string
  function configToToml(config: Record<string, any>): string {
    let toml = '';

    // Species section
    if (config.species) {
      toml += `[species]\n`;
      toml += `name = "${config.species.name}"\n\n`;
    }

    // Trunk section
    if (config.trunk) {
      toml += `[trunk]\n`;
      toml += `height = ${config.trunk.height}\n`;
      toml += `radius = ${config.trunk.radius}\n`;
      toml += `taper = ${config.trunk.taper}\n`;
      toml += `curve = ${config.trunk.curve}\n`;
      toml += `segments = ${config.trunk.segments}\n\n`;
    }

    // Branch sections
    if (config.branches) {
      for (const branch of config.branches) {
        toml += `[[branches]]\n`;
        toml += `level = ${branch.level}\n`;
        toml += `count = ${branch.count}\n`;
        toml += `length = ${branch.length}\n`;
        toml += `angle = ${branch.angle}\n`;
        toml += `rotation = ${branch.rotation}\n`;
        toml += `gravity = ${branch.gravity}\n\n`;
      }
    }

    // Crown section
    if (config.crown) {
      toml += `[crown]\n`;
      toml += `shape = "${config.crown.shape}"\n`;
      toml += `offset = ${config.crown.offset}\n\n`;
    }

    // Leaves section
    if (config.leaves) {
      toml += `[leaves]\n`;
      toml += `count = ${config.leaves.count}\n`;
      toml += `size = ${config.leaves.size}\n`;
      toml += `geometry = "${config.leaves.geometry}"\n`;
    }

    return toml;
  }

  // Add new node of type
  function addNode(type: string) {
    const id = `${type}-${Date.now()}`;
    const defaultData: Record<string, any> = {
      species: { name: 'New Species' },
      trunk: { height: 8.0, radius: 0.4, taper: 0.7, curve: 0.1, segments: 12 },
      branch: { level: 1, count: 5, length: 3.0, angle: 45, rotation: 137.5, gravity: 0.2 },
      crown: { shape: 'spherical', offset: 0.7 },
      leaves: { count: 1000, size: 0.15, geometry: 'quad' },
      output: { onGenerate: handleGenerate, onExport: handleExportToml }
    };

    const newNode: Node = {
      id,
      type,
      position: { x: 100 + Math.random() * 200, y: 100 + Math.random() * 200 },
      data: defaultData[type] || {}
    };

    nodes.update(n => [...n, newNode]);
  }

  // Reset to default layout
  function resetLayout() {
    nodes.set(initialNodes.map(n => ({
      ...n,
      data: n.type === 'output'
        ? { ...n.data, onGenerate: handleGenerate, onExport: handleExportToml }
        : { ...n.data }
    })));
    edges.set([...initialEdges]);
  }
</script>

<div class="node-editor">
  <div class="toolbar">
    <div class="toolbar-group">
      <button on:click={() => addNode('species')} title="Add Species Node">+ Species</button>
      <button on:click={() => addNode('trunk')} title="Add Trunk Node">+ Trunk</button>
      <button on:click={() => addNode('branch')} title="Add Branch Node">+ Branch</button>
      <button on:click={() => addNode('crown')} title="Add Crown Node">+ Crown</button>
      <button on:click={() => addNode('leaves')} title="Add Leaves Node">+ Leaves</button>
      <button on:click={() => addNode('output')} title="Add Output Node">+ Output</button>
    </div>
    <div class="toolbar-group">
      <button on:click={resetLayout} title="Reset to default layout">Reset</button>
    </div>
  </div>

  <div class="flow-container">
    <SvelteFlow
      {nodes}
      {edges}
      {nodeTypes}
      fitView
      snapToGrid
      snapGrid={[15, 15]}
      on:nodeclick={handleNodeClick}
      on:paneclick={handlePaneClick}
      on:connect={handleConnect}
      on:edgesdelete={handleEdgesDelete}
      on:nodesdelete={handleNodesDelete}
      defaultEdgeOptions={{
        type: 'smoothstep',
        style: 'stroke: #4ade80; stroke-width: 2px;'
      }}
    >
      <Controls position="bottom-left" />
      <MiniMap
        position="bottom-right"
        nodeColor={(node) => {
          switch (node.type) {
            case 'species': return '#2d5a27';
            case 'trunk': return '#5a4a2d';
            case 'branch': return '#4a5a2d';
            case 'crown': return '#2d5a4a';
            case 'leaves': return '#3d5a2d';
            case 'output': return '#4a2d5a';
            default: return '#1e1e3f';
          }
        }}
      />
      <Background variant={BackgroundVariant.Dots} gap={20} size={1} />
    </SvelteFlow>
  </div>
</div>

<style>
  .node-editor {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-secondary, #16213e);
  }

  .toolbar {
    display: flex;
    justify-content: space-between;
    padding: 8px 12px;
    background: var(--bg-tertiary, #0f0f1a);
    border-bottom: 1px solid var(--border, #2a2a4a);
    gap: 8px;
    flex-wrap: wrap;
  }

  .toolbar-group {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
  }

  .toolbar button {
    background: var(--bg-secondary, #16213e);
    border: 1px solid var(--border, #2a2a4a);
    color: var(--text-primary, #eaeaea);
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 11px;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .toolbar button:hover {
    border-color: var(--accent, #4ade80);
    background: var(--node-bg, #1e1e3f);
  }

  .flow-container {
    flex: 1;
    width: 100%;
  }

  /* Override Svelte Flow styles for dark theme */
  .flow-container :global(.svelte-flow) {
    background: var(--bg-secondary, #16213e);
  }

  .flow-container :global(.svelte-flow__node) {
    cursor: grab;
  }

  .flow-container :global(.svelte-flow__node.selected) {
    box-shadow: 0 0 0 2px var(--accent, #4ade80);
  }

  .flow-container :global(.svelte-flow__edge-path) {
    stroke: var(--accent, #4ade80);
    stroke-width: 2px;
  }

  .flow-container :global(.svelte-flow__handle) {
    width: 10px;
    height: 10px;
    background: var(--border, #2a2a4a);
    border: 2px solid var(--accent, #4ade80);
  }

  .flow-container :global(.svelte-flow__handle:hover) {
    background: var(--accent, #4ade80);
  }

  .flow-container :global(.svelte-flow__controls) {
    background: var(--bg-tertiary, #0f0f1a);
    border: 1px solid var(--border, #2a2a4a);
    border-radius: 4px;
  }

  .flow-container :global(.svelte-flow__controls button) {
    background: var(--bg-secondary, #16213e);
    border-bottom: 1px solid var(--border, #2a2a4a);
    color: var(--text-primary, #eaeaea);
  }

  .flow-container :global(.svelte-flow__controls button:hover) {
    background: var(--node-bg, #1e1e3f);
  }

  .flow-container :global(.svelte-flow__controls button svg) {
    fill: var(--text-primary, #eaeaea);
  }

  .flow-container :global(.svelte-flow__minimap) {
    background: var(--bg-tertiary, #0f0f1a);
    border: 1px solid var(--border, #2a2a4a);
    border-radius: 4px;
  }

  .flow-container :global(.svelte-flow__minimap-mask) {
    fill: var(--bg-secondary, #16213e);
    opacity: 0.8;
  }

  .flow-container :global(.svelte-flow__background pattern circle) {
    fill: var(--border, #2a2a4a);
  }

  .flow-container :global(.svelte-flow__attribution) {
    display: none;
  }
</style>

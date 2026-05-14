<script lang="ts">
  import { treeStore } from '$lib/stores/tree';
  import { editorStore } from '$lib/stores/editor';

  // Local state for property values
  let seed = 12345;

  $: seed = $treeStore.seed;

  function updateSeed() {
    treeStore.setSeed(seed);
    if ($editorStore.autoRegenerate) {
      treeStore.regenerate();
    }
  }
</script>

<div class="inspector">
  <div class="section">
    <div class="section-header">
      <h3>Tree Properties</h3>
    </div>

    <div class="property">
      <label for="species">Species</label>
      <input
        id="species"
        type="text"
        value={$treeStore.species || 'No species loaded'}
        disabled
      />
    </div>

    <div class="property">
      <label for="seed">Seed</label>
      <div class="seed-input">
        <input
          id="seed"
          type="number"
          bind:value={seed}
          on:change={updateSeed}
        />
        <button on:click={() => { treeStore.randomizeSeed(); treeStore.regenerate(); }}>
          Rand
        </button>
      </div>
    </div>
  </div>

  <div class="section">
    <div class="section-header">
      <h3>View Options</h3>
    </div>

    <div class="property checkbox">
      <label>
        <input
          type="checkbox"
          checked={$editorStore.showWireframe}
          on:change={editorStore.toggleWireframe}
        />
        Show Wireframe
      </label>
    </div>

    <div class="property checkbox">
      <label>
        <input
          type="checkbox"
          checked={$editorStore.showNormals}
          on:change={editorStore.toggleNormals}
        />
        Show Normals
      </label>
    </div>

    <div class="property checkbox">
      <label>
        <input
          type="checkbox"
          checked={$editorStore.autoRegenerate}
          on:change={editorStore.toggleAutoRegenerate}
        />
        Auto Regenerate
      </label>
    </div>
  </div>

  <div class="section">
    <div class="section-header">
      <h3>Selected Node</h3>
    </div>

    {#if $editorStore.selectedNode}
      <div class="property">
        <label>Node ID</label>
        <input type="text" value={$editorStore.selectedNode} disabled />
      </div>

      <!-- Node-specific properties would go here -->
      <p class="hint">Select a node to edit its properties</p>
    {:else}
      <p class="hint">No node selected</p>
    {/if}
  </div>

  <div class="section">
    <div class="section-header">
      <h3>Statistics</h3>
    </div>

    <div class="stats">
      <div class="stat">
        <span class="stat-label">Vertices</span>
        <span class="stat-value">--</span>
      </div>
      <div class="stat">
        <span class="stat-label">Triangles</span>
        <span class="stat-value">--</span>
      </div>
      <div class="stat">
        <span class="stat-label">Branches</span>
        <span class="stat-value">--</span>
      </div>
      <div class="stat">
        <span class="stat-label">Leaves</span>
        <span class="stat-value">--</span>
      </div>
    </div>
  </div>
</div>

<style>
  .inspector {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .section {
    border-bottom: 1px solid var(--border);
  }

  .section-header {
    padding: 0.75rem 1rem;
    background: var(--bg-tertiary);
  }

  .section-header h3 {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .property {
    padding: 0.5rem 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .property label {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .property input[type="text"],
  .property input[type="number"] {
    width: 100%;
  }

  .property.checkbox {
    flex-direction: row;
  }

  .property.checkbox label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-primary);
    cursor: pointer;
  }

  .seed-input {
    display: flex;
    gap: 0.5rem;
  }

  .seed-input input {
    flex: 1;
  }

  .seed-input button {
    padding: 0.375rem 0.75rem;
  }

  .hint {
    padding: 0.5rem 1rem;
    font-size: 0.8125rem;
    color: var(--text-secondary);
    font-style: italic;
  }

  .stats {
    padding: 0.5rem 1rem;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
  }

  .stat {
    display: flex;
    justify-content: space-between;
    padding: 0.375rem 0.5rem;
    background: var(--bg-tertiary);
    border-radius: 4px;
  }

  .stat-label {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .stat-value {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--accent);
  }
</style>

<script lang="ts">
  import { treeStore } from '$lib/stores/tree';
  import { editorStore } from '$lib/stores/editor';
  import Section from './inspector/Section.svelte';
  import NumberInput from './inspector/NumberInput.svelte';
  import SelectInput from './inspector/SelectInput.svelte';

  // Crown shape options
  const crownShapeOptions = [
    { value: 'spherical', label: 'Spherical' },
    { value: 'conical', label: 'Conical' },
    { value: 'hemispherical', label: 'Hemispherical' },
    { value: 'flame', label: 'Flame' },
    { value: 'columnar', label: 'Columnar' }
  ];

  // Leaf geometry options
  const leafGeometryOptions = [
    { value: 'polygon', label: 'Polygon' },
    { value: 'cross_billboard', label: 'Cross Billboard' },
    { value: 'billboard', label: 'Billboard' },
    { value: 'none', label: 'None' }
  ];

  // Local state for seed input
  let seed = 12345;
  $: seed = $treeStore.seed;

  // Species inputs
  let speciesName = '';
  let scientificName = '';
  $: speciesName = $treeStore.params.name;
  $: scientificName = $treeStore.params.scientificName;

  function updateSeed() {
    treeStore.setSeed(seed);
    if ($editorStore.autoRegenerate) {
      treeStore.regenerate();
    }
  }

  function handleRegenerate() {
    treeStore.regenerate();
  }

  function handleRandomize() {
    treeStore.randomizeSeed();
    treeStore.regenerate();
  }

  function updateSpeciesName() {
    treeStore.setSpeciesName(speciesName);
  }

  function updateScientificName() {
    treeStore.setScientificName(scientificName);
  }

  // Handler helper for auto-regenerate
  function withAutoRegenerate(updateFn: () => void) {
    return () => {
      updateFn();
      if ($editorStore.autoRegenerate) {
        treeStore.regenerate();
      }
    };
  }
</script>

<div class="inspector">
  <!-- Species Section -->
  <Section title="Species">
    <div class="text-input">
      <label for="species-name">Name</label>
      <input
        id="species-name"
        type="text"
        bind:value={speciesName}
        on:change={updateSpeciesName}
      />
    </div>
    <div class="text-input">
      <label for="scientific-name">Scientific Name</label>
      <input
        id="scientific-name"
        type="text"
        bind:value={scientificName}
        on:change={updateScientificName}
        class="italic"
      />
    </div>
  </Section>

  <!-- Trunk Section -->
  <Section title="Trunk">
    <NumberInput
      label="Height"
      value={$treeStore.params.trunk.height}
      min={0.5}
      max={20}
      step={0.1}
      on:change={(e) => {
        treeStore.updateTrunk('height', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Radius"
      value={$treeStore.params.trunk.radius}
      min={0.05}
      max={2}
      step={0.01}
      on:change={(e) => {
        treeStore.updateTrunk('radius', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Taper"
      value={$treeStore.params.trunk.taper}
      min={0}
      max={1}
      step={0.01}
      on:change={(e) => {
        treeStore.updateTrunk('taper', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Curve"
      value={$treeStore.params.trunk.curve}
      min={0}
      max={90}
      step={1}
      on:change={(e) => {
        treeStore.updateTrunk('curve', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Segments"
      value={$treeStore.params.trunk.segments}
      min={3}
      max={16}
      step={1}
      showSlider={false}
      on:change={(e) => {
        treeStore.updateTrunk('segments', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
  </Section>

  <!-- Branches Level 1 Section -->
  <Section title="Branches Level 1">
    <NumberInput
      label="Count"
      value={$treeStore.params.branches.level1.count}
      min={0}
      max={20}
      step={1}
      on:change={(e) => {
        treeStore.updateBranchLevel1('count', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Length"
      value={$treeStore.params.branches.level1.length}
      min={0.1}
      max={10}
      step={0.1}
      on:change={(e) => {
        treeStore.updateBranchLevel1('length', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Angle"
      value={$treeStore.params.branches.level1.angle}
      min={0}
      max={90}
      step={1}
      on:change={(e) => {
        treeStore.updateBranchLevel1('angle', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Rotation"
      value={$treeStore.params.branches.level1.rotation}
      min={0}
      max={360}
      step={1}
      on:change={(e) => {
        treeStore.updateBranchLevel1('rotation', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Gravity"
      value={$treeStore.params.branches.level1.gravity}
      min={-1}
      max={1}
      step={0.01}
      on:change={(e) => {
        treeStore.updateBranchLevel1('gravity', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
  </Section>

  <!-- Branches Level 2 Section -->
  <Section title="Branches Level 2" expanded={false}>
    <NumberInput
      label="Count"
      value={$treeStore.params.branches.level2.count}
      min={0}
      max={20}
      step={1}
      on:change={(e) => {
        treeStore.updateBranchLevel2('count', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Length"
      value={$treeStore.params.branches.level2.length}
      min={0.1}
      max={10}
      step={0.1}
      on:change={(e) => {
        treeStore.updateBranchLevel2('length', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Angle"
      value={$treeStore.params.branches.level2.angle}
      min={0}
      max={90}
      step={1}
      on:change={(e) => {
        treeStore.updateBranchLevel2('angle', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Rotation"
      value={$treeStore.params.branches.level2.rotation}
      min={0}
      max={360}
      step={1}
      on:change={(e) => {
        treeStore.updateBranchLevel2('rotation', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Gravity"
      value={$treeStore.params.branches.level2.gravity}
      min={-1}
      max={1}
      step={0.01}
      on:change={(e) => {
        treeStore.updateBranchLevel2('gravity', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
  </Section>

  <!-- Crown Section -->
  <Section title="Crown">
    <SelectInput
      label="Shape"
      value={$treeStore.params.crown.shape}
      options={crownShapeOptions}
      on:change={(e) => {
        treeStore.updateCrown('shape', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Offset"
      value={$treeStore.params.crown.offset}
      min={0}
      max={1}
      step={0.01}
      on:change={(e) => {
        treeStore.updateCrown('offset', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
  </Section>

  <!-- Leaves Section -->
  <Section title="Leaves">
    <NumberInput
      label="Count"
      value={$treeStore.params.leaves.count}
      min={0}
      max={10000}
      step={100}
      on:change={(e) => {
        treeStore.updateLeaves('count', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <NumberInput
      label="Size"
      value={$treeStore.params.leaves.size}
      min={0.01}
      max={0.5}
      step={0.01}
      on:change={(e) => {
        treeStore.updateLeaves('size', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
    <SelectInput
      label="Geometry"
      value={$treeStore.params.leaves.geometry}
      options={leafGeometryOptions}
      on:change={(e) => {
        treeStore.updateLeaves('geometry', e.detail);
        if ($editorStore.autoRegenerate) treeStore.regenerate();
      }}
    />
  </Section>

  <!-- Generation Section -->
  <Section title="Generation">
    <div class="seed-row">
      <div class="seed-input">
        <label for="seed">Seed</label>
        <input
          id="seed"
          type="number"
          bind:value={seed}
          on:change={updateSeed}
        />
      </div>
      <button class="btn-small" on:click={handleRandomize} title="Randomize Seed">
        Rand
      </button>
    </div>
    <div class="regenerate-row">
      <button class="btn-primary" on:click={handleRegenerate}>
        Regenerate Tree
      </button>
    </div>
    <div class="auto-regenerate">
      <label>
        <input
          type="checkbox"
          checked={$editorStore.autoRegenerate}
          on:change={editorStore.toggleAutoRegenerate}
        />
        Auto-regenerate on change
      </label>
    </div>
  </Section>

  <!-- View Options Section -->
  <Section title="View Options" expanded={false}>
    <div class="checkbox-row">
      <label>
        <input
          type="checkbox"
          checked={$editorStore.showWireframe}
          on:change={editorStore.toggleWireframe}
        />
        Show Wireframe
      </label>
    </div>
    <div class="checkbox-row">
      <label>
        <input
          type="checkbox"
          checked={$editorStore.showNormals}
          on:change={editorStore.toggleNormals}
        />
        Show Normals
      </label>
    </div>
  </Section>

  <!-- Statistics Section -->
  <Section title="Statistics" expanded={false}>
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
  </Section>
</div>

<style>
  .inspector {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow-y: auto;
  }

  .text-input {
    padding: 0.375rem 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .text-input label {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .text-input input {
    width: 100%;
    padding: 0.375rem 0.5rem;
    font-size: 0.8125rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
  }

  .text-input input:focus {
    outline: none;
    border-color: var(--accent);
  }

  .text-input input.italic {
    font-style: italic;
  }

  .seed-row {
    padding: 0.375rem 1rem;
    display: flex;
    gap: 0.5rem;
    align-items: flex-end;
  }

  .seed-input {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .seed-input label {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .seed-input input {
    width: 100%;
    padding: 0.375rem 0.5rem;
    font-size: 0.8125rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
  }

  .seed-input input:focus {
    outline: none;
    border-color: var(--accent);
  }

  .btn-small {
    padding: 0.375rem 0.75rem;
    font-size: 0.75rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    cursor: pointer;
  }

  .btn-small:hover {
    background: var(--bg-secondary);
    border-color: var(--text-secondary);
  }

  .regenerate-row {
    padding: 0.5rem 1rem;
  }

  .btn-primary {
    width: 100%;
    padding: 0.5rem 1rem;
    font-size: 0.8125rem;
    font-weight: 500;
    background: var(--accent);
    border: none;
    border-radius: 4px;
    color: var(--bg-primary);
    cursor: pointer;
  }

  .btn-primary:hover {
    filter: brightness(1.1);
  }

  .auto-regenerate {
    padding: 0.375rem 1rem;
  }

  .auto-regenerate label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.8125rem;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .checkbox-row {
    padding: 0.375rem 1rem;
  }

  .checkbox-row label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.8125rem;
    color: var(--text-primary);
    cursor: pointer;
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

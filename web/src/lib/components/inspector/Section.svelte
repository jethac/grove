<script lang="ts">
  export let title: string;
  export let expanded: boolean = true;

  function toggle() {
    expanded = !expanded;
  }
</script>

<div class="section" class:collapsed={!expanded}>
  <button class="section-header" on:click={toggle} type="button">
    <span class="chevron">{expanded ? '▼' : '▶'}</span>
    <h3>{title}</h3>
  </button>

  {#if expanded}
    <div class="section-content">
      <slot />
    </div>
  {/if}
</div>

<style>
  .section {
    border-bottom: 1px solid var(--border);
  }

  .section-header {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background: var(--bg-tertiary);
    border: none;
    cursor: pointer;
    text-align: left;
  }

  .section-header:hover {
    background: var(--bg-secondary);
  }

  .chevron {
    font-size: 0.625rem;
    color: var(--text-secondary);
    width: 1rem;
  }

  .section-header h3 {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin: 0;
  }

  .section-content {
    padding: 0.5rem 0;
  }

  .section.collapsed .section-header {
    background: transparent;
  }
</style>

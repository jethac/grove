<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let label: string;
  export let value: string;
  export let options: Array<{ value: string; label: string }>;

  const dispatch = createEventDispatcher<{ change: string }>();

  function handleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    dispatch('change', target.value);
  }
</script>

<div class="select-input">
  <label for={label}>{label}</label>
  <select id={label} {value} on:change={handleChange}>
    {#each options as option}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
</div>

<style>
  .select-input {
    padding: 0.375rem 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  label {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  select {
    width: 100%;
    padding: 0.375rem 0.5rem;
    font-size: 0.8125rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    cursor: pointer;
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%23888' d='M6 8L2 4h8z'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 0.5rem center;
    padding-right: 1.5rem;
  }

  select:focus {
    outline: none;
    border-color: var(--accent);
  }

  select:hover {
    border-color: var(--text-secondary);
  }

  option {
    background: var(--bg-secondary);
    color: var(--text-primary);
  }
</style>

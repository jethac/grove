<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let label: string;
  export let value: number;
  export let min: number = 0;
  export let max: number = 100;
  export let step: number = 1;
  export let showSlider: boolean = true;

  const dispatch = createEventDispatcher<{ change: number }>();

  let internalValue = value;

  $: internalValue = value;

  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    const newValue = parseFloat(target.value);
    if (!isNaN(newValue)) {
      internalValue = Math.min(max, Math.max(min, newValue));
      dispatch('change', internalValue);
    }
  }

  function handleSlider(event: Event) {
    const target = event.target as HTMLInputElement;
    internalValue = parseFloat(target.value);
    dispatch('change', internalValue);
  }

  // Format display value based on step precision
  function formatValue(val: number): string {
    if (step >= 1) {
      return Math.round(val).toString();
    }
    const decimals = Math.max(0, -Math.floor(Math.log10(step)));
    return val.toFixed(decimals);
  }
</script>

<div class="number-input">
  <div class="header">
    <label for={label}>{label}</label>
    <input
      id={label}
      type="number"
      {min}
      {max}
      {step}
      value={formatValue(internalValue)}
      on:input={handleInput}
      on:change={handleInput}
    />
  </div>

  {#if showSlider}
    <input
      type="range"
      {min}
      {max}
      {step}
      bind:value={internalValue}
      on:input={handleSlider}
      class="slider"
    />
  {/if}
</div>

<style>
  .number-input {
    padding: 0.375rem 1rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.25rem;
  }

  label {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  input[type="number"] {
    width: 70px;
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    text-align: right;
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
  }

  input[type="number"]:focus {
    outline: none;
    border-color: var(--accent);
  }

  .slider {
    width: 100%;
    height: 4px;
    -webkit-appearance: none;
    appearance: none;
    background: var(--bg-tertiary);
    border-radius: 2px;
    outline: none;
    cursor: pointer;
  }

  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--accent);
    cursor: pointer;
    border: 2px solid var(--bg-primary);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
  }

  .slider::-moz-range-thumb {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--accent);
    cursor: pointer;
    border: 2px solid var(--bg-primary);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
  }

  .slider::-webkit-slider-runnable-track {
    height: 4px;
    background: linear-gradient(
      to right,
      var(--accent) 0%,
      var(--accent) calc(100% * var(--progress, 0)),
      var(--bg-tertiary) calc(100% * var(--progress, 0)),
      var(--bg-tertiary) 100%
    );
    border-radius: 2px;
  }

  .slider:hover::-webkit-slider-thumb {
    transform: scale(1.1);
  }
</style>

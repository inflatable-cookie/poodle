<script lang="ts">
  /** Label displayed centered in the region */
  export let label: string = "";

  /** Custom color for border and label text (any valid CSS color) */
  export let color: string | null = null;

  /** Minimum height of the region */
  export let minHeight: string = "4rem";
</script>

<div
  class="flint-region"
  style:--region-color={color}
  style:min-height={minHeight}
  role="presentation"
>
  {#if label}
    <span class="flint-region__label">{label}</span>
  {/if}
</div>

<style>
  .flint-region {
    display: flex;
    align-items: center;
    justify-content: center;
    border: 0.125rem dashed var(--region-color, var(--flint-color-border-default, #444));
    border-radius: var(--flint-radius-surface, 0.375rem);
    padding: var(--flint-space-inline-md, 0.75rem);
  }

  .flint-region__label {
    color: var(--region-color, var(--flint-color-text-tertiary, #888));
    font-family: var(--flint-typography-label-family, sans-serif);
    font-size: var(--flint-typography-label-size, 0.75rem);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    user-select: none;
  }
</style>

<script lang="ts">
  import { default as Icon } from "./Icon.svelte";
  import { default as TextLink } from "./TextLink.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface SelectionItem {
    id: string;
    label: string;
  }

  interface Props {
    items?: SelectionItem[];
    maxVisibleItems?: number;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onRemove?: ((id: string) => void) | null;
    onClear?: (() => void) | null;
  }

  let {
    items = [],
    maxVisibleItems = 4,
    size = null,
    sizeRole = "control",
    density = null,
    onRemove = null,
    onClear = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const visibleItems = $derived(items.slice(0, maxVisibleItems));
  const overflowCount = $derived(Math.max(0, items.length - visibleItems.length));
</script>

<section class="poodle-selection-summary" aria-label="Current selection" data-size={resolvedSize} data-density={resolvedDensity}>
  <div class="poodle-selection-summary__chips">
    {#if items.length === 0}
      <span class="poodle-selection-summary__empty">No selection</span>
    {:else}
      {#each visibleItems as item}
        <button
          type="button"
          class="poodle-selection-summary__chip"
          onclick={() => onRemove?.(item.id)}
          aria-label={`Remove ${item.label}`}
        >
          {item.label}
          <span aria-hidden="true"><Icon name="x" /></span>
        </button>
      {/each}
      {#if overflowCount > 0}
        <span class="poodle-selection-summary__overflow">+{overflowCount} more</span>
      {/if}
      <TextLink className="poodle-selection-summary__clear" onClick={() => onClear?.()}>
        Clear
      </TextLink>
    {/if}
  </div>
</section>

<style>
  .poodle-selection-summary {
    --poodle-selection-summary-gap: var(--poodle-space-inline-sm);
    --poodle-selection-summary-padding-bottom: 0.625rem;
    --poodle-selection-summary-font-size: 0.75rem;
    --poodle-selection-summary-chip-font-size: 0.75rem;
    --poodle-selection-summary-chip-padding-x: 0.75rem;
    --poodle-selection-summary-chip-min-height: 1.5rem;
    --poodle-selection-summary-chips-min-height: 1.25rem;
    --poodle-selection-summary-overflow-padding-x: 0.625rem;
    --poodle-selection-summary-overflow-font-size: 0.8125rem;
    --poodle-selection-summary-overflow-line-height: 2rem;
    --poodle-selection-summary-clear-font-size: 0.75rem;
    --poodle-selection-summary-icon-size: 0.875rem;
    font-size: var(--poodle-selection-summary-font-size);
    padding-bottom: var(--poodle-selection-summary-padding-bottom);
  }

  .poodle-selection-summary__chips {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--poodle-selection-summary-gap);
    min-height: var(--poodle-selection-summary-chips-min-height);
  }

  .poodle-selection-summary__chip {
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-control);
    background: color-mix(
      in srgb,
      var(--poodle-surface, var(--poodle-color-background-surface)) 60%,
      var(--poodle-color-background-elevated)
    );
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 70%, transparent);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
  }

  .poodle-selection-summary__empty {
    color: var(--poodle-color-text-tertiary);
    font-style: italic;
  }

  :global(.poodle-selection-summary__clear) {
    margin-left: auto;
    font-size: var(--poodle-selection-summary-clear-font-size);
  }

  .poodle-selection-summary__chip {
    display: inline-flex;
    gap: var(--poodle-selection-summary-gap);
    align-items: center;
    min-height: var(--poodle-selection-summary-chip-min-height);
    padding: 0 var(--poodle-selection-summary-chip-padding-x);
    font-size: var(--poodle-selection-summary-chip-font-size);
  }

  .poodle-selection-summary__chip :global(.poodle-icon) {
    width: var(--poodle-selection-summary-icon-size);
    height: var(--poodle-selection-summary-icon-size);
  }

  .poodle-selection-summary__overflow {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-selection-summary-overflow-font-size);
    line-height: var(--poodle-selection-summary-overflow-line-height);
    padding: 0 var(--poodle-selection-summary-overflow-padding-x);
    border-radius: var(--poodle-radius-control);
    background: color-mix(
      in srgb,
      var(--poodle-surface, var(--poodle-color-background-surface)) 68%,
      var(--poodle-color-background-elevated)
    );
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent);
  }

  .poodle-selection-summary[data-density="compact"] {
    --poodle-selection-summary-gap: 0.375rem;
    --poodle-selection-summary-padding-bottom: 0.5rem;
    --poodle-selection-summary-chip-padding-x: 0.625rem;
    --poodle-selection-summary-overflow-padding-x: 0.5rem;
  }

  .poodle-selection-summary[data-density="default"] {
    --poodle-selection-summary-gap: var(--poodle-space-inline-sm);
    --poodle-selection-summary-padding-bottom: 0.625rem;
    --poodle-selection-summary-chip-padding-x: 0.75rem;
    --poodle-selection-summary-overflow-padding-x: 0.625rem;
  }

  .poodle-selection-summary[data-density="comfortable"] {
    --poodle-selection-summary-gap: var(--poodle-space-inline-md);
    --poodle-selection-summary-padding-bottom: 0.75rem;
    --poodle-selection-summary-chip-padding-x: 0.875rem;
    --poodle-selection-summary-overflow-padding-x: 0.75rem;
  }


  /* ── Size variants ──────────────────────────────────────────── */

  .poodle-selection-summary[data-size="xs"] { font-size: 0.6875rem; }
  .poodle-selection-summary[data-size="xs"] {
    --poodle-selection-summary-font-size: 0.6875rem;
    --poodle-selection-summary-chip-font-size: 0.6875rem;
    --poodle-selection-summary-chip-min-height: 1rem;
    --poodle-selection-summary-chips-min-height: 0.875rem;
    --poodle-selection-summary-chip-padding-x: 0.5rem;
    --poodle-selection-summary-clear-font-size: 0.6875rem;
    --poodle-selection-summary-overflow-font-size: 0.6875rem;
    --poodle-selection-summary-overflow-line-height: 1.5rem;
    --poodle-selection-summary-icon-size: 0.75rem;
  }

  .poodle-selection-summary[data-size="sm"] {
    --poodle-selection-summary-font-size: 0.71875rem;
    --poodle-selection-summary-chip-font-size: 0.71875rem;
    --poodle-selection-summary-chip-min-height: 1.125rem;
    --poodle-selection-summary-chips-min-height: 1rem;
    --poodle-selection-summary-clear-font-size: 0.71875rem;
    --poodle-selection-summary-overflow-font-size: 0.75rem;
    --poodle-selection-summary-overflow-line-height: 1.625rem;
    --poodle-selection-summary-icon-size: 0.8125rem;
  }

  .poodle-selection-summary[data-size="md"] {
    --poodle-selection-summary-font-size: 0.75rem;
    --poodle-selection-summary-chip-font-size: 0.75rem;
    --poodle-selection-summary-chip-min-height: 1.5rem;
    --poodle-selection-summary-chips-min-height: 1.25rem;
    --poodle-selection-summary-chip-padding-x: 0.75rem;
    --poodle-selection-summary-clear-font-size: 0.75rem;
    --poodle-selection-summary-overflow-font-size: 0.8125rem;
    --poodle-selection-summary-overflow-line-height: 2rem;
    --poodle-selection-summary-icon-size: 0.875rem;
  }

  .poodle-selection-summary[data-size="lg"] {
    --poodle-selection-summary-font-size: 0.8125rem;
    --poodle-selection-summary-chip-font-size: 0.8125rem;
    --poodle-selection-summary-chip-min-height: 1.75rem;
    --poodle-selection-summary-chips-min-height: 1.5rem;
    --poodle-selection-summary-chip-padding-x: 0.875rem;
    --poodle-selection-summary-clear-font-size: 0.8125rem;
    --poodle-selection-summary-overflow-font-size: 0.875rem;
    --poodle-selection-summary-overflow-line-height: 2.125rem;
    --poodle-selection-summary-icon-size: 1rem;
  }

  .poodle-selection-summary[data-size="xl"] {
    --poodle-selection-summary-font-size: 0.875rem;
    --poodle-selection-summary-chip-font-size: 0.875rem;
    --poodle-selection-summary-chip-min-height: 2rem;
    --poodle-selection-summary-chips-min-height: 1.75rem;
    --poodle-selection-summary-chip-padding-x: 1rem;
    --poodle-selection-summary-clear-font-size: 0.875rem;
    --poodle-selection-summary-overflow-font-size: 0.9375rem;
    --poodle-selection-summary-overflow-line-height: 2.25rem;
    --poodle-selection-summary-icon-size: 1.125rem;
  }
</style>

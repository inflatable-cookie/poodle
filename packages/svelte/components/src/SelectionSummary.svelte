<script lang="ts">
  import Icon from "./Icon.svelte";
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
      <button type="button" class="poodle-selection-summary__clear" onclick={() => onClear?.()}>Clear</button>
    {/if}
  </div>
</section>

<style>
  .poodle-selection-summary {
    font-size: var(--poodle-typography-label-size, 0.75rem);
    padding-bottom: 0.625rem;
  }

  .poodle-selection-summary__chips {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-height: calc(var(--poodle-size-control-height) - 0.5rem);
  }

  .poodle-selection-summary__chip {
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
  }

  .poodle-selection-summary__empty {
    color: var(--poodle-color-text-tertiary);
    font-style: italic;
  }

  .poodle-selection-summary__clear {
    margin-left: auto;
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-label-size, 0.75rem);
  }

  .poodle-selection-summary__clear:hover {
    color: var(--poodle-color-text-primary);
  }

  .poodle-selection-summary__chip {
    display: inline-flex;
    gap: var(--poodle-space-inline-md);
    align-items: center;
    min-height: calc(var(--poodle-size-control-height) - 0.25rem);
    padding: 0 0.75rem;
  }

  .poodle-selection-summary__overflow {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 2rem;
    padding: 0 0.625rem;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 58%, transparent);
  }


  /* ── Size variants ──────────────────────────────────────────── */

  .poodle-selection-summary[data-size="xs"] { font-size: 0.6875rem; }
  .poodle-selection-summary[data-size="xs"] .poodle-selection-summary__chip { min-height: 1rem; padding: 0 0.5rem; font-size: 0.6875rem; }

  .poodle-selection-summary[data-size="sm"] { font-size: 0.71875rem; }
  .poodle-selection-summary[data-size="sm"] .poodle-selection-summary__chip { min-height: 1.125rem; font-size: 0.71875rem; }

  .poodle-selection-summary[data-size="lg"] { font-size: 0.8125rem; }
  .poodle-selection-summary[data-size="lg"] .poodle-selection-summary__chip { min-height: 1.75rem; padding: 0 0.875rem; font-size: 0.8125rem; }

  .poodle-selection-summary[data-size="xl"] { font-size: 0.875rem; }
  .poodle-selection-summary[data-size="xl"] .poodle-selection-summary__chip { min-height: 2rem; padding: 0 1rem; font-size: 0.875rem; }
</style>

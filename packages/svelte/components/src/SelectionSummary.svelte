<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/selection-summary.css";
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
    /** When set, each chip splits into a separate activation button (the label)
     * and a remove button — no nested buttons. When null, the whole chip removes
     * the item (backward-compatible default). */
    onActivate?: ((id: string) => void) | null;
    onRemove?: ((id: string) => void) | null;
    onClear?: (() => void) | null;
  }

  let {
    items = [],
    maxVisibleItems = 4,
    size = null,
    sizeRole = "control",
    density = null,
    onActivate = null,
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
        {#if onActivate}
          <span class="poodle-selection-summary__chip poodle-selection-summary__chip--split">
            <button
              type="button"
              class="poodle-selection-summary__chip-activate"
              onclick={() => onActivate?.(item.id)}
              aria-label={`Edit ${item.label}`}
            >
              {item.label}
            </button>
            <button
              type="button"
              class="poodle-selection-summary__chip-remove"
              onclick={() => onRemove?.(item.id)}
              aria-label={`Remove ${item.label}`}
            >
              <Icon name="x" size="xs" />
            </button>
          </span>
        {:else}
          <button
            type="button"
            class="poodle-selection-summary__chip"
            onclick={() => onRemove?.(item.id)}
            aria-label={`Remove ${item.label}`}
          >
            {item.label}
            <span aria-hidden="true"><Icon name="x" /></span>
          </button>
        {/if}
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


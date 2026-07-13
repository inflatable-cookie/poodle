<script lang="ts">
  import "@poodle/styles/selection-summary.css";
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


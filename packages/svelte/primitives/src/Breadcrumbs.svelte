<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { BreadcrumbItem, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let items: BreadcrumbItem[] = [];
  export let ariaLabel = "Breadcrumb";
  export let maxVisibleItems: number | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let size: ControlSize | null = null;
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    navigate: { value: string };
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";
  $: visibleItems =
    maxVisibleItems !== null && items.length > maxVisibleItems
      ? [items[0], { value: "__ellipsis__", label: "…", current: false }, ...items.slice(items.length - (maxVisibleItems - 1))]
      : items;

  function handleNavigate(item: BreadcrumbItem): void {
    if (item.current || item.value === "__ellipsis__") {
      return;
    }

    dispatch("navigate", { value: item.value });
  }
</script>

<nav class="breadcrumbs" aria-label={ariaLabel} data-size={resolvedSize} data-density={resolvedDensity}>
  <ol class="breadcrumbs__list">
    {#each visibleItems as item, index}
      <li class="breadcrumbs__item">
        {#if item.current || index === visibleItems.length - 1}
          <span aria-current="page">{item.label}</span>
        {:else if item.href}
          <a href={item.href}>{item.label}</a>
        {:else if item.value === "__ellipsis__"}
          <span aria-hidden="true">{item.label}</span>
        {:else}
          <button type="button" on:click={() => handleNavigate(item)}>{item.label}</button>
        {/if}
        {#if index < visibleItems.length - 1}
          <span class="breadcrumbs__separator" aria-hidden="true"><Icon name="chevron-right" /></span>
        {/if}
      </li>
    {/each}
  </ol>
</nav>

<style>
  .breadcrumbs__list {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    margin: 0;
    padding: 0;
    list-style: none;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .breadcrumbs__item {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
  }

  .breadcrumbs a,
  .breadcrumbs button {
    border: 0;
    padding: 0;
    background: transparent;
    color: inherit;
    cursor: pointer;
    font: inherit;
    text-decoration: none;
  }

  .breadcrumbs__separator {
    display: inline-flex;
    opacity: 0.4;
  }

  .breadcrumbs [aria-current="page"] {
    color: var(--poodle-color-text-primary);
  }

  /* Size variants */
  .breadcrumbs[data-size="xs"] .breadcrumbs__list {
    font-size: 0.75rem;
  }

  .breadcrumbs[data-size="sm"] .breadcrumbs__list {
    font-size: 0.8125rem;
  }

  .breadcrumbs[data-size="lg"] .breadcrumbs__list {
    font-size: 0.9375rem;
  }

  .breadcrumbs[data-size="xl"] .breadcrumbs__list {
    font-size: 1rem;
  }

  /* Density variants */
  .breadcrumbs[data-density="compact"] .breadcrumbs__list { gap: var(--poodle-space-inline-xs); }
  .breadcrumbs[data-density="compact"] .breadcrumbs__item { gap: var(--poodle-space-inline-xs); }
  .breadcrumbs[data-density="comfortable"] .breadcrumbs__list { gap: var(--poodle-space-inline-md); }
  .breadcrumbs[data-density="comfortable"] .breadcrumbs__item { gap: var(--poodle-space-inline-md); }
</style>

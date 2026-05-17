<script lang="ts">
  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { BreadcrumbItem, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  let {
    items = [],
    ariaLabel = "Breadcrumb",
    maxVisibleItems = null,
    forceLastItemCurrent = true,
    sizeRole = "chrome",
    size = null,
    density = null,
    onNavigate = undefined,
  }: {
    items?: BreadcrumbItem[];
    ariaLabel?: string;
    maxVisibleItems?: number | null;
    forceLastItemCurrent?: boolean;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    onNavigate?: ((value: string) => void) | undefined;
  } = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const visibleItems = $derived(
    maxVisibleItems !== null && items.length > maxVisibleItems
      ? [items[0], { value: "__ellipsis__", label: "…", current: false }, ...items.slice(items.length - (maxVisibleItems - 1))]
      : items,
  );

  function handleNavigate(item: BreadcrumbItem): void {
    if (item.current || item.value === "__ellipsis__") {
      return;
    }

    onNavigate?.(item.value);
  }
</script>

<nav class="poodle-breadcrumbs" aria-label={ariaLabel} data-size={resolvedSize} data-density={resolvedDensity}>
  <ol class="poodle-breadcrumbs__list">
    {#each visibleItems as item, index}
      <li class="poodle-breadcrumbs__item">
        {#if item.current || (forceLastItemCurrent && index === visibleItems.length - 1)}
          <span aria-current="page">{item.label}</span>
        {:else if item.href}
          <a href={item.href}>{item.label}</a>
        {:else if item.value === "__ellipsis__"}
          <span aria-hidden="true">{item.label}</span>
        {:else}
          <button type="button" onclick={() => handleNavigate(item)}>{item.label}</button>
        {/if}
        {#if index < visibleItems.length - 1}
          <span class="poodle-breadcrumbs__separator" aria-hidden="true"><Icon name="chevron-right" /></span>
        {/if}
      </li>
    {/each}
  </ol>
</nav>

<style>
  .poodle-breadcrumbs__list {
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

  .poodle-breadcrumbs__item {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
  }

  .poodle-breadcrumbs a,
  .poodle-breadcrumbs button {
    border: 0;
    padding: 0;
    background: transparent;
    color: inherit;
    cursor: pointer;
    font: inherit;
    text-decoration: none;
  }

  .poodle-breadcrumbs__separator {
    display: inline-flex;
    opacity: 0.4;
  }

  .poodle-breadcrumbs [aria-current="page"] {
    color: var(--poodle-color-text-primary);
  }

  /* Size variants */
  .poodle-breadcrumbs[data-size="xs"] .poodle-breadcrumbs__list {
    gap: 0.25rem;
    font-size: 0.6875rem;
  }
  .poodle-breadcrumbs[data-size="xs"] .poodle-breadcrumbs__item { gap: 0.25rem; }

  .poodle-breadcrumbs[data-size="sm"] .poodle-breadcrumbs__list {
    gap: 0.375rem;
    font-size: 0.78125rem;
  }
  .poodle-breadcrumbs[data-size="sm"] .poodle-breadcrumbs__item { gap: 0.375rem; }

  .poodle-breadcrumbs[data-size="lg"] .poodle-breadcrumbs__list {
    gap: 0.625rem;
    font-size: 1rem;
  }
  .poodle-breadcrumbs[data-size="lg"] .poodle-breadcrumbs__item { gap: 0.625rem; }

  .poodle-breadcrumbs[data-size="xl"] .poodle-breadcrumbs__list {
    gap: 0.75rem;
    font-size: 1.125rem;
  }
  .poodle-breadcrumbs[data-size="xl"] .poodle-breadcrumbs__item { gap: 0.75rem; }

  /* Density variants */
  .poodle-breadcrumbs[data-density="compact"] .poodle-breadcrumbs__list { gap: 0.25rem; }
  .poodle-breadcrumbs[data-density="compact"] .poodle-breadcrumbs__item { gap: 0.25rem; }
  .poodle-breadcrumbs[data-density="comfortable"] .poodle-breadcrumbs__list { gap: 0.75rem; }
  .poodle-breadcrumbs[data-density="comfortable"] .poodle-breadcrumbs__item { gap: 0.75rem; }
</style>

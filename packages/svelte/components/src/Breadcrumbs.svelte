<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/breadcrumbs.css";
  import { default as Icon } from "./Icon.svelte";
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

<!-- Icon and label live inside the same anchor, button, or current span: one
     navigation target, one accessible name. An icon-only item hides the label
     visually instead of dropping it, so it is still announced. -->
{#snippet itemContent(item: BreadcrumbItem)}
  {#if item.icon}
    <span class="poodle-breadcrumbs__content">
      <Icon icon={item.icon} size={resolvedSize} />
      <span class="poodle-breadcrumbs__label" class:poodle-breadcrumbs__label--hidden={item.iconOnly}>{item.label}</span>
    </span>
  {:else}
    {item.label}
  {/if}
{/snippet}

<nav class="poodle-breadcrumbs" aria-label={ariaLabel} data-size={resolvedSize} data-density={resolvedDensity}>
  <ol class="poodle-breadcrumbs__list">
    {#each visibleItems as item, index}
      <li class="poodle-breadcrumbs__item">
        {#if item.current || (forceLastItemCurrent && index === visibleItems.length - 1)}
          <span aria-current="page">{@render itemContent(item)}</span>
        {:else if item.href}
          <a href={item.href}>{@render itemContent(item)}</a>
        {:else if item.value === "__ellipsis__"}
          <span aria-hidden="true">{item.label}</span>
        {:else}
          <button type="button" onclick={() => handleNavigate(item)}>{@render itemContent(item)}</button>
        {/if}
        {#if index < visibleItems.length - 1}
          <span class="poodle-breadcrumbs__separator" aria-hidden="true"><Icon name="chevron-right" /></span>
        {/if}
      </li>
    {/each}
  </ol>
</nav>

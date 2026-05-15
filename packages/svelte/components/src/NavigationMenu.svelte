<script module lang="ts">
  let nextNavigationMenuId = 0;
</script>

<script lang="ts">
  import { onMount, type Snippet } from "svelte";

  import { findNextEnabledIndex, firstEnabledIndex } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, NavigationMenuItem, SemanticControlSizeRole } from "./types";

  interface Props {
    value?: string | null;
    defaultValue?: string | null;
    items?: NavigationMenuItem[];
    ariaLabel?: string | null;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    onValueChange?: ((value: string | null) => void) | undefined;
    children?: Snippet<[string | null, NavigationMenuItem | null]>;
  }

  let {
    value = $bindable<string | null>(null),
    defaultValue = null,
    items = [],
    ariaLabel = null,
    sizeRole = "chrome",
    size = null,
    density = null,
    onValueChange = undefined,
    children,
  }: Props = $props();

  const menuId = ++nextNavigationMenuId;
  const uiPresentation = getUiPresentation();

  let rootElement = $state<HTMLDivElement | null>(null);
  let triggerElements = $state<Array<HTMLButtonElement | null>>([]);
  let uncontrolledValue = $state<string | null>(null);
  let focusIndex = $state(0);

  $effect.pre(() => {
    if (uncontrolledValue === null) {
      uncontrolledValue = defaultValue;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(value !== null);
  const currentValue = $derived(isControlled ? value : uncontrolledValue);
  const currentItem = $derived(items.find((item) => item.value === currentValue) ?? null);
  const selectedIndex = $derived(items.findIndex((item) => item.value === currentValue));

  $effect(() => {
    if (selectedIndex >= 0) {
      focusIndex = selectedIndex;
    } else if (firstEnabledIndex(items) >= 0 && focusIndex === 0) {
      focusIndex = firstEnabledIndex(items);
    }
  });

  function setValue(nextValue: string | null): void {
    if (!isControlled) {
      uncontrolledValue = nextValue;
    } else {
      value = nextValue;
    }

    onValueChange?.(nextValue);
  }

  function toggleValue(nextValue: string): void {
    setValue(currentValue === nextValue ? null : nextValue);
  }

  function moveFocus(nextIndex: number): void {
    focusIndex = nextIndex;
    triggerElements[nextIndex]?.focus();
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    if (event.key === "ArrowRight") {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(items, index, 1));
    }

    if (event.key === "ArrowLeft") {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(items, index, -1));
    }

    if (event.key === "Home") {
      event.preventDefault();
      moveFocus(firstEnabledIndex(items));
    }

    if (event.key === "End") {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(items, 0, -1));
    }

    if (event.key === "ArrowDown" || event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      const nextValue = items[index]?.value;

      if (nextValue) {
        setValue(nextValue);
      }
    }

    if (event.key === "Escape") {
      event.preventDefault();
      setValue(null);
    }
  }

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!currentValue || !rootElement) {
        return;
      }

      if (!rootElement.contains(event.target as Node)) {
        setValue(null);
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && currentValue) {
        event.preventDefault();
        setValue(null);
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeydown);
    };
  });
</script>

<div class="poodle-navigation-menu" bind:this={rootElement} data-size={resolvedSize} data-density={resolvedDensity}>
  <nav
    class="poodle-navigation-menu__list"
    aria-label={ariaLabel ?? undefined}
  >
    {#each items as item, index (item.value)}
      <button
        bind:this={triggerElements[index]}
        type="button"
        class="poodle-navigation-menu__trigger"
        data-open={currentValue === item.value}
        disabled={item.disabled === true}
        tabindex={index === focusIndex ? 0 : -1}
        id={`poodle-navigation-menu-trigger-${menuId}-${item.value}`}
        aria-expanded={currentValue === item.value ? "true" : "false"}
        aria-controls={currentValue === item.value ? `poodle-navigation-menu-panel-${menuId}-${item.value}` : undefined}
        onfocus={() => (focusIndex = index)}
        onclick={() => toggleValue(item.value)}
        onkeydown={(event) => handleKeydown(event, index)}
      >
        <span class="poodle-navigation-menu__label">{item.label}</span>
      </button>
    {/each}
  </nav>

  {#if currentItem}
    <div
      class="poodle-navigation-menu__viewport"
      id={`poodle-navigation-menu-panel-${menuId}-${currentItem.value}`}
      aria-labelledby={`poodle-navigation-menu-trigger-${menuId}-${currentItem.value}`}
    >
      {@render children?.(currentItem.value, currentItem)}
    </div>
  {/if}
</div>

<style>
  .poodle-navigation-menu {
    display: grid;
    gap: var(--poodle-space-stack-md);
    min-width: 0;
  }

  .poodle-navigation-menu__list {
    display: inline-flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    align-items: center;
  }

  .poodle-navigation-menu__trigger {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 88%, transparent);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
  }

  .poodle-navigation-menu__trigger[data-open="true"] {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 42%, var(--poodle-color-border-default));
  }

  .poodle-navigation-menu__trigger:hover:not(:disabled),
  .poodle-navigation-menu__trigger:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
    outline: none;
  }

  .poodle-navigation-menu__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-navigation-menu__viewport {
    min-width: 0;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 96%, transparent);
    box-shadow: var(--poodle-elevation-overlay);
  }

  /* Size variants */
  .poodle-navigation-menu[data-size="xs"] .poodle-navigation-menu__trigger {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.6875rem;
  }

  .poodle-navigation-menu[data-size="sm"] .poodle-navigation-menu__trigger {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
  }

  .poodle-navigation-menu[data-size="lg"] .poodle-navigation-menu__trigger {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.8125rem;
  }

  .poodle-navigation-menu[data-size="xl"] .poodle-navigation-menu__trigger {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.875rem;
  }

  /* Density variants */
  .poodle-navigation-menu[data-density="compact"] .poodle-navigation-menu__trigger { padding-inline: 0.5rem; }
  .poodle-navigation-menu[data-density="comfortable"] .poodle-navigation-menu__trigger { padding-inline: 0.75rem; }
</style>

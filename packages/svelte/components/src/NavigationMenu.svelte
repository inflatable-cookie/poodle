<script module lang="ts">
  let nextNavigationMenuId = 0;
</script>

<script lang="ts">
  import "@poodle/styles/navigation-menu.css";
  import { registerDismissLayer } from "@poodle/headless";
  import type { Snippet } from "svelte";

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
  let seededDefaultValue = $state(false);
  let focusIndex = $state(0);

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue;
      seededDefaultValue = true;
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
      if (focusIndex !== selectedIndex) {
        focusIndex = selectedIndex;
      }
    } else {
      const nextEnabledIndex = firstEnabledIndex(items);
      if (nextEnabledIndex >= 0 && focusIndex !== nextEnabledIndex) {
        focusIndex = nextEnabledIndex;
      }
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

  $effect(() => {
    if (!currentValue) {
      return;
    }

    return registerDismissLayer({
      contains: (target) => rootElement?.contains(target) ?? false,
      dismissOnOutsideInteract: true,
      onDismiss: () => setValue(null),
    });
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


<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";

  import type { CollapseDirection } from "./types";

  export let isCollapsed = false;
  export let direction: CollapseDirection = "left";
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    toggle: { isCollapsed: boolean };
  }>();

  $: expandDirection = (
    { left: "right", right: "left", up: "down", down: "up" } as const
  )[direction];

  $: iconName = (() => {
    const dir = isCollapsed ? expandDirection : direction;
    return `chevron-${dir}` as const;
  })();

  $: label = ariaLabel ?? (isCollapsed ? "Expand" : "Collapse");

  function handleClick(): void {
    if (isDisabled) return;
    dispatch("toggle", { isCollapsed: !isCollapsed });
  }
</script>

<button
  type="button"
  class="collapse-toggle"
  data-collapsed={isCollapsed || undefined}
  data-direction={direction}
  disabled={isDisabled}
  aria-expanded={!isCollapsed}
  aria-label={label}
  on:click={handleClick}
>
  <Icon name={iconName} size="sm" />
</button>

<style>
  .collapse-toggle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.125rem;
    border: 0;
    border-radius: var(--pug-radius-sm, 0.25rem);
    background: transparent;
    color: var(--pug-color-text-muted);
    cursor: pointer;
    line-height: 1;
  }

  .collapse-toggle:hover {
    background: var(--pug-color-surface-hover);
    color: var(--pug-color-text-default);
  }

  .collapse-toggle:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .collapse-toggle:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }
</style>

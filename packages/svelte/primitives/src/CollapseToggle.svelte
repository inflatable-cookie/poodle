<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";

  import type { CollapseDirection } from "./types";

  export let collapsed = false;
  export let direction: CollapseDirection = "left";
  export let disabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    toggle: { isCollapsed: boolean };
  }>();

  $: expandDirection = (
    { left: "right", right: "left", up: "down", down: "up" } as const
  )[direction];

  $: iconName = (() => {
    const dir = collapsed ? expandDirection : direction;
    return `chevron-${dir}` as const;
  })();

  $: label = ariaLabel ?? (collapsed ? "Expand" : "Collapse");

  function handleClick(): void {
    if (disabled) return;
    dispatch("toggle", { isCollapsed: !collapsed });
  }
</script>

<button
  type="button"
  class="collapse-toggle"
  data-collapsed={collapsed || undefined}
  data-direction={direction}
  disabled={disabled}
  aria-expanded={!collapsed}
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
    border-radius: var(--poodle-radius-sm, 0.25rem);
    background: transparent;
    color: var(--poodle-color-text-muted);
    cursor: pointer;
    line-height: 1;
  }

  .collapse-toggle:hover {
    background: var(--poodle-color-surface-hover);
    color: var(--poodle-color-text-default);
  }

  .collapse-toggle:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .collapse-toggle:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }
</style>

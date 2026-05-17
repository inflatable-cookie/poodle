<script lang="ts">
  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { CollapseDirection, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    collapsed?: boolean;
    direction?: CollapseDirection;
    disabled?: boolean;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onToggle?: ((isCollapsed: boolean) => void) | null;
  }

  let {
    collapsed = false,
    direction = "left",
    disabled = false,
    ariaLabel = null,
    size = null,
    sizeRole = "chrome",
    density = null,
    onToggle = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const expandDirection = $derived((
    { left: "right", right: "left", up: "down", down: "up" } as const
  )[direction]);

  const iconName = $derived.by(() => {
    const dir = collapsed ? expandDirection : direction;
    return `chevron-${dir}` as const;
  });

  const label = $derived(ariaLabel ?? (collapsed ? "Expand" : "Collapse"));

  function handleClick(): void {
    if (disabled) return;
    onToggle?.(!collapsed);
  }
</script>

<button
  type="button"
  class="poodle-collapse-toggle"
  data-collapsed={collapsed || undefined}
  data-direction={direction}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  disabled={disabled}
  aria-expanded={!collapsed}
  aria-label={label}
  onclick={handleClick}
>
  <Icon name={iconName} size={resolvedSize} />
</button>

<style>
  .poodle-collapse-toggle {
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

  .poodle-collapse-toggle:hover {
    background: var(--poodle-color-surface-hover);
    color: var(--poodle-color-text-default);
  }

  .poodle-collapse-toggle:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .poodle-collapse-toggle:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  /* Size variants */
  .poodle-collapse-toggle[data-size="xs"] {
    padding: 0.0625rem;
  }

  .poodle-collapse-toggle[data-size="lg"] {
    padding: 0.1875rem;
  }

  .poodle-collapse-toggle[data-size="xl"] {
    padding: 0.25rem;
  }

  /* Density variants */
  .poodle-collapse-toggle[data-density="compact"] { padding-inline: 0.125rem; }
  .poodle-collapse-toggle[data-density="comfortable"] { padding-inline: 0.375rem; }
</style>

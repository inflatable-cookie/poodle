<script lang="ts">
  import "@poodle/styles/collapse-toggle.css";
  import { default as Icon } from "./Icon.svelte";
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


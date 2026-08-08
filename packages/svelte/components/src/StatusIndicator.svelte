<script lang="ts">
  import "@inflatable-cookie/poodle-styles/status-indicator.css";
  import type { Snippet } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone } from "./types";

  type StatusIndicatorTypography = "label" | "inherit";

  let {
    status = "neutral",
    label = null,
    ariaLabel = null,
    size = null,
    sizeRole = "control",
    density = null,
    typography = "label",
    children = undefined,
  }: {
    status?: StatusTone;
    label?: string | null;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    typography?: StatusIndicatorTypography;
    children?: Snippet;
  } = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
</script>

<span
  class="poodle-status-indicator"
  data-status={status}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-typography={typography}
  aria-label={ariaLabel ?? undefined}
>
  <span class="poodle-status-indicator__dot" aria-hidden="true"></span>
  {#if label}
    <span class="poodle-status-indicator__label">{label}</span>
  {:else}
    {@render children?.()}
  {/if}
</span>


<script lang="ts">
  import "@poodle/styles/meter.css";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlSize, SemanticControlSizeRole } from "./types";

  let {
    value = 0,
    min = 0,
    max = 100,
    low = null,
    high = null,
    optimum = null,
    ariaLabel = null,
    shape = "linear",
    tone = "success",
    showValue = false,
    valueText = null,
    size = null,
    sizeRole = "control",
  }: {
    value?: number;
    min?: number;
    max?: number;
    low?: number | null;
    high?: number | null;
    optimum?: number | null;
    ariaLabel?: string | null;
    shape?: "linear" | "ring";
    tone?: "success" | "accent" | "warning" | "danger" | "neutral";
    showValue?: boolean;
    valueText?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
  } = $props();

  const uiPresentation = getUiPresentation();
  const safeMax = $derived(max <= min ? min + 1 : max);
  const safeValue = $derived(Math.min(Math.max(value, min), safeMax));
  const percentage = $derived(((safeValue - min) / (safeMax - min)) * 100);
  // `high` wins over `low`, and drives the warning fill override in CSS.
  const level = $derived(
    high !== null && safeValue >= high ? "high" : low !== null && safeValue <= low ? "low" : "normal"
  );
  const displayText = $derived(valueText ?? `${Math.round(percentage)}%`);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
</script>

<div
  class="poodle-meter"
  aria-label={ariaLabel ?? undefined}
  data-size={resolvedSize}
  data-shape={shape}
  data-tone={tone}
  data-level={level}
  style={shape === "ring" ? `--poodle-meter-percentage: ${percentage};` : undefined}
>
  <meter
    class="poodle-meter__native"
    min={min}
    max={safeMax}
    {low}
    {high}
    {optimum}
    value={safeValue}
  ></meter>
  <span class="poodle-meter__track" aria-hidden="true">
    <span
      class="poodle-meter__fill"
      style={shape === "ring" ? undefined : `width: ${percentage}%;`}
    ></span>
  </span>
  {#if showValue}
    <span class="poodle-meter__value" aria-hidden="true">{displayText}</span>
  {/if}
</div>

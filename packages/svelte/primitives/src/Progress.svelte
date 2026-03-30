<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlSize, SemanticControlSizeRole } from "./types";

  export let value: number | null = null;
  export let max = 100;
  export let indeterminate = false;
  export let ariaLabel: string | null = null;
  export let valueText: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";

  const uiPresentation = getUiPresentation();

  $: safeMax = max <= 0 ? 100 : max;
  $: safeValue = value === null ? null : Math.min(Math.max(value, 0), safeMax);
  $: percentage = safeValue === null ? 0 : safeValue / safeMax;
  $: computedValueText =
    !indeterminate && safeValue !== null ? `${Math.round(percentage * 100)}%` : null;
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
</script>

<div
  class="progress"
  data-size={resolvedSize}
  data-indeterminate={indeterminate}
  role="progressbar"
  aria-label={ariaLabel ?? undefined}
  aria-valuemin={indeterminate ? undefined : 0}
  aria-valuemax={indeterminate ? undefined : safeMax}
  aria-valuenow={indeterminate || safeValue === null ? undefined : safeValue}
  aria-valuetext={valueText ?? computedValueText ?? undefined}
>
  <span
    class="progress__indicator"
    style={indeterminate ? undefined : `transform: scaleX(${percentage});`}
  ></span>
</div>

<style>
  .progress {
    position: relative;
    overflow: hidden;
    width: 100%;
    min-height: 0.5rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--poodle-surface) 96%, var(--poodle-color-text-primary));
  }

  .progress[data-size="xs"],
  .progress[data-size="sm"] {
    min-height: 0.375rem;
  }

  .progress[data-size="lg"],
  .progress[data-size="xl"] {
    min-height: 0.75rem;
  }

  .progress__indicator {
    position: absolute;
    inset: 0;
    transform-origin: left center;
    border-radius: inherit;
    background: linear-gradient(
      90deg,
      color-mix(in srgb, var(--poodle-color-accent-base) 88%, white),
      var(--poodle-color-accent-base)
    );
    transition: transform var(--poodle-motion-duration-standard) var(--poodle-motion-easing-standard);
  }

  .progress[data-indeterminate="true"] .progress__indicator {
    width: 40%;
    transform: translateX(-100%);
    animation: progress-indeterminate 1.2s ease-in-out infinite;
  }

  @keyframes progress-indeterminate {
    to {
      transform: translateX(250%);
    }
  }
</style>

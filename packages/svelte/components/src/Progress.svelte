<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    value?: number | null;
    max?: number;
    indeterminate?: boolean;
    ariaLabel?: string | null;
    valueText?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
  }

  let {
    value = null,
    max = 100,
    indeterminate = false,
    ariaLabel = null,
    valueText = null,
    size = null,
    sizeRole = "control",
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const safeMax = $derived(max <= 0 ? 100 : max);
  const safeValue = $derived(value === null ? null : Math.min(Math.max(value, 0), safeMax));
  const percentage = $derived(safeValue === null ? 0 : safeValue / safeMax);
  const computedValueText = $derived(
    !indeterminate && safeValue !== null ? `${Math.round(percentage * 100)}%` : null
  );
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
</script>

<div
  class="poodle-progress"
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
    class="poodle-progress__indicator"
    style={indeterminate ? undefined : `transform: scaleX(${percentage});`}
  ></span>
</div>

<style>
  .poodle-progress {
    position: relative;
    overflow: hidden;
    width: 100%;
    min-height: 0.5rem;
    border-radius: 999px;
    background: var(--poodle-recipe-progress-fill, color-mix(in srgb, var(--poodle-surface) 96%, var(--poodle-color-text-primary)));
  }

  .poodle-progress[data-size="xs"],
  .poodle-progress[data-size="sm"] {
    min-height: 0.375rem;
  }

  .poodle-progress[data-size="lg"],
  .poodle-progress[data-size="xl"] {
    min-height: 0.75rem;
  }

  .poodle-progress__indicator {
    position: absolute;
    inset: 0;
    transform-origin: left center;
    border-radius: inherit;
    background: var(--poodle-recipe-progress-indicator-fill, linear-gradient(
      90deg,
      color-mix(in srgb, var(--poodle-color-accent-base) 88%, white),
      var(--poodle-color-accent-base)
    ));
    transition: transform var(--poodle-motion-duration-standard) var(--poodle-motion-easing-standard);
  }

  .poodle-progress[data-indeterminate="true"] .poodle-progress__indicator {
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

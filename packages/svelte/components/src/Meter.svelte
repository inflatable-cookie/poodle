<script lang="ts">
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
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
  } = $props();

  const uiPresentation = getUiPresentation();
  const safeMax = $derived(max <= min ? min + 1 : max);
  const safeValue = $derived(Math.min(Math.max(value, min), safeMax));
  const percentage = $derived(((safeValue - min) / (safeMax - min)) * 100);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
</script>

<div class="poodle-meter" aria-label={ariaLabel ?? undefined} data-size={resolvedSize}>
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
    <span class="poodle-meter__fill" style={`width: ${percentage}%;`}></span>
  </span>
</div>

<style>
  .poodle-meter {
    --poodle-meter-track-thickness: 0.5rem;
    display: grid;
    gap: 0;
    width: 100%;
  }

  .poodle-meter__native {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }

  .poodle-meter__track {
    position: relative;
    display: block;
    overflow: hidden;
    min-height: var(--poodle-meter-track-thickness);
    border-radius: 999px;
    background: var(--poodle-recipe-meter-track-fill, color-mix(in srgb, var(--poodle-surface) 96%, var(--poodle-color-text-primary)));
  }

  .poodle-meter__fill {
    display: block;
    height: 100%;
    border-radius: inherit;
    background: var(--poodle-recipe-meter-fill-fill, linear-gradient(
      90deg,
      color-mix(in srgb, var(--poodle-color-status-success) 82%, white),
      var(--poodle-color-status-success)
    ));
  }

  .poodle-meter[data-size="xs"] {
    --poodle-meter-track-thickness: 0.25rem;
  }

  .poodle-meter[data-size="sm"] {
    --poodle-meter-track-thickness: 0.375rem;
  }

  .poodle-meter[data-size="md"] {
    --poodle-meter-track-thickness: 0.5rem;
  }

  .poodle-meter[data-size="lg"] {
    --poodle-meter-track-thickness: 0.625rem;
  }

  .poodle-meter[data-size="xl"] {
    --poodle-meter-track-thickness: 0.75rem;
  }
</style>

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


<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/progress.css";
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


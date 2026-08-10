<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/gain-reduction-meter.css";
  import {
    createGainReductionMeterContext, formatAudioValue, gainReductionMeterTransition,
    gainReductionMeterVisualState, type GainReductionFrame, type GainReductionMeterContext,
  } from "@inflatable-cookie/poodle-core";
  import GainReductionMeterVisual from "./audio/GainReductionMeterVisual.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    context?: GainReductionMeterContext;
    style?: "bar" | "segments";
    orientation?: "horizontal" | "vertical";
    segments?: number;
    ariaLabel?: string | null;
  }

  let {
    size = null, sizeRole = "control", density = null,
    context = $bindable(createGainReductionMeterContext()),
    style = "segments", orientation = "vertical", segments = 20, ariaLabel = "Gain reduction",
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  const visualState = $derived(gainReductionMeterVisualState(context));
  const valueText = $derived(`${formatAudioValue(context.ballisticDb, { type: "db", decimals: 1 })} reduction`);

  export function push(frame: GainReductionFrame): void {
    context = gainReductionMeterTransition(context, { type: "PUSH_FRAME", frame }).context;
  }

  export function reset(): void {
    context = gainReductionMeterTransition(context, { type: "RESET" }).context;
  }
</script>

<div
  class="poodle-gain-reduction-meter"
  role="meter"
  aria-label={ariaLabel ?? "Gain reduction"}
  aria-valuemin="0"
  aria-valuemax={context.maxReductionDb}
  aria-valuenow={context.ballisticDb}
  aria-valuetext={valueText}
  data-scope="gain-reduction-meter"
  data-part="root"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-orientation={orientation}
>
  <GainReductionMeterVisual {visualState} {style} {orientation} {segments} />
</div>

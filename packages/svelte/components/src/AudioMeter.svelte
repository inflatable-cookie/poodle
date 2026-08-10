<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/audio-meter.css";
  import {
    audioMeterTransition, audioMeterVisualState, createAudioMeterContext,
    formatAudioValue, type AudioMeterContext, type MeterFeedFrame,
  } from "@inflatable-cookie/poodle-core";
  import AudioMeterVisual from "./audio/AudioMeterVisual.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    context?: AudioMeterContext;
    rightContext?: AudioMeterContext | null;
    style?: "bar" | "segments";
    orientation?: "horizontal" | "vertical";
    segments?: number;
    ariaLabel?: string | null;
  }

  let {
    size = null, sizeRole = "control", density = null,
    context = $bindable(createAudioMeterContext()),
    rightContext = $bindable(null),
    style = "segments", orientation = "vertical", segments = 20,
    ariaLabel = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  const leftVisual = $derived(audioMeterVisualState(context));
  const rightVisual = $derived(rightContext === null ? null : audioMeterVisualState(rightContext));
  const leftValueText = $derived(formatAudioValue(context.ballisticDb, { type: "db", decimals: 1 }));
  const valueText = $derived(rightContext === null
    ? leftValueText
    : `Left ${leftValueText}, right ${formatAudioValue(rightContext.ballisticDb, { type: "db", decimals: 1 })}`);

  export function push(frame: MeterFeedFrame, channel: "left" | "right" = "left"): void {
    if (channel === "right") {
      if (rightContext !== null) rightContext = audioMeterTransition(rightContext, { type: "PUSH_FRAME", frame }).context;
      return;
    }
    context = audioMeterTransition(context, { type: "PUSH_FRAME", frame }).context;
  }

  export function resetClip(channel: "left" | "right" | "both" = "both"): void {
    if (channel !== "right") context = audioMeterTransition(context, { type: "RESET_CLIP" }).context;
    if (channel !== "left" && rightContext !== null) rightContext = audioMeterTransition(rightContext, { type: "RESET_CLIP" }).context;
  }
</script>

<div
  class="poodle-audio-meter"
  role="meter"
  aria-label={ariaLabel ?? undefined}
  aria-valuemin={context.minDb}
  aria-valuemax={context.maxDb}
  aria-valuenow={context.ballisticDb}
  aria-valuetext={valueText}
  data-scope="audio-meter"
  data-part="root"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-orientation={orientation}
  data-channels={rightVisual === null ? "mono" : "stereo"}
>
  <AudioMeterVisual visualState={leftVisual} {style} {orientation} {segments} />
  {#if rightVisual !== null}
    <AudioMeterVisual visualState={rightVisual} {style} {orientation} {segments} />
  {/if}
</div>

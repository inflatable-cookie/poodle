<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/audio-meter.css";
  import { onDestroy, untrack } from "svelte";
  import {
    audioMeterTransition, audioMeterVisualState, createAudioMeterContext,
    formatAudioValue, type AudioMeterContext, type MeterBus, type MeterBusChannelId,
    type MeterFeedFrame, type MeterPlaceholderHandle,
  } from "@inflatable-cookie/poodle-core";
  import AudioMeterVisual from "./audio/AudioMeterVisual.svelte";
  import { getMeterSurfaceRegistry } from "./meter-surface-context";
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
    surface?: MeterBus | null;
    channel?: MeterBusChannelId | null;
    rightChannel?: MeterBusChannelId | null;
  }

  let {
    size = null, sizeRole = "control", density = null,
    context = $bindable(createAudioMeterContext()),
    rightContext = $bindable(null),
    style = "segments", orientation = "vertical", segments = 20,
    ariaLabel = null,
    surface = null, channel = null, rightChannel = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  // Surface tier: the registered bus is state authority and the root stays a
  // layout/accessibility box painted by the enclosing MeterSurface canvas.
  const registry = getMeterSurfaceRegistry();
  untrack(() => {
    if (surface === null) return;
    if (channel === null) throw new Error("AudioMeter: surface mode requires a registered `channel` id");
    if (registry === null) throw new Error("AudioMeter: surface mode requires an enclosing MeterSurface");
    if (registry.bus !== surface) throw new Error("AudioMeter: `surface` must be the bus of the enclosing MeterSurface");
    surface.slotOf(channel);
    if (rightChannel !== null) surface.slotOf(rightChannel);
  });

  const surfaceStereo = $derived(surface !== null && rightChannel !== null);
  let meterEl: HTMLDivElement | undefined = $state();
  let placeholder: MeterPlaceholderHandle | null = null;
  let ariaMin = $state(-60);
  let ariaMax = $state(0);
  let ariaNow = $state(-60);
  let ariaText = $state(formatAudioValue(-60, { type: "db", decimals: 1 }));
  const feedScratch = new Float32Array(3);

  function sampleSurfaceAria(): void {
    if (surface === null || channel === null || surface.destroyed) return;
    const view = surface.view;
    const leftSlot = surface.slotOf(channel);
    const leftDb = view.ballisticDb[leftSlot]!;
    const leftText = formatAudioValue(leftDb, { type: "db", decimals: 1 });
    ariaMin = view.minDb[leftSlot]!;
    ariaMax = view.maxDb[leftSlot]!;
    ariaNow = leftDb;
    ariaText = rightChannel === null
      ? leftText
      : `Left ${leftText}, right ${formatAudioValue(view.ballisticDb[surface.slotOf(rightChannel)]!, { type: "db", decimals: 1 })}`;
  }

  $effect(() => {
    if (surface === null || channel === null || registry === null || meterEl === undefined) return;
    const spec = {
      slot: surface.slotOf(channel),
      rightSlot: rightChannel === null ? null : surface.slotOf(rightChannel),
      style, orientation, segments,
    };
    if (placeholder === null) {
      placeholder = registry.registerMeter(meterEl, spec, sampleSurfaceAria);
      sampleSurfaceAria();
    } else {
      placeholder.update(spec);
    }
  });

  onDestroy(() => {
    placeholder?.detach();
    placeholder = null;
  });

  const leftVisual = $derived(audioMeterVisualState(context));
  const rightVisual = $derived(rightContext === null ? null : audioMeterVisualState(rightContext));
  const leftValueText = $derived(formatAudioValue(context.ballisticDb, { type: "db", decimals: 1 }));
  const valueText = $derived(rightContext === null
    ? leftValueText
    : `Left ${leftValueText}, right ${formatAudioValue(rightContext.ballisticDb, { type: "db", decimals: 1 })}`);

  export function push(frame: MeterFeedFrame, channelSide: "left" | "right" = "left"): void {
    if (surface !== null) {
      const id = channelSide === "right" ? rightChannel : channel;
      if (id === null) return;
      feedScratch[0] = surface.slotOf(id);
      feedScratch[1] = frame.peak;
      feedScratch[2] = frame.meanSquare;
      surface.pushFrames(feedScratch, frame.atMs, frame.durationMs);
      return;
    }
    if (channelSide === "right") {
      if (rightContext !== null) rightContext = audioMeterTransition(rightContext, { type: "PUSH_FRAME", frame }).context;
      return;
    }
    context = audioMeterTransition(context, { type: "PUSH_FRAME", frame }).context;
  }

  export function resetClip(channelSide: "left" | "right" | "both" = "both"): void {
    if (surface !== null) {
      if (channelSide !== "right" && channel !== null) surface.resetClip(channel);
      if (channelSide !== "left" && rightChannel !== null) surface.resetClip(rightChannel);
      return;
    }
    if (channelSide !== "right") context = audioMeterTransition(context, { type: "RESET_CLIP" }).context;
    if (channelSide !== "left" && rightContext !== null) rightContext = audioMeterTransition(rightContext, { type: "RESET_CLIP" }).context;
  }
</script>

<div
  bind:this={meterEl}
  class="poodle-audio-meter"
  role="meter"
  aria-label={ariaLabel ?? undefined}
  aria-valuemin={surface === null ? context.minDb : ariaMin}
  aria-valuemax={surface === null ? context.maxDb : ariaMax}
  aria-valuenow={surface === null ? context.ballisticDb : ariaNow}
  aria-valuetext={surface === null ? valueText : ariaText}
  data-scope="audio-meter"
  data-part="root"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-orientation={orientation}
  data-surface={surface === null ? undefined : "true"}
  data-channels={(surface === null ? rightVisual !== null : surfaceStereo) ? "stereo" : "mono"}
>
  {#if surface === null}
    <AudioMeterVisual visualState={leftVisual} {style} {orientation} {segments} />
    {#if rightVisual !== null}
      <AudioMeterVisual visualState={rightVisual} {style} {orientation} {segments} />
    {/if}
  {/if}
</div>

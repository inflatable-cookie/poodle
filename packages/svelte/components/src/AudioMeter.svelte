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

  function validateSurfaceOwnership(
    bus: MeterBus | null,
    left: MeterBusChannelId | null,
    right: MeterBusChannelId | null,
  ): void {
    if (bus === null) return;
    if (left === null) throw new Error("AudioMeter: surface mode requires a registered `channel` id");
    if (registry === null) throw new Error("AudioMeter: surface mode requires an enclosing MeterSurface");
    if (registry.bus !== bus) throw new Error("AudioMeter: `surface` must be the bus of the enclosing MeterSurface");
    bus.slotOf(left);
    if (right !== null) bus.slotOf(right);
  }

  // Validated at init so a bad initial configuration fails where the caller
  // can see it, and again on every ownership change so a later transition
  // into surface mode is held to the same rule.
  untrack(() => validateSurfaceOwnership(surface, channel, rightChannel));

  const surfaceStereo = $derived(surface !== null && rightChannel !== null);
  let meterEl: HTMLDivElement | undefined = $state();
  let placeholder: MeterPlaceholderHandle | null = null;
  // Slots are resolved once per registration and cached: after the bus is
  // destroyed no id resolves, and the ARIA sampler must not throw there.
  let slots: { left: number; right: number | null } | null = null;
  let ariaMin = $state(-60);
  let ariaMax = $state(0);
  let ariaNow = $state(-60);
  let ariaText = $state(formatAudioValue(-60, { type: "db", decimals: 1 }));
  const feedScratch = new Float32Array(3);

  function sampleSurfaceAria(): void {
    if (surface === null || slots === null) return;
    const view = surface.view;
    if (view.active[slots.left] !== 1) return;
    const leftDb = view.ballisticDb[slots.left]!;
    const leftText = formatAudioValue(leftDb, { type: "db", decimals: 1 });
    ariaMin = view.minDb[slots.left]!;
    ariaMax = view.maxDb[slots.left]!;
    ariaNow = leftDb;
    ariaText = slots.right === null
      ? leftText
      : `Left ${leftText}, right ${formatAudioValue(view.ballisticDb[slots.right]!, { type: "db", decimals: 1 })}`;
  }

  // Registration is keyed on tier and channel ownership. Leaving surface mode,
  // switching bus, or replacing a channel tears the old record down before any
  // new one is created — otherwise the canvas would keep painting a stale
  // placeholder and the shared ARIA sampler would keep firing for it.
  $effect(() => {
    const bus = surface;
    const left = channel;
    const right = rightChannel;
    const element = meterEl;
    validateSurfaceOwnership(bus, left, right);
    if (bus === null || left === null || registry === null || element === undefined) return;
    const resolved = { left: bus.slotOf(left), right: right === null ? null : bus.slotOf(right) };
    slots = resolved;
    const handle = registry.registerMeter(
      element,
      { slot: resolved.left, rightSlot: resolved.right, ...untrack(() => ({ style, orientation, segments })) },
      sampleSurfaceAria,
    );
    placeholder = handle;
    sampleSurfaceAria();
    return () => {
      handle.detach();
      if (placeholder === handle) placeholder = null;
      slots = null;
    };
  });

  // Geometry-only changes update in place rather than churning registration.
  $effect(() => {
    const geometry = { style, orientation, segments };
    untrack(() => {
      if (placeholder === null || slots === null) return;
      placeholder.update({ slot: slots.left, rightSlot: slots.right, ...geometry });
    });
  });

  onDestroy(() => {
    placeholder?.detach();
    placeholder = null;
    slots = null;
  });

  const leftVisual = $derived(audioMeterVisualState(context));
  const rightVisual = $derived(rightContext === null ? null : audioMeterVisualState(rightContext));
  const leftValueText = $derived(formatAudioValue(context.ballisticDb, { type: "db", decimals: 1 }));
  const valueText = $derived(rightContext === null
    ? leftValueText
    : `Left ${leftValueText}, right ${formatAudioValue(rightContext.ballisticDb, { type: "db", decimals: 1 })}`);

  export function push(frame: MeterFeedFrame, channelSide: "left" | "right" = "left"): void {
    if (surface !== null) {
      const slot = channelSide === "right" ? slots?.right ?? null : slots?.left ?? null;
      if (slot === null) return;
      feedScratch[0] = slot;
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

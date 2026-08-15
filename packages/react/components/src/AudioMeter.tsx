import { forwardRef, useCallback, useEffect, useImperativeHandle, useRef, useState } from "react";
import {
  audioMeterTransition, audioMeterVisualState, createAudioMeterContext, formatAudioValue,
  type AudioMeterContext, type MeterBus, type MeterBusChannelId, type MeterFeedFrame,
  type MeterPlaceholderHandle,
} from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/audio-meter.css";
import { AudioMeterVisual } from "./audio/AudioMeterVisual";
import { useAudioPresentation, type AudioPresentationProps } from "./audio/useAudioPresentation";
import { useMeterSurfaceRegistry } from "./MeterSurface";

export interface AudioMeterProps extends AudioPresentationProps {
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

export interface AudioMeterHandle {
  push(frame: MeterFeedFrame, channel?: "left" | "right"): void;
  resetClip(channel?: "left" | "right" | "both"): void;
}

interface SurfaceAria {
  min: number;
  max: number;
  now: number;
  text: string;
}

export const AudioMeter = forwardRef<AudioMeterHandle, AudioMeterProps>(function AudioMeter({ size, sizeRole, density, context, rightContext = null, style = "segments", orientation = "vertical", segments = 20, ariaLabel = null, surface = null, channel = null, rightChannel = null }, ref) {
  const presentation = useAudioPresentation({ size, sizeRole, density });
  const registry = useMeterSurfaceRegistry();
  if (surface !== null) {
    if (channel === null) throw new Error("AudioMeter: surface mode requires a registered `channel` id");
    if (registry === null) throw new Error("AudioMeter: surface mode requires an enclosing MeterSurface");
    if (registry.bus !== surface) throw new Error("AudioMeter: `surface` must be the bus of the enclosing MeterSurface");
  }
  // The default context must be created once, not per render: a per-render
  // default retriggers the context-sync effect on every re-render and loops.
  const [left, setLeft] = useState(() => context ?? createAudioMeterContext());
  const [right, setRight] = useState(rightContext);
  useEffect(() => {
    if (context !== undefined) setLeft(context);
  }, [context]);
  useEffect(() => setRight(rightContext), [rightContext]);

  const rootRef = useRef<HTMLDivElement>(null);
  const placeholderRef = useRef<MeterPlaceholderHandle | null>(null);
  const scratchRef = useRef<Float32Array | null>(null);
  const [surfaceAria, setSurfaceAria] = useState<SurfaceAria>(() => ({
    min: -60, max: 0, now: -60, text: formatAudioValue(-60, { type: "db", decimals: 1 }),
  }));

  const sampleSurfaceAria = useCallback(() => {
    if (surface === null || channel === null || surface.destroyed) return;
    const view = surface.view;
    const leftSlot = surface.slotOf(channel);
    const leftDb = view.ballisticDb[leftSlot]!;
    const leftText = formatAudioValue(leftDb, { type: "db", decimals: 1 });
    setSurfaceAria({
      min: view.minDb[leftSlot]!,
      max: view.maxDb[leftSlot]!,
      now: leftDb,
      text: rightChannel === null
        ? leftText
        : `Left ${leftText}, right ${formatAudioValue(view.ballisticDb[surface.slotOf(rightChannel)]!, { type: "db", decimals: 1 })}`,
    });
  }, [surface, channel, rightChannel]);

  useEffect(() => {
    const element = rootRef.current;
    if (surface === null || channel === null || registry === null || element === null) return;
    const spec = {
      slot: surface.slotOf(channel),
      rightSlot: rightChannel === null ? null : surface.slotOf(rightChannel),
      style, orientation, segments,
    };
    if (placeholderRef.current === null) {
      placeholderRef.current = registry.registerMeter(element, spec, sampleSurfaceAria);
      sampleSurfaceAria();
    } else {
      placeholderRef.current.update(spec);
    }
  }, [surface, channel, rightChannel, registry, style, orientation, segments, sampleSurfaceAria]);

  useEffect(() => () => {
    placeholderRef.current?.detach();
    placeholderRef.current = null;
  }, []);

  useImperativeHandle(ref, () => ({
    push(frame, channelSide = "left") {
      if (surface !== null) {
        const id = channelSide === "right" ? rightChannel : channel;
        if (id === null) return;
        const scratch = scratchRef.current ?? (scratchRef.current = new Float32Array(3));
        scratch[0] = surface.slotOf(id);
        scratch[1] = frame.peak;
        scratch[2] = frame.meanSquare;
        surface.pushFrames(scratch, frame.atMs, frame.durationMs);
        return;
      }
      if (channelSide === "right") setRight((current) => current === null ? null : audioMeterTransition(current, { type: "PUSH_FRAME", frame }).context);
      else setLeft((current) => audioMeterTransition(current, { type: "PUSH_FRAME", frame }).context);
    },
    resetClip(channelSide = "both") {
      if (surface !== null) {
        if (channelSide !== "right" && channel !== null) surface.resetClip(channel);
        if (channelSide !== "left" && rightChannel !== null) surface.resetClip(rightChannel);
        return;
      }
      if (channelSide !== "right") setLeft((current) => audioMeterTransition(current, { type: "RESET_CLIP" }).context);
      if (channelSide !== "left") setRight((current) => current === null ? null : audioMeterTransition(current, { type: "RESET_CLIP" }).context);
    },
  }), [surface, channel, rightChannel]);

  const leftVisual = audioMeterVisualState(left);
  const rightVisual = right === null ? null : audioMeterVisualState(right);
  const leftText = formatAudioValue(left.ballisticDb, { type: "db", decimals: 1 });
  const valueText = right === null ? leftText : `Left ${leftText}, right ${formatAudioValue(right.ballisticDb, { type: "db", decimals: 1 })}`;
  const surfaceStereo = surface !== null && rightChannel !== null;
  return <div ref={rootRef} className="poodle-audio-meter" data-size={presentation.size} data-density={presentation.density} role="meter" aria-label={ariaLabel ?? undefined} aria-valuemin={surface === null ? left.minDb : surfaceAria.min} aria-valuemax={surface === null ? left.maxDb : surfaceAria.max} aria-valuenow={surface === null ? left.ballisticDb : surfaceAria.now} aria-valuetext={surface === null ? valueText : surfaceAria.text} data-scope="audio-meter" data-part="root" data-orientation={orientation} data-surface={surface === null ? undefined : "true"} data-channels={(surface === null ? rightVisual !== null : surfaceStereo) ? "stereo" : "mono"}>
    {surface === null && <AudioMeterVisual visualState={leftVisual} style={style} orientation={orientation} segments={segments} />}
    {surface === null && rightVisual && <AudioMeterVisual visualState={rightVisual} style={style} orientation={orientation} segments={segments} />}
  </div>;
});

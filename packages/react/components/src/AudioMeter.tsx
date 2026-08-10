import { forwardRef, useEffect, useImperativeHandle, useState } from "react";
import { audioMeterTransition, audioMeterVisualState, createAudioMeterContext, formatAudioValue, type AudioMeterContext, type MeterFeedFrame } from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/audio-meter.css";
import { AudioMeterVisual } from "./audio/AudioMeterVisual";
import { useAudioPresentation, type AudioPresentationProps } from "./audio/useAudioPresentation";

export interface AudioMeterProps extends AudioPresentationProps {
  context?: AudioMeterContext;
  rightContext?: AudioMeterContext | null;
  style?: "bar" | "segments";
  orientation?: "horizontal" | "vertical";
  segments?: number;
  ariaLabel?: string | null;
}

export interface AudioMeterHandle {
  push(frame: MeterFeedFrame, channel?: "left" | "right"): void;
  resetClip(channel?: "left" | "right" | "both"): void;
}

export const AudioMeter = forwardRef<AudioMeterHandle, AudioMeterProps>(function AudioMeter({ size, sizeRole, density, context = createAudioMeterContext(), rightContext = null, style = "segments", orientation = "vertical", segments = 20, ariaLabel = null }, ref) {
  const presentation = useAudioPresentation({ size, sizeRole, density });
  const [left, setLeft] = useState(context);
  const [right, setRight] = useState(rightContext);
  useEffect(() => setLeft(context), [context]);
  useEffect(() => setRight(rightContext), [rightContext]);
  useImperativeHandle(ref, () => ({
    push(frame, channel = "left") {
      if (channel === "right") setRight((current) => current === null ? null : audioMeterTransition(current, { type: "PUSH_FRAME", frame }).context);
      else setLeft((current) => audioMeterTransition(current, { type: "PUSH_FRAME", frame }).context);
    },
    resetClip(channel = "both") {
      if (channel !== "right") setLeft((current) => audioMeterTransition(current, { type: "RESET_CLIP" }).context);
      if (channel !== "left") setRight((current) => current === null ? null : audioMeterTransition(current, { type: "RESET_CLIP" }).context);
    },
  }), []);
  const leftVisual = audioMeterVisualState(left);
  const rightVisual = right === null ? null : audioMeterVisualState(right);
  const leftText = formatAudioValue(left.ballisticDb, { type: "db", decimals: 1 });
  const valueText = right === null ? leftText : `Left ${leftText}, right ${formatAudioValue(right.ballisticDb, { type: "db", decimals: 1 })}`;
  return <div className="poodle-audio-meter" data-size={presentation.size} data-density={presentation.density} role="meter" aria-label={ariaLabel ?? undefined} aria-valuemin={left.minDb} aria-valuemax={left.maxDb} aria-valuenow={left.ballisticDb} aria-valuetext={valueText} data-scope="audio-meter" data-part="root" data-orientation={orientation} data-channels={rightVisual === null ? "mono" : "stereo"}>
    <AudioMeterVisual visualState={leftVisual} style={style} orientation={orientation} segments={segments} />
    {rightVisual && <AudioMeterVisual visualState={rightVisual} style={style} orientation={orientation} segments={segments} />}
  </div>;
});

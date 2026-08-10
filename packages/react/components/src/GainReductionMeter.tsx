import { forwardRef, useEffect, useImperativeHandle, useState } from "react";
import { createGainReductionMeterContext, formatAudioValue, gainReductionMeterTransition, gainReductionMeterVisualState, type GainReductionFrame, type GainReductionMeterContext } from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/gain-reduction-meter.css";
import { GainReductionMeterVisual } from "./audio/GainReductionMeterVisual";
import { useAudioPresentation, type AudioPresentationProps } from "./audio/useAudioPresentation";

export interface GainReductionMeterProps extends AudioPresentationProps {
  context?: GainReductionMeterContext;
  style?: "bar" | "segments";
  orientation?: "horizontal" | "vertical";
  segments?: number;
  ariaLabel?: string | null;
}
export interface GainReductionMeterHandle { push(frame: GainReductionFrame): void; reset(): void }

export const GainReductionMeter = forwardRef<GainReductionMeterHandle, GainReductionMeterProps>(function GainReductionMeter({ size, sizeRole, density, context = createGainReductionMeterContext(), style = "segments", orientation = "vertical", segments = 20, ariaLabel = "Gain reduction" }, ref) {
  const presentation = useAudioPresentation({ size, sizeRole, density });
  const [current, setCurrent] = useState(context);
  useEffect(() => setCurrent(context), [context]);
  useImperativeHandle(ref, () => ({
    push(frame) { setCurrent((value) => gainReductionMeterTransition(value, { type: "PUSH_FRAME", frame }).context); },
    reset() { setCurrent((value) => gainReductionMeterTransition(value, { type: "RESET" }).context); },
  }), []);
  const visualState = gainReductionMeterVisualState(current);
  return <div className="poodle-gain-reduction-meter" data-size={presentation.size} data-density={presentation.density} role="meter" aria-label={ariaLabel ?? "Gain reduction"} aria-valuemin={0} aria-valuemax={current.maxReductionDb} aria-valuenow={current.ballisticDb} aria-valuetext={`${formatAudioValue(current.ballisticDb, { type: "db", decimals: 1 })} reduction`} data-scope="gain-reduction-meter" data-part="root" data-orientation={orientation}>
    <GainReductionMeterVisual visualState={visualState} style={style} orientation={orientation} segments={segments} />
  </div>;
});

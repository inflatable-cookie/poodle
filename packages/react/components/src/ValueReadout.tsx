import { formatAudioValue, valueReadoutVisualState, type AudioValueFormat, type AudioValueLaw } from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/value-readout.css";
import { ValueVisual } from "./audio/ValueVisual";
import { useAudioPresentation, type AudioPresentationProps } from "./audio/useAudioPresentation";

export interface ValueReadoutProps extends AudioPresentationProps {
  value?: number;
  min?: number;
  max?: number;
  law?: AudioValueLaw;
  format?: AudioValueFormat;
  disabled?: boolean;
  ariaLabel?: string | null;
}

export function ValueReadout({ size, sizeRole, density, value = 0, min = 0, max = 1, law = { type: "linear" }, format = { type: "number", decimals: 2 }, disabled = false, ariaLabel = null }: ValueReadoutProps) {
  const presentation = useAudioPresentation({ size, sizeRole, density });
  const visualState = valueReadoutVisualState(value, min, max, law, !disabled);
  return <output className="poodle-value-readout" data-size={presentation.size} data-density={presentation.density} aria-label={ariaLabel ?? undefined} data-disabled={disabled} data-scope="value-readout" data-part="root">
    <ValueVisual visualState={visualState} text={formatAudioValue(value, format)} kind="readout" />
  </output>;
}

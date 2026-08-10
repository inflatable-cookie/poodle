import { resolveSemanticControlSize, useUiPresentation } from "../presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "../types";

export interface AudioPresentationProps {
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
}

export function useAudioPresentation({
  size = null,
  sizeRole = "control",
  density = null,
}: AudioPresentationProps): { size: ControlSize; density: ControlDensity } {
  const presentation = useUiPresentation();
  return {
    size: size ?? resolveSemanticControlSize(presentation.sizeScale, sizeRole),
    density: density ?? presentation.density,
  };
}

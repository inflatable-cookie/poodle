export type AudioDragState = "none" | "coarse" | "fine";

export type AudioAutomationState = "none" | "touched" | "latched" | "writing" | "read";

/** Renderer-neutral, JSON-serializable state from an audio control core. */
export interface AudioControlVisualState {
  valueNorm: number;
  rawValue: number;
  bipolarCenter: number | null;
  hover: boolean;
  focus: boolean;
  drag: AudioDragState;
  automation: AudioAutomationState;
  enabled: boolean;
}

export interface AudioMeterVisualState extends AudioControlVisualState {
  ballisticValue: number;
  peakHold: number | null;
  clip: boolean;
}

export interface EnvelopeVisualPoint {
  id: string;
  xNorm: number;
  yNorm: number;
  curve: number;
  selected: boolean;
  dragging: boolean;
}

export interface EnvelopeVisualState {
  points: EnvelopeVisualPoint[];
  hoverPointId: string | null;
  focus: boolean;
  enabled: boolean;
}

export interface XYPadVisualState {
  xNorm: number;
  yNorm: number;
  rawX: number;
  rawY: number;
  hover: boolean;
  focus: boolean;
  drag: AudioDragState;
  automation: AudioAutomationState;
  enabled: boolean;
}

export interface AudioSwitchVisualState {
  state: number;
  stateCount: number;
  pressed: boolean;
  lampOn: boolean;
  hover: boolean;
  focus: boolean;
  enabled: boolean;
}

export interface GainReductionMeterVisualState extends AudioMeterVisualState {
  reductionDb: number;
}

export interface AudioPoint {
  x: number;
  y: number;
}

export interface AudioRect {
  left: number;
  top: number;
  width: number;
  height: number;
}

export function hitTestRect(point: AudioPoint, rect: AudioRect): boolean {
  return point.x >= rect.left && point.x <= rect.left + rect.width
    && point.y >= rect.top && point.y <= rect.top + rect.height;
}

export function hitTestCircle(point: AudioPoint, rect: AudioRect): boolean {
  const radius = Math.min(rect.width, rect.height) / 2;
  const centerX = rect.left + rect.width / 2;
  const centerY = rect.top + rect.height / 2;
  return Math.hypot(point.x - centerX, point.y - centerY) <= radius;
}

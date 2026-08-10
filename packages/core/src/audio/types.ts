import type { AudioValueLaw } from "./laws";

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

export type KeyboardOrientation = "horizontal" | "vertical";

export interface KeyboardKeyVisualState {
  note: number;
  kind: "white" | "black";
  startNorm: number;
  lengthNorm: number;
  breadthNorm: number;
  held: boolean;
  externallyHeld: boolean;
  velocity: number | null;
  focused: boolean;
}

export interface KeyboardVisualState {
  orientation: KeyboardOrientation;
  firstNote: number;
  lastNote: number;
  octaveShift: number;
  keys: KeyboardKeyVisualState[];
  heldNotes: number[];
  externalHeldNotes: number[];
  enabled: boolean;
}

export interface WaveformPeakPair { min: number; max: number }
export interface WaveformPeakLevel { samplesPerPeak: number; peaks: WaveformPeakPair[] }
export interface WaveformPeakPyramid { sampleCount: number; levels: WaveformPeakLevel[] }
export interface WaveformSelection { start: number; end: number }

export interface WaveformVisualState {
  sampleCount: number;
  visibleStart: number;
  visibleEnd: number;
  columns: WaveformPeakPair[];
  cursorSample: number | null;
  selection: WaveformSelection | null;
  focus: boolean;
  enabled: boolean;
}

export interface ModMatrixHeader { id: string; label: string }
export interface ModMatrixCellParameters {
  min?: number;
  max?: number;
  step?: number;
  law?: AudioValueLaw;
}
export interface ResolvedModMatrixCellParameters {
  min: number;
  max: number;
  step: number;
  law: AudioValueLaw;
}
export interface ModMatrixCell {
  sourceId: string;
  destinationId: string;
  amount: number;
  enabled: boolean;
  parameters?: ModMatrixCellParameters;
}
export interface ModMatrixVisualCell extends Omit<ModMatrixCell, "parameters"> {
  parameters: ResolvedModMatrixCellParameters;
  amountNorm: number;
  zeroNorm: number;
  fillStartNorm: number;
  fillSpanNorm: number;
  focused: boolean;
}
export interface ModMatrixVisualState {
  sources: ModMatrixHeader[];
  destinations: ModMatrixHeader[];
  cells: ModMatrixVisualCell[];
  focus: { sourceId: string; destinationId: string } | null;
  enabled: boolean;
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

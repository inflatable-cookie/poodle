import type {
  AudioPoint,
  AudioRect,
  KeyboardKeyVisualState,
  KeyboardOrientation,
  KeyboardVisualState,
} from "./types";

export const DEFAULT_COMPUTER_KEY_MAP: Readonly<Record<string, number>> = Object.freeze({
  a: 0, w: 1, s: 2, e: 3, d: 4, f: 5, t: 6,
  g: 7, y: 8, h: 9, u: 10, j: 11, k: 12,
});

export interface KeyboardContext {
  firstNote: number;
  lastNote: number;
  orientation: KeyboardOrientation;
  octaveShift: number;
  computerBaseNote: number;
  computerKeyMap: Record<string, number>;
  activeInputs: Record<string, { note: number; velocity: number }>;
  externalHeldNotes: number[];
  focusedNote: number | null;
  disabled: boolean;
}

export type KeyboardEffect =
  | { type: "noteOn"; note: number; velocity: number }
  | { type: "noteOff"; note: number };

export type KeyboardEvent =
  | { type: "PRESS"; inputId: string; note: number; velocity: number }
  | { type: "RETARGET"; inputId: string; note: number | null; velocity: number }
  | { type: "RELEASE"; inputId: string }
  | { type: "RELEASE_ALL" }
  | { type: "COMPUTER_KEY_DOWN"; key: string; velocity?: number; repeat?: boolean }
  | { type: "COMPUTER_KEY_UP"; key: string }
  | { type: "SET_RANGE"; firstNote: number; lastNote: number }
  | { type: "SET_OCTAVE_SHIFT"; value: number }
  | { type: "SET_EXTERNAL_HELD"; notes: number[] }
  | { type: "SET_DISABLED"; value: boolean }
  | { type: "FOCUS_NOTE"; note: number | null }
  | { type: "MOVE_FOCUS"; direction: -1 | 1 };

export interface KeyboardResult { context: KeyboardContext; effects: KeyboardEffect[] }

const clampMidiNote = (note: number): number => Math.min(Math.max(Math.round(Number.isFinite(note) ? note : 0), 0), 127);
const clampVelocity = (velocity: number): number => Math.min(Math.max(Math.round(Number.isFinite(velocity) ? velocity : 1), 1), 127);
const isBlackNote = (note: number): boolean => [1, 3, 6, 8, 10].includes(((note % 12) + 12) % 12);

function normalizedRange(firstNote: number, lastNote: number): [number, number] {
  const first = clampMidiNote(Math.min(firstNote, lastNote));
  const last = clampMidiNote(Math.max(firstNote, lastNote));
  return [first, last];
}

function uniqueNotes(notes: number[]): number[] {
  return [...new Set(notes.map(clampMidiNote))].sort((a, b) => a - b);
}

export function createKeyboardContext(input: Partial<KeyboardContext> = {}): KeyboardContext {
  const [firstNote, lastNote] = normalizedRange(input.firstNote ?? 48, input.lastNote ?? 72);
  return {
    firstNote,
    lastNote,
    orientation: input.orientation ?? "horizontal",
    octaveShift: Math.trunc(input.octaveShift ?? 0),
    computerBaseNote: clampMidiNote(input.computerBaseNote ?? 60),
    computerKeyMap: { ...DEFAULT_COMPUTER_KEY_MAP, ...(input.computerKeyMap ?? {}) },
    activeInputs: { ...(input.activeInputs ?? {}) },
    externalHeldNotes: uniqueNotes(input.externalHeldNotes ?? []),
    focusedNote: input.focusedNote == null ? null : clampMidiNote(input.focusedNote),
    disabled: input.disabled ?? false,
  };
}

function noteHeldByOtherInput(context: KeyboardContext, note: number, exceptInput: string): boolean {
  return Object.entries(context.activeInputs).some(([id, active]) => id !== exceptInput && active.note === note);
}

function releaseInputs(context: KeyboardContext, predicate: (id: string, note: number) => boolean): KeyboardResult {
  const activeInputs = { ...context.activeInputs };
  const effects: KeyboardEffect[] = [];
  for (const [id, active] of Object.entries(context.activeInputs)) {
    if (!predicate(id, active.note)) continue;
    delete activeInputs[id];
    if (!Object.values(activeInputs).some((candidate) => candidate.note === active.note)) {
      effects.push({ type: "noteOff", note: active.note });
    }
  }
  return { context: { ...context, activeInputs }, effects };
}

export function keyboardTransition(context: KeyboardContext, event: KeyboardEvent): KeyboardResult {
  switch (event.type) {
    case "PRESS": {
      if (context.disabled || event.note < context.firstNote || event.note > context.lastNote || context.activeInputs[event.inputId]) {
        return { context, effects: [] };
      }
      const note = clampMidiNote(event.note);
      const velocity = clampVelocity(event.velocity);
      const alreadyHeld = Object.values(context.activeInputs).some((active) => active.note === note);
      return {
        context: { ...context, activeInputs: { ...context.activeInputs, [event.inputId]: { note, velocity } }, focusedNote: note },
        effects: alreadyHeld ? [] : [{ type: "noteOn", note, velocity }],
      };
    }
    case "RETARGET": {
      if (context.disabled) return keyboardTransition(context, { type: "RELEASE", inputId: event.inputId });
      const active = context.activeInputs[event.inputId];
      if (active && event.note === active.note) {
        const velocity = clampVelocity(event.velocity);
        return {
          context: {
            ...context,
            activeInputs: { ...context.activeInputs, [event.inputId]: { note: active.note, velocity } },
            focusedNote: active.note,
          },
          effects: [],
        };
      }
      const released = keyboardTransition(context, { type: "RELEASE", inputId: event.inputId });
      if (event.note === null) return released;
      const pressed = keyboardTransition(released.context, {
        type: "PRESS", inputId: event.inputId, note: event.note, velocity: event.velocity,
      });
      return { context: pressed.context, effects: [...released.effects, ...pressed.effects] };
    }
    case "RELEASE": {
      const active = context.activeInputs[event.inputId];
      if (!active) return { context, effects: [] };
      const activeInputs = { ...context.activeInputs };
      delete activeInputs[event.inputId];
      return {
        context: { ...context, activeInputs },
        effects: noteHeldByOtherInput(context, active.note, event.inputId) ? [] : [{ type: "noteOff", note: active.note }],
      };
    }
    case "RELEASE_ALL": return releaseInputs(context, () => true);
    case "COMPUTER_KEY_DOWN": {
      const key = event.key.toLowerCase();
      if (event.repeat || context.activeInputs[`key:${key}`]) return { context, effects: [] };
      const offset = context.computerKeyMap[key];
      if (offset == null) return { context, effects: [] };
      const note = context.computerBaseNote + offset + context.octaveShift * 12;
      return keyboardTransition(context, { type: "PRESS", inputId: `key:${key}`, note, velocity: event.velocity ?? 100 });
    }
    case "COMPUTER_KEY_UP": return keyboardTransition(context, { type: "RELEASE", inputId: `key:${event.key.toLowerCase()}` });
    case "SET_RANGE": {
      const [firstNote, lastNote] = normalizedRange(event.firstNote, event.lastNote);
      const released = releaseInputs(context, (_id, note) => note < firstNote || note > lastNote);
      return { context: { ...released.context, firstNote, lastNote }, effects: released.effects };
    }
    case "SET_OCTAVE_SHIFT": {
      const released = releaseInputs(context, (id) => id.startsWith("key:"));
      return { context: { ...released.context, octaveShift: Math.trunc(event.value) }, effects: released.effects };
    }
    case "SET_EXTERNAL_HELD": return { context: { ...context, externalHeldNotes: uniqueNotes(event.notes) }, effects: [] };
    case "SET_DISABLED": {
      if (!event.value) return { context: { ...context, disabled: false }, effects: [] };
      const released = releaseInputs(context, () => true);
      return { context: { ...released.context, disabled: true }, effects: released.effects };
    }
    case "FOCUS_NOTE": return { context: { ...context, focusedNote: event.note == null ? null : clampMidiNote(event.note) }, effects: [] };
    case "MOVE_FOCUS": {
      const current = context.focusedNote ?? (event.direction > 0 ? context.firstNote - 1 : context.lastNote + 1);
      return { context: { ...context, focusedNote: Math.min(Math.max(current + event.direction, context.firstNote), context.lastNote) }, effects: [] };
    }
  }
}

export function keyboardKeyGeometry(firstNote: number, lastNote: number, orientation: KeyboardOrientation): KeyboardKeyVisualState[] {
  const [first, last] = normalizedRange(firstNote, lastNote);
  const notes = Array.from({ length: last - first + 1 }, (_, index) => first + index);
  const whiteNotes = notes.filter((note) => !isBlackNote(note));
  const whiteIndex = new Map(whiteNotes.map((note, index) => [note, index]));
  const whiteLength = 1 / Math.max(whiteNotes.length, 1);
  return notes.map((note) => {
    const black = isBlackNote(note);
    const precedingWhites = whiteNotes.filter((candidate) => candidate < note).length;
    const logicalStart = black
      ? Math.max((precedingWhites - 0.32) * whiteLength, 0)
      : (whiteIndex.get(note) ?? 0) * whiteLength;
    const lengthNorm = black ? whiteLength * 0.64 : whiteLength;
    return {
      note,
      kind: black ? "black" : "white",
      startNorm: orientation === "vertical" ? 1 - logicalStart - lengthNorm : logicalStart,
      lengthNorm,
      breadthNorm: black ? 0.62 : 1,
      held: false,
      externallyHeld: false,
      velocity: null,
      focused: false,
    };
  });
}

export function keyboardVisualState(context: KeyboardContext): KeyboardVisualState {
  const held = new Map<number, number>();
  for (const active of Object.values(context.activeInputs)) held.set(active.note, Math.max(held.get(active.note) ?? 0, active.velocity));
  const external = new Set(context.externalHeldNotes);
  return {
    orientation: context.orientation,
    firstNote: context.firstNote,
    lastNote: context.lastNote,
    octaveShift: context.octaveShift,
    keys: keyboardKeyGeometry(context.firstNote, context.lastNote, context.orientation).map((key) => ({
      ...key,
      held: held.has(key.note),
      externallyHeld: external.has(key.note),
      velocity: held.get(key.note) ?? null,
      focused: key.note === context.focusedNote,
    })),
    heldNotes: [...held.keys()].sort((a, b) => a - b),
    externalHeldNotes: context.externalHeldNotes,
    enabled: !context.disabled,
  };
}

export function keyboardVelocityAtPoint(point: AudioPoint, rect: AudioRect, orientation: KeyboardOrientation): number {
  const depth = orientation === "horizontal"
    ? (point.y - rect.top) / Math.max(rect.height, 1)
    : (point.x - rect.left) / Math.max(rect.width, 1);
  return clampVelocity(1 + Math.min(Math.max(depth, 0), 1) * 126);
}

export function keyboardHitTest(context: KeyboardContext, point: AudioPoint, rect: AudioRect): number | null {
  const axis = context.orientation === "horizontal"
    ? (point.x - rect.left) / Math.max(rect.width, 1)
    : (point.y - rect.top) / Math.max(rect.height, 1);
  const depth = context.orientation === "horizontal"
    ? (point.y - rect.top) / Math.max(rect.height, 1)
    : (point.x - rect.left) / Math.max(rect.width, 1);
  if (axis < 0 || axis > 1 || depth < 0 || depth > 1) return null;
  const keys = keyboardKeyGeometry(context.firstNote, context.lastNote, context.orientation);
  const black = keys.find((key) => key.kind === "black" && depth <= key.breadthNorm && axis >= key.startNorm && axis <= key.startNorm + key.lengthNorm);
  return black?.note ?? keys.find((key) => key.kind === "white" && axis >= key.startNorm && axis <= key.startNorm + key.lengthNorm)?.note ?? null;
}

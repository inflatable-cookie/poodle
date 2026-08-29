/**
 * TimeInput entry machinery.
 * Contract: docs/contracts/components/time-input.md, "Behavior Machine".
 *
 * Pure parse/format, bounds, step alignment, and draft-versus-commit
 * transitions. Adapters own focus, drawing, native events, and callback
 * execution. The machine emits `emitValueChange` only when the contract
 * fires `onValueChange`.
 */

export interface TimeParts {
  hour: number;
  minute: number;
  second: number;
}

export type TimeSegment = "hour" | "minute" | "second";

export interface TimeInputDraft {
  hour: string;
  minute: string;
  second: string;
}

export interface TimeInputContext {
  committed: string | null;
  defaultValue: string | null;
  draft: TimeInputDraft | null;
  min: string | null;
  max: string | null;
  step: number;
  disabled: boolean;
}

export type TimeInputEvent =
  | { type: "DIGIT"; segment: TimeSegment; digit: number }
  | { type: "CLEAR_SEGMENT"; segment: TimeSegment }
  | { type: "CLEAR_ALL" }
  | { type: "STEP"; direction: 1 | -1 }
  | { type: "BLUR" }
  | { type: "ESCAPE" }
  | { type: "REPLACE"; value: string | null }
  | { type: "COMMIT_TEXT"; text: string }
  | { type: "SET_DISABLED"; disabled: boolean }
  | { type: "SET_CONSTRAINTS"; min: string | null; max: string | null; step: number; defaultValue: string | null };

export type TimeInputEffect = { type: "emitValueChange"; value: string | null };

export interface TimeInputResult {
  context: TimeInputContext;
  effects: TimeInputEffect[];
}

const SECONDS_PER_DAY = 24 * 60 * 60;

function pad2(value: number): string {
  return value.toString().padStart(2, "0");
}

function emptyDraft(): TimeInputDraft {
  return { hour: "", minute: "", second: "" };
}

export function isPositiveWholeStep(step: number): boolean {
  return Number.isInteger(step) && step > 0;
}

export function timeHasSeconds(value: string | null | undefined): boolean {
  if (value === null || value === undefined) {
    return false;
  }

  return (value.match(/:/g) ?? []).length >= 2;
}

export function parseTime(value: string | null | undefined): TimeParts | null {
  if (value === null || value === undefined) {
    return null;
  }

  const match = /^(\d{2}):(\d{2})(?::(\d{2}))?$/.exec(value);

  if (match === null) {
    return null;
  }

  const hour = Number(match[1]);
  const minute = Number(match[2]);
  const second = match[3] === undefined ? 0 : Number(match[3]);

  if (hour > 23 || minute > 59 || second > 59) {
    return null;
  }

  return { hour, minute, second };
}

export function formatTime(parts: TimeParts, withSeconds: boolean): string {
  const clock = `${pad2(parts.hour)}:${pad2(parts.minute)}`;

  return withSeconds ? `${clock}:${pad2(parts.second)}` : clock;
}

export function timeToSeconds(parts: TimeParts): number {
  return parts.hour * 3600 + parts.minute * 60 + parts.second;
}

export function secondsToTime(total: number): TimeParts {
  const wrapped = wrapSeconds(total);

  return {
    hour: Math.floor(wrapped / 3600),
    minute: Math.floor((wrapped % 3600) / 60),
    second: wrapped % 60,
  };
}

export function wrapSeconds(total: number): number {
  return ((total % SECONDS_PER_DAY) + SECONDS_PER_DAY) % SECONDS_PER_DAY;
}

export function timeSecondsVisible(context: {
  committed?: string | null;
  defaultValue?: string | null;
  min?: string | null;
  max?: string | null;
  step: number;
}): boolean {
  return (
    context.step < 60 ||
    timeHasSeconds(context.committed ?? null) ||
    timeHasSeconds(context.defaultValue ?? null) ||
    timeHasSeconds(context.min ?? null) ||
    timeHasSeconds(context.max ?? null)
  );
}

function boundSeconds(value: string | null): number | null {
  const parts = parseTime(value);

  return parts === null ? null : timeToSeconds(parts);
}

export function timeInBounds(parts: TimeParts, min: string | null, max: string | null): boolean {
  const seconds = timeToSeconds(parts);
  const minSeconds = boundSeconds(min);
  const maxSeconds = boundSeconds(max);

  if (minSeconds === null && maxSeconds === null) {
    return true;
  }

  if (minSeconds !== null && maxSeconds !== null && minSeconds > maxSeconds) {
    return seconds >= minSeconds || seconds <= maxSeconds;
  }

  if (minSeconds !== null && seconds < minSeconds) {
    return false;
  }

  if (maxSeconds !== null && seconds > maxSeconds) {
    return false;
  }

  return true;
}

export function timeStepAligned(
  parts: TimeParts,
  min: string | null,
  step: number,
): boolean {
  if (!isPositiveWholeStep(step)) {
    return false;
  }

  const origin = boundSeconds(min) ?? 0;
  const delta = timeToSeconds(parts) - origin;

  return ((delta % step) + step) % step === 0;
}

export function timeConstraintValid(
  value: string | null,
  min: string | null,
  max: string | null,
  step: number,
): boolean {
  if (value === null) {
    return true;
  }

  const parts = parseTime(value);

  return parts !== null && timeInBounds(parts, min, max) && timeStepAligned(parts, min, step);
}

function formatFromSeconds(seconds: number, withSeconds: boolean): string {
  return formatTime(secondsToTime(seconds), withSeconds);
}

function secondsConstraintValid(
  seconds: number,
  min: string | null,
  max: string | null,
  step: number,
): boolean {
  const wrapped = wrapSeconds(seconds);
  const formatted = formatTime(secondsToTime(wrapped), true);

  return timeConstraintValid(formatted, min, max, step);
}

function lastOnGridSeconds(min: string | null, max: string | null, step: number): number {
  const origin = boundSeconds(min) ?? 0;
  const high = boundSeconds(max) ?? SECONDS_PER_DAY - 1;
  const remainder = ((high - origin) % step + step) % step;
  const last = wrapSeconds(high - remainder);

  if (secondsConstraintValid(last, min, max, step)) {
    return last;
  }

  const minSeconds = boundSeconds(min);

  if (minSeconds !== null && secondsConstraintValid(minSeconds, min, max, step)) {
    return minSeconds;
  }

  return wrapSeconds(origin);
}

function firstOnGridSeconds(min: string | null): number {
  return wrapSeconds(boundSeconds(min) ?? 0);
}

export function stepTimeSeconds(
  current: number | null,
  direction: 1 | -1,
  min: string | null,
  max: string | null,
  step: number,
): number | null {
  if (!isPositiveWholeStep(step)) {
    return current;
  }

  if (current === null) {
    return direction > 0 ? firstOnGridSeconds(min) : lastOnGridSeconds(min, max, step);
  }

  const minSeconds = boundSeconds(min);
  const maxSeconds = boundSeconds(max);
  const wrap =
    (minSeconds === null && maxSeconds === null) ||
    (minSeconds !== null && maxSeconds !== null && minSeconds > maxSeconds);
  const candidate = wrap ? wrapSeconds(current + direction * step) : current + direction * step;

  if (secondsConstraintValid(candidate, min, max, step)) {
    return wrapSeconds(candidate);
  }

  if (secondsConstraintValid(current, min, max, step)) {
    return wrapSeconds(current);
  }

  return direction > 0 ? firstOnGridSeconds(min) : lastOnGridSeconds(min, max, step);
}

function showSeconds(context: TimeInputContext): boolean {
  return timeSecondsVisible(context);
}

function draftFromCommitted(committed: string | null): TimeInputDraft {
  const parts = parseTime(committed);

  if (parts === null) {
    return emptyDraft();
  }

  return {
    hour: pad2(parts.hour),
    minute: pad2(parts.minute),
    second: pad2(parts.second),
  };
}

function textToDraft(text: string): TimeInputDraft {
  const parts = text.split(":");

  return {
    hour: parts[0] ?? "",
    minute: parts[1] ?? "",
    second: parts[2] ?? "",
  };
}

function visibleEmpty(draft: TimeInputDraft, secondsVisible: boolean): boolean {
  return draft.hour === "" && draft.minute === "" && (!secondsVisible || draft.second === "");
}

function draftCandidate(draft: TimeInputDraft, secondsVisible: boolean): string | null {
  if (draft.hour.length !== 2 || draft.minute.length !== 2) {
    return null;
  }

  if (secondsVisible) {
    if (draft.second.length !== 2) {
      return null;
    }

    return `${draft.hour}:${draft.minute}:${draft.second}`;
  }

  return `${draft.hour}:${draft.minute}`;
}

function commitValue(
  context: TimeInputContext,
  value: string | null,
): TimeInputResult {
  if (context.committed === value && context.draft === null) {
    return { context, effects: [] };
  }

  return {
    context: { ...context, committed: value, draft: null },
    effects: [{ type: "emitValueChange", value }],
  };
}

function tryCommitDraft(context: TimeInputContext, draft: TimeInputDraft): TimeInputResult {
  const secondsVisible = showSeconds(context);

  if (visibleEmpty(draft, secondsVisible)) {
    return commitValue(context, null);
  }

  const candidate = draftCandidate(draft, secondsVisible);
  const parts = candidate === null ? null : parseTime(candidate);

  if (
    parts !== null &&
    timeInBounds(parts, context.min, context.max) &&
    timeStepAligned(parts, context.min, context.step)
  ) {
    return commitValue(context, formatTime(parts, secondsVisible));
  }

  return {
    context: { ...context, draft },
    effects: [],
  };
}

function withDraft(context: TimeInputContext): TimeInputDraft {
  return context.draft ?? draftFromCommitted(context.committed);
}

function applyDigit(draft: TimeInputDraft, segment: TimeSegment, digit: number): TimeInputDraft {
  const current = draft[segment];
  const next = current.length === 0 || current.length >= 2 ? String(digit) : `${current}${digit}`;

  return { ...draft, [segment]: next };
}

function idle(context: TimeInputContext): TimeInputResult {
  return { context, effects: [] };
}

export function timeInputInvalid(context: TimeInputContext): boolean {
  return context.draft !== null;
}

export function timeInputContext(init: Partial<TimeInputContext> = {}): TimeInputContext {
  return {
    committed: null,
    defaultValue: null,
    draft: null,
    min: null,
    max: null,
    step: 60,
    disabled: false,
    ...init,
  };
}

export function timeInputTransition(context: TimeInputContext, event: TimeInputEvent): TimeInputResult {
  switch (event.type) {
    case "SET_DISABLED":
      return { context: { ...context, disabled: event.disabled }, effects: [] };
    case "SET_CONSTRAINTS":
      return {
        context: {
          ...context,
          min: event.min,
          max: event.max,
          step: event.step,
          defaultValue: event.defaultValue,
        },
        effects: [],
      };
    case "REPLACE":
      return { context: { ...context, committed: event.value, draft: null }, effects: [] };
    default:
      break;
  }

  if (context.disabled) {
    return idle(context);
  }

  switch (event.type) {
    case "DIGIT": {
      if (!Number.isInteger(event.digit) || event.digit < 0 || event.digit > 9) {
        return idle(context);
      }

      return tryCommitDraft(context, applyDigit(withDraft(context), event.segment, event.digit));
    }
    case "CLEAR_SEGMENT": {
      return tryCommitDraft(context, { ...withDraft(context), [event.segment]: "" });
    }
    case "CLEAR_ALL":
      return commitValue(context, null);
    case "COMMIT_TEXT": {
      if (event.text === "") {
        return commitValue(context, null);
      }

      const secondsVisible = showSeconds(context);
      const parts = parseTime(event.text);

      if (parts !== null) {
        const formatted = formatTime(parts, secondsVisible);

        if (timeConstraintValid(formatted, context.min, context.max, context.step)) {
          return commitValue(context, formatted);
        }
      }

      return {
        context: { ...context, draft: textToDraft(event.text) },
        effects: [],
      };
    }
    case "STEP": {
      if (!isPositiveWholeStep(context.step)) {
        return { context: { ...context, draft: null }, effects: [] };
      }

      const current = parseTime(context.committed);
      const nextSeconds = stepTimeSeconds(
        current === null ? null : timeToSeconds(current),
        event.direction,
        context.min,
        context.max,
        context.step,
      );

      if (nextSeconds === null) {
        return { context: { ...context, draft: null }, effects: [] };
      }

      const formatted = formatFromSeconds(nextSeconds, showSeconds(context));

      if (!timeConstraintValid(formatted, context.min, context.max, context.step)) {
        return { context: { ...context, draft: null }, effects: [] };
      }

      return commitValue(context, formatted);
    }
    case "BLUR":
    case "ESCAPE":
      if (context.draft === null) {
        return idle(context);
      }

      return { context: { ...context, draft: null }, effects: [] };
  }
}

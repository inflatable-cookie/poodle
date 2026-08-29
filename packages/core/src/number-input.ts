/**
 * NumberInput entry machinery.
 * Contract: docs/contracts/components/number-input.md, "Behavior Machine".
 *
 * Pure decimal draft classification, configuration checks, committed
 * value/raw-draft transitions, and step/commit effects. Adapters own
 * focus, caret, DOM/native events, async validation, and callback
 * execution.
 */

export interface NumberInputContext {
  committed: number | null;
  defaultValue: number | null;
  draft: string | null;
  min: number | null;
  max: number | null;
  /** Authored step; `null` means omitted (effective step is `1`). */
  step: number | null;
  /** Authored precision; `null` means omitted. */
  precision: number | null;
  disabled: boolean;
  readOnly: boolean;
}

export type NumberInputEvent =
  | { type: "RAW_EDIT"; text: string }
  | { type: "CLEAR" }
  | { type: "ENTER" }
  | { type: "BLUR" }
  | { type: "ESCAPE" }
  | { type: "STEP"; direction: 1 | -1 }
  | { type: "HOME" }
  | { type: "END" }
  | { type: "REPLACE"; value: number | null }
  | { type: "SET_DISABLED"; disabled: boolean }
  | { type: "SET_READ_ONLY"; readOnly: boolean }
  | {
      type: "SET_CONSTRAINTS";
      min: number | null;
      max: number | null;
      step: number | null;
      precision: number | null;
      defaultValue: number | null;
    };

export type NumberInputEffect =
  | { type: "emitDraftValueChange"; draft: string | null }
  | { type: "emitValueChange"; value: number | null }
  | { type: "emitCommit"; value: number | null };

export interface NumberInputResult {
  context: NumberInputContext;
  effects: NumberInputEffect[];
}

export type NumberDraftKind = "empty" | "incomplete" | "malformed" | "complete";

export interface NumberDecimal {
  negative: boolean;
  /** Absolute integer coefficient; value = ± digits / 10^scale. */
  digits: bigint;
  scale: number;
}

export interface NumberDraftClassification {
  kind: NumberDraftKind;
  decimal: NumberDecimal | null;
  fractionalDigits: number | null;
}

const DRAFT_SYNTAX = /^-?(?:\d+(?:\.\d*)?|\.\d+)$/;
const INCOMPLETE_DRAFTS = new Set(["-", ".", "-."]);

function idle(context: NumberInputContext): NumberInputResult {
  return { context, effects: [] };
}

function isFiniteNumber(value: number): boolean {
  return typeof value === "number" && Number.isFinite(value);
}

function isNonNegativeInteger(value: number): boolean {
  return Number.isInteger(value) && value >= 0;
}

/** Effective step used for alignment and stepping when config is valid. */
export function numberInputEffectiveStep(step: number | null): number {
  return step === null ? 1 : step;
}

export function numberInputConfigValid(context: {
  committed?: number | null;
  defaultValue?: number | null;
  min?: number | null;
  max?: number | null;
  step?: number | null;
  precision?: number | null;
}): boolean {
  const checkOptional = (value: number | null | undefined): boolean =>
    value === null || value === undefined || isFiniteNumber(value);

  if (!checkOptional(context.committed)) return false;
  if (!checkOptional(context.defaultValue)) return false;
  if (!checkOptional(context.min)) return false;
  if (!checkOptional(context.max)) return false;

  if (context.step !== null && context.step !== undefined) {
    if (!isFiniteNumber(context.step) || context.step <= 0) {
      return false;
    }
  }

  if (context.precision !== null && context.precision !== undefined) {
    if (!isNonNegativeInteger(context.precision)) {
      return false;
    }
  }

  if (
    context.min !== null &&
    context.min !== undefined &&
    context.max !== null &&
    context.max !== undefined &&
    context.min > context.max
  ) {
    return false;
  }

  return true;
}

function stripSign(text: string): { negative: boolean; body: string } {
  if (text.startsWith("-")) {
    return { negative: true, body: text.slice(1) };
  }

  return { negative: false, body: text };
}

export function parseNumberDecimal(text: string): NumberDecimal | null {
  if (!DRAFT_SYNTAX.test(text) || INCOMPLETE_DRAFTS.has(text) || text.endsWith(".")) {
    return null;
  }

  const { negative, body } = stripSign(text);
  const [intRaw, fracRaw = ""] = body.split(".");
  const intPart = intRaw === "" ? "0" : intRaw;
  const digitsText = `${intPart}${fracRaw}`.replace(/^0+(?=\d)/, "") || "0";

  return {
    negative: negative && digitsText !== "0",
    digits: BigInt(digitsText),
    scale: fracRaw.length,
  };
}

export function classifyNumberDraft(text: string): NumberDraftClassification {
  if (text === "") {
    return { kind: "empty", decimal: null, fractionalDigits: null };
  }

  if (INCOMPLETE_DRAFTS.has(text) || (DRAFT_SYNTAX.test(text) && text.endsWith("."))) {
    return { kind: "incomplete", decimal: null, fractionalDigits: null };
  }

  if (!DRAFT_SYNTAX.test(text)) {
    return { kind: "malformed", decimal: null, fractionalDigits: null };
  }

  const decimal = parseNumberDecimal(text);

  if (decimal === null) {
    return { kind: "malformed", decimal: null, fractionalDigits: null };
  }

  const { body } = stripSign(text);
  const frac = body.includes(".") ? body.split(".")[1]!.length : 0;

  return { kind: "complete", decimal, fractionalDigits: frac };
}

function rescaleDigits(decimal: NumberDecimal, targetScale: number): bigint {
  const delta = targetScale - decimal.scale;
  const magnitude =
    delta >= 0 ? decimal.digits * 10n ** BigInt(delta) : decimal.digits / 10n ** BigInt(-delta);
  return decimal.negative ? -magnitude : magnitude;
}

function decimalFromNumber(value: number): NumberDecimal | null {
  if (!isFiniteNumber(value)) {
    return null;
  }

  return parseNumberDecimal(formatShortestDecimal(value));
}

function commonScale(...scales: number[]): number {
  return Math.max(0, ...scales);
}

export function formatNumberDecimal(decimal: NumberDecimal, precision: number | null): string {
  if (precision !== null) {
    const scaled = rescaleDigits(decimal, precision);
    const negative = scaled < 0n;
    const abs = negative ? -scaled : scaled;
    const raw = abs.toString().padStart(precision + 1, "0");
    const intPart = precision === 0 ? raw : raw.slice(0, -precision) || "0";
    const fracPart = precision === 0 ? "" : raw.slice(-precision);
    const body = precision === 0 ? intPart : `${intPart}.${fracPart}`;
    return negative && abs !== 0n ? `-${body}` : body;
  }

  if (decimal.digits === 0n) {
    return "0";
  }

  const raw = decimal.digits.toString().padStart(decimal.scale + 1, "0");
  let intPart = decimal.scale === 0 ? raw : raw.slice(0, -decimal.scale) || "0";
  let fracPart = decimal.scale === 0 ? "" : raw.slice(-decimal.scale);
  fracPart = fracPart.replace(/0+$/, "");
  intPart = intPart.replace(/^0+(?=\d)/, "") || "0";
  const body = fracPart === "" ? intPart : `${intPart}.${fracPart}`;
  return decimal.negative ? `-${body}` : body;
}

/** Shortest canonical decimal for a finite number. */
export function formatShortestDecimal(value: number): string {
  if (!isFiniteNumber(value)) {
    return "";
  }

  if (Object.is(value, -0) || value === 0) {
    return "0";
  }

  const abs = Math.abs(value);
  let text = abs.toString();

  if (text.includes("e") || text.includes("E")) {
    text = abs.toFixed(16).replace(/\.?0+$/, "");
  }

  return value < 0 ? `-${text}` : text;
}

export function formatNumberCommitted(value: number | null, precision: number | null): string {
  if (value === null) {
    return "";
  }

  const decimal = decimalFromNumber(value);

  if (decimal === null) {
    return "";
  }

  return formatNumberDecimal(decimal, precision);
}

export function numberInBounds(value: number, min: number | null, max: number | null): boolean {
  if (min !== null && value < min) {
    return false;
  }

  if (max !== null && value > max) {
    return false;
  }

  return true;
}

export function numberStepAligned(
  value: number,
  min: number | null,
  step: number | null,
): boolean {
  const effectiveStep = numberInputEffectiveStep(step);

  if (!isFiniteNumber(effectiveStep) || effectiveStep <= 0) {
    return false;
  }

  const valueDecimal = decimalFromNumber(value);
  const originDecimal = decimalFromNumber(min ?? 0);
  const stepDecimal = decimalFromNumber(effectiveStep);

  if (valueDecimal === null || originDecimal === null || stepDecimal === null) {
    return false;
  }

  const scale = commonScale(valueDecimal.scale, originDecimal.scale, stepDecimal.scale);
  const valueDigits = rescaleDigits(valueDecimal, scale);
  const originDigits = rescaleDigits(originDecimal, scale);
  const stepDigits = rescaleDigits(stepDecimal, scale);

  if (stepDigits === 0n) {
    return false;
  }

  const delta = valueDigits - originDigits;
  return delta % stepDigits === 0n;
}

export function numberPrecisionOk(fractionalDigits: number, precision: number | null): boolean {
  return precision === null || fractionalDigits <= precision;
}

export function numberDraftConstraintValid(
  text: string,
  min: number | null,
  max: number | null,
  step: number | null,
  precision: number | null,
): boolean {
  const classified = classifyNumberDraft(text);

  if (classified.kind !== "complete" || classified.decimal === null || classified.fractionalDigits === null) {
    return false;
  }

  if (!numberPrecisionOk(classified.fractionalDigits, precision)) {
    return false;
  }

  const value = numberDecimalToNumber(classified.decimal);

  return (
    isFiniteNumber(value) &&
    numberInBounds(value, min, max) &&
    numberStepAligned(value, min, step)
  );
}

export function numberDecimalToNumber(decimal: NumberDecimal): number {
  return Number(formatNumberDecimal(decimal, null));
}

export function numberValueConstraintValid(
  value: number | null,
  min: number | null,
  max: number | null,
  step: number | null,
  precision: number | null,
): boolean {
  if (value === null) {
    return true;
  }

  if (!isFiniteNumber(value)) {
    return false;
  }

  const text = formatNumberCommitted(value, precision);
  const classified = classifyNumberDraft(text);

  if (classified.kind !== "complete" || classified.fractionalDigits === null) {
    return false;
  }

  return (
    numberPrecisionOk(classified.fractionalDigits, precision) &&
    numberInBounds(value, min, max) &&
    numberStepAligned(value, min, step)
  );
}

function lastOnGrid(min: number | null, max: number, step: number | null): number | null {
  const origin = min ?? 0;
  const effectiveStep = numberInputEffectiveStep(step);
  const originDecimal = decimalFromNumber(origin);
  const maxDecimal = decimalFromNumber(max);
  const stepDecimal = decimalFromNumber(effectiveStep);

  if (originDecimal === null || maxDecimal === null || stepDecimal === null) {
    return null;
  }

  const scale = commonScale(originDecimal.scale, maxDecimal.scale, stepDecimal.scale);
  const originDigits = rescaleDigits(originDecimal, scale);
  const maxDigits = rescaleDigits(maxDecimal, scale);
  const stepDigits = rescaleDigits(stepDecimal, scale);

  if (stepDigits <= 0n) {
    return null;
  }

  const delta = maxDigits - originDigits;
  const remainder = ((delta % stepDigits) + stepDigits) % stepDigits;
  const lastDigits = maxDigits - remainder;
  const negative = lastDigits < 0n;
  const abs = negative ? -lastDigits : lastDigits;
  const decimal: NumberDecimal = {
    negative,
    digits: abs,
    scale,
  };
  const value = numberDecimalToNumber(decimal);

  return numberValueConstraintValid(value, min, max, step, null) ? value : null;
}

export function stepNumberValue(
  current: number | null,
  direction: 1 | -1,
  min: number | null,
  max: number | null,
  step: number | null,
  precision: number | null,
): number | null {
  const effectiveStep = numberInputEffectiveStep(step);

  if (!numberInputConfigValid({ min, max, step, precision })) {
    return null;
  }

  if (current === null) {
    if (direction > 0) {
      const origin = min ?? 0;
      return numberValueConstraintValid(origin, min, max, step, precision) ? origin : null;
    }

    if (max !== null) {
      return lastOnGrid(min, max, step);
    }

    const origin = min ?? 0;
    const candidate = origin - effectiveStep;
    return numberValueConstraintValid(candidate, min, max, step, precision) ? candidate : null;
  }

  const currentDecimal = decimalFromNumber(current);
  const stepDecimal = decimalFromNumber(effectiveStep);

  if (currentDecimal === null || stepDecimal === null) {
    return null;
  }

  const scale = commonScale(currentDecimal.scale, stepDecimal.scale, precision ?? 0);
  const currentDigits = rescaleDigits(currentDecimal, scale);
  const stepDigits = rescaleDigits(stepDecimal, scale);
  const nextDigits = currentDigits + BigInt(direction) * stepDigits;
  const negative = nextDigits < 0n;
  const abs = negative ? -nextDigits : nextDigits;
  const next = numberDecimalToNumber({ negative, digits: abs, scale });

  if (!numberValueConstraintValid(next, min, max, step, precision)) {
    return null;
  }

  return next;
}

function activeDraftText(context: NumberInputContext): string | null {
  return context.draft;
}

function displayText(context: NumberInputContext): string {
  if (context.draft !== null) {
    return context.draft;
  }

  return formatNumberCommitted(context.committed, context.precision);
}

function validDraftValue(context: NumberInputContext, text: string): number | null {
  if (!numberDraftConstraintValid(text, context.min, context.max, context.step, context.precision)) {
    return null;
  }

  const decimal = parseNumberDecimal(text);
  return decimal === null ? null : numberDecimalToNumber(decimal);
}

function pushUnique(
  effects: NumberInputEffect[],
  effect: NumberInputEffect,
): void {
  const last = effects[effects.length - 1];

  if (
    last &&
    last.type === effect.type &&
    JSON.stringify(last) === JSON.stringify(effect)
  ) {
    return;
  }

  effects.push(effect);
}

function resolveCommitted(
  context: NumberInputContext,
  value: number | null,
  options: { commit: boolean; draft: string | null },
): NumberInputResult {
  const effects: NumberInputEffect[] = [];
  let next: NumberInputContext = {
    ...context,
    committed: value,
    draft: options.draft,
  };

  if (options.draft !== context.draft) {
    pushUnique(effects, { type: "emitDraftValueChange", draft: options.draft });
  }

  if (value !== context.committed) {
    pushUnique(effects, { type: "emitValueChange", value });
  }

  if (options.commit) {
    pushUnique(effects, { type: "emitCommit", value });
  }

  // After a successful resolve, normalize draft display through committed formatting.
  if (options.draft === null && value !== null) {
    next = { ...next, draft: null };
  }

  return { context: next, effects };
}

function setDraft(context: NumberInputContext, draft: string, emitValue: number | null | undefined): NumberInputResult {
  const effects: NumberInputEffect[] = [];
  const next = { ...context, draft };

  if (draft !== context.draft) {
    pushUnique(effects, { type: "emitDraftValueChange", draft });
  }

  if (emitValue !== undefined && emitValue !== context.committed) {
    next.committed = emitValue;
    pushUnique(effects, { type: "emitValueChange", value: emitValue });
  }

  return { context: next, effects };
}

function discardDraft(context: NumberInputContext): NumberInputResult {
  if (context.draft === null) {
    return idle(context);
  }

  return {
    context: { ...context, draft: null },
    effects: [{ type: "emitDraftValueChange", draft: null }],
  };
}

function commitCurrent(context: NumberInputContext): NumberInputResult {
  const draft = activeDraftText(context);

  if (draft === null) {
    return {
      context,
      effects: [{ type: "emitCommit", value: context.committed }],
    };
  }

  if (draft === "") {
    return resolveCommitted(context, null, { commit: true, draft: null });
  }

  const value = validDraftValue(context, draft);

  if (value === null) {
    return idle(context);
  }

  return resolveCommitted(context, value, { commit: true, draft: null });
}

export function numberInputInvalid(context: NumberInputContext): boolean {
  if (!numberInputConfigValid(context)) {
    return true;
  }

  const draft = activeDraftText(context);

  if (draft === null) {
    return false;
  }

  if (draft === "") {
    return false;
  }

  return validDraftValue(context, draft) === null;
}

export function numberInputContext(init: Partial<NumberInputContext> = {}): NumberInputContext {
  return {
    committed: null,
    defaultValue: null,
    draft: null,
    min: null,
    max: null,
    step: null,
    precision: null,
    disabled: false,
    readOnly: false,
    ...init,
  };
}

export function numberInputTransition(
  context: NumberInputContext,
  event: NumberInputEvent,
): NumberInputResult {
  switch (event.type) {
    case "SET_DISABLED":
      return { context: { ...context, disabled: event.disabled }, effects: [] };
    case "SET_READ_ONLY":
      return { context: { ...context, readOnly: event.readOnly }, effects: [] };
    case "SET_CONSTRAINTS":
      return {
        context: {
          ...context,
          min: event.min,
          max: event.max,
          step: event.step,
          precision: event.precision,
          defaultValue: event.defaultValue,
        },
        effects: [],
      };
    case "REPLACE":
      return {
        context: { ...context, committed: event.value, draft: null },
        effects:
          context.draft === null
            ? []
            : [{ type: "emitDraftValueChange", draft: null }],
      };
    default:
      break;
  }

  if (context.disabled || context.readOnly) {
    return idle(context);
  }

  if (!numberInputConfigValid(context) && event.type !== "REPLACE") {
    return idle(context);
  }

  switch (event.type) {
    case "RAW_EDIT": {
      const text = event.text;

      if (text === "") {
        return setDraft(context, "", null);
      }

      const classified = classifyNumberDraft(text);

      if (classified.kind === "complete" && classified.decimal !== null) {
        const value = validDraftValue(context, text);
        return setDraft(context, text, value === null ? undefined : value);
      }

      return setDraft(context, text, undefined);
    }
    case "CLEAR":
      return setDraft(context, "", null);
    case "ENTER":
      return commitCurrent(context);
    case "BLUR": {
      const draft = activeDraftText(context);

      if (draft === null) {
        return idle(context);
      }

      if (draft === "" || validDraftValue(context, draft) !== null) {
        return commitCurrent(context);
      }

      return discardDraft(context);
    }
    case "ESCAPE":
      return discardDraft(context);
    case "STEP": {
      const draft = activeDraftText(context);
      let from: number | null = context.committed;

      if (draft !== null) {
        if (draft === "") {
          from = null;
        } else {
          const draftValue = validDraftValue(context, draft);

          if (draftValue === null) {
            return idle(context);
          }

          from = draftValue;
        }
      }

      const next = stepNumberValue(
        from,
        event.direction,
        context.min,
        context.max,
        context.step,
        context.precision,
      );

      if (next === null) {
        return idle(context);
      }

      return resolveCommitted(context, next, { commit: true, draft: null });
    }
    case "HOME": {
      if (context.min === null) {
        return idle(context);
      }

      if (!numberValueConstraintValid(context.min, context.min, context.max, context.step, context.precision)) {
        return idle(context);
      }

      return resolveCommitted(context, context.min, { commit: true, draft: null });
    }
    case "END": {
      if (context.max === null) {
        return idle(context);
      }

      if (!numberValueConstraintValid(context.max, context.min, context.max, context.step, context.precision)) {
        return idle(context);
      }

      return resolveCommitted(context, context.max, { commit: true, draft: null });
    }
  }
}

export function numberInputDisplayText(context: NumberInputContext): string {
  return displayText(context);
}

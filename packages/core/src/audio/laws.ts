export type ContinuousValueLaw =
  | { type: "linear" }
  | { type: "logarithmic" }
  | { type: "exponential"; exponent: number }
  | { type: "bipolar-center"; center: number };

export type AudioValueLaw = ContinuousValueLaw | {
  type: "stepped";
  step: number;
  law?: ContinuousValueLaw;
};

export const linearValueLaw: ContinuousValueLaw = { type: "linear" };

export function clampAudioValue(value: number, min: number, max: number): number {
  if (max <= min) return min;
  return Math.min(Math.max(value, min), max);
}

function assertLaw(law: ContinuousValueLaw, min: number, max: number): void {
  if (law.type === "logarithmic" && !(min > 0 && max > min)) {
    throw new RangeError("logarithmic laws require 0 < min < max");
  }
  if (law.type === "exponential" && !(law.exponent > 0 && Number.isFinite(law.exponent))) {
    throw new RangeError("exponential laws require a finite exponent greater than zero");
  }
  if (law.type === "bipolar-center" && !(law.center > min && law.center < max)) {
    throw new RangeError("bipolar-center laws require min < center < max");
  }
}

function continuousLaw(law: AudioValueLaw): ContinuousValueLaw {
  return law.type === "stepped" ? law.law ?? linearValueLaw : law;
}

export function snapAudioValue(value: number, min: number, step: number): number {
  if (!(step > 0) || !Number.isFinite(step)) return value;
  const snapped = min + Math.round((value - min) / step) * step;
  const [coefficient, exponentText] = step.toString().toLowerCase().split("e");
  const coefficientDecimals = coefficient?.split(".")[1]?.length ?? 0;
  const exponent = Number(exponentText ?? 0);
  const precision = Math.max(0, coefficientDecimals - exponent);
  return Number(snapped.toFixed(Math.min(precision, 12)));
}

export function constrainAudioValue(value: number, min: number, max: number, law: AudioValueLaw): number {
  const stepped = law.type === "stepped" ? snapAudioValue(value, min, law.step) : value;
  return clampAudioValue(stepped, min, max);
}

export function normalizeAudioValue(value: number, min: number, max: number, law: AudioValueLaw = linearValueLaw): number {
  if (max <= min) return 0;
  const base = continuousLaw(law);
  assertLaw(base, min, max);
  const plain = constrainAudioValue(value, min, max, law);

  switch (base.type) {
    case "linear":
      return (plain - min) / (max - min);
    case "logarithmic":
      return Math.log(plain / min) / Math.log(max / min);
    case "exponential":
      return Math.pow((plain - min) / (max - min), 1 / base.exponent);
    case "bipolar-center":
      return plain <= base.center
        ? ((plain - min) / (base.center - min)) * 0.5
        : 0.5 + ((plain - base.center) / (max - base.center)) * 0.5;
  }
}

export function denormalizeAudioValue(norm: number, min: number, max: number, law: AudioValueLaw = linearValueLaw): number {
  if (max <= min) return min;
  const base = continuousLaw(law);
  assertLaw(base, min, max);
  const position = Math.min(Math.max(norm, 0), 1);
  let value: number;

  switch (base.type) {
    case "linear":
      value = min + position * (max - min);
      break;
    case "logarithmic":
      value = min * Math.pow(max / min, position);
      break;
    case "exponential":
      value = min + Math.pow(position, base.exponent) * (max - min);
      break;
    case "bipolar-center":
      value = position <= 0.5
        ? min + (position / 0.5) * (base.center - min)
        : base.center + ((position - 0.5) / 0.5) * (max - base.center);
      break;
  }

  return constrainAudioValue(value, min, max, law);
}

export function bipolarCenterForLaw(law: AudioValueLaw): number | null {
  const base = continuousLaw(law);
  return base.type === "bipolar-center" ? base.center : null;
}

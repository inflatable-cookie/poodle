/**
 * Duration entry machinery (DurationInput).
 * Contract: docs/contracts/components/duration-input.md, "Behavior Machine".
 *
 * Pure segment adjustment with carry/borrow between seconds → minutes →
 * hours; hours clamp to [0, maxHours], carries at the hour bound are
 * swallowed (matches the pre-machine component).
 */

export interface DurationValue {
  hours: number;
  minutes: number;
  seconds: number;
}

export type DurationSegment = "hours" | "minutes" | "seconds";

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

export function durationTotalSeconds(value: DurationValue): number {
  return value.hours * 3600 + value.minutes * 60 + value.seconds;
}

/** Step a segment by ±delta with carry/borrow, hours clamped to [0, maxHours]. */
export function adjustDurationSegment(
  value: DurationValue,
  segment: DurationSegment,
  delta: number,
  maxHours: number,
): DurationValue {
  let { hours, minutes, seconds } = value;

  if (segment === "hours") {
    return { hours: clamp(hours + delta, 0, maxHours), minutes, seconds };
  }

  if (segment === "minutes") {
    let nextMinutes = minutes + delta;

    if (nextMinutes >= 60) {
      nextMinutes = 0;
      hours = clamp(hours + 1, 0, maxHours);
    }

    if (nextMinutes < 0) {
      nextMinutes = 59;
      hours = clamp(hours - 1, 0, maxHours);
    }

    return { hours, minutes: nextMinutes, seconds };
  }

  let nextSeconds = seconds + delta;
  let nextMinutes = minutes;
  let nextHours = hours;

  if (nextSeconds >= 60) {
    nextSeconds = 0;
    nextMinutes += 1;
  }

  if (nextSeconds < 0) {
    nextSeconds = 59;
    nextMinutes -= 1;
  }

  if (nextMinutes >= 60) {
    nextMinutes = 0;
    nextHours = clamp(nextHours + 1, 0, maxHours);
  }

  if (nextMinutes < 0) {
    nextMinutes = 59;
    nextHours = clamp(nextHours - 1, 0, maxHours);
  }

  return { hours: nextHours, minutes: clamp(nextMinutes, 0, 59), seconds: clamp(nextSeconds, 0, 59) };
}

/** Direct segment entry: clamp into the segment's valid range. */
export function setDurationSegment(
  value: DurationValue,
  segment: DurationSegment,
  raw: number,
  maxHours: number,
): DurationValue {
  if (segment === "hours") {
    return { ...value, hours: clamp(raw, 0, maxHours) };
  }

  if (segment === "minutes") {
    return { ...value, minutes: clamp(raw, 0, 59) };
  }

  return { ...value, seconds: clamp(raw, 0, 59) };
}

export function padDurationSegment(value: number): string {
  return value.toString().padStart(2, "0");
}

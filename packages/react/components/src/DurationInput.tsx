import { useId, useState, type ChangeEvent, type FocusEvent, type KeyboardEvent } from "react";
import {
  adjustDurationSegment,
  durationTotalSeconds,
  padDurationSegment,
  setDurationSegment,
  type DurationSegment,
} from "@poodle/headless";

import "@poodle/styles/duration-input.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface DurationInputChange {
  hours: number;
  minutes: number;
  seconds: number;
  totalSeconds: number;
}

export interface DurationInputProps {
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  hours?: number;
  minutes?: number;
  seconds?: number;
  defaultHours?: number;
  defaultMinutes?: number;
  defaultSeconds?: number;
  showSeconds?: boolean;
  maxHours?: number;
  minTotalSeconds?: number;
  maxTotalSeconds?: number | null;
  disabled?: boolean;
  ariaLabel?: string;
  onChange?: (detail: DurationInputChange) => void;
}

export function DurationInput({
  size = null,
  sizeRole = "control",
  density = null,
  hours,
  minutes,
  seconds,
  defaultHours = 0,
  defaultMinutes = 0,
  defaultSeconds = 0,
  showSeconds = true,
  maxHours = 99,
  minTotalSeconds = 0,
  maxTotalSeconds = null,
  disabled = false,
  ariaLabel = "Duration",
  onChange,
}: DurationInputProps) {
  const uiPresentation = useUiPresentation();
  const idBase = useId();

  const [uncontrolled, setUncontrolled] = useState({
    hours: defaultHours,
    minutes: defaultMinutes,
    seconds: defaultSeconds,
  });

  const isControlled = hours !== undefined || minutes !== undefined || seconds !== undefined;
  const current = isControlled
    ? { hours: hours ?? 0, minutes: minutes ?? 0, seconds: seconds ?? 0 }
    : uncontrolled;

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const totalSeconds = durationTotalSeconds(current);
  const isUnderMin = totalSeconds < minTotalSeconds;
  const isOverMax = maxTotalSeconds !== null && totalSeconds > maxTotalSeconds;

  function applyValue(next: { hours: number; minutes: number; seconds: number }): void {
    if (!isControlled) setUncontrolled(next);
    onChange?.({ ...next, totalSeconds: durationTotalSeconds(next) });
  }

  function handleSegmentKeydown(event: KeyboardEvent<HTMLInputElement>, segment: DurationSegment): void {
    if (disabled) return;
    if (event.key === "ArrowUp") {
      event.preventDefault();
      applyValue(adjustDurationSegment(current, segment, 1, maxHours));
    }
    if (event.key === "ArrowDown") {
      event.preventDefault();
      applyValue(adjustDurationSegment(current, segment, -1, maxHours));
    }
  }

  function handleSegmentInput(event: ChangeEvent<HTMLInputElement>, segment: DurationSegment): void {
    const val = parseInt(event.currentTarget.value, 10);
    if (Number.isNaN(val)) return;
    applyValue(setDurationSegment(current, segment, val, maxHours));
  }

  function selectInputText(event: FocusEvent<HTMLInputElement>): void {
    event.currentTarget.select();
  }

  const separator = (
    <span className="poodle-duration-input__separator" aria-hidden="true">
      <span className="poodle-duration-input__separator-spacer" />
      <span className="poodle-duration-input__separator-glyph">:</span>
    </span>
  );

  function segment(name: DurationSegment, shortLabel: string, ariaSegLabel: string, value: number) {
    const id = `${idBase}-${name}`;
    return (
      <div className="poodle-duration-input__segment">
        <label className="poodle-duration-input__label" htmlFor={id}>
          {shortLabel}
        </label>
        <input
          id={id}
          type="text"
          inputMode="numeric"
          className="poodle-duration-input__field"
          value={padDurationSegment(value)}
          disabled={disabled}
          aria-label={ariaSegLabel}
          onChange={(e) => handleSegmentInput(e, name)}
          onKeyDown={(e) => handleSegmentKeydown(e, name)}
          onFocus={selectInputText}
        />
      </div>
    );
  }

  return (
    <div
      className="poodle-duration-input"
      role="group"
      aria-label={ariaLabel}
      data-disabled={disabled}
      data-invalid={isUnderMin || isOverMax}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {segment("hours", "h", "Hours", current.hours)}
      {separator}
      {segment("minutes", "m", "Minutes", current.minutes)}
      {showSeconds ? (
        <>
          {separator}
          {segment("seconds", "s", "Seconds", current.seconds)}
        </>
      ) : null}
    </div>
  );
}

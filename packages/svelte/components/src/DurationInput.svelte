<script lang="ts">
  import "@inflatable-cookie/poodle-styles/duration-input.css";
  import {
    adjustDurationSegment,
    durationTotalSeconds,
    padDurationSegment,
    setDurationSegment,
    type DurationSegment,
  } from "@inflatable-cookie/poodle-headless";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface DurationInputChange {
    hours: number;
    minutes: number;
    seconds: number;
    totalSeconds: number;
  }

  interface Props {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    hours?: number;
    minutes?: number;
    seconds?: number;
    showSeconds?: boolean;
    maxHours?: number;
    minTotalSeconds?: number;
    maxTotalSeconds?: number | null;
    disabled?: boolean;
    ariaLabel?: string;
    onChange?: ((detail: DurationInputChange) => void) | undefined;
  }

  const uiPresentation = getUiPresentation();

  let {
    size = null,
    sizeRole = "control",
    density = null,
    hours = $bindable(0),
    minutes = $bindable(0),
    seconds = $bindable(0),
    showSeconds = true,
    maxHours = 99,
    minTotalSeconds = 0,
    maxTotalSeconds = null,
    disabled = false,
    ariaLabel = "Duration",
    onChange = undefined,
  }: Props = $props();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const totalSeconds = $derived(durationTotalSeconds({ hours, minutes, seconds }));
  const isUnderMin = $derived(totalSeconds < minTotalSeconds);
  const isOverMax = $derived(maxTotalSeconds !== null && totalSeconds > maxTotalSeconds);

  function emitChange(h: number, m: number, s: number): void {
    hours = h;
    minutes = m;
    seconds = s;
    onChange?.({
      hours: h,
      minutes: m,
      seconds: s,
      totalSeconds: durationTotalSeconds({ hours: h, minutes: m, seconds: s }),
    });
  }

  function applyValue(next: { hours: number; minutes: number; seconds: number }): void {
    emitChange(next.hours, next.minutes, next.seconds);
  }

  function handleSegmentKeydown(event: KeyboardEvent, segment: DurationSegment): void {
    if (disabled) return;

    if (event.key === "ArrowUp") {
      event.preventDefault();
      applyValue(adjustDurationSegment({ hours, minutes, seconds }, segment, 1, maxHours));
    }

    if (event.key === "ArrowDown") {
      event.preventDefault();
      applyValue(adjustDurationSegment({ hours, minutes, seconds }, segment, -1, maxHours));
    }
  }

  function handleSegmentInput(event: Event, segment: DurationSegment): void {
    const raw = (event.currentTarget as HTMLInputElement).value;
    const val = parseInt(raw, 10);
    if (Number.isNaN(val)) return;

    applyValue(setDurationSegment({ hours, minutes, seconds }, segment, val, maxHours));
  }

  function pad(n: number): string {
    return padDurationSegment(n);
  }

  function selectInputText(event: FocusEvent): void {
    if (event.currentTarget instanceof HTMLInputElement) {
      event.currentTarget.select();
    }
  }
</script>

<div
  class="poodle-duration-input"
  role="group"
  aria-label={ariaLabel}
  data-disabled={disabled}
  data-invalid={isUnderMin || isOverMax}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <div class="poodle-duration-input__segment">
    <label class="poodle-duration-input__label" for="dur-hours">h</label>
    <input
      id="dur-hours"
      type="text"
      inputmode="numeric"
      class="poodle-duration-input__field"
      value={pad(hours)}
      disabled={disabled}
      aria-label="Hours"
      oninput={(e) => handleSegmentInput(e, "hours")}
      onkeydown={(e) => handleSegmentKeydown(e, "hours")}
      onfocus={selectInputText}
    />
  </div>

  <span class="poodle-duration-input__separator" aria-hidden="true">
    <span class="poodle-duration-input__separator-spacer"></span>
    <span class="poodle-duration-input__separator-glyph">:</span>
  </span>

  <div class="poodle-duration-input__segment">
    <label class="poodle-duration-input__label" for="dur-minutes">m</label>
    <input
      id="dur-minutes"
      type="text"
      inputmode="numeric"
      class="poodle-duration-input__field"
      value={pad(minutes)}
      disabled={disabled}
      aria-label="Minutes"
      oninput={(e) => handleSegmentInput(e, "minutes")}
      onkeydown={(e) => handleSegmentKeydown(e, "minutes")}
      onfocus={selectInputText}
    />
  </div>

  {#if showSeconds}
    <span class="poodle-duration-input__separator" aria-hidden="true">
      <span class="poodle-duration-input__separator-spacer"></span>
      <span class="poodle-duration-input__separator-glyph">:</span>
    </span>

    <div class="poodle-duration-input__segment">
      <label class="poodle-duration-input__label" for="dur-seconds">s</label>
      <input
        id="dur-seconds"
        type="text"
        inputmode="numeric"
        class="poodle-duration-input__field"
        value={pad(seconds)}
        disabled={disabled}
        aria-label="Seconds"
        oninput={(e) => handleSegmentInput(e, "seconds")}
        onkeydown={(e) => handleSegmentKeydown(e, "seconds")}
        onfocus={selectInputText}
      />
    </div>
  {/if}
</div>


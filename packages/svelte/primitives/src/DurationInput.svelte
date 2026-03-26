<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let hours = 0;
  export let minutes = 0;
  export let seconds = 0;
  export let showSeconds = true;
  export let maxHours = 99;
  export let minTotalSeconds = 0;
  export let maxTotalSeconds: number | null = null;
  export let disabled = false;
  export let ariaLabel = "Duration";

  const uiPresentation = getUiPresentation();

  const dispatch = createEventDispatcher<{
    change: { hours: number; minutes: number; seconds: number; totalSeconds: number };
  }>();

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";
  $: totalSeconds = hours * 3600 + minutes * 60 + seconds;
  $: isUnderMin = totalSeconds < minTotalSeconds;
  $: isOverMax = maxTotalSeconds !== null && totalSeconds > maxTotalSeconds;

  function clamp(val: number, min: number, max: number): number {
    return Math.min(Math.max(val, min), max);
  }

  function emitChange(h: number, m: number, s: number): void {
    hours = h;
    minutes = m;
    seconds = s;
    dispatch("change", {
      hours: h,
      minutes: m,
      seconds: s,
      totalSeconds: h * 3600 + m * 60 + s,
    });
  }

  function adjustHours(delta: number): void {
    emitChange(clamp(hours + delta, 0, maxHours), minutes, seconds);
  }

  function adjustMinutes(delta: number): void {
    let m = minutes + delta;
    let h = hours;
    if (m >= 60) { m = 0; h = clamp(h + 1, 0, maxHours); }
    if (m < 0) { m = 59; h = clamp(h - 1, 0, maxHours); }
    emitChange(h, m, seconds);
  }

  function adjustSeconds(delta: number): void {
    let s = seconds + delta;
    let m = minutes;
    let h = hours;
    if (s >= 60) { s = 0; m += 1; }
    if (s < 0) { s = 59; m -= 1; }
    if (m >= 60) { m = 0; h = clamp(h + 1, 0, maxHours); }
    if (m < 0) { m = 59; h = clamp(h - 1, 0, maxHours); }
    emitChange(h, clamp(m, 0, 59), clamp(s, 0, 59));
  }

  function handleSegmentKeydown(
    event: KeyboardEvent,
    segment: "hours" | "minutes" | "seconds"
  ): void {
    if (disabled) return;

    if (event.key === "ArrowUp") {
      event.preventDefault();
      if (segment === "hours") adjustHours(1);
      else if (segment === "minutes") adjustMinutes(1);
      else adjustSeconds(1);
    }

    if (event.key === "ArrowDown") {
      event.preventDefault();
      if (segment === "hours") adjustHours(-1);
      else if (segment === "minutes") adjustMinutes(-1);
      else adjustSeconds(-1);
    }
  }

  function handleSegmentInput(
    event: Event,
    segment: "hours" | "minutes" | "seconds"
  ): void {
    const raw = (event.currentTarget as HTMLInputElement).value;
    const val = parseInt(raw, 10);
    if (Number.isNaN(val)) return;

    if (segment === "hours") {
      emitChange(clamp(val, 0, maxHours), minutes, seconds);
    } else if (segment === "minutes") {
      emitChange(hours, clamp(val, 0, 59), seconds);
    } else {
      emitChange(hours, minutes, clamp(val, 0, 59));
    }
  }

  function pad(n: number): string {
    return n.toString().padStart(2, "0");
  }
</script>

<div
  class="duration-input"
  role="group"
  aria-label={ariaLabel}
  data-disabled={disabled}
  data-invalid={isUnderMin || isOverMax}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <div class="duration-input__segment">
    <label class="duration-input__label" for="dur-hours">h</label>
    <input
      id="dur-hours"
      type="text"
      inputmode="numeric"
      class="duration-input__field"
      value={pad(hours)}
      disabled={disabled}
      aria-label="Hours"
      on:input={(e) => handleSegmentInput(e, "hours")}
      on:keydown={(e) => handleSegmentKeydown(e, "hours")}
      on:focus={(e) => (e.currentTarget as HTMLInputElement).select()}
    />
  </div>

  <span class="duration-input__separator" aria-hidden="true">:</span>

  <div class="duration-input__segment">
    <label class="duration-input__label" for="dur-minutes">m</label>
    <input
      id="dur-minutes"
      type="text"
      inputmode="numeric"
      class="duration-input__field"
      value={pad(minutes)}
      disabled={disabled}
      aria-label="Minutes"
      on:input={(e) => handleSegmentInput(e, "minutes")}
      on:keydown={(e) => handleSegmentKeydown(e, "minutes")}
      on:focus={(e) => (e.currentTarget as HTMLInputElement).select()}
    />
  </div>

  {#if showSeconds}
    <span class="duration-input__separator" aria-hidden="true">:</span>

    <div class="duration-input__segment">
      <label class="duration-input__label" for="dur-seconds">s</label>
      <input
        id="dur-seconds"
        type="text"
        inputmode="numeric"
        class="duration-input__field"
        value={pad(seconds)}
        disabled={disabled}
        aria-label="Seconds"
        on:input={(e) => handleSegmentInput(e, "seconds")}
        on:keydown={(e) => handleSegmentKeydown(e, "seconds")}
        on:focus={(e) => (e.currentTarget as HTMLInputElement).select()}
      />
    </div>
  {/if}
</div>

<style>
  .duration-input {
    display: inline-flex;
    align-items: flex-end;
    gap: 0.125rem;
    width: fit-content;
    padding: 0.25rem var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    font-family: var(--poodle-typography-code-family);
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .duration-input:focus-within {
    border-color: var(--poodle-color-accent-focusRing);
    box-shadow: 0 0 0 var(--poodle-border-width-focus) color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent);
  }

  .duration-input[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .duration-input[data-invalid="true"] {
    border-color: var(--poodle-color-status-danger);
  }

  .duration-input__segment {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.125rem;
    padding: 0.125rem;
    border-radius: 0.1875rem;
  }

  .duration-input__segment:has(.duration-input__field:focus) {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
  }

  .duration-input__label {
    font-size: 0.5625rem;
    color: var(--poodle-color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    line-height: 1;
    user-select: none;
  }

  .duration-input__field {
    width: 1.75rem;
    min-height: 0;
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-code-family);
    font-size: var(--poodle-typography-body-size);
    font-variant-numeric: tabular-nums;
    text-align: center;
    line-height: 1;
    outline: none;
  }

  .duration-input__separator {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size);
    font-weight: 600;
    line-height: 1;
    user-select: none;
  }

  /* Size variants */
  .duration-input[data-size="xs"] {
    padding: 0.125rem calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .duration-input[data-size="xs"] .duration-input__field {
    width: 1.5rem;
    font-size: 0.75rem;
  }

  .duration-input[data-size="xs"] .duration-input__label {
    font-size: 0.5rem;
  }

  .duration-input[data-size="sm"] {
    padding: 0.1875rem calc(var(--poodle-space-control-x) - 0.0625rem);
  }

  .duration-input[data-size="lg"] {
    padding: 0.3125rem calc(var(--poodle-space-control-x) + 0.125rem);
  }

  .duration-input[data-size="lg"] .duration-input__field {
    width: 2rem;
    font-size: 0.9375rem;
  }

  .duration-input[data-size="xl"] {
    padding: 0.375rem calc(var(--poodle-space-control-x) + 0.1875rem);
  }

  .duration-input[data-size="xl"] .duration-input__field {
    width: 2.25rem;
    font-size: 1rem;
  }

  /* Density variants */
  .duration-input[data-density="compact"] { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); gap: 0.125rem; }
  .duration-input[data-density="comfortable"] { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); gap: 0.375rem; }
</style>

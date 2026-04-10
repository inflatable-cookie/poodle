<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    SwitchTone,
  } from "./types";

  export let id: string | undefined = undefined;
  export let checked: boolean | null = null;
  export let defaultChecked = false;
  export let disabled = false;
  export let readOnly = false;
  export let label: string | null = null;
  export let leftLabel: string | null = null;
  export let rightLabel: string | null = null;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let name: string | undefined = undefined;
  export let offColor: string | null = null;
  export let onColor: string | null = null;
  export let leftTone: SwitchTone = "default";
  export let rightTone: SwitchTone = "primary";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    checkedChange: { checked: boolean };
    checked: boolean;
  }>();

  const uiPresentation = getUiPresentation();
  let uncontrolledChecked = defaultChecked;

  $: isControlled = checked !== null;
  $: currentChecked = isControlled ? checked === true : uncontrolledChecked;
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: resolvedOffColor = offColor ?? toneToColor(leftTone);
  $: resolvedOnColor = onColor ?? toneToColor(rightTone);
  $: fallbackAriaLabel = [leftLabel, rightLabel]
    .filter((value): value is string => Boolean(value && value.trim()))
    .join(" / ");
  $: computedAriaLabel =
    label ??
    ariaLabel ??
    (fallbackAriaLabel || null);
  $: switchStyles = [
    resolvedOffColor ? `--poodle-switch-off-color: ${resolvedOffColor}` : "",
    resolvedOnColor ? `--poodle-switch-on-color: ${resolvedOnColor}` : "",
  ].filter(Boolean).join("; ") || undefined;

  function toneToColor(tone: SwitchTone): string | null {
    switch (tone) {
      case "primary":
        return "var(--poodle-color-accent-base)";
      case "success":
        return "var(--poodle-color-status-success)";
      case "warning":
        return "var(--poodle-color-status-warning)";
      case "danger":
        return "var(--poodle-color-status-danger)";
      default:
        return null;
    }
  }

  function handleChange(event: Event): void {
    const nextChecked = (event.currentTarget as HTMLInputElement).checked;

    if (readOnly) {
      (event.currentTarget as HTMLInputElement).checked = currentChecked;
      return;
    }

    if (!isControlled) {
      uncontrolledChecked = nextChecked;
    }

    dispatch("checkedChange", { checked: nextChecked });
    dispatch("checked", nextChecked);
  }
</script>

<label
  class="switch"
  data-disabled={disabled}
  data-read-only={readOnly}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-dual-label={leftLabel || rightLabel ? "true" : undefined}
  style={switchStyles}
>
  <input
    {id}
    {name}
    class="switch__control"
    type="checkbox"
    role="switch"
    checked={currentChecked}
    disabled={disabled}
    aria-label={computedAriaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    aria-readonly={readOnly ? "true" : undefined}
    on:change={handleChange}
  />
  {#if leftLabel}
    <span class="switch__label switch__label--left">{leftLabel}</span>
  {/if}
  <span class="switch__track" aria-hidden="true">
    <span class="switch__thumb"></span>
  </span>
  {#if rightLabel}
    <span class="switch__label switch__label--right">{rightLabel}</span>
  {:else if label}
    <span class="switch__label">{label}</span>
  {/if}
</label>

<style>
  .switch {
    --switch-unit: var(--poodle-size-icon-md);
    --poodle-switch-off-color: var(--poodle-color-text-primary);
    --poodle-switch-on-color: var(--poodle-color-accent-base);
    --poodle-switch-off-track: color-mix(in srgb, var(--poodle-switch-off-color) 18%, var(--poodle-color-background-surface));
    --poodle-switch-on-track: color-mix(in srgb, var(--poodle-switch-on-color) 24%, var(--poodle-color-background-surface));
    --poodle-switch-off-thumb: var(--poodle-switch-off-color);
    --poodle-switch-on-thumb: var(--poodle-switch-on-color);
    --poodle-switch-off-border: var(--poodle-color-border-default);
    --poodle-switch-on-border: color-mix(in srgb, var(--poodle-switch-on-thumb) 58%, var(--poodle-color-border-default));
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
  }

  .switch[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .switch[data-read-only="true"] {
    cursor: default;
  }

  .switch__control {
    position: absolute;
    width: 1px;
    height: 1px;
    margin: -1px;
    padding: 0;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  .switch__track {
    display: inline-flex;
    align-items: center;
    width: calc(var(--switch-unit) * 2 + 0.125rem);
    height: calc(var(--switch-unit) + 0.25rem);
    padding: 0.125rem;
    border: 0.0625rem solid var(--poodle-switch-off-border);
    border-radius: 999px;
    background: var(--poodle-switch-off-track);
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent);
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .switch__thumb {
    width: calc(var(--switch-unit) - 0.125rem);
    height: calc(var(--switch-unit) - 0.125rem);
    border-radius: 999px;
    background: var(--poodle-switch-off-thumb);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
    transform: translateX(0);
    transition:
      transform var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .switch__control:checked ~ .switch__track {
    border-color: var(--poodle-switch-on-border);
    background: var(--poodle-switch-on-track);
  }

  .switch__control:checked ~ .switch__track .switch__thumb {
    background: var(--poodle-switch-on-thumb);
    transform: translateX(calc(var(--switch-unit) - 0.125rem));
  }

  .switch__control:focus-visible + .switch__track {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .switch__label {
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  .switch__label--left,
  .switch__label--right {
    color: var(--poodle-color-text-muted);
    transition: color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .switch:not([data-dual-label]) .switch__label {
    color: var(--poodle-color-text-primary);
  }

  .switch__control:not(:checked) ~ .switch__label--left {
    color: var(--poodle-switch-off-color);
  }

  .switch__control:checked ~ .switch__label--right {
    color: var(--poodle-switch-on-color);
  }

  /* Density variants */
  .switch[data-density="compact"] {
    gap: 0.25rem;
  }

  .switch[data-density="comfortable"] {
    gap: var(--poodle-space-inline-md);
  }

  /* Size variants — set --switch-unit per size, geometry calcs use it */
  .switch[data-size="xs"] { --switch-unit: var(--poodle-size-icon-xs); }
  .switch[data-size="xs"] .switch__track {
    width: calc(var(--switch-unit) * 1.75);
    height: calc(var(--switch-unit) * 0.875);
    padding: 0.0625rem;
  }

  .switch[data-size="xs"] .switch__thumb {
    width: calc(var(--switch-unit) * 0.75 - 0.125rem);
    height: calc(var(--switch-unit) * 0.75 - 0.125rem);
  }

  .switch[data-size="xs"] .switch__control:checked ~ .switch__track .switch__thumb {
    transform: translateX(calc(var(--switch-unit) * 0.875));
  }

  .switch[data-size="sm"] { --switch-unit: var(--poodle-size-icon-sm); }
  .switch[data-size="sm"] .switch__track {
    width: calc(var(--switch-unit) * 1.875);
    height: calc(var(--switch-unit) + 0.125rem);
    padding: 0.09375rem;
  }

  .switch[data-size="sm"] .switch__thumb {
    width: calc(var(--switch-unit) - 0.1875rem);
    height: calc(var(--switch-unit) - 0.1875rem);
  }

  .switch[data-size="sm"] .switch__control:checked ~ .switch__track .switch__thumb {
    transform: translateX(calc(var(--switch-unit) - 0.0625rem));
  }

  .switch[data-size="lg"] { --switch-unit: var(--poodle-size-icon-lg); }
  .switch[data-size="lg"] .switch__track {
    width: calc(var(--switch-unit) * 2.25 + 0.25rem);
    height: calc(var(--switch-unit) + 0.5rem);
    padding: 0.1875rem;
  }

  .switch[data-size="lg"] .switch__thumb {
    width: var(--switch-unit);
    height: var(--switch-unit);
  }

  .switch[data-size="lg"] .switch__control:checked ~ .switch__track .switch__thumb {
    transform: translateX(calc(var(--switch-unit) + 0.0625rem));
  }

  .switch[data-size="xl"] { --switch-unit: var(--poodle-size-icon-xl); }
  .switch[data-size="xl"] .switch__track {
    width: calc(var(--switch-unit) * 2.5 + 0.375rem);
    height: calc(var(--switch-unit) + 0.75rem);
    padding: 0.25rem;
  }

  .switch[data-size="xl"] .switch__thumb {
    width: calc(var(--switch-unit) + 0.125rem);
    height: calc(var(--switch-unit) + 0.125rem);
  }

  .switch[data-size="xl"] .switch__control:checked ~ .switch__track .switch__thumb {
    transform: translateX(calc(var(--switch-unit) + 0.125rem));
  }

  /* Label size variants */
  .switch[data-size="xs"] .switch__label { font-size: 0.6875rem; }
  .switch[data-size="sm"] .switch__label { font-size: 0.75rem; }
  .switch[data-size="lg"] .switch__label { font-size: 0.875rem; }
  .switch[data-size="xl"] .switch__label { font-size: 0.9375rem; }
</style>

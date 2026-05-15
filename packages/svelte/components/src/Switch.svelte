<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    SwitchTone,
  } from "./types";

  interface Props {
    id?: string | undefined;
    checked?: boolean | undefined;
    defaultChecked?: boolean;
    disabled?: boolean;
    readOnly?: boolean;
    label?: string | null;
    leftLabel?: string | null;
    rightLabel?: string | null;
    ariaLabel?: string | null;
    describedBy?: string | null;
    name?: string | undefined;
    offColor?: string | null;
    onColor?: string | null;
    leftTone?: SwitchTone;
    rightTone?: SwitchTone;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onCheckedChange?: ((checked: boolean) => void) | undefined;
  }

  const uiPresentation = getUiPresentation();

  let {
    id = undefined,
    checked = $bindable<boolean | undefined>(undefined),
    defaultChecked = false,
    disabled = false,
    readOnly = false,
    label = null,
    leftLabel = null,
    rightLabel = null,
    ariaLabel = null,
    describedBy = null,
    name = undefined,
    offColor = null,
    onColor = null,
    leftTone = "default",
    rightTone = "primary",
    size = null,
    sizeRole = "control",
    density = null,
    onCheckedChange = undefined,
  }: Props = $props();

  let seededDefaultChecked = $state(false);
  let uncontrolledChecked = $state(false);

  const isControlled = $derived(checked !== undefined);
  const currentChecked = $derived(isControlled ? checked === true : uncontrolledChecked);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedOffColor = $derived(offColor ?? toneToColor(leftTone));
  const resolvedOnColor = $derived(onColor ?? toneToColor(rightTone));
  const fallbackAriaLabel = $derived(
    [leftLabel, rightLabel]
      .filter((value): value is string => Boolean(value && value.trim()))
      .join(" / ")
  );
  const computedAriaLabel = $derived(label ?? ariaLabel ?? (fallbackAriaLabel || null));
  const switchStyles = $derived([
    resolvedOffColor ? `--poodle-switch-off-color: ${resolvedOffColor}` : "",
    resolvedOnColor ? `--poodle-switch-on-color: ${resolvedOnColor}` : "",
  ].filter(Boolean).join("; ") || undefined);

  $effect(() => {
    if (!seededDefaultChecked && checked === undefined) {
      uncontrolledChecked = defaultChecked;
      seededDefaultChecked = true;
    }
  });

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

    checked = nextChecked;
    onCheckedChange?.(nextChecked);
  }
</script>

<label
  class="poodle-switch"
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
    class="poodle-switch__control"
    type="checkbox"
    role="switch"
    checked={currentChecked}
    disabled={disabled}
    aria-label={computedAriaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    aria-readonly={readOnly ? "true" : undefined}
    onchange={handleChange}
  />
  {#if leftLabel}
    <span class="poodle-switch__label poodle-switch__label--left">{leftLabel}</span>
  {/if}
  <span class="poodle-switch__track" aria-hidden="true">
    <span class="poodle-switch__thumb"></span>
  </span>
  {#if rightLabel}
    <span class="poodle-switch__label poodle-switch__label--right">{rightLabel}</span>
  {:else if label}
    <span class="poodle-switch__label">{label}</span>
  {/if}
</label>

<style>
  .poodle-switch {
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

  .poodle-switch[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-switch[data-read-only="true"] {
    cursor: default;
  }

  .poodle-switch__control {
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

  .poodle-switch__track {
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

  .poodle-switch__thumb {
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

  .poodle-switch__control:checked ~ .poodle-switch__track {
    border-color: var(--poodle-switch-on-border);
    background: var(--poodle-switch-on-track);
  }

  .poodle-switch__control:checked ~ .poodle-switch__track .poodle-switch__thumb {
    background: var(--poodle-switch-on-thumb);
    transform: translateX(calc(var(--switch-unit) - 0.125rem));
  }

  .poodle-switch__control:focus-visible + .poodle-switch__track {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-switch__label {
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  .poodle-switch__label--left,
  .poodle-switch__label--right {
    color: var(--poodle-color-text-muted);
    transition: color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-switch:not([data-dual-label]) .poodle-switch__label {
    color: var(--poodle-color-text-primary);
  }

  .poodle-switch__control:not(:checked) ~ .poodle-switch__label--left {
    color: var(--poodle-switch-off-color);
  }

  .poodle-switch__control:checked ~ .poodle-switch__label--right {
    color: var(--poodle-switch-on-color);
  }

  /* Density variants */
  .poodle-switch[data-density="compact"] {
    gap: 0.25rem;
  }

  .poodle-switch[data-density="comfortable"] {
    gap: var(--poodle-space-inline-md);
  }

  /* Size variants — set --switch-unit per size, geometry calcs use it */
  .poodle-switch[data-size="xs"] { --switch-unit: var(--poodle-size-icon-xs); }
  .poodle-switch[data-size="xs"] .poodle-switch__track {
    width: calc(var(--switch-unit) * 1.75);
    height: calc(var(--switch-unit) * 0.875);
    padding: 0.0625rem;
  }

  .poodle-switch[data-size="xs"] .poodle-switch__thumb {
    width: calc(var(--switch-unit) * 0.75 - 0.125rem);
    height: calc(var(--switch-unit) * 0.75 - 0.125rem);
  }

  .poodle-switch[data-size="xs"] .poodle-switch__control:checked ~ .poodle-switch__track .poodle-switch__thumb {
    transform: translateX(0.5rem);
  }

  .poodle-switch[data-size="sm"] { --switch-unit: var(--poodle-size-icon-sm); }
  .poodle-switch[data-size="sm"] .poodle-switch__track {
    width: calc(var(--switch-unit) * 1.875);
    height: calc(var(--switch-unit) + 0.125rem);
    padding: 0.09375rem;
  }

  .poodle-switch[data-size="sm"] .poodle-switch__thumb {
    width: calc(var(--switch-unit) - 0.1875rem);
    height: calc(var(--switch-unit) - 0.1875rem);
  }

  .poodle-switch[data-size="sm"] .poodle-switch__control:checked ~ .poodle-switch__track .poodle-switch__thumb {
    transform: translateX(0.53125rem);
  }

  .poodle-switch[data-size="lg"] { --switch-unit: var(--poodle-size-icon-lg); }
  .poodle-switch[data-size="lg"] .poodle-switch__track {
    width: calc(var(--switch-unit) * 2.25 + 0.25rem);
    height: calc(var(--switch-unit) + 0.5rem);
    padding: 0.1875rem;
  }

  .poodle-switch[data-size="lg"] .poodle-switch__thumb {
    width: var(--switch-unit);
    height: var(--switch-unit);
  }

  .poodle-switch[data-size="lg"] .poodle-switch__control:checked ~ .poodle-switch__track .poodle-switch__thumb {
    transform: translateX(calc(var(--switch-unit) + 0.0625rem));
  }

  .poodle-switch[data-size="xl"] { --switch-unit: var(--poodle-size-icon-xl); }
  .poodle-switch[data-size="xl"] .poodle-switch__track {
    width: calc(var(--switch-unit) * 2.5 + 0.375rem);
    height: calc(var(--switch-unit) + 0.75rem);
    padding: 0.25rem;
  }

  .poodle-switch[data-size="xl"] .poodle-switch__thumb {
    width: calc(var(--switch-unit) + 0.125rem);
    height: calc(var(--switch-unit) + 0.125rem);
  }

  .poodle-switch[data-size="xl"] .poodle-switch__control:checked ~ .poodle-switch__track .poodle-switch__thumb {
    transform: translateX(1.875rem);
  }

  /* Label size variants */
  .poodle-switch[data-size="xs"] .poodle-switch__label { font-size: 0.6875rem; }
  .poodle-switch[data-size="sm"] .poodle-switch__label { font-size: 0.75rem; }
  .poodle-switch[data-size="lg"] .poodle-switch__label { font-size: 0.875rem; }
  .poodle-switch[data-size="xl"] .poodle-switch__label { font-size: 0.9375rem; }
</style>

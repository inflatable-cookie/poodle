<script lang="ts">
  import "@poodle/styles/switch.css";
  import { switchTransition } from "@poodle/headless";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation.ts";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    SwitchTone,
  } from "./types.ts";

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
        // Stylesheet default for the on side is already the accent token via
        // the recipe chain; returning null keeps the inline prop channel
        // quiet so app-scope --poodle-recipe-switch-* overrides can reach
        // the component. Explicit onColor/offColor props still win.
        return null;
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
    const control = event.currentTarget as HTMLInputElement;
    const result = switchTransition(
      { checked: currentChecked, disabled, readOnly },
      { type: "TOGGLE", nextChecked: control.checked },
    );

    for (const effect of result.effects) {
      if (effect.type === "revertNativeChecked") {
        control.checked = currentChecked;
      } else if (effect.type === "emitCheckedChange") {
        if (!isControlled) {
          uncontrolledChecked = effect.checked;
        } else {
          checked = effect.checked;
        }

        onCheckedChange?.(effect.checked);
      }
    }
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


<script lang="ts">
  import "@poodle/styles/radio.css";
  import { switchTransition } from "@poodle/headless";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    id?: string | undefined;
    name?: string | undefined;
    value?: string | undefined;
    checked?: boolean | undefined;
    defaultChecked?: boolean;
    disabled?: boolean;
    readOnly?: boolean;
    label?: string | null;
    ariaLabel?: string | null;
    describedBy?: string | null;
    selectedColor?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onCheckedChange?: ((checked: boolean) => void) | undefined;
  }

  const uiPresentation = getUiPresentation();

  let {
    id = undefined,
    name = undefined,
    value = undefined,
    checked = $bindable<boolean | undefined>(undefined),
    defaultChecked = false,
    disabled = false,
    readOnly = false,
    label = null,
    ariaLabel = null,
    describedBy = null,
    selectedColor = null,
    size = null,
    sizeRole = "control",
    density = null,
    onCheckedChange = undefined,
  }: Props = $props();

  let seededDefaultChecked = $state(false);
  let uncontrolledChecked = $state(false);

  const currentChecked = $derived(
    checked === undefined ? uncontrolledChecked : checked,
  );
  const resolvedSize = $derived(
    size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole),
  );
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const radioStyles = $derived(
    selectedColor ? `--poodle-radio-selected-color: ${selectedColor}` : undefined,
  );

  $effect(() => {
    if (!seededDefaultChecked && checked === undefined) {
      uncontrolledChecked = defaultChecked;
      seededDefaultChecked = true;
    }
  });

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
        if (checked === undefined) {
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
  class="poodle-radio"
  data-disabled={disabled}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  style={radioStyles}
>
  <input
    {id}
    {name}
    {value}
    class="poodle-radio__control"
    type="radio"
    checked={currentChecked}
    disabled={disabled}
    aria-label={label ? undefined : ariaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    onchange={handleChange}
  />
  <span class="poodle-radio__indicator" aria-hidden="true">
    <span class="poodle-radio__dot"></span>
  </span>
  {#if label}
    <span class="poodle-radio__label">{label}</span>
  {/if}
</label>


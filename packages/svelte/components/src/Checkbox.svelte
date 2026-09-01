<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/checkbox.css";
  import { checkboxParts, checkboxTransition, type CheckboxContext } from "@inflatable-cookie/poodle-core";

  import { default as Icon } from "./Icon.svelte";
  import { useMotionReady } from "./motion-ready.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    id?: string | undefined;
    checked?: boolean | undefined;
    defaultChecked?: boolean;
    mixed?: boolean;
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

  let input: HTMLInputElement | null = null;
  const uiPresentation = getUiPresentation();
  const motionReady = useMotionReady();

  let {
    id = undefined,
    checked = $bindable<boolean | undefined>(undefined),
    defaultChecked = false,
    mixed = false,
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

  const currentChecked = $derived(checked === undefined ? uncontrolledChecked : checked);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const checkboxStyles = $derived(selectedColor ? `--poodle-checkbox-selected-color: ${selectedColor}` : undefined);

  $effect(() => {
    if (!seededDefaultChecked && checked === undefined) {
      uncontrolledChecked = defaultChecked;
      seededDefaultChecked = true;
    }
  });

  $effect(() => {
    if (input) {
      input.indeterminate = mixed;
    }
  });

  const machineContext = $derived<CheckboxContext>({
    checked: currentChecked,
    mixed,
    disabled,
    readOnly,
  });

  const parts = $derived(
    checkboxParts(machineContext, {
      id,
      ariaLabel,
      describedBy,
      hasVisibleLabel: label !== null && label !== "",
    }),
  );

  function handleChange(event: Event): void {
    const control = event.currentTarget as HTMLInputElement;
    const result = checkboxTransition(machineContext, {
      type: "TOGGLE",
      nextChecked: control.checked,
    });

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

<label {...parts.root} class="poodle-checkbox" data-size={resolvedSize} data-density={resolvedDensity} data-motion-ready={motionReady.ready} style={checkboxStyles}>
  <input
    bind:this={input}
    {...parts.control}
    class="poodle-checkbox__control"
    onchange={handleChange}
  />
  <span {...parts.indicator} class="poodle-checkbox__indicator">
    {#if mixed}
      <span class="poodle-checkbox__mark"><Icon name="minus" /></span>
    {:else if currentChecked}
      <span class="poodle-checkbox__mark"><Icon name="check" /></span>
    {/if}
  </span>
  {#if label}
    <span {...parts.label} class="poodle-checkbox__label">{label}</span>
  {/if}
</label>


<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/toggle-group.css";
  import {
    toggleGroupArrowTarget,
    toggleGroupIsSelected,
    toggleGroupTabStopValue,
    toggleGroupTransition,
    type ToggleGroupContext,
  } from "@inflatable-cookie/poodle-core";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    ToggleGroupOption,
  } from "./types";

  interface Props {
    value?: string | string[] | null | undefined;
    defaultValue?: string | string[] | null;
    options?: ToggleGroupOption[];
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    selectionMode?: "single" | "multiple";
    allowDeactivation?: boolean;
    disabled?: boolean;
    ariaLabel?: string | null;
    onValueChange?: ((value: string | string[] | null) => void) | undefined;
  }

  let {
    value = $bindable<string | string[] | null | undefined>(undefined),
    defaultValue = null,
    options = [],
    size = null,
    sizeRole = "control",
    density = null,
    selectionMode = "single",
    allowDeactivation = false,
    disabled = false,
    ariaLabel = null,
    onValueChange = undefined,
  }: Props = $props();

  let root: HTMLDivElement | undefined;
  let uncontrolledValue = $state<string | string[] | null>(null);
  let seededDefaultValue = $state(false);
  const uiPresentation = getUiPresentation();

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue ?? (selectionMode === "multiple" ? [] : null);
      seededDefaultValue = true;
    }
  });

  const controlled = $derived(value !== undefined);
  const currentValue = $derived(controlled ? value : uncontrolledValue);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  const machineContext = $derived<ToggleGroupContext>({
    value: currentValue ?? null,
    options: options.map((option) => ({ value: option.value, disabled: disabled || option.disabled === true })),
    selectionMode,
    allowDeactivation,
    disabled,
  });

  const tabStop = $derived(toggleGroupTabStopValue(machineContext));

  function isSelected(optionValue: string): boolean {
    return toggleGroupIsSelected(machineContext, optionValue);
  }

  function itemTabIndex(optionValue: string, itemDisabled: boolean): number | undefined {
    if (selectionMode !== "single") {
      return undefined;
    }

    if (itemDisabled) {
      return -1;
    }

    return tabStop === optionValue ? 0 : -1;
  }

  function itemElement(optionValue: string): HTMLButtonElement | undefined {
    if (!root) {
      return undefined;
    }

    return Array.from(root.querySelectorAll<HTMLButtonElement>("[data-toggle-value]")).find(
      (element) => element.dataset.toggleValue === optionValue,
    );
  }

  function toggle(optionValue: string): void {
    const result = toggleGroupTransition(machineContext, { type: "TOGGLE", value: optionValue });

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (!controlled) {
          uncontrolledValue = effect.value;
        } else {
          value = effect.value;
        }

        onValueChange?.(effect.value);
      }
    }
  }

  function handleKeydown(event: KeyboardEvent, optionValue: string): void {
    if (selectionMode !== "single") {
      return;
    }

    const direction = event.key === "ArrowRight" ? 1 : event.key === "ArrowLeft" ? -1 : null;

    if (direction === null) {
      return;
    }

    event.preventDefault();
    const target = toggleGroupArrowTarget(machineContext, optionValue, direction);

    if (!target) {
      return;
    }

    toggle(target);
    itemElement(target)?.focus();
  }
</script>

<div
  bind:this={root}
  class="poodle-toggle-group"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  role={selectionMode === "multiple" ? "group" : "radiogroup"}
  aria-label={ariaLabel ?? undefined}
>
  {#each options as option (option.value)}
    {@const itemDisabled = disabled || option.disabled === true}
    <button
      type="button"
      class="poodle-toggle-group__item"
      class:poodle-selected={isSelected(option.value)}
      data-selected={isSelected(option.value) ? "true" : "false"}
      data-toggle-value={option.value}
      tabindex={itemTabIndex(option.value, itemDisabled)}
      disabled={itemDisabled}
      role={selectionMode === "multiple" ? "button" : "radio"}
      aria-label={option.ariaLabel ?? undefined}
      aria-pressed={selectionMode === "multiple" ? (isSelected(option.value) ? "true" : "false") : undefined}
      aria-checked={selectionMode === "single" ? (isSelected(option.value) ? "true" : "false") : undefined}
      onclick={() => toggle(option.value)}
      onkeydown={(event) => handleKeydown(event, option.value)}
    >
      {option.label}
    </button>
  {/each}
</div>

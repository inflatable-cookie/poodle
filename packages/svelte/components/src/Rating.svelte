<script lang="ts">
  import "@poodle/styles/rating.css";
  import {
    normalizeRatingValue,
    ratingFillRatio,
    ratingKeyboardStep,
    ratingPointerValue,
    ratingSelectValue,
    resolveRatingStep,
    clampRatingDisplayValue,
    trimRatingFraction,
  } from "@poodle/headless";
  import { default as Icon } from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    value?: number | null | undefined;
    defaultValue?: number | null;
    max?: number;
    step?: number;
    allowClear?: boolean;
    disabled?: boolean;
    ariaLabel?: string | null;
    onValueChange?: ((value: number | null) => void) | undefined;
  }

  const uiPresentation = getUiPresentation();

  let {
    size = null,
    sizeRole = "control",
    density = null,
    value = $bindable<number | null | undefined>(undefined),
    defaultValue = null,
    max = 5,
    step = 0.5,
    allowClear = false,
    disabled = false,
    ariaLabel = null,
    onValueChange = undefined,
  }: Props = $props();

  let itemElements: Array<HTMLButtonElement | null> = [];
  let uncontrolledValue = $state<number | null>(null);
  let seededDefaultValue = $state(false);
  let focusIndex = $state(0);
  let hoverIndex = $state(-1);
  let hoverValue = $state<number | null>(null);

  const resolveStep = resolveRatingStep;
  const clampDisplayValue = clampRatingDisplayValue;
  const normalizeInteractiveValue = normalizeRatingValue;
  const trimFraction = trimRatingFraction;
  const getFillRatio = ratingFillRatio;

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const itemCount = $derived(Math.max(1, Math.floor(max)));
  const effectiveStep = $derived(resolveStep(step));
  const isFractional = $derived(effectiveStep < 1);
  const minSelectableValue = $derived(allowClear ? 0 : effectiveStep);
  const currentValue = $derived(clampDisplayValue(value === undefined ? uncontrolledValue : value, itemCount));
  const displayValue = $derived(hoverValue ?? currentValue ?? 0);
  const sliderValueText = $derived(
    currentValue === null || currentValue === 0
      ? `No rating selected out of ${itemCount}`
      : `${trimFraction(currentValue)} out of ${itemCount}`,
  );

  $effect(() => {
    if (!seededDefaultValue && value === undefined) {
      uncontrolledValue = clampDisplayValue(defaultValue, itemCount);
      seededDefaultValue = true;
    }
  });

  $effect(() => {
    if (currentValue !== null && currentValue > 0) {
      focusIndex = Math.max(0, Math.min(itemCount - 1, Math.ceil(currentValue) - 1));
    }
  });

  function setValue(nextValue: number | null): void {
    const normalized = normalizeInteractiveValue(nextValue, itemCount, effectiveStep);

    if (value === undefined) {
      uncontrolledValue = normalized;
    } else {
      value = normalized;
    }

    onValueChange?.(normalized);
  }

  function selectIndex(index: number): void {
    setValue(ratingSelectValue(index + 1, currentValue, allowClear));
  }

  function getPointerValue(event: MouseEvent, index: number): number {
    const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
    const relativeX = Math.max(0, Math.min(rect.width, event.clientX - rect.left));
    const rawWithinStar = rect.width === 0 ? 1 : relativeX / rect.width;

    return ratingPointerValue(rawWithinStar, index, effectiveStep, itemCount);
  }

  function handleFractionalHover(event: MouseEvent, index: number): void {
    if (disabled) return;
    hoverIndex = index;
    hoverValue = getPointerValue(event, index);
  }

  function handleFractionalSelect(event: MouseEvent, index: number): void {
    if (disabled) return;

    setValue(ratingSelectValue(getPointerValue(event, index), currentValue, allowClear));
  }

  function moveFocus(nextIndex: number): void {
    focusIndex = Math.max(0, Math.min(itemCount - 1, nextIndex));
    itemElements[focusIndex]?.focus();
  }

  function handleSliderKeydown(event: KeyboardEvent): void {
    if (disabled) return;

    const currentNumericValue = normalizeInteractiveValue(currentValue ?? 0, itemCount, effectiveStep) ?? 0;

    if (event.key === "ArrowRight" || event.key === "ArrowUp") {
      event.preventDefault();
      setValue(ratingKeyboardStep(currentNumericValue, 1, effectiveStep, itemCount, minSelectableValue));
    }

    if (event.key === "ArrowLeft" || event.key === "ArrowDown") {
      event.preventDefault();
      setValue(ratingKeyboardStep(currentNumericValue, -1, effectiveStep, itemCount, minSelectableValue));
    }

    if (event.key === "Home") {
      event.preventDefault();
      setValue(minSelectableValue);
    }

    if (event.key === "End") {
      event.preventDefault();
      setValue(itemCount);
    }

    if ((event.key === "Enter" || event.key === " ") && allowClear && currentValue !== null) {
      event.preventDefault();
      setValue(null);
    }
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div
  class="poodle-rating"
  role={isFractional ? "slider" : "radiogroup"}
  tabindex={disabled ? -1 : isFractional ? 0 : -1}
  aria-label={ariaLabel ?? undefined}
  aria-valuemin={isFractional ? 0 : undefined}
  aria-valuemax={isFractional ? itemCount : undefined}
  aria-valuenow={isFractional ? (currentValue ?? 0) : undefined}
  aria-valuetext={isFractional ? sliderValueText : undefined}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-mode={isFractional ? "fractional" : "whole"}
  onmouseleave={() => {
    hoverIndex = -1;
    hoverValue = null;
  }}
  onkeydown={isFractional ? handleSliderKeydown : undefined}
>
  {#each Array.from({ length: itemCount }, (_, index) => index) as index}
    <button
      bind:this={itemElements[index]}
      type="button"
      class="poodle-rating__item"
      data-hovered={hoverIndex === index}
      disabled={disabled}
      role={isFractional ? undefined : "radio"}
      aria-hidden={isFractional ? "true" : undefined}
      aria-checked={isFractional ? undefined : currentValue === index + 1 ? "true" : "false"}
      aria-label={isFractional ? undefined : `${index + 1} of ${itemCount}`}
      tabindex={isFractional ? -1 : focusIndex === index ? 0 : -1}
      onmouseenter={(event) => {
        if (disabled) return;
        if (isFractional) {
          handleFractionalHover(event, index);
        } else {
          hoverIndex = index;
          hoverValue = index + 1;
        }
      }}
      onmousemove={isFractional ? (event) => handleFractionalHover(event, index) : undefined}
      onfocus={() => {
        if (!isFractional) {
          moveFocus(index);
        }
      }}
      onclick={(event) => {
        if (isFractional) {
          handleFractionalSelect(event, index);
        } else {
          selectIndex(index);
        }
      }}
      onkeydown={!isFractional
        ? (event) => {
            if (event.key === "ArrowRight" || event.key === "ArrowUp") {
              event.preventDefault();
              moveFocus(index + 1);
            }

            if (event.key === "ArrowLeft" || event.key === "ArrowDown") {
              event.preventDefault();
              moveFocus(index - 1);
            }

            if (event.key === "Home") {
              event.preventDefault();
              moveFocus(0);
            }

            if (event.key === "End") {
              event.preventDefault();
              moveFocus(itemCount - 1);
            }

            if (event.key === "Enter" || event.key === " ") {
              event.preventDefault();
              selectIndex(index);
            }
          }
        : undefined}
    >
      <span class="poodle-rating__glyph" aria-hidden="true">
        <span class="poodle-rating__glyph-base">
          <Icon name="star" size={resolvedSize} />
        </span>
        <span
          class="poodle-rating__glyph-fill"
          style={`width: ${getFillRatio(index, displayValue) * 100}%;`}
        >
          <span class="poodle-rating__glyph-fill-inner">
            <Icon name="star" size={resolvedSize} />
          </span>
        </span>
      </span>
    </button>
  {/each}
</div>


import { useRef, useState, type KeyboardEvent, type MouseEvent } from "react";
import {
  clampRatingDisplayValue,
  normalizeRatingValue,
  ratingFillRatio,
  ratingKeyboardStep,
  ratingPointerValue,
  ratingSelectValue,
  resolveRatingStep,
  trimRatingFraction,
} from "@poodle/headless";

import "@poodle/styles/rating.css";

import { Icon } from "./Icon";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface RatingProps {
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  value?: number | null;
  defaultValue?: number | null;
  max?: number;
  step?: number;
  allowClear?: boolean;
  disabled?: boolean;
  ariaLabel?: string | null;
  onValueChange?: (value: number | null) => void;
}

export function Rating({
  size = null,
  sizeRole = "control",
  density = null,
  value,
  defaultValue = null,
  max = 5,
  step = 0.5,
  allowClear = false,
  disabled = false,
  ariaLabel = null,
  onValueChange,
}: RatingProps) {
  const uiPresentation = useUiPresentation();

  const itemCount = Math.max(1, Math.floor(max));
  const [uncontrolledValue, setUncontrolledValue] = useState<number | null>(() =>
    clampRatingDisplayValue(defaultValue, itemCount),
  );
  const [focusIndex, setFocusIndex] = useState(0);
  const [hoverIndex, setHoverIndex] = useState(-1);
  const [hoverValue, setHoverValue] = useState<number | null>(null);
  const itemRefs = useRef<Array<HTMLButtonElement | null>>([]);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const effectiveStep = resolveRatingStep(step);
  const isFractional = effectiveStep < 1;
  const minSelectableValue = allowClear ? 0 : effectiveStep;
  const isControlled = value !== undefined;
  const currentValue = clampRatingDisplayValue(isControlled ? value : uncontrolledValue, itemCount);
  const displayValue = hoverValue ?? currentValue ?? 0;
  const sliderValueText =
    currentValue === null || currentValue === 0
      ? `No rating selected out of ${itemCount}`
      : `${trimRatingFraction(currentValue)} out of ${itemCount}`;

  function setValue(nextValue: number | null): void {
    const normalized = normalizeRatingValue(nextValue, itemCount, effectiveStep);
    if (!isControlled) setUncontrolledValue(normalized);
    if (normalized !== null && normalized > 0) {
      setFocusIndex(Math.max(0, Math.min(itemCount - 1, Math.ceil(normalized) - 1)));
    }
    onValueChange?.(normalized);
  }

  function selectIndex(index: number): void {
    setValue(ratingSelectValue(index + 1, currentValue, allowClear));
  }

  function getPointerValue(event: MouseEvent<HTMLElement>, index: number): number {
    const rect = event.currentTarget.getBoundingClientRect();
    const relativeX = Math.max(0, Math.min(rect.width, event.clientX - rect.left));
    const rawWithinStar = rect.width === 0 ? 1 : relativeX / rect.width;
    return ratingPointerValue(rawWithinStar, index, effectiveStep, itemCount);
  }

  function handleFractionalHover(event: MouseEvent<HTMLElement>, index: number): void {
    if (disabled) return;
    setHoverIndex(index);
    setHoverValue(getPointerValue(event, index));
  }

  function moveFocus(nextIndex: number): void {
    const clamped = Math.max(0, Math.min(itemCount - 1, nextIndex));
    setFocusIndex(clamped);
    itemRefs.current[clamped]?.focus();
  }

  function handleSliderKeydown(event: KeyboardEvent<HTMLDivElement>): void {
    if (disabled) return;
    const currentNumericValue = normalizeRatingValue(currentValue ?? 0, itemCount, effectiveStep) ?? 0;

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

  return (
    <div
      className="poodle-rating"
      role={isFractional ? "slider" : "radiogroup"}
      tabIndex={disabled ? -1 : isFractional ? 0 : -1}
      aria-label={ariaLabel ?? undefined}
      aria-valuemin={isFractional ? 0 : undefined}
      aria-valuemax={isFractional ? itemCount : undefined}
      aria-valuenow={isFractional ? (currentValue ?? 0) : undefined}
      aria-valuetext={isFractional ? sliderValueText : undefined}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-mode={isFractional ? "fractional" : "whole"}
      onMouseLeave={() => {
        setHoverIndex(-1);
        setHoverValue(null);
      }}
      onKeyDown={isFractional ? handleSliderKeydown : undefined}
    >
      {Array.from({ length: itemCount }, (_, index) => (
        <button
          key={index}
          ref={(node) => {
            itemRefs.current[index] = node;
          }}
          type="button"
          className="poodle-rating__item"
          data-hovered={hoverIndex === index}
          disabled={disabled}
          role={isFractional ? undefined : "radio"}
          aria-hidden={isFractional ? "true" : undefined}
          aria-checked={isFractional ? undefined : currentValue === index + 1}
          aria-label={isFractional ? undefined : `${index + 1} of ${itemCount}`}
          tabIndex={isFractional ? -1 : focusIndex === index ? 0 : -1}
          onMouseEnter={(event) => {
            if (disabled) return;
            if (isFractional) {
              handleFractionalHover(event, index);
            } else {
              setHoverIndex(index);
              setHoverValue(index + 1);
            }
          }}
          onMouseMove={isFractional ? (event) => handleFractionalHover(event, index) : undefined}
          onFocus={() => {
            if (!isFractional) setFocusIndex(index);
          }}
          onClick={(event) => {
            if (isFractional) {
              setValue(ratingSelectValue(getPointerValue(event, index), currentValue, allowClear));
            } else {
              selectIndex(index);
            }
          }}
          onKeyDown={
            !isFractional
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
              : undefined
          }
        >
          <span className="poodle-rating__glyph" aria-hidden="true">
            <span className="poodle-rating__glyph-base">
              <Icon name="star" size={resolvedSize} />
            </span>
            <span className="poodle-rating__glyph-fill" style={{ width: `${ratingFillRatio(index, displayValue) * 100}%` }}>
              <span className="poodle-rating__glyph-fill-inner">
                <Icon name="star" size={resolvedSize} />
              </span>
            </span>
          </span>
        </button>
      ))}
    </div>
  );
}

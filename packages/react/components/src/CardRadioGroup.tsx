import "@inflatable-cookie/poodle-styles/card-radio-group.css";

import { useState, type KeyboardEvent as ReactKeyboardEvent, type ReactNode } from "react";

import { menuListNavigate, toggleGroupTransition } from "@inflatable-cookie/poodle-headless";

import { Card } from "./Card";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { CardRadioItem, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface CardRadioGroupCardProps {
  item: CardRadioItem;
  checked: boolean;
  disabled: boolean;
}

export interface CardRadioGroupProps {
  items?: CardRadioItem[];
  value?: string | null;
  columns?: 1 | 2 | 3 | 4;
  ariaLabel?: string | null;
  disabled?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: ((value: string) => void) | undefined;
  card?: (props: CardRadioGroupCardProps) => ReactNode;
}

export function CardRadioGroup({
  items = [],
  value,
  columns = 2,
  ariaLabel = null,
  disabled = false,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange = undefined,
  card,
}: CardRadioGroupProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = value !== undefined;
  const currentValue = isControlled ? (value ?? null) : uncontrolledValue;

  function select(itemValue: string): void {
    const result = toggleGroupTransition(
      {
        value: currentValue,
        options: items.map((item) => ({ value: item.value, disabled: item.disabled === true })),
        selectionMode: "single",
        allowDeactivation: false,
        disabled,
      },
      { type: "TOGGLE", value: itemValue },
    );

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (effect.value === null) continue;
        const nextValue = effect.value as string;

        if (!isControlled) {
          setUncontrolledValue(nextValue);
        }

        onValueChange?.(nextValue);
      }
    }
  }

  function handleKeydown(event: ReactKeyboardEvent, index: number): void {
    const enabledItems = items.filter((item) => !item.disabled);
    const currentEnabledIndex = enabledItems.findIndex((item) => item.value === items[index].value);

    let nextItem: CardRadioItem | undefined;

    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      nextItem = enabledItems[menuListNavigate(enabledItems, currentEnabledIndex, "next")];
    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      nextItem = enabledItems[menuListNavigate(enabledItems, currentEnabledIndex, "prev")];
    }

    if (!nextItem) {
      return;
    }

    select(nextItem.value);
    const nextIndex = items.findIndex((item) => item.value === nextItem.value);
    const element = document.querySelector<HTMLElement>(`[data-card-radio-index="${nextIndex}"]`);
    element?.focus();
  }

  return (
    <div
      className="poodle-card-radio-group"
      role="radiogroup"
      aria-label={ariaLabel ?? undefined}
      style={{ "--columns": columns } as React.CSSProperties}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {items.map((item, index) => {
        const isChecked = currentValue === item.value;
        const isItemDisabled = disabled || item.disabled === true;

        return (
          <div
            key={item.value}
            className="poodle-card-radio-group__option"
            role="radio"
            tabIndex={isItemDisabled ? -1 : isChecked || (currentValue === null && index === 0) ? 0 : -1}
            aria-checked={isChecked ? "true" : "false"}
            aria-disabled={isItemDisabled ? "true" : undefined}
            data-card-radio-index={index}
            onClick={() => !isItemDisabled && select(item.value)}
            onKeyDown={(event) => !isItemDisabled && handleKeydown(event, index)}
          >
            <Card
              interactive={!isItemDisabled}
              selected={isChecked}
              density={resolvedDensity}
              ariaLabel={item.label}
              header={
                <div className="poodle-card-radio-group__header">
                  <span
                    className="poodle-card-radio-group__indicator"
                    data-checked={isChecked}
                    data-disabled={isItemDisabled}
                    aria-hidden="true"
                  >
                    {isChecked ? <span className="poodle-card-radio-group__dot" /> : null}
                  </span>
                  <span className="poodle-card-radio-group__title" data-disabled={isItemDisabled}>
                    {item.label}
                  </span>
                </div>
              }
            >
              {item.description ? (
                <p className="poodle-card-radio-group__description" data-disabled={isItemDisabled}>
                  {item.description}
                </p>
              ) : null}

              {card?.({ item, checked: isChecked, disabled: isItemDisabled })}
            </Card>
          </div>
        );
      })}
    </div>
  );
}

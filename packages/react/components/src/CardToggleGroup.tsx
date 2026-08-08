import "@inflatable-cookie/poodle-core/styles/card-toggle-group.css";

import { useState, type KeyboardEvent as ReactKeyboardEvent, type ReactNode } from "react";

import { menuListNavigate, toggleGroupTransition } from "@inflatable-cookie/poodle-core";

import { Card } from "./Card";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { CardToggleItem, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface CardToggleGroupCardProps {
  item: CardToggleItem;
  selected: boolean;
  disabled: boolean;
}

export interface CardToggleGroupProps {
  items?: CardToggleItem[];
  value?: string | null;
  defaultValue?: string | null;
  allowDeactivation?: boolean;
  columns?: 1 | 2 | 3 | 4;
  ariaLabel?: string | null;
  disabled?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: ((value: string | null) => void) | undefined;
  card?: (props: CardToggleGroupCardProps) => ReactNode;
}

export function CardToggleGroup({
  items = [],
  value,
  defaultValue = null,
  allowDeactivation = false,
  columns = 2,
  ariaLabel = null,
  disabled = false,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange = undefined,
  card,
}: CardToggleGroupProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = value !== undefined;
  const currentValue = isControlled ? (value ?? null) : uncontrolledValue;
  const firstEnabledIndex = items.findIndex((item) => !item.disabled);

  function select(itemValue: string): void {
    const result = toggleGroupTransition(
      {
        value: currentValue,
        options: items.map((item) => ({ value: item.value, disabled: item.disabled === true })),
        selectionMode: "single",
        allowDeactivation: allowDeactivation,
        disabled,
      },
      { type: "TOGGLE", value: itemValue },
    );

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        const nextValue = effect.value as string | null;

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

    let nextItem: CardToggleItem | undefined;

    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      nextItem = enabledItems[menuListNavigate(enabledItems, currentEnabledIndex, "next")];
    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      nextItem = enabledItems[menuListNavigate(enabledItems, currentEnabledIndex, "prev")];
    } else if (event.key === " " || event.key === "Enter") {
      event.preventDefault();
      select(items[index].value);
      return;
    }

    if (!nextItem) {
      return;
    }

    select(nextItem.value);
    const nextIndex = items.findIndex((item) => item.value === nextItem.value);
    const element = document.querySelector<HTMLElement>(`[data-card-toggle-index="${nextIndex}"]`);
    element?.focus();
  }

  return (
    <div
      className="poodle-card-toggle-group"
      role="group"
      aria-label={ariaLabel ?? undefined}
      style={{ "--columns": columns } as React.CSSProperties}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {items.map((item, index) => {
        const isSelected = currentValue === item.value;
        const isItemDisabled = disabled || item.disabled === true;

        return (
          <div
            key={item.value}
            className="poodle-card-toggle-group__option"
            role="button"
            tabIndex={
              isItemDisabled ? -1 : isSelected || (currentValue === null && index === firstEnabledIndex) ? 0 : -1
            }
            aria-pressed={isSelected ? "true" : "false"}
            aria-disabled={isItemDisabled ? "true" : undefined}
            data-card-toggle-index={index}
            onClick={() => !isItemDisabled && select(item.value)}
            onKeyDown={(event) => !isItemDisabled && handleKeydown(event, index)}
          >
            <Card
              interactive={!isItemDisabled}
              selected={isSelected}
              density={resolvedDensity}
              ariaLabel={item.label}
              header={
                <div className="poodle-card-toggle-group__header">
                  <span className="poodle-card-toggle-group__title" data-disabled={isItemDisabled}>
                    {item.label}
                  </span>
                  {item.count !== null && item.count !== undefined ? (
                    <span className="poodle-card-toggle-group__count" data-disabled={isItemDisabled}>
                      {item.count}
                    </span>
                  ) : null}
                </div>
              }
            >
              {item.description ? (
                <p className="poodle-card-toggle-group__description" data-disabled={isItemDisabled}>
                  {item.description}
                </p>
              ) : null}

              {card?.({ item, selected: isSelected, disabled: isItemDisabled })}
            </Card>
          </div>
        );
      })}
    </div>
  );
}

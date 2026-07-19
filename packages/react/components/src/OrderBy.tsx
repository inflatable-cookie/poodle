import { useEffect, useId, useRef, useState, type DragEvent, type MouseEvent } from "react";
import { registerDismissLayer } from "@poodle/headless";

import "@poodle/styles/order-by.css";

import { IconButton } from "./IconButton";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import { Select } from "./Select";
import type {
  ActiveSort,
  ControlDensity,
  ControlSize,
  OrderByFieldDefinition,
  OrderByTriggerVariant,
  OrderByValue,
  SemanticControlSizeRole,
  SortDirection,
  SortField,
} from "./types";

export interface OrderByProps {
  fields?: SortField[];
  value?: OrderByValue;
  activeSort?: ActiveSort | null;
  ariaLabel?: string;
  disabled?: boolean;
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  maxFields?: number | null;
  compact?: boolean;
  triggerVariant?: OrderByTriggerVariant;
  showClearButton?: boolean;
  onChange?: ((value: OrderByValue) => void) | null;
  onActiveSortChange?: ((value: ActiveSort | null) => void) | null;
}

export function OrderBy({
  fields = [],
  value,
  activeSort,
  ariaLabel = "Sort by",
  disabled = false,
  sizeRole = "control",
  size = null,
  density = null,
  maxFields = null,
  compact = false,
  triggerVariant = "summary",
  showClearButton = true,
  onChange = null,
  onActiveSortChange = null,
}: OrderByProps) {
  const uiPresentation = useUiPresentation();
  const panelId = useId();

  const [open, setOpen] = useState(false);
  const [addFieldValue, setAddFieldValue] = useState("");
  const [dragIndex, setDragIndex] = useState<number | null>(null);
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);
  const [uncontrolledValue, setUncontrolledValue] = useState<OrderByValue>([]);
  const rootRef = useRef<HTMLDivElement | null>(null);
  const panelRef = useRef<HTMLDivElement | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const normalizedFields: OrderByFieldDefinition[] = fields
    .map((field) => ({
      key: field.key ?? field.value ?? "",
      label: field.label,
      disabled: field.disabled,
      defaultDirection: field.defaultDirection ?? "asc",
    }))
    .filter((field) => field.key.length > 0);
  const fieldMap = new Map(normalizedFields.map((field) => [field.key, field]));
  const legacyValue: OrderByValue = activeSort ? [{ key: activeSort.field, direction: activeSort.direction }] : [];
  const hasValueProp = value !== undefined;
  const hasLegacyProp = activeSort !== undefined;
  const effectiveValue = hasValueProp ? (value ?? []) : hasLegacyProp ? legacyValue : uncontrolledValue;
  const canAddMore = maxFields === null || effectiveValue.length < maxFields;
  const availableFields = normalizedFields.filter((field) => !effectiveValue.some((item) => item.key === field.key));
  const selectItems = availableFields.map((field) => ({ value: field.key, label: field.label }));

  function summarizeValue(nextValue: OrderByValue): string {
    if (nextValue.length === 0) return "Sort by...";
    const items = nextValue.map((item) => {
      const field = fieldMap.get(item.key);
      const directionLabel = item.direction === "asc" ? "↑" : "↓";
      return `${field?.label ?? item.key} ${directionLabel}`;
    });
    if (compact && items.length > 2) return `${items.slice(0, 2).join(", ")} +${items.length - 2}`;
    return items.join(", ");
  }

  const triggerText = summarizeValue(effectiveValue);

  // initial focus into the panel on open
  useEffect(() => {
    if (!open) return;
    const firstFocusable = panelRef.current?.querySelector<HTMLElement>(
      'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])',
    );
    firstFocusable?.focus();
  }, [open]);

  useEffect(() => {
    if (!open) return;
    return registerDismissLayer({
      contains: (target) => rootRef.current?.contains(target as Node) ?? false,
      dismissOnOutsideInteract: true,
      onDismiss: () => setOpen(false),
    });
  }, [open]);

  function sync(nextValue: OrderByValue): void {
    if (!hasValueProp && !hasLegacyProp) {
      setUncontrolledValue(nextValue);
    }
    if (hasLegacyProp || (hasValueProp && activeSort !== undefined)) {
      onActiveSortChange?.(
        nextValue.length > 0 ? { field: nextValue[0].key, direction: nextValue[0].direction } : null,
      );
    }
    onChange?.(nextValue);
  }

  function addField(key: string): void {
    if (!key || disabled || effectiveValue.some((item) => item.key === key)) return;
    const field = fieldMap.get(key);
    const direction: SortDirection = field?.defaultDirection ?? "asc";
    setAddFieldValue("");
    sync([...effectiveValue, { key, direction }]);
  }

  function removeField(index: number): void {
    if (disabled) return;
    sync(effectiveValue.filter((_, itemIndex) => itemIndex !== index));
  }

  function toggleDirection(index: number): void {
    if (disabled) return;
    sync(
      effectiveValue.map((item, itemIndex) =>
        itemIndex === index ? { ...item, direction: item.direction === "asc" ? ("desc" as const) : ("asc" as const) } : item,
      ),
    );
  }

  function moveField(index: number, offset: -1 | 1): void {
    if (disabled) return;
    const nextIndex = index + offset;
    if (nextIndex < 0 || nextIndex >= effectiveValue.length) return;
    const nextValue = [...effectiveValue];
    const [item] = nextValue.splice(index, 1);
    nextValue.splice(nextIndex, 0, item);
    sync(nextValue);
  }

  function handleDrop(index: number): void {
    if (dragIndex === null || disabled) return;
    const nextValue = [...effectiveValue];
    const [item] = nextValue.splice(dragIndex, 1);
    nextValue.splice(index, 0, item);
    setDragIndex(null);
    setDragOverIndex(null);
    sync(nextValue);
  }

  function clearAll(): void {
    if (disabled) return;
    sync([]);
  }

  function handleResetClick(event: MouseEvent): void {
    event.preventDefault();
    event.stopPropagation();
    clearAll();
  }

  return (
    <div
      ref={rootRef}
      className="poodle-order-by-popover"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-trigger-variant={triggerVariant}
    >
      <div
        className="poodle-order-by"
        role="group"
        aria-label={ariaLabel}
        data-disabled={disabled}
        data-compact={compact}
        data-trigger-variant={triggerVariant}
        data-size={resolvedSize}
        data-density={resolvedDensity}
      >
        {triggerVariant === "icon" ? (
          <IconButton
            icon="arrow-up-down"
            ariaLabel={ariaLabel}
            tooltip={ariaLabel}
            variant="secondary"
            size={resolvedSize}
            disabled={disabled}
            expanded={open}
            controls={open ? panelId : null}
            onClick={() => {
              if (!disabled) setOpen((o) => !o);
            }}
          />
        ) : (
          <div className="poodle-order-by__trigger-wrap">
            <button
              type="button"
              className="poodle-order-by__trigger"
              disabled={disabled}
              aria-label={ariaLabel}
              aria-expanded={open}
              aria-controls={open ? panelId : undefined}
              onClick={() => {
                if (!disabled) setOpen((o) => !o);
              }}
            >
              <span className="poodle-order-by__label">Sort by</span>
              <span className="poodle-order-by__summary" data-placeholder={effectiveValue.length === 0}>
                {triggerText}
              </span>
              <span className="poodle-order-by__chevron" aria-hidden="true">
                ▾
              </span>
            </button>

            {showClearButton && effectiveValue.length > 0 ? (
              <span className="poodle-order-by__reset">
                <IconButton
                  icon="x"
                  ariaLabel="Clear sort"
                  variant="ghost"
                  size={resolvedSize}
                  disabled={disabled}
                  onClick={handleResetClick}
                />
              </span>
            ) : null}
          </div>
        )}
      </div>

      {open ? (
        <div ref={panelRef} id={panelId} className="poodle-order-by__surface" role="dialog" aria-label={ariaLabel} tabIndex={-1}>
          <div className="poodle-order-by__panel">
            {triggerVariant === "icon" ? (
              <div className="poodle-order-by__panel-header">
                <span className="poodle-order-by__panel-title">Sort order</span>
                {showClearButton && effectiveValue.length > 0 ? (
                  <IconButton
                    icon="x"
                    ariaLabel="Clear sort"
                    tooltip="Clear sort"
                    variant="ghost"
                    size="xs"
                    disabled={disabled}
                    onClick={handleResetClick}
                  />
                ) : null}
              </div>
            ) : null}

            {effectiveValue.length > 0 ? (
              <div className="poodle-order-by__list" role="list">
                {effectiveValue.map((item, index) => {
                  const field = fieldMap.get(item.key);
                  return (
                    <div
                      key={`${item.key}-${index}`}
                      className={[
                        "poodle-order-by__item",
                        dragIndex === index ? "poodle-order-by__item--dragging" : "",
                        dragOverIndex === index && dragIndex !== index ? "poodle-order-by__item--drop-target" : "",
                      ]
                        .filter(Boolean)
                        .join(" ")}
                      role="listitem"
                    >
                      <button
                        type="button"
                        className="poodle-order-by__drag-handle"
                        draggable={!disabled}
                        disabled={disabled}
                        aria-label={`Reorder ${field?.label ?? item.key}. Drag or use Alt plus arrow keys.`}
                        onDragStart={() => {
                          if (disabled) return;
                          setDragIndex(index);
                          setDragOverIndex(index);
                        }}
                        onDragEnter={() => {
                          if (dragIndex === null || disabled) return;
                          setDragOverIndex(index);
                        }}
                        onDragOver={(event: DragEvent) => event.preventDefault()}
                        onDrop={(event: DragEvent) => {
                          event.preventDefault();
                          handleDrop(index);
                        }}
                        onDragEnd={() => {
                          setDragIndex(null);
                          setDragOverIndex(null);
                        }}
                        onKeyDown={(event) => {
                          if (event.altKey && event.key === "ArrowUp" && index > 0) {
                            event.preventDefault();
                            moveField(index, -1);
                          }
                          if (event.altKey && event.key === "ArrowDown" && index < effectiveValue.length - 1) {
                            event.preventDefault();
                            moveField(index, 1);
                          }
                        }}
                      >
                        ⠿
                      </button>
                      <span className="poodle-order-by__item-label">{field?.label ?? item.key}</span>
                      <IconButton
                        icon={item.direction === "asc" ? "arrow-up" : "arrow-down"}
                        ariaLabel={`${field?.label ?? item.key}: ${item.direction === "asc" ? "ascending" : "descending"}. Click to toggle.`}
                        tooltip={item.direction === "asc" ? "Asc" : "Desc"}
                        size="xs"
                        variant="ghost"
                        disabled={disabled}
                        onClick={() => toggleDirection(index)}
                      />
                      <IconButton
                        icon="x"
                        ariaLabel={`Remove ${field?.label ?? item.key}`}
                        tooltip="Remove"
                        size="xs"
                        variant="ghost"
                        disabled={disabled}
                        onClick={() => removeField(index)}
                      />
                    </div>
                  );
                })}
              </div>
            ) : (
              <p className="poodle-order-by__empty">No sort fields</p>
            )}

            {canAddMore && availableFields.length > 0 ? (
              <div className="poodle-order-by__add">
                <Select
                  options={selectItems}
                  value={addFieldValue}
                  placeholder="+ Add field"
                  ariaLabel="Add sort field"
                  size={resolvedSize}
                  density={resolvedDensity}
                  onValueChange={(next) => {
                    setAddFieldValue(next);
                    addField(next);
                  }}
                  disabled={disabled}
                />
              </div>
            ) : null}
          </div>
        </div>
      ) : null}
    </div>
  );
}

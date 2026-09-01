import { useEffect, useId, useRef, useState, type MouseEvent } from "react";
import {
  createDragDropController,
  layerContains,
  registerDismissLayer,
  type DragDropCommitResult,
  type DropIntent,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/order-by.css";

import { AnchoredSurface } from "./AnchoredSurface";
import { DragDropProvider } from "./drag-drop";
import { IconButton } from "./IconButton";
import { OrderByRow } from "./order-by/OrderByRow";
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
  dismissOnOutsideInteract?: boolean;
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
  dismissOnOutsideInteract = true,
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
  const [uncontrolledValue, setUncontrolledValue] = useState<OrderByValue>([]);
  // The root is state, not a ref: the portalled surface has to re-render
  // once it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const panelRef = useRef<HTMLDivElement | null>(null);

  /**
   * OrderBy always owns its controller. It is the one programme component that
   * cannot join an ambient provider.
   *
   * The sort panel is portalled to the document body, so it is not inside any
   * ancestor provider's connected root — and the substrate refuses a pointer
   * press whose source is outside the root it was connected to
   * (`onPointerDown`). A joined OrderBy would draw grips that never drag.
   */
  const [dragController] = useState(() => createDragDropController());

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
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target as Node, rootElement, panelRef.current),
      dismissOnOutsideInteract,
      onDismiss: () => setOpen(false),
    });
  }, [open, dismissOnOutsideInteract]);

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

  /**
   * The registration namespace and the semantic family are both scoped to this
   * builder: two mounted OrderBys can legitimately sort the same field keys,
   * and under one ambient provider neither duplicate ids nor a cross-instance
   * drop are acceptable.
   */
  /** A single clause has nowhere to go, and a disabled builder moves nothing. */
  const canReorder = !disabled && effectiveValue.length > 1;

  const subjectKind = `poodle.reorder-item:order-by:${panelId}`;
  const registrationScope = `order-by:${panelId}`;

  function sourceIdOf(key: string): string {
    return `${registrationScope}:source:${key}`;
  }

  function targetIdOf(key: string): string {
    return `${registrationScope}:target:${key}`;
  }

  function keyOfTargetId(targetId: string): string {
    const prefix = `${registrationScope}:target:`;
    return targetId.startsWith(prefix) ? targetId.slice(prefix.length) : "";
  }

  function indexOfKey(key: string): number {
    return effectiveValue.findIndex((item) => item.key === key);
  }

  /**
   * One accepted drop, one complete ordering.
   *
   * Both indices are resolved again here rather than trusted from hover: the
   * host may have replaced `value` while the pointer was down, and a stale
   * index would move the wrong field.
   */
  function handleDrop(intent: DropIntent): DragDropCommitResult {
    if (disabled) return { status: "rejected", reason: "disabled" };

    const from = indexOfKey(dragController.getSnapshot().session?.subject.id ?? "");
    const target = indexOfKey(keyOfTargetId(intent.targetId));
    if (from < 0 || target < 0 || from === target) {
      return { status: "rejected", reason: "missing field" };
    }

    const to =
      intent.position === "before"
        ? from < target
          ? target - 1
          : target
        : from < target
          ? target
          : target + 1;

    const nextValue = [...effectiveValue];
    const [item] = nextValue.splice(from, 1);
    nextValue.splice(to, 0, item);
    sync(nextValue);
    return { status: "committed" };
  }

  /**
   * Alt+Arrow: the contract's keyboard reorder, run as a real session so it
   * shares eligibility, revalidation, and the single commit with a drop.
   */
  function moveField(index: number, offset: -1 | 1): void {
    if (disabled) return;

    const from = effectiveValue[index];
    const target = effectiveValue[index + offset];
    if (!from || !target) return;

    dragController.requestKeyboardDrop({
      sourceId: sourceIdOf(from.key),
      targetId: targetIdOf(target.key),
      position: offset === 1 ? "after" : "before",
    });
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
      ref={setRootElement}
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
        <AnchoredSurface
          ref={panelRef}
          anchor={rootElement}
          placement="bottom-start"
          offset={8}
          id={panelId}
          className="poodle-order-by__surface"
          role="dialog"
          aria-label={ariaLabel}
          tabIndex={-1}
        >
          <DragDropProvider controller={dragController}>
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
                {effectiveValue.map((item, index) => (
                  <OrderByRow
                    key={item.key}
                    item={item}
                    index={index}
                    total={effectiveValue.length}
                    label={fieldMap.get(item.key)?.label ?? item.key}
                    disabled={disabled}
                    subjectKind={subjectKind}
                    sourceId={sourceIdOf(item.key)}
                    targetId={targetIdOf(item.key)}
                    indexOfKey={indexOfKey}
                    canReorder={canReorder}
                    onDrop={handleDrop}
                    onMove={moveField}
                    onToggleDirection={toggleDirection}
                    onRemove={removeField}
                  />
                ))}
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
          </DragDropProvider>
        </AnchoredSurface>
      ) : null}
    </div>
  );
}

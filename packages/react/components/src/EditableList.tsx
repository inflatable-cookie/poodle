import { useEffect, useRef, useState, type KeyboardEvent, type ReactNode } from "react";
import {
  applyReorder,
  type DragDropCommitResult,
  type DragSession,
  type DragTerminalOutcome,
  type DropIntent,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/editable-list.css";

import { Button } from "./Button";
import { DragDropProvider, useDragDrop, useDragSource, useDropTarget, useKeyboardDropTarget } from "./drag-drop";
import { IconButton } from "./IconButton";
import { resolveSemanticControlSize, UiPresentationProvider, useUiPresentation } from "./presentation";
import { TextInput } from "./TextInput";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface EditableListItemLike {
  id: string;
  label?: string;
}

export interface EditableListProps<T extends EditableListItemLike> {
  items?: T[];
  ariaLabel?: string;
  disabled?: boolean;
  reorderable?: boolean;
  editable?: boolean;
  addLabel?: string;
  addPlaceholder?: string;
  maxItems?: number | null;
  removable?: boolean;
  embeddedHandle?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  dirty?: boolean;
  submitting?: boolean;
  errorMessage?: string | null;
  infoMessage?: string | null;
  longListThreshold?: number | null;
  longListWarningText?: string | null;
  windowSize?: number | null;
  submitLabel?: string;
  cancelLabel?: string;
  showWorkflowChrome?: boolean;
  onSubmit?: (() => void | Promise<void>) | null;
  onCancel?: (() => void) | null;
  onReorder?: ((items: T[]) => void) | null;
  onAdd?: ((item: T) => void) | null;
  onRemove?: ((id: string) => void) | null;
  onChange?: ((items: T[]) => void) | null;
  item?: (item: T) => ReactNode;
}

function destinationIndex(fromIndex: number, toIndex: number, position: string, count: number): number {
  let dest = position === "after" ? toIndex + 1 : toIndex;
  if (fromIndex < dest) dest -= 1;
  if (dest < 0) return 0;
  if (dest >= count) return Math.max(0, count - 1);
  return dest;
}

interface EditableListRowProps<T extends EditableListItemLike> {
  item: T;
  index: number;
  indexOfId: (id: string) => number;
  total: number;
  reorderable: boolean;
  embeddedHandle: boolean;
  isUnavailable: boolean;
  showRemove: boolean;
  lastMoved: boolean;
  resolvedSize: ControlSize;
  resolvedDensity: ControlDensity;
  itemRender?: (item: T) => ReactNode;
  onDrop: (intent: DropIntent) => DragDropCommitResult;
  onDragStart: (session: DragSession) => void;
  onDragEnd: (outcome: DragTerminalOutcome) => void;
  onRemove: (id: string) => void;
  onIdleKeydown: (event: KeyboardEvent<HTMLLIElement>, index: number) => void;
}

function EditableListRow<T extends EditableListItemLike>({
  item,
  index,
  indexOfId,
  total,
  reorderable,
  embeddedHandle,
  isUnavailable,
  showRemove,
  lastMoved,
  resolvedSize,
  resolvedDensity,
  itemRender,
  onDrop,
  onDragStart,
  onDragEnd,
  onRemove,
  onIdleKeydown,
}: EditableListRowProps<T>) {
  const canDrag = reorderable && !isUnavailable;
  const { snapshot } = useDragDrop();
  const { getSourceProps, dragging } = useDragSource({
    sourceId: item.id,
    subject: { kind: "poodle.editable-list", id: item.id },
    allowedOperations: ["move"],
    label: item.label ?? item.id,
    disabled: !canDrag,
    handle: canDrag && !embeddedHandle ? ".poodle-editable-list__handle" : undefined,
    keyboardOrder: index,
    onDragStart,
    onDragEnd,
  });
  const { getTargetProps, accepted } = useDropTarget({
    targetId: item.id,
    acceptedKinds: ["poodle.editable-list"],
    disabled: !canDrag,
    label: item.label ?? item.id,
    resolvePosition: ({ subject }) => (indexOfId(subject.id) < index ? "after" : "before"),
    canDrop: (intent, subject) =>
      subject.id === intent.targetId ? { accepted: false, reason: "self" } : { accepted: true, intent },
    onDrop,
  });

  const grabbed = dragging && snapshot.inputKind === "keyboard";
  const sourceProps = getSourceProps(
    getTargetProps({
      className: [
        "poodle-editable-list__item",
        dragging ? "poodle-editable-list__item--dragging" : "",
        accepted && !dragging ? "poodle-editable-list__item--drop-target" : "",
        grabbed ? "poodle-editable-list__item--grabbed" : "",
        lastMoved ? "poodle-editable-list__item--last-moved" : "",
        embeddedHandle ? "poodle-editable-list__item--embedded-handle" : "",
      ]
        .filter(Boolean)
        .join(" "),
      role: "option",
      tabIndex: isUnavailable ? -1 : 0,
      "aria-selected": false,
      "aria-label": `Reorder ${item.label ?? item.id}. Position ${index + 1} of ${total}. Press space to grab, then arrow keys to move.`,
      "data-reorder-index": index,
      onKeyDown: (event) => onIdleKeydown(event, index),
    }),
  );

  return (
    <li {...sourceProps}>
      {reorderable && !embeddedHandle ? (
        <span className="poodle-editable-list__handle" aria-hidden="true">
          <svg viewBox="0 0 16 16" fill="currentColor">
            <circle cx="5" cy="4" r="1.25" />
            <circle cx="11" cy="4" r="1.25" />
            <circle cx="5" cy="8" r="1.25" />
            <circle cx="11" cy="8" r="1.25" />
            <circle cx="5" cy="12" r="1.25" />
            <circle cx="11" cy="12" r="1.25" />
          </svg>
        </span>
      ) : null}
      <span className="poodle-editable-list__content">
        {itemRender ? itemRender(item) : (item.label ?? item.id)}
      </span>
      {showRemove ? (
        <div className="poodle-editable-list__remove poodle-editable-list__remove--danger-on-hover">
          <IconButton
            icon="x"
            variant="ghost"
            size={resolvedSize}
            sizeRole="chrome"
            density={resolvedDensity}
            disabled={isUnavailable}
            ariaLabel={`Remove ${item.label ?? item.id}`}
            onClick={(event) => {
              event.stopPropagation();
              onRemove(item.id);
            }}
          />
        </div>
      ) : null}
    </li>
  );
}

function EditableListKeyboardTarget<T extends EditableListItemLike>({
  item,
  index,
  disabled,
  onDrop,
}: {
  item: T;
  index: number;
  disabled: boolean;
  onDrop: (intent: DropIntent) => DragDropCommitResult;
}) {
  useKeyboardDropTarget({
    targetId: item.id,
    acceptedKinds: ["poodle.editable-list"],
    disabled,
    label: item.label ?? item.id,
    order: index,
    resolvePosition: (input) =>
      input.direction === "previous" || input.direction === "first" ? "before" : "after",
    canDrop: (intent, subject) =>
      subject.id === intent.targetId ? { accepted: false, reason: "self" } : { accepted: true, intent },
    onDrop,
  });
  return null;
}

export function EditableList<T extends EditableListItemLike>({
  items: itemsProp,
  ariaLabel = "Editable list",
  disabled = false,
  reorderable = true,
  editable = false,
  addLabel = "Add item",
  addPlaceholder = "New item",
  maxItems = null,
  removable = false,
  embeddedHandle = false,
  size = null,
  sizeRole = "control",
  density = null,
  dirty = false,
  submitting = false,
  errorMessage = null,
  infoMessage = null,
  longListThreshold = 50,
  longListWarningText = null,
  windowSize = null,
  submitLabel = "Save Order",
  cancelLabel = "Cancel",
  showWorkflowChrome = true,
  onSubmit = null,
  onCancel = null,
  onReorder = null,
  onAdd = null,
  onRemove = null,
  onChange = null,
  item: itemRender,
}: EditableListProps<T>) {
  const uiPresentation = useUiPresentation();

  const [uncontrolledItems, setUncontrolledItems] = useState<T[]>([]);
  const isControlled = itemsProp !== undefined;
  const items = isControlled ? itemsProp : uncontrolledItems;

  const [newItemText, setNewItemText] = useState("");
  const [lastMovedId, setLastMovedId] = useState<string | null>(null);
  const [windowPageIndex, setWindowPageIndex] = useState(0);
  const [liveMessage, setLiveMessage] = useState("");
  const clearLastMovedTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);
  const itemsRef = useRef(items);
  itemsRef.current = items;
  const activeSourceIdRef = useRef<string | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isUnavailable = disabled || submitting;
  const canAdd = editable && !isUnavailable && (maxItems === null || items.length < maxItems);
  const showRemove = editable || removable;
  const effectiveShowWorkflowChrome = showWorkflowChrome && (onSubmit !== null || onCancel !== null);
  const isLongList = longListThreshold !== null && longListThreshold > 0 && items.length > longListThreshold;
  const effectiveLongListWarning =
    longListWarningText ??
    `This list has ${items.length} items. Reordering large lists can be error-prone; consider chunked moves and save often.`;
  const effectiveWindowSize = windowSize !== null && windowSize > 0 ? windowSize : items.length;
  const isWindowed = windowSize !== null && windowSize > 0 && items.length > effectiveWindowSize;
  const windowPageCount = isWindowed ? Math.ceil(items.length / effectiveWindowSize) : 1;
  const windowStart = isWindowed ? windowPageIndex * effectiveWindowSize : 0;
  const windowEnd = Math.min(windowStart + effectiveWindowSize, items.length);
  const visibleItems = items.slice(windowStart, windowEnd);

  useEffect(() => {
    if (!isWindowed && windowPageIndex !== 0) {
      setWindowPageIndex(0);
      return;
    }
    if (isWindowed && windowPageIndex >= windowPageCount) {
      setWindowPageIndex(Math.max(windowPageCount - 1, 0));
    }
  }, [isWindowed, windowPageIndex, windowPageCount]);

  useEffect(
    () => () => {
      if (clearLastMovedTimeout.current) clearTimeout(clearLastMovedTimeout.current);
    },
    [],
  );

  function commitItems(nextItems: T[]): void {
    if (!isControlled) setUncontrolledItems(nextItems);
    onChange?.(nextItems);
  }

  function ensureIndexVisible(index: number): void {
    if (!isWindowed || effectiveWindowSize <= 0) return;
    setWindowPageIndex(Math.floor(index / effectiveWindowSize));
  }

  function markLastMoved(id: string): void {
    setLastMovedId(id);
    if (clearLastMovedTimeout.current) clearTimeout(clearLastMovedTimeout.current);
    clearLastMovedTimeout.current = setTimeout(() => {
      setLastMovedId((current) => (current === id ? null : current));
      clearLastMovedTimeout.current = null;
    }, 1400);
  }

  function moveItem(fromIndex: number, toIndex: number): void {
    const current = itemsRef.current;
    if (fromIndex === toIndex || fromIndex < 0 || toIndex < 0) return;
    if (fromIndex >= current.length || toIndex >= current.length) return;

    const { items: updated } = applyReorder(current, fromIndex, toIndex);
    const moved = updated[toIndex];
    if (!isControlled) setUncontrolledItems(updated);
    onReorder?.(updated);
    onChange?.(updated);
    ensureIndexVisible(toIndex);
    markLastMoved(moved.id);
    setLiveMessage(`Moved ${moved.label ?? moved.id} to position ${toIndex + 1} of ${updated.length}.`);
    requestAnimationFrame(() => {
      document.querySelector<HTMLElement>(`[data-reorder-index="${toIndex}"]`)?.focus();
    });
  }

  function handleDragStart(session: DragSession): void {
    activeSourceIdRef.current = session.subject.id;
    const current = itemsRef.current.find((entry) => entry.id === session.subject.id);
    setLiveMessage(
      `Grabbed ${current?.label ?? session.subject.id}. Use arrow keys to move, Escape to cancel.`,
    );
  }

  function handleDrop(intent: DropIntent): DragDropCommitResult {
    const current = itemsRef.current;
    const fromIndex = current.findIndex((entry) => entry.id === activeSourceIdRef.current);
    const toIndex = current.findIndex((entry) => entry.id === intent.targetId);
    if (fromIndex < 0 || toIndex < 0) return { status: "rejected", reason: "missing-item" };
    const dest = destinationIndex(fromIndex, toIndex, String(intent.position), current.length);
    if (dest !== fromIndex) moveItem(fromIndex, dest);
    return { status: "committed" };
  }

  function handleDragEnd(outcome: DragTerminalOutcome): void {
    if (outcome.status === "cancelled" && outcome.reason === "escape") {
      setLiveMessage("Cancelled keyboard move.");
    }
    activeSourceIdRef.current = null;
  }

  function handleIdleKeydown(event: KeyboardEvent<HTMLLIElement>, index: number): void {
    if (isUnavailable || activeSourceIdRef.current !== null) return;
    if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;
    const next = event.key === "ArrowDown" ? index + 1 : index - 1;
    event.preventDefault();
    if (next < 0 || next >= items.length) {
      setLiveMessage("Reached list boundary.");
      return;
    }
    ensureIndexVisible(next);
    requestAnimationFrame(() => {
      document.querySelector<HTMLElement>(`[data-reorder-index="${next}"]`)?.focus();
    });
  }

  function addItem(): void {
    const label = newItemText.trim();
    if (!label || !canAdd) return;
    const newItem = {
      id: `item-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
      label,
    } as T;
    const updated = [...items, newItem];
    commitItems(updated);
    setNewItemText("");
    onAdd?.(newItem);
  }

  function removeItem(id: string): void {
    if (isUnavailable) return;
    const updated = items.filter((entry) => entry.id !== id);
    commitItems(updated);
    onRemove?.(id);
  }

  return (
    <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
      <div
        className="poodle-editable-list-session"
        data-disabled={isUnavailable}
        data-size={resolvedSize}
        data-density={resolvedDensity}
      >
        <div className="poodle-editable-list-session__sr" aria-live="polite" aria-atomic="true">
          {liveMessage}
        </div>

        {effectiveShowWorkflowChrome ? (
          <div className="poodle-editable-list-session__header">
            <Button
              variant="secondary"
              onClick={() => {
                if (!onCancel || isUnavailable) return;
                onCancel();
              }}
              disabled={isUnavailable}
            >
              {cancelLabel}
            </Button>
            <Button
              variant="primary"
              onClick={async () => {
                if (!onSubmit || isUnavailable || !dirty) return;
                await onSubmit();
              }}
              disabled={!dirty || isUnavailable}
            >
              {submitting ? "Saving..." : submitLabel}
            </Button>
          </div>
        ) : null}

        {errorMessage ? (
          <div className="poodle-editable-list-session__error" role="alert">
            {errorMessage}
          </div>
        ) : null}

        {infoMessage ? (
          <div className="poodle-editable-list-session__info" role="status">
            {infoMessage}
          </div>
        ) : null}

        {isLongList ? (
          <div className="poodle-editable-list-session__info" role="status">
            {effectiveLongListWarning}
          </div>
        ) : null}

        {isWindowed ? (
          <div className="poodle-editable-list-session__window-nav">
            <Button
              variant="secondary"
              onClick={() => setWindowPageIndex((i) => Math.max(i - 1, 0))}
              disabled={isUnavailable || windowPageIndex === 0}
            >
              Previous
            </Button>
            <span className="poodle-editable-list-session__window-label">
              Page {windowPageIndex + 1} of {windowPageCount} · Items {windowStart + 1}-{windowEnd} of {items.length}
            </span>
            <Button
              variant="secondary"
              onClick={() => setWindowPageIndex((i) => Math.min(i + 1, windowPageCount - 1))}
              disabled={isUnavailable || windowPageIndex >= windowPageCount - 1}
            >
              Next
            </Button>
          </div>
        ) : null}

        <DragDropProvider describeAnnouncement={() => null}>
          {items.map((item, index) => (
            <EditableListKeyboardTarget
              key={`keyboard-${item.id}`}
              item={item}
              index={index}
              disabled={!reorderable || isUnavailable}
              onDrop={handleDrop}
            />
          ))}
          <ul
            className={`poodle-editable-list${embeddedHandle ? " poodle-editable-list--embedded-handle" : ""}`}
            role="listbox"
            aria-label={ariaLabel}
            data-disabled={isUnavailable}
            data-size={resolvedSize}
            data-density={resolvedDensity}
          >
            {visibleItems.map((reorderItem, localIndex) => {
              const index = windowStart + localIndex;
              return (
                <EditableListRow
                  key={reorderItem.id}
                  item={reorderItem}
                  index={index}
                  indexOfId={(id) => items.findIndex((entry) => entry.id === id)}
                  total={items.length}
                  reorderable={reorderable}
                  embeddedHandle={embeddedHandle}
                  isUnavailable={isUnavailable}
                  showRemove={showRemove}
                  lastMoved={lastMovedId === reorderItem.id}
                  resolvedSize={resolvedSize}
                  resolvedDensity={resolvedDensity}
                  itemRender={itemRender}
                  onDrop={handleDrop}
                  onDragStart={handleDragStart}
                  onDragEnd={handleDragEnd}
                  onRemove={removeItem}
                  onIdleKeydown={handleIdleKeydown}
                />
              );
            })}
          </ul>
        </DragDropProvider>

        {canAdd ? (
          <div className="poodle-editable-list__add">
            <div className="poodle-editable-list__add-input">
              <TextInput
                value={newItemText}
                onValueChange={setNewItemText}
                placeholder={addPlaceholder}
                disabled={isUnavailable}
                size={resolvedSize}
                density={resolvedDensity}
                onKeyDown={(event) => {
                  if (event.key === "Enter") {
                    event.preventDefault();
                    addItem();
                  }
                }}
              />
            </div>
            <div className="poodle-editable-list__add-btn">
              <Button
                type="button"
                variant="primary"
                size={resolvedSize}
                disabled={!newItemText.trim() || !canAdd}
                onClick={addItem}
              >
                {addLabel}
              </Button>
            </div>
          </div>
        ) : null}

        {editable && maxItems !== null ? (
          <span className="poodle-editable-list__count">
            {items.length}/{maxItems}
          </span>
        ) : null}
      </div>
    </UiPresentationProvider>
  );
}

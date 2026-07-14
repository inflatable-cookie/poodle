import { useEffect, useRef, useState, type DragEvent, type KeyboardEvent, type ReactNode } from "react";
import { applyReorder, listReorderKeyIntent } from "@poodle/headless";

import "@poodle/styles/editable-list.css";

import { Button } from "./Button";
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
  const [draggingIndex, setDraggingIndex] = useState<number | null>(null);
  const [dropTargetIndex, setDropTargetIndex] = useState<number | null>(null);
  const [grabbedIndex, setGrabbedIndex] = useState<number | null>(null);
  const [lastMovedId, setLastMovedId] = useState<string | null>(null);
  const [windowPageIndex, setWindowPageIndex] = useState(0);
  const [liveMessage, setLiveMessage] = useState("");
  const clearLastMovedTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);

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
    if (fromIndex === toIndex || fromIndex < 0 || toIndex < 0) return;
    if (fromIndex >= items.length || toIndex >= items.length) return;

    const { items: updated } = applyReorder(items, fromIndex, toIndex);
    const moved = updated[toIndex];
    if (!isControlled) setUncontrolledItems(updated);
    onReorder?.(updated);
    onChange?.(updated);
    ensureIndexVisible(toIndex);
    markLastMoved(moved.id);
    setLiveMessage(`Moved ${moved.label ?? moved.id} to position ${toIndex + 1} of ${updated.length}.`);
  }

  function handleKeydown(event: KeyboardEvent<HTMLLIElement>, index: number): void {
    if (isUnavailable) return;
    const intent = listReorderKeyIntent(event.key, index, grabbedIndex, items.length);
    if (!intent) return;
    event.preventDefault();

    switch (intent.type) {
      case "grab":
        setGrabbedIndex(index);
        setLiveMessage(
          `Grabbed ${items[index]?.label ?? items[index]?.id ?? "item"}. Use arrow keys to move, Escape to cancel.`,
        );
        break;
      case "drop":
        setGrabbedIndex(null);
        setLiveMessage("Dropped item.");
        break;
      case "cancelGrab":
        setGrabbedIndex(null);
        setLiveMessage("Cancelled keyboard move.");
        break;
      case "boundary":
        setLiveMessage("Reached list boundary.");
        break;
      case "move": {
        moveItem(intent.from, intent.to);
        if (grabbedIndex !== null) setGrabbedIndex(intent.to);
        requestAnimationFrame(() => {
          document.querySelector<HTMLElement>(`[data-reorder-index="${intent.to}"]`)?.focus();
        });
        break;
      }
    }
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
                setGrabbedIndex(null);
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
              <li
                key={reorderItem.id}
                className={[
                  "poodle-editable-list__item",
                  draggingIndex === index ? "poodle-editable-list__item--dragging" : "",
                  dropTargetIndex === index && draggingIndex !== index ? "poodle-editable-list__item--drop-target" : "",
                  grabbedIndex === index ? "poodle-editable-list__item--grabbed" : "",
                  lastMovedId === reorderItem.id ? "poodle-editable-list__item--last-moved" : "",
                  embeddedHandle ? "poodle-editable-list__item--embedded-handle" : "",
                ]
                  .filter(Boolean)
                  .join(" ")}
                role="option"
                tabIndex={isUnavailable ? -1 : 0}
                aria-selected="false"
                aria-label={`Reorder ${reorderItem.label ?? reorderItem.id}. Position ${index + 1} of ${items.length}. Press space to grab, then arrow keys to move.`}
                data-reorder-index={index}
                draggable={reorderable && !isUnavailable}
                onDragStart={(event: DragEvent<HTMLLIElement>) => {
                  if (isUnavailable) return;
                  setDraggingIndex(index);
                  setDropTargetIndex(index);
                  if (event.dataTransfer) {
                    event.dataTransfer.effectAllowed = "move";
                    event.dataTransfer.setData("text/plain", String(index));
                  }
                }}
                onDragOver={(event: DragEvent<HTMLLIElement>) => {
                  if (isUnavailable || draggingIndex === null) return;
                  event.preventDefault();
                  setDropTargetIndex(index);
                }}
                onDrop={(event: DragEvent<HTMLLIElement>) => {
                  event.preventDefault();
                  if (draggingIndex !== null && draggingIndex !== index) moveItem(draggingIndex, index);
                  setDraggingIndex(null);
                  setDropTargetIndex(null);
                }}
                onDragEnd={() => {
                  setDraggingIndex(null);
                  setDropTargetIndex(null);
                }}
                onKeyDown={(event) => handleKeydown(event, index)}
              >
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
                  {itemRender ? itemRender(reorderItem) : (reorderItem.label ?? reorderItem.id)}
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
                      ariaLabel={`Remove ${reorderItem.label ?? reorderItem.id}`}
                      onClick={(event) => {
                        event.stopPropagation();
                        removeItem(reorderItem.id);
                      }}
                    />
                  </div>
                ) : null}
              </li>
            );
          })}
        </ul>

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

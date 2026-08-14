import "@inflatable-cookie/poodle-core/styles/model-connection.css";

import { useMemo, useRef, useState, type DragEvent, type KeyboardEvent, type ReactNode } from "react";

import {
  hiddenModelCatalogueItems,
  listReorderKeyIntent,
  modelCatalogueFocusAfterHide,
  modelCatalogueReorderAnnouncement,
  modelCatalogueStateCopy,
  modelCatalogueVisibilityAnnouncement,
  requestModelCatalogueOrder,
  requestModelCatalogueVisibility,
  shownModelCatalogueItems,
  type ModelCatalogueItem,
  type ModelCatalogueState,
} from "@inflatable-cookie/poodle-core";

import { Collapsible } from "./Collapsible";
import { EmptyState } from "./EmptyState";
import { Icon } from "./Icon";
import { IconButton } from "./IconButton";
import { Pill } from "./Pill";
import { Spinner } from "./Spinner";

export interface ModelCatalogueEditorItemProps {
  item: ModelCatalogueItem;
}

export interface ModelCatalogueEditorProps {
  items?: ModelCatalogueItem[];
  state?: ModelCatalogueState;
  title?: string;
  hiddenTitle?: string;
  ariaLabel?: string | null;
  isDisabled?: boolean;
  isPending?: boolean;
  isDragEnabled?: boolean;
  showMoveActions?: boolean;
  stateTitle?: string | null;
  stateMessage?: string | null;
  onOrderChange?: ((orderedIds: string[]) => void) | null;
  onVisibilityChange?: ((change: { id: string; visible: boolean }) => void) | null;
  onInfo?: ((id: string) => void) | null;
  leading?: (props: ModelCatalogueEditorItemProps) => ReactNode;
  customAction?: () => ReactNode;
  rowMeta?: (props: ModelCatalogueEditorItemProps) => ReactNode;
}

export function ModelCatalogueEditor({
  items = [],
  state = "ready",
  title = "Models",
  hiddenTitle = "Hidden models",
  ariaLabel = null,
  isDisabled = false,
  isPending = false,
  isDragEnabled = true,
  showMoveActions = true,
  stateTitle = null,
  stateMessage = null,
  onOrderChange = null,
  onVisibilityChange = null,
  onInfo = null,
  leading,
  customAction,
  rowMeta,
}: ModelCatalogueEditorProps) {
  const rootRef = useRef<HTMLElement | null>(null);

  const [grabbedIndex, setGrabbedIndex] = useState<number | null>(null);
  const [draggingIndex, setDraggingIndex] = useState<number | null>(null);
  const [dropTargetIndex, setDropTargetIndex] = useState<number | null>(null);
  const [liveMessage, setLiveMessage] = useState("");
  const [hiddenOpen, setHiddenOpen] = useState(false);

  const shown = useMemo(() => shownModelCatalogueItems(items), [items]);
  const hidden = useMemo(() => hiddenModelCatalogueItems(items), [items]);
  const locked = isDisabled || isPending;
  const defaults = useMemo(() => modelCatalogueStateCopy(state), [state]);
  const resolvedStateTitle = stateTitle ?? defaults.title;
  const resolvedStateMessage = stateMessage ?? defaults.message;

  function announce(message: string): void {
    setLiveMessage(message);
  }

  function focusShown(id: string): void {
    requestAnimationFrame(() => {
      rootRef.current
        ?.querySelector<HTMLElement>(
          `[data-model-catalogue-id="${CSS.escape(id)}"] [data-reorder-handle]`,
        )
        ?.focus();
    });
  }

  function focusHiddenSection(): void {
    setHiddenOpen(true);
    requestAnimationFrame(() => {
      rootRef.current
        ?.querySelector<HTMLElement>(".poodle-model-catalogue-editor__hidden button")
        ?.focus();
    });
  }

  function emitOrder(fromIndex: number, toIndex: number): void {
    if (locked) return;
    const ids = shown.map((item) => item.id);
    const next = requestModelCatalogueOrder(ids, fromIndex, toIndex);
    if (!next) return;
    const moved = shown[fromIndex];
    onOrderChange?.(next);
    announce(
      modelCatalogueReorderAnnouncement(moved?.label ?? moved?.id ?? "model", toIndex + 1, next.length),
    );
    const focusId = next[toIndex];
    if (focusId) focusShown(focusId);
    if (grabbedIndex !== null) setGrabbedIndex(toIndex);
  }

  function hideItem(item: ModelCatalogueItem): void {
    if (locked || item.isDisabled) return;
    const ids = shown.map((entry) => entry.id);
    const focus = modelCatalogueFocusAfterHide(ids, item.id);
    onVisibilityChange?.(requestModelCatalogueVisibility(item.id, false));
    announce(modelCatalogueVisibilityAnnouncement(item.label, false));
    if (focus.kind === "shown") focusShown(focus.id);
    else focusHiddenSection();
  }

  function restoreItem(item: ModelCatalogueItem): void {
    if (locked || item.isDisabled) return;
    onVisibilityChange?.(requestModelCatalogueVisibility(item.id, true));
    announce(modelCatalogueVisibilityAnnouncement(item.label, true));
  }

  function handleKeydown(event: KeyboardEvent<HTMLButtonElement>, index: number): void {
    if (locked) return;
    const intent = listReorderKeyIntent(event.key, index, grabbedIndex, shown.length);
    if (!intent) return;
    event.preventDefault();

    switch (intent.type) {
      case "grab":
        setGrabbedIndex(index);
        announce(
          `Grabbed ${shown[index]?.label ?? "model"}. Use arrow keys to move, Escape to cancel.`,
        );
        break;
      case "drop":
        setGrabbedIndex(null);
        announce("Dropped item.");
        break;
      case "cancelGrab":
        setGrabbedIndex(null);
        announce("Cancelled keyboard move.");
        break;
      case "boundary":
        announce("Reached list boundary.");
        break;
      case "move":
        emitOrder(intent.from, intent.to);
        break;
    }
  }

  function handleDragStart(event: DragEvent<HTMLLIElement>, index: number): void {
    if (locked || !isDragEnabled) return;
    setDraggingIndex(index);
    setDropTargetIndex(index);
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData("text/plain", String(index));
    }
  }

  function handleDragOver(event: DragEvent<HTMLLIElement>, index: number): void {
    if (locked || draggingIndex === null) return;
    event.preventDefault();
    setDropTargetIndex(index);
  }

  function handleDrop(event: DragEvent<HTMLLIElement>, index: number): void {
    event.preventDefault();
    if (draggingIndex !== null && draggingIndex !== index) {
      emitOrder(draggingIndex, index);
    }
    setDraggingIndex(null);
    setDropTargetIndex(null);
  }

  function handleDragEnd(): void {
    setDraggingIndex(null);
    setDropTargetIndex(null);
  }

  return (
    <section
      ref={rootRef}
      className="poodle-model-catalogue-editor"
      aria-label={ariaLabel ?? title}
      data-state={state}
      data-pending={isPending ? "true" : "false"}
      aria-busy={isPending ? "true" : undefined}
    >
      <div className="poodle-model-catalogue-editor__header">
        <div>
          <h3 className="poodle-model-catalogue-editor__title">{title}</h3>
          {state === "ready" ? (
            <span className="poodle-model-catalogue-editor__count">
              {shown.length} shown{hidden.length > 0 ? `, ${hidden.length} hidden` : ""}
            </span>
          ) : null}
        </div>
        {customAction ? customAction() : null}
      </div>

      <p
        className="poodle-model-catalogue-editor__live"
        role="status"
        aria-live="polite"
        aria-atomic="true"
      >
        {liveMessage}
      </p>

      {state !== "ready" ? (
        state === "loading" ? (
          <EmptyState title={resolvedStateTitle} message={resolvedStateMessage} visual={<Spinner variant="grid" tone="accent" />} />
        ) : (
          <EmptyState
            title={resolvedStateTitle}
            message={resolvedStateMessage}
            variant={state === "error" ? "neutral" : state === "empty" ? "firstRun" : "neutral"}
          />
        )
      ) : (
        <>
          <ol className="poodle-model-catalogue-editor__list" aria-label="Shown models">
            {shown.map((item, index) => (
              <li
                key={item.id}
                className="poodle-model-catalogue-editor__row"
                data-model-catalogue-id={item.id}
                data-grabbed={grabbedIndex === index ? "true" : "false"}
                data-drop-target={dropTargetIndex === index ? "true" : "false"}
                draggable={isDragEnabled && !locked && !item.isDisabled}
                onDragStart={(event) => handleDragStart(event, index)}
                onDragOver={(event) => handleDragOver(event, index)}
                onDrop={(event) => handleDrop(event, index)}
                onDragEnd={handleDragEnd}
              >
                <button
                  type="button"
                  className="poodle-icon-button"
                  data-variant="ghost"
                  data-size-role="chrome"
                  data-reorder-handle=""
                  aria-label={`${item.label}, position ${index + 1} of ${shown.length}`}
                  disabled={locked || item.isDisabled}
                  onKeyDown={(event) => handleKeydown(event, index)}
                  onClick={() => {
                    if (locked || item.isDisabled) return;
                    if (grabbedIndex === index) {
                      setGrabbedIndex(null);
                      announce("Dropped item.");
                    } else {
                      setGrabbedIndex(index);
                      announce(
                        `Grabbed ${item.label}. Use arrow keys to move, Escape to cancel.`,
                      );
                    }
                  }}
                >
                  <span className="poodle-icon-button__glyph" aria-hidden="true">
                    <Icon name="grip-vertical" />
                  </span>
                </button>

                <div className="poodle-model-catalogue-editor__identity">
                  <div className="poodle-model-catalogue-editor__label-row">
                    {leading ? (
                      <span className="poodle-model-catalogue-editor__leading" aria-hidden="true">
                        {leading({ item })}
                      </span>
                    ) : null}
                    <p className="poodle-model-catalogue-editor__label">{item.label}</p>
                    {item.badges.map((badge) => (
                      <Pill key={badge.label} tone={badge.tone ?? "neutral"} appearance="subtle">
                        {badge.label}
                      </Pill>
                    ))}
                  </div>
                  {item.providerLabel ? (
                    <p className="poodle-model-catalogue-editor__provider">{item.providerLabel}</p>
                  ) : null}
                  {item.description ? (
                    <p className="poodle-model-catalogue-editor__description">{item.description}</p>
                  ) : null}
                  {rowMeta ? (
                    <div className="poodle-model-catalogue-editor__meta">{rowMeta({ item })}</div>
                  ) : null}
                </div>

                <div className="poodle-model-catalogue-editor__utilities">
                  {onInfo ? (
                    <IconButton
                      icon="info"
                      ariaLabel={`About ${item.label}`}
                      variant="ghost"
                      sizeRole="chrome"
                      disabled={locked}
                      onClick={() => onInfo(item.id)}
                    />
                  ) : null}
                  {showMoveActions ? (
                    <>
                      <IconButton
                        icon="arrow-up"
                        ariaLabel={`Move ${item.label} up`}
                        variant="ghost"
                        sizeRole="chrome"
                        disabled={locked || item.isDisabled || index === 0}
                        onClick={() => emitOrder(index, index - 1)}
                      />
                      <IconButton
                        icon="arrow-down"
                        ariaLabel={`Move ${item.label} down`}
                        variant="ghost"
                        sizeRole="chrome"
                        disabled={locked || item.isDisabled || index === shown.length - 1}
                        onClick={() => emitOrder(index, index + 1)}
                      />
                    </>
                  ) : null}
                  <IconButton
                    icon="eye"
                    ariaLabel={`Hide ${item.label}`}
                    variant="ghost"
                    sizeRole="chrome"
                    disabled={locked || item.isDisabled}
                    onClick={() => hideItem(item)}
                  />
                </div>
              </li>
            ))}
          </ol>

          {hidden.length > 0 ? (
            <div className="poodle-model-catalogue-editor__hidden">
              <Collapsible title={hiddenTitle} open={hiddenOpen} onOpenChange={(next) => setHiddenOpen(next)}>
                <ul
                  className="poodle-model-catalogue-editor__list poodle-model-catalogue-editor__hidden-list"
                  aria-label={hiddenTitle}
                >
                  {hidden.map((item) => (
                    <li
                      key={item.id}
                      className="poodle-model-catalogue-editor__row"
                      data-model-catalogue-id={item.id}
                    >
                      <div className="poodle-model-catalogue-editor__identity">
                        <div className="poodle-model-catalogue-editor__label-row">
                          <p className="poodle-model-catalogue-editor__label">{item.label}</p>
                        </div>
                        {item.providerLabel ? (
                          <p className="poodle-model-catalogue-editor__provider">{item.providerLabel}</p>
                        ) : null}
                      </div>
                      <div className="poodle-model-catalogue-editor__utilities">
                        <IconButton
                          icon="undo"
                          ariaLabel={`Restore ${item.label}`}
                          variant="ghost"
                          sizeRole="chrome"
                          disabled={locked || item.isDisabled}
                          onClick={() => restoreItem(item)}
                        />
                      </div>
                    </li>
                  ))}
                </ul>
              </Collapsible>
            </div>
          ) : null}
        </>
      )}
    </section>
  );
}

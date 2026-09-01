import "@inflatable-cookie/poodle-core/styles/model-connection.css";

import { useId, useMemo, useRef, useState, type KeyboardEvent, type ReactNode } from "react";

import {
  createDragDropController,
  hiddenModelCatalogueItems,
  listReorderKeyIntent,
  modelCatalogueFocusAfterHide,
  modelCatalogueReorderAnnouncement,
  modelCatalogueStateCopy,
  modelCatalogueVisibilityAnnouncement,
  requestModelCatalogueOrder,
  requestModelCatalogueVisibility,
  shownModelCatalogueItems,
  type DragDropCommitResult,
  type DropIntent,
  type ModelCatalogueItem,
  type ModelCatalogueState,
} from "@inflatable-cookie/poodle-core";

import { Collapsible } from "./Collapsible";
import { Callout } from "./Callout";
import { DragDropProvider, useOptionalDragDrop } from "./drag-drop";
import { EmptyState } from "./EmptyState";
import { IconButton } from "./IconButton";
import { ModelCatalogueRow } from "./model-catalogue-editor/ModelCatalogueRow";
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

  const [grabbedId, setGrabbedId] = useState<string | null>(null);
  const [liveMessage, setLiveMessage] = useState("");
  const [hiddenOpen, setHiddenOpen] = useState(false);
  const instanceId = useId();

  /**
   * Join the nearest provider, or own a controller.
   *
   * The registration ids and the subject family are both scoped to this
   * editor, so two catalogues showing the same model ids under one ambient
   * provider can neither mint one id nor resolve each other's rows.
   */
  const ambient = useOptionalDragDrop();
  const [ownDragController] = useState(() => (ambient ? null : createDragDropController()));
  const dragController = ambient?.controller ?? ownDragController!;

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
    const grabbedIndex = grabbedId === null
      ? null
      : shown.findIndex((item) => item.id === grabbedId);
    const intent = listReorderKeyIntent(event.key, index, grabbedIndex, shown.length);
    if (!intent) return;
    event.preventDefault();

    switch (intent.type) {
      case "grab":
        setGrabbedId(shown[index]?.id ?? null);
        announce(
          `Grabbed ${shown[index]?.label ?? "model"}. Use arrow keys to move, Escape to cancel.`,
        );
        break;
      case "drop":
        setGrabbedId(null);
        announce("Dropped item.");
        break;
      case "cancelGrab":
        setGrabbedId(null);
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

  const subjectKind = `poodle.reorder-item:model-catalogue-editor:${instanceId}`;
  const registrationScope = `model-catalogue-editor:${instanceId}`;

  function sourceIdOf(id: string): string {
    return `${registrationScope}:source:${id}`;
  }

  function targetIdOf(id: string): string {
    return `${registrationScope}:target:${id}`;
  }

  function idOfTargetId(targetId: string): string {
    const prefix = `${registrationScope}:target:`;
    return targetId.startsWith(prefix) ? targetId.slice(prefix.length) : "";
  }

  function indexOfShown(id: string): number {
    return shown.findIndex((item) => item.id === id);
  }

  /**
   * One accepted drop, one complete shown-id order.
   *
   * Both indices are resolved again here: `items` may have been replaced while
   * the pointer was down, and a stale index would move the wrong model.
   */
  function handleDrop(intent: DropIntent): DragDropCommitResult {
    if (locked) return { status: "rejected", reason: "locked" };

    const from = indexOfShown(dragController.getSnapshot().session?.subject.id ?? "");
    const target = indexOfShown(idOfTargetId(intent.targetId));
    if (from < 0 || target < 0 || from === target) {
      return { status: "rejected", reason: "missing model" };
    }

    const to =
      intent.position === "before"
        ? from < target
          ? target - 1
          : target
        : from < target
          ? target
          : target + 1;

    // A pointer drop ends any live keyboard grab: one row cannot be both
    // dropped and still held.
    setGrabbedId(null);
    emitOrder(from, to);
    return { status: "committed" };
  }

  const editor = (
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
        ) : state === "error" ? (
          <Callout
            tone="danger"
            title={resolvedStateTitle}
            message={resolvedStateMessage}
            announceMode="assertive"
          />
        ) : (
          <EmptyState
            title={resolvedStateTitle}
            message={resolvedStateMessage}
            variant={state === "empty" ? "firstRun" : "neutral"}
          />
        )
      ) : (
        <>
          <ol className="poodle-model-catalogue-editor__list" aria-label="Shown models">
            {shown.map((item, index) => (
              <ModelCatalogueRow
                key={item.id}
                item={item}
                index={index}
                total={shown.length}
                grabbed={grabbedId === item.id}
                locked={locked}
                isDragEnabled={isDragEnabled}
                subjectKind={subjectKind}
                sourceId={sourceIdOf(item.id)}
                targetId={targetIdOf(item.id)}
                indexOfShown={indexOfShown}
                onDrop={handleDrop}
                onHandleKeyDown={handleKeydown}
                onToggleGrab={() => {
                  if (locked || item.isDisabled) return;
                  if (grabbedId === item.id) {
                    setGrabbedId(null);
                    announce("Dropped item.");
                  } else {
                    setGrabbedId(item.id);
                    announce(
                      `Grabbed ${item.label}. Use arrow keys to move, Escape to cancel.`,
                    );
                  }
                }}
              >
                <div className="poodle-model-catalogue-editor__identity">
                  <div className="poodle-model-catalogue-editor__label-row">
                    {leading ? (
                      <span className="poodle-model-catalogue-editor__leading" aria-hidden="true">
                        {leading({ item })}
                      </span>
                    ) : null}
                    <p className="poodle-model-catalogue-editor__label">
                      {item.label}
                      {item.providerLabel ? (
                        <span className="poodle-model-catalogue-editor__provider">
                          {` ${item.providerLabel}`}
                        </span>
                      ) : null}
                    </p>
                  </div>
                  {item.description ? (
                    <p className="poodle-model-catalogue-editor__description">{item.description}</p>
                  ) : null}
                  {rowMeta ? (
                    <div className="poodle-model-catalogue-editor__meta">{rowMeta({ item })}</div>
                  ) : null}
                </div>

                <div className="poodle-model-catalogue-editor__utilities">
                  {item.badges.map((badge) => (
                    <Pill key={badge.label} tone={badge.tone ?? "neutral"} appearance="subtle">
                      {badge.label}
                    </Pill>
                  ))}
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
              </ModelCatalogueRow>
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
                          <p className="poodle-model-catalogue-editor__label">
                            {item.label}
                            {item.providerLabel ? (
                              <span className="poodle-model-catalogue-editor__provider">
                                {` ${item.providerLabel}`}
                              </span>
                            ) : null}
                          </p>
                        </div>
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

  // An editor that joined a provider contributes registrations to it. One with
  // no provider owns a controller so it still reorders on its own. The editor
  // owns its live region either way, so the substrate's own announcements are
  // suppressed on the owned controller: one terminal must not be read twice.
  return ambient ? (
    editor
  ) : (
    <DragDropProvider controller={ownDragController!} describeAnnouncement={() => null}>
      {editor}
    </DragDropProvider>
  );
}

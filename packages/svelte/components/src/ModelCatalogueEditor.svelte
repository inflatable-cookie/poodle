<script module lang="ts">
  let nextModelCatalogueId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/model-connection.css";
  import {
    createDragDropController,
    listReorderKeyIntent,
    modelCatalogueFocusAfterHide,
    modelCatalogueReorderAnnouncement,
    modelCatalogueStateCopy,
    modelCatalogueVisibilityAnnouncement,
    requestModelCatalogueOrder,
    requestModelCatalogueVisibility,
    shownModelCatalogueItems,
    hiddenModelCatalogueItems,
    type DragDropCommitResult,
    type DragSourceRegistration,
    type DropIntent,
    type DropPosition,
    type DropTargetRegistration,
    type ModelCatalogueItem,
    type ModelCatalogueState,
  } from "@inflatable-cookie/poodle-core";
  import type { Snippet } from "svelte";

  import { default as Collapsible } from "./Collapsible.svelte";
  import { default as Callout } from "./Callout.svelte";
  import { default as DragDropProvider } from "./DragDropProvider.svelte";
  import { default as EmptyState } from "./EmptyState.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as Pill } from "./Pill.svelte";
  import { default as Spinner } from "./Spinner.svelte";
  import {
    dragDropSnapshotStore,
    dragSourceAction,
    dropTargetAction,
    tryDragDrop,
  } from "./drag-drop-context";

  interface ItemProps {
    item: ModelCatalogueItem;
  }

  interface Props {
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
    leading?: Snippet<[ItemProps]>;
    customAction?: Snippet<[]>;
    rowMeta?: Snippet<[ItemProps]>;
  }

  let {
    items = [],
    state: catalogueState = "ready",
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
  }: Props = $props();

  let grabbedId = $state<string | null>(null);
  let liveMessage = $state("");
  let rootEl = $state<HTMLElement | null>(null);
  let hiddenOpen = $state(false);
  const instanceId = ++nextModelCatalogueId;

  const shown = $derived(shownModelCatalogueItems(items));
  const hidden = $derived(hiddenModelCatalogueItems(items));
  const locked = $derived(isDisabled || isPending);
  const defaults = $derived(modelCatalogueStateCopy(catalogueState));
  const resolvedStateTitle = $derived(stateTitle ?? defaults.title);
  const resolvedStateMessage = $derived(stateMessage ?? defaults.message);

  function announce(message: string): void {
    liveMessage = message;
  }

  function focusShown(id: string): void {
    requestAnimationFrame(() => {
      rootEl
        ?.querySelector<HTMLElement>(`[data-model-catalogue-id="${CSS.escape(id)}"] [data-reorder-handle]`)
        ?.focus();
    });
  }

  function focusHiddenSection(): void {
    hiddenOpen = true;
    requestAnimationFrame(() => {
      rootEl
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

  function handleKeydown(event: KeyboardEvent, index: number): void {
    if (locked) return;
    const grabbedIndex = grabbedId === null
      ? null
      : shown.findIndex((item) => item.id === grabbedId);
    const intent = listReorderKeyIntent(event.key, index, grabbedIndex, shown.length);
    if (!intent) return;
    event.preventDefault();

    switch (intent.type) {
      case "grab":
        grabbedId = shown[index]?.id ?? null;
        announce(
          `Grabbed ${shown[index]?.label ?? "model"}. Use arrow keys to move, Escape to cancel.`,
        );
        break;
      case "drop":
        grabbedId = null;
        announce("Dropped item.");
        break;
      case "cancelGrab":
        grabbedId = null;
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

  /**
   * Join the nearest provider, or own a controller.
   *
   * The registration ids and the subject family are both scoped to this
   * editor, so two catalogues showing the same model ids under one ambient
   * provider can neither mint one id nor resolve each other's rows.
   */
  const ambient = tryDragDrop();
  const ownDragController = ambient ? undefined : createDragDropController();
  const dragController = ambient?.controller ?? ownDragController!;
  const dragSource = dragSourceAction(dragController);
  const dropTarget = dropTargetAction(dragController);
  const dragSnapshot = dragDropSnapshotStore(dragController);

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

  function sourceRegistration(item: ModelCatalogueItem): DragSourceRegistration {
    return {
      sourceId: sourceIdOf(item.id),
      subject: { kind: subjectKind, id: item.id },
      allowedOperations: ["move"],
      label: item.label,
      // A locked editor or a disabled model cannot be picked up. It is still a
      // place to put one, which is why the target below does not read
      // `item.isDisabled`.
      disabled: locked || item.isDisabled,
      // This editor has its own contract live region and announces every move
      // through it. Without this, an editor that joined an ambient provider
      // would have one drop read out twice: once as "Dropped Alpha on Gamma"
      // by the provider, once as "Moved Alpha to position 3 of 4" here.
      ownsAnnouncements: true,
    };
  }

  function targetRegistration(item: ModelCatalogueItem, index: number): DropTargetRegistration {
    return {
      targetId: targetIdOf(item.id),
      acceptedKinds: [subjectKind],
      disabled: locked,
      label: item.label,
      // One band per row: a model travelling down lands after its target and
      // one travelling up lands before it, so the dropped model ends up *at*
      // the row it was dropped on — the same result the native renderer emits.
      resolvePosition: ({ subject }): DropPosition =>
        indexOfShown(subject.id) < index ? "after" : "before",
      canDrop: (intent, subject) => {
        if (indexOfShown(subject.id) < 0) {
          return { accepted: false, reason: "not this catalogue" };
        }
        return subject.id === item.id
          ? { accepted: false, reason: "same model" }
          : { accepted: true, intent };
      },
      onDrop: handleDrop,
    };
  }

  /**
   * One accepted drop, one complete shown-id order.
   *
   * Both indices are resolved again here: `items` may have been replaced while
   * the pointer was down, and a stale index would move the wrong model.
   */
  function handleDrop(intent: DropIntent): DragDropCommitResult {
    if (locked) return { status: "rejected", reason: "locked" };

    const from = indexOfShown($dragSnapshot.session?.subject.id ?? "");
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
    grabbedId = null;
    emitOrder(from, to);
    return { status: "committed" };
  }
</script>

{#snippet editor()}
<section
  class="poodle-model-catalogue-editor"
  bind:this={rootEl}
  aria-label={ariaLabel ?? title}
  data-state={catalogueState}
  data-pending={isPending ? "true" : "false"}
  aria-busy={isPending ? "true" : undefined}
>
  <div class="poodle-model-catalogue-editor__header">
    <div>
      <h3 class="poodle-model-catalogue-editor__title">{title}</h3>
      {#if catalogueState === "ready"}
        <span class="poodle-model-catalogue-editor__count">
          {shown.length} shown{#if hidden.length > 0}, {hidden.length} hidden{/if}
        </span>
      {/if}
    </div>
    {#if customAction}
      {@render customAction()}
    {/if}
  </div>

  <p class="poodle-model-catalogue-editor__live" role="status" aria-live="polite" aria-atomic="true">
    {liveMessage}
  </p>

  {#if catalogueState !== "ready"}
    {#if catalogueState === "loading"}
      <EmptyState title={resolvedStateTitle} message={resolvedStateMessage}>
        {#snippet visual()}
          <Spinner variant="grid" tone="accent" />
        {/snippet}
      </EmptyState>
    {:else if catalogueState === "error"}
      <Callout
        tone="danger"
        title={resolvedStateTitle}
        message={resolvedStateMessage}
        announceMode="assertive"
      />
    {:else}
      <EmptyState
        title={resolvedStateTitle}
        message={resolvedStateMessage}
        variant={catalogueState === "empty" ? "firstRun" : "neutral"}
      />
    {/if}
  {:else}
    <ol class="poodle-model-catalogue-editor__list" aria-label="Shown models">
      {#each shown as item, index (item.id)}
        <li
          class="poodle-model-catalogue-editor__row"
          data-model-catalogue-id={item.id}
          data-grabbed={grabbedId === item.id ? "true" : "false"}
          data-drop-target={$dragSnapshot.targetId === targetIdOf(item.id) &&
            $dragSnapshot.targetPosture === "accepted"
            ? "true"
            : "false"}
          use:dropTarget={isDragEnabled ? targetRegistration(item, index) : null}
        >
          <button
            type="button"
            class="poodle-icon-button"
            data-variant="ghost"
            data-size-role="chrome"
            data-reorder-handle=""
            aria-pressed={grabbedId === item.id}
            aria-label={`${item.label}, position ${index + 1} of ${shown.length}`}
            disabled={locked || item.isDisabled}
            use:dragSource={isDragEnabled ? sourceRegistration(item) : null}
            onkeydown={(event) => handleKeydown(event, index)}
            onclick={() => {
              if (locked || item.isDisabled) return;
              if (grabbedId === item.id) {
                grabbedId = null;
                announce("Dropped item.");
              } else {
                grabbedId = item.id;
                announce(
                  `Grabbed ${item.label}. Use arrow keys to move, Escape to cancel.`,
                );
              }
            }}
          >
            <span class="poodle-icon-button__glyph" aria-hidden="true">
              <Icon name="grip-vertical" />
            </span>
          </button>

          <div class="poodle-model-catalogue-editor__identity">
            <div class="poodle-model-catalogue-editor__label-row">
              {#if leading}
                <span class="poodle-model-catalogue-editor__leading" aria-hidden="true">
                  {@render leading({ item })}
                </span>
              {/if}
              <p class="poodle-model-catalogue-editor__label">
                {item.label}{#if item.providerLabel}
                  {" "}<span class="poodle-model-catalogue-editor__provider">{item.providerLabel}</span>
                {/if}
              </p>
            </div>
            {#if item.description}
              <p class="poodle-model-catalogue-editor__description">{item.description}</p>
            {/if}
            {#if rowMeta}
              <div class="poodle-model-catalogue-editor__meta">
                {@render rowMeta({ item })}
              </div>
            {/if}
          </div>

          <div class="poodle-model-catalogue-editor__utilities">
            {#each item.badges as badge (badge.label)}
              <Pill tone={badge.tone ?? "neutral"} appearance="subtle">{badge.label}</Pill>
            {/each}
            {#if onInfo}
              <IconButton
                icon="info"
                ariaLabel={`About ${item.label}`}
                variant="ghost"
                sizeRole="chrome"
                disabled={locked}
                onClick={() => onInfo?.(item.id)}
              />
            {/if}
            {#if showMoveActions}
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
            {/if}
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
      {/each}
    </ol>

    {#if hidden.length > 0}
      <div class="poodle-model-catalogue-editor__hidden">
        <Collapsible title={hiddenTitle} open={hiddenOpen} onOpenChange={(next) => (hiddenOpen = next)}>
          <ul class="poodle-model-catalogue-editor__list poodle-model-catalogue-editor__hidden-list" aria-label={hiddenTitle}>
            {#each hidden as item (item.id)}
              <li class="poodle-model-catalogue-editor__row" data-model-catalogue-id={item.id}>
                <div class="poodle-model-catalogue-editor__identity">
                  <div class="poodle-model-catalogue-editor__label-row">
                    <p class="poodle-model-catalogue-editor__label">
                      {item.label}{#if item.providerLabel}
                        {" "}<span class="poodle-model-catalogue-editor__provider">{item.providerLabel}</span>
                      {/if}
                    </p>
                  </div>
                </div>
                <div class="poodle-model-catalogue-editor__utilities">
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
            {/each}
          </ul>
        </Collapsible>
      </div>
    {/if}
  {/if}
</section>
{/snippet}

{#if ambient}
  {@render editor()}
{:else}
  <!-- The editor owns its live region, so the substrate's own announcements
       are suppressed: one terminal must not be read out twice. -->
  <DragDropProvider controller={ownDragController} describeAnnouncement={() => null}>
    {@render editor()}
  </DragDropProvider>
{/if}

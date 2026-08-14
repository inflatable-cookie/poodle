<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/model-connection.css";
  import {
    listReorderKeyIntent,
    modelCatalogueFocusAfterHide,
    modelCatalogueReorderAnnouncement,
    modelCatalogueStateCopy,
    modelCatalogueVisibilityAnnouncement,
    requestModelCatalogueOrder,
    requestModelCatalogueVisibility,
    shownModelCatalogueItems,
    hiddenModelCatalogueItems,
    type ModelCatalogueItem,
    type ModelCatalogueState,
  } from "@inflatable-cookie/poodle-core";
  import type { Snippet } from "svelte";

  import { default as Collapsible } from "./Collapsible.svelte";
  import { default as Callout } from "./Callout.svelte";
  import { default as EmptyState } from "./EmptyState.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as Pill } from "./Pill.svelte";
  import { default as Spinner } from "./Spinner.svelte";

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
  let draggingId = $state<string | null>(null);
  let dropTargetId = $state<string | null>(null);
  let liveMessage = $state("");
  let rootEl = $state<HTMLElement | null>(null);
  let hiddenOpen = $state(false);

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

  function handleDragStart(event: DragEvent, index: number): void {
    if (locked || !isDragEnabled) return;
    draggingId = shown[index]?.id ?? null;
    dropTargetId = shown[index]?.id ?? null;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData("text/plain", String(index));
    }
  }

  function handleDragOver(event: DragEvent, index: number): void {
    if (locked || draggingId === null) return;
    event.preventDefault();
    dropTargetId = shown[index]?.id ?? null;
  }

  function handleDrop(event: DragEvent, index: number): void {
    event.preventDefault();
    const fromIndex = draggingId === null
      ? -1
      : shown.findIndex((item) => item.id === draggingId);
    if (fromIndex >= 0 && fromIndex !== index) {
      emitOrder(fromIndex, index);
    }
    draggingId = null;
    dropTargetId = null;
  }

  function handleDragEnd(): void {
    draggingId = null;
    dropTargetId = null;
  }
</script>

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
          data-drop-target={dropTargetId === item.id ? "true" : "false"}
          ondragover={(event) => handleDragOver(event, index)}
          ondrop={(event) => handleDrop(event, index)}
          ondragend={handleDragEnd}
        >
          <button
            type="button"
            class="poodle-icon-button"
            data-variant="ghost"
            data-size-role="chrome"
            data-reorder-handle=""
            draggable={isDragEnabled && !locked && !item.isDisabled}
            aria-pressed={grabbedId === item.id}
            aria-label={`${item.label}, position ${index + 1} of ${shown.length}`}
            disabled={locked || item.isDisabled}
            ondragstart={(event) => handleDragStart(event, index)}
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

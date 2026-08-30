<script lang="ts" generics="T extends { id: string; label?: string }">
  import "@inflatable-cookie/poodle-core/styles/editable-list.css";
  import {
    applyReorder,
    type DragDropCommitResult,
    type DragSession,
    type DragTerminalOutcome,
    type DropIntent,
  } from "@inflatable-cookie/poodle-core";
  import { onDestroy, type Snippet } from "svelte";

  import { default as Button } from "./Button.svelte";
  import DragDropProvider from "./DragDropProvider.svelte";
  import EditableListKeyboardTargets from "./editable-list/EditableListKeyboardTargets.svelte";
  import EditableListRow from "./editable-list/EditableListRow.svelte";
  import { default as TextInput } from "./TextInput.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
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
    item?: Snippet<[T]> | undefined;
  }

  let {
    items = $bindable<T[]>([]),
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
    item,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let newItemText = $state("");
  let lastMovedId = $state<string | null>(null);
  let activeSourceId = $state<string | null>(null);
  let windowPageIndex = $state(0);
  let liveMessage = $state("");
  let clearLastMovedTimeout = $state<ReturnType<typeof setTimeout> | null>(null);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isUnavailable = $derived(disabled || submitting);
  const canAdd = $derived(
    editable && !isUnavailable && (maxItems === null || items.length < maxItems),
  );
  const showRemove = $derived(editable || removable);
  const effectiveShowWorkflowChrome = $derived(
    showWorkflowChrome && (onSubmit !== null || onCancel !== null),
  );
  const isLongList = $derived(
    longListThreshold !== null && longListThreshold > 0 && items.length > longListThreshold,
  );
  const effectiveLongListWarning = $derived(
    longListWarningText ??
      `This list has ${items.length} items. Reordering large lists can be error-prone; consider chunked moves and save often.`,
  );
  const effectiveWindowSize = $derived(
    windowSize !== null && windowSize > 0 ? windowSize : items.length,
  );
  const isWindowed = $derived(
    windowSize !== null && windowSize > 0 && items.length > effectiveWindowSize,
  );
  const windowPageCount = $derived(
    isWindowed ? Math.ceil(items.length / effectiveWindowSize) : 1,
  );
  const windowStart = $derived(isWindowed ? windowPageIndex * effectiveWindowSize : 0);
  const windowEnd = $derived(Math.min(windowStart + effectiveWindowSize, items.length));
  const visibleItems = $derived(items.slice(windowStart, windowEnd));

  $effect(() => {
    if (!isWindowed && windowPageIndex !== 0) {
      windowPageIndex = 0;
      return;
    }

    if (isWindowed && windowPageIndex >= windowPageCount) {
      windowPageIndex = Math.max(windowPageCount - 1, 0);
    }
  });

  function commitItems(nextItems: T[]): void {
    items = nextItems;
    onChange?.(nextItems);
  }

  function announce(message: string): void {
    liveMessage = message;
  }

  function ensureIndexVisible(index: number): void {
    if (!isWindowed || effectiveWindowSize <= 0) return;
    windowPageIndex = Math.floor(index / effectiveWindowSize);
  }

  function markLastMoved(id: string): void {
    lastMovedId = id;
    if (clearLastMovedTimeout) {
      clearTimeout(clearLastMovedTimeout);
    }
    clearLastMovedTimeout = setTimeout(() => {
      if (lastMovedId === id) {
        lastMovedId = null;
      }
      clearLastMovedTimeout = null;
    }, 1400);
  }

  function destinationIndex(fromIndex: number, toIndex: number, position: string, count: number): number {
    let dest = position === "after" ? toIndex + 1 : toIndex;
    if (fromIndex < dest) dest -= 1;
    if (dest < 0) return 0;
    if (dest >= count) return Math.max(0, count - 1);
    return dest;
  }

  function moveItem(fromIndex: number, toIndex: number): void {
    if (fromIndex === toIndex || fromIndex < 0 || toIndex < 0) return;
    if (fromIndex >= items.length || toIndex >= items.length) return;

    const { items: updated } = applyReorder(items, fromIndex, toIndex);
    const moved = updated[toIndex];
    items = updated;
    onReorder?.(updated);
    onChange?.(updated);
    ensureIndexVisible(toIndex);
    markLastMoved(moved.id);
    announce(`Moved ${moved.label ?? moved.id} to position ${toIndex + 1} of ${updated.length}.`);
    requestAnimationFrame(() => {
      document.querySelector<HTMLElement>(`[data-reorder-index="${toIndex}"]`)?.focus();
    });
  }

  function handleDragStart(session: DragSession): void {
    activeSourceId = session.subject.id;
    const current = items.find((entry) => entry.id === session.subject.id);
    announce(
      `Grabbed ${current?.label ?? session.subject.id}. Use arrow keys to move, Escape to cancel.`,
    );
  }

  function handleDrop(intent: DropIntent): DragDropCommitResult {
    const fromIndex = items.findIndex((entry) => entry.id === activeSourceId);
    const toIndex = items.findIndex((entry) => entry.id === intent.targetId);
    if (fromIndex < 0 || toIndex < 0) return { status: "rejected", reason: "missing-item" };
    const dest = destinationIndex(fromIndex, toIndex, String(intent.position), items.length);
    if (dest !== fromIndex) moveItem(fromIndex, dest);
    return { status: "committed" };
  }

  function handleDragEnd(outcome: DragTerminalOutcome): void {
    if (outcome.status === "cancelled" && outcome.reason === "escape") {
      announce("Cancelled keyboard move.");
    }
    activeSourceId = null;
  }

  function handleIdleKeydown(event: KeyboardEvent, index: number): void {
    if (isUnavailable || activeSourceId !== null) return;
    if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;

    const next = event.key === "ArrowDown" ? index + 1 : index - 1;
    event.preventDefault();
    if (next < 0 || next >= items.length) {
      announce("Reached list boundary.");
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
    items = updated;
    newItemText = "";
    onAdd?.(newItem);
    onChange?.(updated);
  }

  function removeItem(id: string): void {
    if (isUnavailable) return;
    const updated = items.filter((entry) => entry.id !== id);
    items = updated;
    onRemove?.(id);
    onChange?.(updated);
  }

  function handleAddKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter") {
      event.preventDefault();
      addItem();
    }
  }

  async function handleSubmit(): Promise<void> {
    if (!onSubmit || isUnavailable || !dirty) return;
    await onSubmit();
  }

  function handleCancel(): void {
    if (!onCancel || isUnavailable) return;
    onCancel();
  }

  function previousWindowPage(): void {
    windowPageIndex = Math.max(windowPageIndex - 1, 0);
  }

  function nextWindowPage(): void {
    windowPageIndex = Math.min(windowPageIndex + 1, windowPageCount - 1);
  }

  onDestroy(() => {
    if (clearLastMovedTimeout) {
      clearTimeout(clearLastMovedTimeout);
    }
  });
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div class="poodle-editable-list-session" data-disabled={isUnavailable} data-size={resolvedSize} data-density={resolvedDensity}>
    <div class="poodle-editable-list-session__sr" aria-live="polite" aria-atomic="true">{liveMessage}</div>

    {#if effectiveShowWorkflowChrome}
      <div class="poodle-editable-list-session__header">
        <Button variant="secondary" onClick={handleCancel} disabled={isUnavailable}>
          {cancelLabel}
        </Button>
        <Button variant="primary" onClick={handleSubmit} disabled={!dirty || isUnavailable}>
          {#if submitting}
            Saving...
          {:else}
            {submitLabel}
          {/if}
        </Button>
      </div>
    {/if}

    {#if errorMessage}
      <div class="poodle-editable-list-session__error" role="alert">
        {errorMessage}
      </div>
    {/if}

    {#if infoMessage}
      <div class="poodle-editable-list-session__info" role="status">
        {infoMessage}
      </div>
    {/if}

    {#if isLongList}
      <div class="poodle-editable-list-session__info" role="status">
        {effectiveLongListWarning}
      </div>
    {/if}

    {#if isWindowed}
      <div class="poodle-editable-list-session__window-nav">
        <Button variant="secondary" onClick={previousWindowPage} disabled={isUnavailable || windowPageIndex === 0}>
          Previous
        </Button>
        <span class="poodle-editable-list-session__window-label">
          Page {windowPageIndex + 1} of {windowPageCount} · Items {windowStart + 1}-{windowEnd} of {items.length}
        </span>
        <Button variant="secondary" onClick={nextWindowPage} disabled={isUnavailable || windowPageIndex >= windowPageCount - 1}>
          Next
        </Button>
      </div>
    {/if}

    <DragDropProvider describeAnnouncement={() => null}>
      <EditableListKeyboardTargets {items} {reorderable} {isUnavailable} onDrop={handleDrop} />
      <ul
        class="poodle-editable-list"
        class:poodle-editable-list--embedded-handle={embeddedHandle}
        role="listbox"
        aria-label={ariaLabel}
        data-disabled={isUnavailable}
        data-size={resolvedSize}
        data-density={resolvedDensity}
      >
        {#each visibleItems as reorderItem, localIndex (reorderItem.id)}
          {@const index = windowStart + localIndex}
          <EditableListRow
            item={reorderItem}
            {index}
            total={items.length}
            {reorderable}
            {embeddedHandle}
            {isUnavailable}
            {showRemove}
            lastMoved={lastMovedId === reorderItem.id}
            {resolvedSize}
            {resolvedDensity}
            itemSnippet={item}
            onDrop={handleDrop}
            onDragStart={handleDragStart}
            onDragEnd={handleDragEnd}
            onRemove={removeItem}
            onIdleKeydown={handleIdleKeydown}
          />
        {/each}
      </ul>
    </DragDropProvider>

    {#if canAdd}
      <div class="poodle-editable-list__add">
        <div class="poodle-editable-list__add-input">
          <TextInput
            bind:value={newItemText}
            placeholder={addPlaceholder}
            disabled={isUnavailable}
            size={resolvedSize}
            density={resolvedDensity}
            onKeyDown={handleAddKeydown}
          />
        </div>
        <div class="poodle-editable-list__add-btn">
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
    {/if}

    {#if editable && maxItems !== null}
      <span class="poodle-editable-list__count">
        {items.length}/{maxItems}
      </span>
    {/if}
  </div>
</UiPresentationProvider>


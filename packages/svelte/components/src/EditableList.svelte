<script lang="ts" generics="T extends { id: string; label?: string }">
  import { onDestroy, type Snippet } from "svelte";

  import { default as Button } from "./Button.svelte";
  import { default as IconButton } from "./IconButton.svelte";
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
  let draggingIndex = $state<number | null>(null);
  let dropTargetIndex = $state<number | null>(null);
  let grabbedIndex = $state<number | null>(null);
  let lastMovedId = $state<string | null>(null);
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

  function moveItem(fromIndex: number, toIndex: number): void {
    if (fromIndex === toIndex || fromIndex < 0 || toIndex < 0) return;
    if (fromIndex >= items.length || toIndex >= items.length) return;

    const updated = [...items];
    const [moved] = updated.splice(fromIndex, 1);
    updated.splice(toIndex, 0, moved);
    items = updated;
    onReorder?.(updated);
    onChange?.(updated);
    ensureIndexVisible(toIndex);
    markLastMoved(moved.id);
    announce(`Moved ${moved.label ?? moved.id} to position ${toIndex + 1} of ${updated.length}.`);
  }

  function handleDragStart(event: DragEvent, index: number): void {
    if (isUnavailable) return;
    draggingIndex = index;
    dropTargetIndex = index;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData("text/plain", String(index));
    }
  }

  function handleDragOver(event: DragEvent, index: number): void {
    if (isUnavailable || draggingIndex === null) return;
    event.preventDefault();
    dropTargetIndex = index;
  }

  function handleDrop(event: DragEvent, index: number): void {
    event.preventDefault();
    if (draggingIndex !== null && draggingIndex !== index) {
      moveItem(draggingIndex, index);
    }
    draggingIndex = null;
    dropTargetIndex = null;
  }

  function handleDragEnd(): void {
    draggingIndex = null;
    dropTargetIndex = null;
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    if (isUnavailable) return;

    const key = event.key;
    if (key === " " || key === "Enter") {
      event.preventDefault();
      if (grabbedIndex === index) {
        grabbedIndex = null;
        announce("Dropped item.");
      } else {
        grabbedIndex = index;
        announce(
          `Grabbed ${items[index]?.label ?? items[index]?.id ?? "item"}. Use arrow keys to move, Escape to cancel.`,
        );
      }
      return;
    }

    if (key === "Escape" && grabbedIndex !== null) {
      event.preventDefault();
      grabbedIndex = null;
      announce("Cancelled keyboard move.");
      return;
    }

    if (key !== "ArrowUp" && key !== "ArrowDown") return;

    event.preventDefault();
    const activeIndex = grabbedIndex ?? index;
    const targetIndex = key === "ArrowUp" ? activeIndex - 1 : activeIndex + 1;
    if (targetIndex < 0 || targetIndex >= items.length) {
      announce("Reached list boundary.");
      return;
    }

    moveItem(activeIndex, targetIndex);
    if (grabbedIndex !== null) {
      grabbedIndex = targetIndex;
    }

    requestAnimationFrame(() => {
      const element = document.querySelector<HTMLElement>(`[data-reorder-index="${targetIndex}"]`);
      element?.focus();
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
    grabbedIndex = null;
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
        <li
          class="poodle-editable-list__item"
          class:poodle-editable-list__item--dragging={draggingIndex === index}
          class:poodle-editable-list__item--drop-target={dropTargetIndex === index && draggingIndex !== index}
          class:poodle-editable-list__item--grabbed={grabbedIndex === index}
          class:poodle-editable-list__item--last-moved={lastMovedId === reorderItem.id}
          class:poodle-editable-list__item--embedded-handle={embeddedHandle}
          role="option"
          tabindex={isUnavailable ? -1 : 0}
          aria-selected="false"
          aria-label={`Reorder ${reorderItem.label ?? reorderItem.id}. Position ${index + 1} of ${items.length}. Press space to grab, then arrow keys to move.`}
          data-reorder-index={index}
          draggable={reorderable && !isUnavailable}
          ondragstart={(event) => handleDragStart(event, index)}
          ondragover={(event) => handleDragOver(event, index)}
          ondrop={(event) => handleDrop(event, index)}
          ondragend={handleDragEnd}
          onkeydown={(event) => handleKeydown(event, index)}
        >
          {#if reorderable && !embeddedHandle}
            <span class="poodle-editable-list__handle" aria-hidden="true">
              <svg viewBox="0 0 16 16" fill="currentColor">
                <circle cx="5" cy="4" r="1.25" />
                <circle cx="11" cy="4" r="1.25" />
                <circle cx="5" cy="8" r="1.25" />
                <circle cx="11" cy="8" r="1.25" />
                <circle cx="5" cy="12" r="1.25" />
                <circle cx="11" cy="12" r="1.25" />
              </svg>
            </span>
          {/if}
          <span class="poodle-editable-list__content">
            {#if item}
              {@render item(reorderItem)}
            {:else}
              {reorderItem.label ?? reorderItem.id}
            {/if}
          </span>
          {#if showRemove}
            <div class="poodle-editable-list__remove poodle-editable-list__remove--danger-on-hover">
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
          {/if}
        </li>
      {/each}
    </ul>

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

<style>
  .poodle-editable-list-session {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .poodle-editable-list-session__sr {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  .poodle-editable-list-session__header {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 0.5rem;
    padding-bottom: 0.5rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 76%, transparent);
  }

  .poodle-editable-list-session__error,
  .poodle-editable-list-session__info {
    padding: 0.75rem;
    border-radius: var(--poodle-radius-surface);
    font-size: 0.875rem;
  }

  .poodle-editable-list-session__error {
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-status-danger) 40%, transparent);
    background: color-mix(in srgb, var(--poodle-color-status-danger) 8%, var(--poodle-color-background-surface));
    color: var(--poodle-color-status-danger);
  }

  .poodle-editable-list-session__info {
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-accent-base) 22%, transparent);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 6%, var(--poodle-color-background-surface));
    color: var(--poodle-color-text-primary);
  }

  .poodle-editable-list-session__window-nav {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 0.5rem;
  }

  .poodle-editable-list-session__window-label {
    min-width: 13rem;
    text-align: center;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-editable-list {
    --poodle-editable-list-gap: 0.125rem;
    --poodle-editable-list-item-gap: 0.5rem;
    --poodle-editable-list-item-x: 0.625rem;
    --poodle-editable-list-item-y: 0.5rem;
    --poodle-editable-list-handle-size: 1rem;
    --poodle-editable-list-font-size: 0.8125rem;
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--poodle-editable-list-gap);
  }

  .poodle-editable-list--embedded-handle {
    --poodle-editable-list-gap: 0.5rem;
  }

  .poodle-editable-list[data-size="xs"] {
    --poodle-editable-list-handle-size: 0.875rem;
    --poodle-editable-list-item-x: 0.5rem;
    --poodle-editable-list-item-y: 0.375rem;
    --poodle-editable-list-font-size: 0.6875rem;
  }

  .poodle-editable-list[data-size="sm"] {
    --poodle-editable-list-handle-size: 1rem;
    --poodle-editable-list-item-y: 0.4375rem;
    --poodle-editable-list-font-size: 0.75rem;
  }

  .poodle-editable-list[data-size="lg"] {
    --poodle-editable-list-handle-size: 1.125rem;
    --poodle-editable-list-item-x: 0.75rem;
    --poodle-editable-list-item-y: 0.5625rem;
    --poodle-editable-list-font-size: 0.875rem;
  }

  .poodle-editable-list[data-size="xl"] {
    --poodle-editable-list-handle-size: 1.25rem;
    --poodle-editable-list-item-x: 0.875rem;
    --poodle-editable-list-item-y: 0.625rem;
    --poodle-editable-list-font-size: 0.9375rem;
  }

  .poodle-editable-list[data-density="compact"] {
    --poodle-editable-list-gap: 0.0625rem;
    --poodle-editable-list-item-gap: 0.375rem;
  }

  .poodle-editable-list--embedded-handle[data-density="compact"] {
    --poodle-editable-list-gap: 0.5rem;
  }

  .poodle-editable-list[data-density="comfortable"] {
    --poodle-editable-list-gap: 0.1875rem;
    --poodle-editable-list-item-gap: 0.625rem;
  }

  .poodle-editable-list--embedded-handle[data-density="comfortable"] {
    --poodle-editable-list-gap: 0.625rem;
  }

  .poodle-editable-list[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-editable-list__item {
    display: flex;
    align-items: center;
    gap: var(--poodle-editable-list-item-gap);
    padding: var(--poodle-editable-list-item-y) var(--poodle-editable-list-item-x);
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    font-size: var(--poodle-editable-list-font-size);
    color: var(--poodle-color-text-primary);
    outline: none;
    transition:
      background-color 120ms ease,
      border-color 120ms ease,
      box-shadow 120ms ease,
      opacity 120ms ease;
  }

  .poodle-editable-list__item:hover {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 82%, transparent);
  }

  .poodle-editable-list__item:focus-visible {
    border-color: var(--poodle-color-accent-focusRing);
    box-shadow: 0 0 0 var(--poodle-border-width-focus) var(--poodle-color-accent-focusRing);
  }

  .poodle-editable-list__item--dragging {
    opacity: 0.4;
  }

  .poodle-editable-list__item--drop-target,
  .poodle-editable-list__item--grabbed,
  .poodle-editable-list__item--last-moved {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 56%, transparent);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent);
  }

  .poodle-editable-list__item--embedded-handle {
    padding: 0;
  }

  .poodle-editable-list__handle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-editable-list-handle-size);
    height: var(--poodle-editable-list-handle-size);
    color: var(--poodle-color-text-secondary);
    flex-shrink: 0;
    cursor: grab;
  }

  .poodle-editable-list__handle svg {
    width: 100%;
    height: 100%;
  }

  .poodle-editable-list__content {
    flex: 1 1 auto;
    min-width: 0;
  }

  .poodle-editable-list__remove {
    flex-shrink: 0;
  }

  .poodle-editable-list__remove--danger-on-hover :global(.poodle-icon-button) {
    color: var(--poodle-color-text-secondary);
  }

  .poodle-editable-list__remove--danger-on-hover :global(.poodle-icon-button:hover:not(:disabled)),
  .poodle-editable-list__remove--danger-on-hover :global(.poodle-icon-button:focus-visible:not(:disabled)) {
    color: var(--poodle-color-status-danger);
    border-color: color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default));
    background: color-mix(in srgb, var(--poodle-color-status-danger) 10%, transparent);
  }

  .poodle-editable-list__add {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .poodle-editable-list__add-input {
    flex: 1 1 auto;
    min-width: 0;
  }

  .poodle-editable-list__add-btn {
    flex-shrink: 0;
  }

  .poodle-editable-list__count {
    align-self: flex-end;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
  }
</style>

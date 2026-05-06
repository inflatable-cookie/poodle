<script lang="ts" generics="T extends { id: string; label?: string }">
  import { createEventDispatcher, onDestroy } from "svelte";
  import type { Snippet } from "svelte";
  import Button from "./Button.svelte";
  import UiPresentationProvider from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  export let items: T[] = [];
  export let ariaLabel = "Editable list";
  export let disabled = false;
  export let reorderable = true;
  /** Show add-item input and remove buttons. */
  export let editable = false;
  export let addLabel = "Add item";
  export let addPlaceholder = "New item";
  export let maxItems: number | null = null;
  export let removable = false;
  export let embeddedHandle = false;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let dirty = false;
  export let submitting = false;
  export let errorMessage: string | null = null;
  export let infoMessage: string | null = null;
  export let longListThreshold: number | null = 50;
  export let longListWarningText: string | null = null;
  export let windowSize: number | null = null;
  export let submitLabel = "Save Order";
  export let cancelLabel = "Cancel";
  export let onsubmit: (() => void | Promise<void>) | null = null;
  export let oncancel: (() => void) | null = null;
  export let item: Snippet<[T]> | undefined = undefined;

  const dispatch = createEventDispatcher<{
    reorder: { items: T[] };
    add: { item: T };
    remove: { id: string };
    change: { items: T[] };
    submit: void;
    cancel: void;
  }>();

  let newItemText = "";
  let draggingIndex: number | null = null;
  let dropTargetIndex: number | null = null;
  let grabbedIndex: number | null = null;
  let lastMovedId: string | null = null;
  let windowPageIndex = 0;
  let liveMessage = "";
  let clearLastMovedTimeout: ReturnType<typeof setTimeout> | null = null;
  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isUnavailable = disabled || submitting;
  $: canAdd = editable && !isUnavailable && (maxItems === null || items.length < maxItems);
  $: showRemove = editable || removable;
  $: showWorkflowChrome = onsubmit !== null || oncancel !== null;
  $: isLongList = longListThreshold !== null && longListThreshold > 0 && items.length > longListThreshold;
  $: effectiveLongListWarning =
    longListWarningText ??
    `This list has ${items.length} items. Reordering large lists can be error-prone; consider chunked moves and save often.`;
  $: effectiveWindowSize = windowSize !== null && windowSize > 0 ? windowSize : items.length;
  $: isWindowed = windowSize !== null && windowSize > 0 && items.length > effectiveWindowSize;
  $: windowPageCount = isWindowed ? Math.ceil(items.length / effectiveWindowSize) : 1;
  $: if (!isWindowed) {
    windowPageIndex = 0;
  } else if (windowPageIndex >= windowPageCount) {
    windowPageIndex = Math.max(windowPageCount - 1, 0);
  }
  $: windowStart = isWindowed ? windowPageIndex * effectiveWindowSize : 0;
  $: windowEnd = Math.min(windowStart + effectiveWindowSize, items.length);
  $: visibleItems = items.slice(windowStart, windowEnd);

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
    dispatch("reorder", { items: updated });
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
        announce(`Grabbed ${items[index]?.label ?? items[index]?.id ?? "item"}. Use arrow keys to move, Escape to cancel.`);
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
      const el = document.querySelector<HTMLElement>(`[data-reorder-index="${targetIndex}"]`);
      el?.focus();
    });
  }

  function addItem(): void {
    const label = newItemText.trim();
    if (!label || !canAdd) return;
    const newItem = {
      id: `item-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
      label,
    } as T;
    items = [...items, newItem];
    newItemText = "";
    dispatch("add", { item: newItem });
    dispatch("change", { items });
  }

  function removeItem(id: string): void {
    if (isUnavailable) return;
    items = items.filter((i) => i.id !== id);
    dispatch("remove", { id });
    dispatch("change", { items });
  }

  function handleAddKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter") {
      event.preventDefault();
      addItem();
    }
  }

  async function handleSubmit(): Promise<void> {
    if (!onsubmit || isUnavailable || !dirty) return;
    dispatch("submit");
    await onsubmit();
  }

  function handleCancel(): void {
    if (!oncancel || isUnavailable) return;
    dispatch("cancel");
    grabbedIndex = null;
    oncancel();
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

    {#if showWorkflowChrome}
      <div class="poodle-editable-list-session__header">
        <Button variant="secondary" on:click={handleCancel} disabled={isUnavailable}>
          {cancelLabel}
        </Button>
        <Button variant="primary" on:click={handleSubmit} disabled={!dirty || isUnavailable}>
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
        <Button variant="secondary" on:click={previousWindowPage} disabled={isUnavailable || windowPageIndex === 0}>
          Previous
        </Button>
        <span class="poodle-editable-list-session__window-label">
          Page {windowPageIndex + 1} of {windowPageCount} · Items {windowStart + 1}-{windowEnd} of {items.length}
        </span>
        <Button variant="secondary" on:click={nextWindowPage} disabled={isUnavailable || windowPageIndex >= windowPageCount - 1}>
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
          on:dragstart={(e) => handleDragStart(e, index)}
          on:dragover={(e) => handleDragOver(e, index)}
          on:drop={(e) => handleDrop(e, index)}
          on:dragend={handleDragEnd}
          on:keydown={(e) => handleKeydown(e, index)}
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
            <button
              type="button"
              class="poodle-editable-list__remove"
              disabled={isUnavailable}
              aria-label={`Remove ${reorderItem.label ?? reorderItem.id}`}
              on:click|stopPropagation={() => removeItem(reorderItem.id)}
            >
              <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
                <path d="M4 4l8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
              </svg>
            </button>
          {/if}
        </li>
      {/each}
    </ul>

    {#if canAdd}
      <div class="poodle-editable-list__add">
        <input
          type="text"
          class="poodle-editable-list__add-input"
          bind:value={newItemText}
          placeholder={addPlaceholder}
          disabled={isUnavailable}
          on:keydown={handleAddKeydown}
        />
        <button
          type="button"
          class="poodle-editable-list__add-btn"
          disabled={!newItemText.trim() || !canAdd}
          on:click={addItem}
        >
          {addLabel}
        </button>
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
    background: var(--poodle-color-background-surface);
    cursor: grab;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-editable-list__item:hover {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 52%, var(--poodle-color-background-surface));
  }

  .poodle-editable-list__item--embedded-handle {
    padding: 0;
    border-color: transparent;
    background: transparent;
  }

  .poodle-editable-list__item--embedded-handle:hover {
    background: transparent;
  }

  .poodle-editable-list__item:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: -0.0625rem;
  }

  .poodle-editable-list__item--dragging {
    opacity: 0.4;
  }

  .poodle-editable-list__item--drop-target,
  .poodle-editable-list__item--grabbed {
    border-color: var(--poodle-color-accent-base);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 8%, var(--poodle-color-background-surface));
  }

  .poodle-editable-list__item--last-moved {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 52%, transparent);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 10%, var(--poodle-color-background-surface));
    box-shadow: 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 32%, transparent);
    animation: poodle-editable-list-last-moved 1.4s ease-out;
  }

  .poodle-editable-list__item--embedded-handle.poodle-editable-list__item--drop-target,
  .poodle-editable-list__item--embedded-handle.poodle-editable-list__item--grabbed {
    background: transparent;
  }

  .poodle-editable-list__item--embedded-handle.poodle-editable-list__item--last-moved {
    border-color: transparent;
    background: transparent;
    box-shadow: none;
  }

  .poodle-editable-list__item--embedded-handle.poodle-editable-list__item--last-moved :global(.poodle-list-card) {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 52%, transparent);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 10%, var(--poodle-color-background-surface));
    box-shadow: 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 32%, transparent);
    animation: poodle-editable-list-last-moved 1.4s ease-out;
  }

  .poodle-editable-list__handle {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: var(--poodle-editable-list-handle-size);
    height: var(--poodle-editable-list-handle-size);
    color: var(--poodle-color-text-secondary);
    cursor: grab;
  }

  .poodle-editable-list__handle svg {
    width: 100%;
    height: 100%;
  }

  .poodle-editable-list__content {
    flex: 1;
    min-width: 0;
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-editable-list-font-size);
    color: var(--poodle-color-text-primary);
  }

  .poodle-editable-list__item--embedded-handle .poodle-editable-list__content {
    display: block;
    width: 100%;
  }

  .poodle-editable-list__remove {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: var(--poodle-editable-list-handle-size);
    height: var(--poodle-editable-list-handle-size);
    padding: 0;
    border: 0;
    border-radius: 0.25rem;
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
  }

  .poodle-editable-list__remove:hover:not(:disabled) {
    color: var(--poodle-color-status-danger);
  }

  .poodle-editable-list__remove svg {
    width: 0.75rem;
    height: 0.75rem;
  }

  @keyframes poodle-editable-list-last-moved {
    0% {
      background: color-mix(in srgb, var(--poodle-color-accent-base) 18%, var(--poodle-color-background-surface));
      box-shadow: 0 0 0 0.125rem color-mix(in srgb, var(--poodle-color-accent-base) 38%, transparent);
    }

    100% {
      background: color-mix(in srgb, var(--poodle-color-accent-base) 10%, var(--poodle-color-background-surface));
      box-shadow: 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 32%, transparent);
    }
  }

  .poodle-editable-list__add {
    display: flex;
    gap: 0.375rem;
  }

  .poodle-editable-list__add-input {
    flex: 1;
    min-width: 0;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-editable-list-font-size);
    outline: none;
  }

  .poodle-editable-list__add-input:focus {
    border-color: var(--poodle-color-accent-focusRing);
    box-shadow: 0 0 0 var(--poodle-border-width-focus)
      color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent);
  }

  .poodle-editable-list__add-input::placeholder {
    color: var(--poodle-color-text-secondary);
  }

  .poodle-editable-list__add-btn {
    display: inline-flex;
    align-items: center;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
  }

  .poodle-editable-list__add-btn:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 84%, var(--poodle-color-background-elevated));
  }

  .poodle-editable-list__add-btn:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-editable-list__count {
    font-size: var(--poodle-typography-label-size);
    color: var(--poodle-color-text-secondary);
    font-variant-numeric: tabular-nums;
    align-self: flex-end;
  }
</style>

<script lang="ts" generics="T extends { id: string; label?: string }">
  import { createEventDispatcher } from "svelte";
  import type { Snippet } from "svelte";
  import {
    Button,
    UiPresentationProvider,
    getUiPresentation,
    resolveSemanticControlSize,
  } from "@poodle/svelte-primitives";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "@poodle/svelte-primitives";

  export let items: T[] = [];
  export let ariaLabel = "Reorderable list";
  export let disabled = false;
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
    submit: void;
    cancel: void;
  }>();

  let draggingIndex: number | null = null;
  let dropTargetIndex: number | null = null;
  let grabbedIndex: number | null = null;
  let windowPageIndex = 0;
  let liveMessage = "";
  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isUnavailable = disabled || submitting;
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

  function moveItem(fromIndex: number, toIndex: number): void {
    if (fromIndex === toIndex || fromIndex < 0 || toIndex < 0) return;
    if (fromIndex >= items.length || toIndex >= items.length) return;

    const updated = [...items];
    const [moved] = updated.splice(fromIndex, 1);
    updated.splice(toIndex, 0, moved);
    items = updated;
    dispatch("reorder", { items: updated });
    ensureIndexVisible(toIndex);
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
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div class="reorderable-list-session" data-disabled={isUnavailable} data-size={resolvedSize} data-density={resolvedDensity}>
    <div class="reorderable-list-session__sr" aria-live="polite" aria-atomic="true">{liveMessage}</div>

    {#if showWorkflowChrome}
      <div class="reorderable-list-session__header">
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
      <div class="reorderable-list-session__error" role="alert">
        {errorMessage}
      </div>
    {/if}

    {#if infoMessage}
      <div class="reorderable-list-session__info" role="status">
        {infoMessage}
      </div>
    {/if}

    {#if isLongList}
      <div class="reorderable-list-session__info" role="status">
        {effectiveLongListWarning}
      </div>
    {/if}

    {#if isWindowed}
      <div class="reorderable-list-session__window-nav">
        <Button variant="secondary" on:click={previousWindowPage} disabled={isUnavailable || windowPageIndex === 0}>
          Previous
        </Button>
        <span class="reorderable-list-session__window-label">
          Page {windowPageIndex + 1} of {windowPageCount} · Items {windowStart + 1}-{windowEnd} of {items.length}
        </span>
        <Button variant="secondary" on:click={nextWindowPage} disabled={isUnavailable || windowPageIndex >= windowPageCount - 1}>
          Next
        </Button>
      </div>
    {/if}

    <ul
      class="reorderable-list"
      role="listbox"
      aria-label={ariaLabel}
      data-disabled={isUnavailable}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {#each visibleItems as reorderItem, localIndex (reorderItem.id)}
        {@const index = windowStart + localIndex}
        <li
          class="reorderable-list__item"
          class:reorderable-list__item--dragging={draggingIndex === index}
          class:reorderable-list__item--drop-target={dropTargetIndex === index && draggingIndex !== index}
          class:reorderable-list__item--grabbed={grabbedIndex === index}
          role="option"
          tabindex={isUnavailable ? -1 : 0}
          aria-selected="false"
          aria-label={`Reorder ${reorderItem.label ?? reorderItem.id}. Position ${index + 1} of ${items.length}. Press space to grab, then arrow keys to move.`}
          data-reorder-index={index}
          draggable={!isUnavailable}
          on:dragstart={(e) => handleDragStart(e, index)}
          on:dragover={(e) => handleDragOver(e, index)}
          on:drop={(e) => handleDrop(e, index)}
          on:dragend={handleDragEnd}
          on:keydown={(e) => handleKeydown(e, index)}
        >
          <span class="reorderable-list__handle" aria-hidden="true">
            <svg viewBox="0 0 16 16" fill="currentColor">
              <circle cx="5" cy="4" r="1.25" />
              <circle cx="11" cy="4" r="1.25" />
              <circle cx="5" cy="8" r="1.25" />
              <circle cx="11" cy="8" r="1.25" />
              <circle cx="5" cy="12" r="1.25" />
              <circle cx="11" cy="12" r="1.25" />
            </svg>
          </span>
          <span class="reorderable-list__content">
            {#if item}
              {@render item(reorderItem)}
            {:else}
              {reorderItem.label ?? reorderItem.id}
            {/if}
          </span>
        </li>
      {/each}
    </ul>
  </div>
</UiPresentationProvider>

<style>
  .reorderable-list-session {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .reorderable-list-session__sr {
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

  .reorderable-list-session__header {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 0.5rem;
    padding-bottom: 0.5rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 76%, transparent);
  }

  .reorderable-list-session__error,
  .reorderable-list-session__info {
    padding: 0.75rem;
    border-radius: var(--poodle-radius-surface);
    font-size: 0.875rem;
  }

  .reorderable-list-session__error {
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-status-danger) 40%, transparent);
    background: color-mix(in srgb, var(--poodle-color-status-danger) 8%, var(--poodle-color-background-surface));
    color: var(--poodle-color-status-danger);
  }

  .reorderable-list-session__info {
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-accent-base) 22%, transparent);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 6%, var(--poodle-color-background-surface));
    color: var(--poodle-color-text-primary);
  }

  .reorderable-list-session__window-nav {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 0.5rem;
  }

  .reorderable-list-session__window-label {
    min-width: 13rem;
    text-align: center;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }

  .reorderable-list {
    --poodle-reorderable-list-gap: 0.125rem;
    --poodle-reorderable-list-item-gap: 0.5rem;
    --poodle-reorderable-list-item-x: 0.625rem;
    --poodle-reorderable-list-item-y: 0.5rem;
    --poodle-reorderable-list-handle-size: 1rem;
    --poodle-reorderable-list-font-size: 0.8125rem;
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--poodle-reorderable-list-gap);
  }

  .reorderable-list[data-size="xs"] {
    --poodle-reorderable-list-handle-size: 0.875rem;
    --poodle-reorderable-list-item-x: 0.5rem;
    --poodle-reorderable-list-item-y: 0.375rem;
    --poodle-reorderable-list-font-size: 0.6875rem;
  }

  .reorderable-list[data-size="sm"] {
    --poodle-reorderable-list-handle-size: 1rem;
    --poodle-reorderable-list-item-y: 0.4375rem;
    --poodle-reorderable-list-font-size: 0.75rem;
  }

  .reorderable-list[data-size="lg"] {
    --poodle-reorderable-list-handle-size: 1.125rem;
    --poodle-reorderable-list-item-x: 0.75rem;
    --poodle-reorderable-list-item-y: 0.5625rem;
    --poodle-reorderable-list-font-size: 0.875rem;
  }

  .reorderable-list[data-size="xl"] {
    --poodle-reorderable-list-handle-size: 1.25rem;
    --poodle-reorderable-list-item-x: 0.875rem;
    --poodle-reorderable-list-item-y: 0.625rem;
    --poodle-reorderable-list-font-size: 0.9375rem;
  }

  .reorderable-list[data-density="compact"] {
    --poodle-reorderable-list-gap: 0.0625rem;
    --poodle-reorderable-list-item-gap: 0.375rem;
    --poodle-reorderable-list-item-y: 0.375rem;
  }

  .reorderable-list[data-density="comfortable"] {
    --poodle-reorderable-list-gap: 0.1875rem;
    --poodle-reorderable-list-item-gap: 0.625rem;
    --poodle-reorderable-list-item-y: 0.625rem;
  }

  .reorderable-list[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .reorderable-list__item {
    display: flex;
    align-items: center;
    gap: var(--poodle-reorderable-list-item-gap);
    padding: var(--poodle-reorderable-list-item-y) var(--poodle-reorderable-list-item-x);
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    cursor: grab;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .reorderable-list__item:hover {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 52%, var(--poodle-color-background-surface));
  }

  .reorderable-list__item:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: -0.0625rem;
  }

  .reorderable-list__item--dragging {
    opacity: 0.4;
  }

  .reorderable-list__item--drop-target,
  .reorderable-list__item--grabbed {
    border-color: var(--poodle-color-accent-base);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 8%, var(--poodle-color-background-surface));
  }

  .reorderable-list__handle {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: var(--poodle-reorderable-list-handle-size);
    height: var(--poodle-reorderable-list-handle-size);
    color: var(--poodle-color-text-secondary);
    cursor: grab;
  }

  .reorderable-list__handle svg {
    width: 100%;
    height: 100%;
  }

  .reorderable-list__content {
    flex: 1;
    min-width: 0;
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-reorderable-list-font-size);
    color: var(--poodle-color-text-primary);
  }
</style>

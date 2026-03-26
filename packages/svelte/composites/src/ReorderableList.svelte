<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import {
    UiPresentationProvider,
    getUiPresentation,
    resolveSemanticControlSize,
  } from "@poodle/svelte-primitives";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "@poodle/svelte-primitives";

  import type { ReorderableItem } from "./types";

  export let items: ReorderableItem[] = [];
  export let ariaLabel = "Reorderable list";
  export let disabled = false;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    reorder: { items: ReorderableItem[] };
  }>();

  let draggingIndex: number | null = null;
  let dropTargetIndex: number | null = null;
  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";

  function moveItem(fromIndex: number, toIndex: number): void {
    if (fromIndex === toIndex || fromIndex < 0 || toIndex < 0) return;
    if (fromIndex >= items.length || toIndex >= items.length) return;

    const updated = [...items];
    const [moved] = updated.splice(fromIndex, 1);
    updated.splice(toIndex, 0, moved);
    items = updated;
    dispatch("reorder", { items: updated });
  }

  function handleDragStart(event: DragEvent, index: number): void {
    if (disabled) return;
    draggingIndex = index;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData("text/plain", String(index));
    }
  }

  function handleDragOver(event: DragEvent, index: number): void {
    if (disabled || draggingIndex === null) return;
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
    if (disabled) return;
    if (event.altKey && event.key === "ArrowUp" && index > 0) {
      event.preventDefault();
      moveItem(index, index - 1);
      requestAnimationFrame(() => {
        const el = document.querySelector<HTMLElement>(
          `[data-reorder-index="${index - 1}"]`
        );
        el?.focus();
      });
    }
    if (event.altKey && event.key === "ArrowDown" && index < items.length - 1) {
      event.preventDefault();
      moveItem(index, index + 1);
      requestAnimationFrame(() => {
        const el = document.querySelector<HTMLElement>(
          `[data-reorder-index="${index + 1}"]`
        );
        el?.focus();
      });
    }
  }
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <ul
    class="reorderable-list"
    role="listbox"
    aria-label={ariaLabel}
    data-disabled={disabled}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    {#each items as item, index (item.id)}
      <li
        class="reorderable-list__item"
        class:reorderable-list__item--dragging={draggingIndex === index}
        class:reorderable-list__item--drop-target={dropTargetIndex === index && draggingIndex !== index}
        role="option"
        tabindex={disabled ? -1 : 0}
        aria-selected="false"
        data-reorder-index={index}
        draggable={!disabled}
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
          <slot name="item" {item} {index}>
            {item.label}
          </slot>
        </span>
      </li>
    {/each}
  </ul>
</UiPresentationProvider>

<style>
  .reorderable-list {
    --poodle-reorderable-list-gap: 0.125rem;
    --poodle-reorderable-list-item-gap: 0.5rem;
    --poodle-reorderable-list-item-x: 0.625rem;
    --poodle-reorderable-list-item-y: 0.5rem;
    --poodle-reorderable-list-handle-size: 1rem;
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
  }

  .reorderable-list[data-size="sm"] {
    --poodle-reorderable-list-handle-size: 1rem;
  }

  .reorderable-list[data-size="lg"] {
    --poodle-reorderable-list-handle-size: 1.125rem;
    --poodle-reorderable-list-item-x: 0.75rem;
  }

  .reorderable-list[data-size="xl"] {
    --poodle-reorderable-list-handle-size: 1.25rem;
    --poodle-reorderable-list-item-x: 0.875rem;
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
    pointer-events: none;
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

  .reorderable-list__item--drop-target {
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
    font-size: var(--poodle-typography-body-size);
    color: var(--poodle-color-text-primary);
  }
</style>

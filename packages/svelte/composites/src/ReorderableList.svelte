<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { ReorderableItem } from "./types";

  export let items: ReorderableItem[] = [];
  export let ariaLabel = "Reorderable list";
  export let isDisabled = false;

  const dispatch = createEventDispatcher<{
    reorder: { items: ReorderableItem[] };
  }>();

  let draggingIndex: number | null = null;
  let dropTargetIndex: number | null = null;

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
    if (isDisabled) return;
    draggingIndex = index;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData("text/plain", String(index));
    }
  }

  function handleDragOver(event: DragEvent, index: number): void {
    if (isDisabled || draggingIndex === null) return;
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
    if (isDisabled) return;
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

<ul class="reorderable-list" role="listbox" aria-label={ariaLabel} data-disabled={isDisabled}>
  {#each items as item, index (item.id)}
    <li
      class="reorderable-list__item"
      class:reorderable-list__item--dragging={draggingIndex === index}
      class:reorderable-list__item--drop-target={dropTargetIndex === index && draggingIndex !== index}
      role="option"
      tabindex={isDisabled ? -1 : 0}
      aria-selected="false"
      data-reorder-index={index}
      draggable={!isDisabled}
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

<style>
  .reorderable-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .reorderable-list[data-disabled="true"] {
    opacity: var(--flint-state-opacity-disabled);
    pointer-events: none;
  }

  .reorderable-list__item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.625rem;
    border: 0.0625rem solid transparent;
    border-radius: var(--flint-radius-control);
    background: var(--flint-color-background-surface);
    cursor: grab;
    transition:
      background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      border-color var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .reorderable-list__item:hover {
    background: color-mix(in srgb, var(--flint-color-background-elevated) 52%, var(--flint-color-background-surface));
  }

  .reorderable-list__item:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: -0.0625rem;
  }

  .reorderable-list__item--dragging {
    opacity: 0.4;
  }

  .reorderable-list__item--drop-target {
    border-color: var(--flint-color-accent-base);
    background: color-mix(in srgb, var(--flint-color-accent-base) 8%, var(--flint-color-background-surface));
  }

  .reorderable-list__handle {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 1rem;
    height: 1rem;
    color: var(--flint-color-text-secondary);
    cursor: grab;
  }

  .reorderable-list__handle svg {
    width: 100%;
    height: 100%;
  }

  .reorderable-list__content {
    flex: 1;
    min-width: 0;
    font-family: var(--flint-typography-body-family);
    font-size: var(--flint-typography-body-size);
    color: var(--flint-color-text-primary);
  }
</style>

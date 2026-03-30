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

  import ReorderableList from "./ReorderableList.svelte";

  import type { ReorderableItem } from "./types";

  export let items: ReorderableItem[] = [];
  export let addLabel = "Add item";
  export let placeholder = "New item";
  export let maxItems: number | null = null;
  export let disabled = false;
  export let ariaLabel = "List";
  export let reorderable = true;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    change: { items: ReorderableItem[] };
  }>();

  let newItemText = "";
  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;

  $: canAdd = !disabled && (maxItems === null || items.length < maxItems);

  function addItem(): void {
    const label = newItemText.trim();
    if (!label || !canAdd) return;

    const item: ReorderableItem = {
      id: `item-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
      label,
    };

    items = [...items, item];
    newItemText = "";
    dispatch("change", { items });
  }

  function removeItem(id: string): void {
    if (disabled) return;
    items = items.filter((i) => i.id !== id);
    dispatch("change", { items });
  }

  function handleReorder(event: CustomEvent<{ items: ReorderableItem[] }>): void {
    items = event.detail.items;
    dispatch("change", { items });
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter") {
      event.preventDefault();
      addItem();
    }
  }
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div
    class="autonomous-list"
    data-disabled={disabled}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    {#if items.length > 0}
      {#if reorderable}
        <ReorderableList
          {items}
          {ariaLabel}
          {disabled}
          size={resolvedSize}
          density={resolvedDensity}
          on:reorder={handleReorder}
        >
          {#snippet item(item)}
            <span class="autonomous-list__item-row">
              <span class="autonomous-list__item-text">{item.label}</span>
              <button
                type="button"
                class="autonomous-list__remove"
                disabled={disabled}
                aria-label={`Remove ${item.label}`}
                on:click|stopPropagation={() => removeItem(item.id)}
              >
                <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
                  <path d="M4 4l8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
                </svg>
              </button>
            </span>
          {/snippet}
        </ReorderableList>
      {:else}
        <ul class="autonomous-list__static" role="list" aria-label={ariaLabel}>
          {#each items as item (item.id)}
            <li class="autonomous-list__static-item">
              <span class="autonomous-list__item-text">{item.label}</span>
              <button
                type="button"
                class="autonomous-list__remove"
                disabled={disabled}
                aria-label={`Remove ${item.label}`}
                on:click={() => removeItem(item.id)}
              >
                <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
                  <path d="M4 4l8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
                </svg>
              </button>
            </li>
          {/each}
        </ul>
      {/if}
    {/if}

    {#if canAdd}
      <div class="autonomous-list__add">
        <input
          type="text"
          class="autonomous-list__input"
          bind:value={newItemText}
          {placeholder}
          disabled={disabled}
          on:keydown={handleKeydown}
        />
        <button
          type="button"
          class="autonomous-list__add-btn"
          disabled={!newItemText.trim() || !canAdd}
          on:click={addItem}
        >
          {addLabel}
        </button>
      </div>
    {/if}

    {#if maxItems !== null}
      <span class="autonomous-list__count">
        {items.length}/{maxItems}
      </span>
    {/if}
  </div>
</UiPresentationProvider>

<style>
  .autonomous-list {
    --poodle-autonomous-list-gap: 0.5rem;
    --poodle-autonomous-list-static-gap: 0.125rem;
    --poodle-autonomous-list-item-y: 0.5rem;
    --poodle-autonomous-list-item-x: 0.625rem;
    --poodle-autonomous-list-remove-size: 1.25rem;
    --poodle-autonomous-list-add-gap: 0.375rem;
    --poodle-autonomous-list-add-x: 0.75rem;
    display: flex;
    flex-direction: column;
    gap: var(--poodle-autonomous-list-gap);
  }

  .autonomous-list[data-size="xs"] {
    --poodle-autonomous-list-remove-size: 1rem;
    --poodle-autonomous-list-item-x: 0.5rem;
    --poodle-autonomous-list-add-x: 0.625rem;
  }

  .autonomous-list[data-size="sm"] {
    --poodle-autonomous-list-remove-size: 1.125rem;
  }

  .autonomous-list[data-size="lg"] {
    --poodle-autonomous-list-remove-size: 1.375rem;
    --poodle-autonomous-list-item-x: 0.75rem;
    --poodle-autonomous-list-add-x: 0.875rem;
  }

  .autonomous-list[data-size="xl"] {
    --poodle-autonomous-list-remove-size: 1.5rem;
    --poodle-autonomous-list-item-x: 0.875rem;
    --poodle-autonomous-list-add-x: 1rem;
  }

  .autonomous-list[data-density="compact"] {
    --poodle-autonomous-list-gap: 0.375rem;
    --poodle-autonomous-list-static-gap: 0.0625rem;
    --poodle-autonomous-list-item-y: 0.375rem;
    --poodle-autonomous-list-add-gap: 0.25rem;
  }

  .autonomous-list[data-density="comfortable"] {
    --poodle-autonomous-list-gap: 0.625rem;
    --poodle-autonomous-list-static-gap: 0.1875rem;
    --poodle-autonomous-list-item-y: 0.625rem;
    --poodle-autonomous-list-add-gap: 0.5rem;
  }

  .autonomous-list[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .autonomous-list__static {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--poodle-autonomous-list-static-gap);
  }

  .autonomous-list__static-item {
    display: flex;
    align-items: center;
    padding: var(--poodle-autonomous-list-item-y) var(--poodle-autonomous-list-item-x);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
  }

  .autonomous-list__item-row {
    display: flex;
    align-items: center;
    flex: 1;
    min-width: 0;
  }

  .autonomous-list__item-text {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .autonomous-list__remove {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: var(--poodle-autonomous-list-remove-size);
    height: var(--poodle-autonomous-list-remove-size);
    padding: 0;
    border: 0;
    border-radius: 0.25rem;
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    transition: color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .autonomous-list__remove:hover:not(:disabled) {
    color: var(--poodle-color-status-danger);
  }

  .autonomous-list__remove svg {
    width: 0.75rem;
    height: 0.75rem;
  }

  .autonomous-list__add {
    display: flex;
    gap: var(--poodle-autonomous-list-add-gap);
  }

  .autonomous-list__input {
    flex: 1;
    min-width: 0;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    outline: none;
  }

  .autonomous-list__input:focus {
    border-color: var(--poodle-color-accent-focusRing);
    box-shadow: 0 0 0 var(--poodle-border-width-focus)
      color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent);
  }

  .autonomous-list__input::placeholder {
    color: var(--poodle-color-text-secondary);
  }

  .autonomous-list__add-btn {
    display: inline-flex;
    align-items: center;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-autonomous-list-add-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .autonomous-list__add-btn:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 84%, var(--poodle-color-background-elevated));
  }

  .autonomous-list__add-btn:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .autonomous-list__count {
    font-size: var(--poodle-typography-label-size);
    color: var(--poodle-color-text-secondary);
    font-variant-numeric: tabular-nums;
    align-self: flex-end;
  }
</style>

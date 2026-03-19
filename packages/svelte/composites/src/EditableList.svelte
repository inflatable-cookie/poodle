<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import ReorderableList from "./ReorderableList.svelte";

  import type { ReorderableItem } from "./types";

  export let items: ReorderableItem[] = [];
  export let addLabel = "Add item";
  export let placeholder = "New item";
  export let maxItems: number | null = null;
  export let isDisabled = false;
  export let ariaLabel = "List";
  export let isReorderable = true;

  const dispatch = createEventDispatcher<{
    change: { items: ReorderableItem[] };
  }>();

  let newItemText = "";

  $: canAdd = !isDisabled && (maxItems === null || items.length < maxItems);

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
    if (isDisabled) return;
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

<div class="autonomous-list" data-disabled={isDisabled}>
  {#if items.length > 0}
    {#if isReorderable}
      <ReorderableList {items} {ariaLabel} {isDisabled} on:reorder={handleReorder}>
        <svelte:fragment slot="item" let:item let:index>
          <span class="autonomous-list__item-row">
            <span class="autonomous-list__item-text">{item.label}</span>
            <button
              type="button"
              class="autonomous-list__remove"
              disabled={isDisabled}
              aria-label="Remove {item.label}"
              on:click|stopPropagation={() => removeItem(item.id)}
            >
              <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
                <path d="M4 4l8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
              </svg>
            </button>
          </span>
        </svelte:fragment>
      </ReorderableList>
    {:else}
      <ul class="autonomous-list__static" role="list" aria-label={ariaLabel}>
        {#each items as item (item.id)}
          <li class="autonomous-list__static-item">
            <span class="autonomous-list__item-text">{item.label}</span>
            <button
              type="button"
              class="autonomous-list__remove"
              disabled={isDisabled}
              aria-label="Remove {item.label}"
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
        disabled={isDisabled}
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

<style>
  .autonomous-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .autonomous-list[data-disabled="true"] {
    opacity: var(--pug-state-opacity-disabled);
  }

  .autonomous-list__static {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .autonomous-list__static-item {
    display: flex;
    align-items: center;
    padding: 0.5rem 0.625rem;
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
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
    width: 1.25rem;
    height: 1.25rem;
    padding: 0;
    border: 0;
    border-radius: 0.25rem;
    background: transparent;
    color: var(--pug-color-text-secondary);
    cursor: pointer;
    transition: color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .autonomous-list__remove:hover:not(:disabled) {
    color: var(--pug-color-status-danger);
  }

  .autonomous-list__remove svg {
    width: 0.75rem;
    height: 0.75rem;
  }

  .autonomous-list__add {
    display: flex;
    gap: 0.375rem;
  }

  .autonomous-list__input {
    flex: 1;
    min-width: 0;
    height: var(--pug-size-control-height);
    padding: 0 var(--pug-space-control-x);
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    outline: none;
  }

  .autonomous-list__input:focus {
    border-color: var(--pug-color-accent-focusRing);
    box-shadow: 0 0 0 var(--pug-border-width-focus)
      color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent);
  }

  .autonomous-list__input::placeholder {
    color: var(--pug-color-text-secondary);
  }

  .autonomous-list__add-btn {
    display: inline-flex;
    align-items: center;
    height: var(--pug-size-control-height);
    padding: 0 0.75rem;
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font-family: var(--pug-typography-label-family);
    font-size: var(--pug-typography-label-size);
    font-weight: var(--pug-typography-label-weight);
    transition:
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .autonomous-list__add-btn:hover:not(:disabled) {
    background: color-mix(in srgb, var(--pug-color-background-surface) 84%, var(--pug-color-background-elevated));
  }

  .autonomous-list__add-btn:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .autonomous-list__count {
    font-size: 0.6875rem;
    color: var(--pug-color-text-secondary);
    font-variant-numeric: tabular-nums;
    align-self: flex-end;
  }
</style>

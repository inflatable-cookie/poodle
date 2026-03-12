<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Checkbox, FormActions, SearchField } from "@pug/svelte-primitives";

  import PickerShell from "./PickerShell.svelte";
  import SelectionSummary from "./SelectionSummary.svelte";
  import type { BrowseState, PickerItem, PickerVariant, SelectionMode } from "./types";

  export let title = "Select items";
  export let description: string | null = null;
  export let items: PickerItem[] = [];
  export let selectedIds: string[] = [];
  export let query = "";
  export let selectionMode: SelectionMode = "multiple";
  export let variant: PickerVariant = "inline";
  export let state: BrowseState = "ready";
  export let ariaLabel: string | null = null;
  export let confirmLabel = "Confirm selection";
  export let cancelLabel = "Cancel";

  const dispatch = createEventDispatcher<{
    queryChange: { value: string };
    selectionChange: { selectedIds: string[] };
    confirm: { selectedIds: string[] };
    cancel: void;
  }>();
  const statusId = "relation-picker-status";
  let candidateButtons: Array<HTMLButtonElement | null> = [];

  $: filteredItems = items.filter((item) =>
    query.trim().length === 0
      ? true
      : [item.label, item.description ?? "", item.meta ?? ""].some((value) =>
          value.toLowerCase().includes(query.trim().toLowerCase()),
        ),
  );
  $: selectedItems = items
    .filter((item) => selectedIds.includes(item.id))
    .map((item) => ({ id: item.id, label: item.label }));
  $: pickerStatusText =
    state === "loading"
      ? "Picker results are loading."
      : state === "error"
        ? "Picker results are unavailable."
        : state === "empty"
          ? "No candidates are available."
          : state === "no-results"
            ? `No candidates match "${query}".`
            : `${filteredItems.length} candidate${filteredItems.length === 1 ? "" : "s"} available, ${selectedIds.length} selected.`;

  function setSelection(nextIds: string[]): void {
    dispatch("selectionChange", { selectedIds: nextIds });
  }

  function toggleItem(id: string): void {
    if (selectionMode === "single") {
      setSelection([id]);
      return;
    }

    setSelection(
      selectedIds.includes(id)
        ? selectedIds.filter((selectedId) => selectedId !== id)
        : [...selectedIds, id],
    );
  }

  function focusCandidate(index: number): void {
    candidateButtons[index]?.focus();
  }

  function handleCandidateKeydown(event: KeyboardEvent, index: number): void {
    if (filteredItems.length === 0) {
      return;
    }

    if (event.key === "ArrowDown" || event.key === "ArrowRight") {
      event.preventDefault();
      focusCandidate((index + 1) % filteredItems.length);
      return;
    }

    if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
      event.preventDefault();
      focusCandidate((index - 1 + filteredItems.length) % filteredItems.length);
      return;
    }

    if (event.key === "Home") {
      event.preventDefault();
      focusCandidate(0);
      return;
    }

    if (event.key === "End") {
      event.preventDefault();
      focusCandidate(filteredItems.length - 1);
    }
  }
</script>

<PickerShell
  {title}
  {description}
  {variant}
  {state}
  {ariaLabel}
  resultCount={filteredItems.length}
  selectionCount={selectedIds.length}
  statusText={pickerStatusText}
  statusId={statusId}
  stateTitle={state === "loading" ? "Loading candidates" : state === "error" ? "Picker unavailable" : state === "empty" ? "No candidates available" : "No matching candidates"}
  stateMessage={state === "loading" ? "Picker results are loading while selection state stays host-owned." : state === "error" ? "Error handling remains host-owned, but the picker preserves its structure." : state === "empty" ? "This relation has no available candidates yet." : "Try widening the search query or clearing selection filters."}
>
  <div slot="toolbar">
    <SearchField
      id="relation-picker-search"
      value={query}
      ariaLabel="Search picker results"
      describedBy={statusId}
      on:valueChange={(event) => dispatch("queryChange", event.detail)}
      on:clear={() => dispatch("queryChange", { value: "" })}
    />
  </div>

  <SelectionSummary
    slot="selection"
    items={selectedItems}
    {selectionMode}
    on:remove={(event) => setSelection(selectedIds.filter((id) => id !== event.detail.id))}
    on:clear={() => setSelection([])}
  />

  <slot name="state" slot="state" />

  <ul class="relation-picker__list" aria-label="Available candidates">
    {#each filteredItems as item, index}
      <li
        class="relation-picker__item"
        data-selected={selectedIds.includes(item.id)}
      >
        {#if selectionMode === "multiple"}
          <Checkbox
            ariaLabel={`Select ${item.label}`}
            isChecked={selectedIds.includes(item.id)}
            on:checkedChange={() => toggleItem(item.id)}
          />
          <button
            bind:this={candidateButtons[index]}
            type="button"
            class="relation-picker__item-button"
            aria-pressed={selectedIds.includes(item.id)}
            aria-describedby={item.description || item.meta ? `relation-picker-item-${item.id}` : undefined}
            on:click={() => toggleItem(item.id)}
            on:keydown={(event) => handleCandidateKeydown(event, index)}
          >
            <span class="relation-picker__item-copy">
              <strong>{item.label}</strong>
              {#if item.description || item.meta}
                <small id={`relation-picker-item-${item.id}`}>
                  {item.description ?? ""}
                  {#if item.description && item.meta}
                    {" · "}
                  {/if}
                  {item.meta ?? ""}
                </small>
              {/if}
            </span>
          </button>
        {:else}
          <button
            bind:this={candidateButtons[index]}
            type="button"
            class="relation-picker__item-button"
            aria-pressed={selectedIds.includes(item.id)}
            aria-describedby={item.description || item.meta ? `relation-picker-item-${item.id}` : undefined}
            on:click={() => toggleItem(item.id)}
            on:keydown={(event) => handleCandidateKeydown(event, index)}
          >
            <span class="relation-picker__item-copy">
              <strong>{item.label}</strong>
              {#if item.description || item.meta}
                <small id={`relation-picker-item-${item.id}`}>
                  {item.description ?? ""}
                  {#if item.description && item.meta}
                    {" · "}
                  {/if}
                  {item.meta ?? ""}
                </small>
              {/if}
            </span>
          </button>
        {/if}
      </li>
    {/each}
  </ul>

  <div slot="footer">
    <FormActions align="between">
      <p class="relation-picker__footer-note">
        {selectionMode === "single" ? "Single-choice selection keeps the picker confirmable without inline radio-group chrome." : "Multi-selection stays explicit through selection summary and confirm/cancel actions."}
      </p>
      <div class="relation-picker__footer-actions">
        <button type="button" class="secondary-action" on:click={() => dispatch("cancel")}>
          {cancelLabel}
        </button>
        <button type="button" class="primary-action" on:click={() => dispatch("confirm", { selectedIds })}>
          {confirmLabel}
        </button>
      </div>
    </FormActions>
  </div>
</PickerShell>

<style>
  .relation-picker__list {
    display: grid;
    gap: var(--pug-space-stack-sm);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .relation-picker__item {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--pug-space-inline-md);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 1px solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-surface) 86%, transparent);
    color: var(--pug-color-text-primary);
  }

  .relation-picker__item[data-selected="true"] {
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 60%, transparent);
    background: color-mix(in srgb, var(--pug-color-accent-base) 10%, transparent);
  }

  .relation-picker__item-button {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    gap: 4px;
    min-width: 0;
    padding: 0;
    border: 0;
    background: transparent;
    color: inherit;
    cursor: pointer;
    text-align: left;
    font: inherit;
  }

  .relation-picker__item-copy {
    display: grid;
    gap: 4px;
  }

  .relation-picker__item-copy strong,
  .relation-picker__item-copy small,
  .relation-picker__footer-note {
    margin: 0;
  }

  .relation-picker__item-copy small,
  .relation-picker__footer-note {
    color: var(--pug-color-text-secondary);
    font-size: 13px;
    line-height: 1.5;
  }

  .relation-picker__item-button:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 2px;
    border-radius: var(--pug-radius-control);
  }

  .relation-picker__footer-actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
    justify-content: flex-end;
  }
</style>

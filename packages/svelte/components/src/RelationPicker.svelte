<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Button from "./Button.svelte";
  import Checkbox from "./Checkbox.svelte";
  import FormActions from "./FormActions.svelte";
  import Icon from "./Icon.svelte";
  import TextInput from "./TextInput.svelte";
  import UiPresentationProvider from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  import PickerShell from "./PickerShell.svelte";
  import SelectionSummary from "./SelectionSummary.svelte";
  import type {
    BrowseState,
    DrillDownConfig,
    DrillDownContext,
    DrillDownItem,
    PickerItem,
    PickerVariant,
    SelectionMode,
  } from "./types";

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
  export let drillDown: DrillDownConfig | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    queryChange: { value: string };
    selectionChange: { selectedIds: string[] };
    confirm: { selectedIds: string[] };
    cancel: void;
    drillContext: { context: DrillDownContext };
  }>();
  const statusId = "relation-picker-status";
  let candidateButtons: Array<HTMLButtonElement | null> = [];
  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;

  // Drill-down state
  let drillDepth = 0;
  let drillSelections: Record<string, DrillDownItem> = {};
  let drillSearchQuery = "";
  let drillItems: DrillDownItem[] = [];
  let drillLoading = false;
  let finalItemsLoaded: PickerItem[] | null = null;
  let finalItemsLoading = false;

  $: isDrilling = drillDown !== null && drillDepth < (drillDown?.levels.length ?? 0);
  $: hasDrillCompleted = drillDown !== null && drillDepth >= (drillDown?.levels.length ?? 0) && Object.keys(drillSelections).length > 0;
  $: currentLevel = drillDown?.levels[drillDepth] ?? null;

  // Load final items when drill-down completes
  $: if (hasDrillCompleted && drillDown?.finalItems) {
    loadFinalItems(drillDown.finalItems, drillContext, query);
  }

  $: drillContext = Object.fromEntries(
    Object.entries(drillSelections).map(([key, item]) => [key, item.id]),
  ) as DrillDownContext;

  $: drillBreadcrumbs = drillDown
    ? drillDown.levels
        .slice(0, drillDepth)
        .map((level) => ({
          key: level.key,
          label: drillSelections[level.key]?.label ?? level.label,
          depth: drillDown!.levels.indexOf(level),
        }))
    : [];

  // Load drill-down items when level changes
  $: if (currentLevel) {
    loadDrillItems(currentLevel, drillContext, drillSearchQuery);
  }

  async function loadDrillItems(
    level: typeof currentLevel,
    context: DrillDownContext,
    searchQuery: string,
  ): Promise<void> {
    if (!level) return;

    if (typeof level.items === "function") {
      drillLoading = true;
      try {
        const result = await level.items(searchQuery, context);
        drillItems = result;
      } catch {
        drillItems = [];
      }
      drillLoading = false;
    } else {
      const q = searchQuery.trim().toLowerCase();
      drillItems = q
        ? level.items.filter((item) =>
            [item.label, item.description ?? "", item.meta ?? ""].some((v) =>
              v.toLowerCase().includes(q),
            ),
          )
        : level.items;
    }
  }

  async function loadFinalItems(
    fn: typeof drillDown extends null ? never : NonNullable<DrillDownConfig['finalItems']>,
    context: DrillDownContext,
    searchQuery: string,
  ): Promise<void> {
    finalItemsLoading = true;
    try {
      const result = await fn(searchQuery, context);
      finalItemsLoaded = result;
    } catch {
      finalItemsLoaded = [];
    }
    finalItemsLoading = false;
  }

  function drillSelect(item: DrillDownItem): void {
    if (!drillDown) return;
    drillSelections[currentLevel!.key] = item;
    drillSelections = drillSelections;
    drillSearchQuery = "";
    drillDepth++;

    const ctx = Object.fromEntries(
      Object.entries(drillSelections).map(([key, i]) => [key, i.id]),
    ) as DrillDownContext;
    dispatch("drillContext", { context: ctx });
  }

  function drillBack(): void {
    if (drillDepth <= 0) return;
    drillDepth--;
    const levelKey = drillDown!.levels[drillDepth].key;
    delete drillSelections[levelKey];
    drillSelections = drillSelections;
    drillSearchQuery = "";
    finalItemsLoaded = null;
  }

  function drillNavigateTo(depth: number): void {
    for (let i = depth; i < (drillDown?.levels.length ?? 0); i++) {
      const key = drillDown!.levels[i].key;
      delete drillSelections[key];
    }
    drillSelections = drillSelections;
    drillDepth = depth;
    drillSearchQuery = "";
    finalItemsLoaded = null;
  }

  function handleDrillSearchKeydown(event: CustomEvent<KeyboardEvent>): void {
    const keyboardEvent = event.detail;

    if (keyboardEvent.key === "Escape" || (keyboardEvent.key === "Backspace" && !drillSearchQuery)) {
      if (drillDepth > 0) {
        keyboardEvent.preventDefault();
        drillBack();
      }
    }
  }

  // Flat picker logic — use finalItemsLoaded when drill-down provides items
  $: activeItems = (hasDrillCompleted && finalItemsLoaded !== null) ? finalItemsLoaded : items;
  $: filteredItems = activeItems.filter((item) =>
    query.trim().length === 0
      ? true
      : [item.label, item.description ?? "", item.meta ?? ""].some((value) =>
          value.toLowerCase().includes(query.trim().toLowerCase()),
        ),
  );
  $: selectedItems = activeItems
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
    selectedIds = nextIds;
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
    if (filteredItems.length === 0) return;

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

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div class="relation-picker" data-size={resolvedSize} data-density={resolvedDensity}>
    <PickerShell
      {title}
      {description}
      {variant}
      state={isDrilling ? (drillLoading ? "loading" : "ready") : state}
      {ariaLabel}
      resultCount={isDrilling ? drillItems.length : filteredItems.length}
      selectionCount={selectedIds.length}
      statusText={isDrilling ? `${drillItems.length} item${drillItems.length === 1 ? "" : "s"}` : pickerStatusText}
      statusId={statusId}
      stateTitle={isDrilling ? "Loading" : (state === "loading" ? "Loading candidates" : state === "error" ? "Picker unavailable" : state === "empty" ? "No candidates available" : "No matching candidates")}
      stateMessage={isDrilling ? "Loading items..." : (state === "loading" ? "Picker results are loading while selection state stays host-owned." : state === "error" ? "Error handling remains host-owned, but the picker preserves its structure." : state === "empty" ? "This relation has no available candidates yet." : "Try widening the search query or clearing selection filters.")}
    >
      <div slot="toolbar">
        {#if isDrilling}
          {#if drillBreadcrumbs.length > 0}
            <div class="drill-breadcrumbs">
              <button
                type="button"
                class="drill-breadcrumbs__back"
                on:click={drillBack}
                aria-label="Go back"
              >
                <Icon name="chevron-left" />
              </button>
              {#each drillBreadcrumbs as crumb, i}
                {#if i > 0}
                  <span class="drill-breadcrumbs__sep">/</span>
                {/if}
                <button
                  type="button"
                  class="drill-breadcrumbs__item"
                  on:click={() => drillNavigateTo(crumb.depth)}
                >
                  {crumb.label}
                </button>
              {/each}
            </div>
          {/if}
          <div class="drill-level-label">{currentLevel?.label}</div>
          <TextInput
            id="drill-search"
            type="search"
            value={drillSearchQuery}
            ariaLabel={`Search ${currentLevel?.label ?? "items"}`}
            placeholder={currentLevel?.searchPlaceholder ?? `Search ${currentLevel?.label?.toLowerCase() ?? 'items'}...`}
            on:valueChange={(e) => { drillSearchQuery = e.detail.value; }}
            on:clear={() => { drillSearchQuery = ""; }}
            on:keydown={handleDrillSearchKeydown}
          />
        {:else}
          {#if drillDown && drillBreadcrumbs.length > 0}
            <div class="drill-breadcrumbs">
              <button
                type="button"
                class="drill-breadcrumbs__back"
                on:click={drillBack}
                aria-label="Go back"
              >
                <Icon name="chevron-left" />
              </button>
              {#each drillBreadcrumbs as crumb, i}
                {#if i > 0}
                  <span class="drill-breadcrumbs__sep">/</span>
                {/if}
                <button
                  type="button"
                  class="drill-breadcrumbs__item"
                  on:click={() => drillNavigateTo(crumb.depth)}
                >
                  {crumb.label}
                </button>
              {/each}
            </div>
          {/if}
          <TextInput
            id="relation-picker-search"
            type="search"
            value={query}
            ariaLabel="Search picker results"
            describedBy={statusId}
            on:valueChange={(event) => { query = event.detail.value; dispatch("queryChange", event.detail); }}
            on:clear={() => { query = ""; dispatch("queryChange", { value: "" }); }}
          />
        {/if}
      </div>

      {#if !isDrilling}
        <SelectionSummary
          slot="selection"
          items={selectedItems}
          on:remove={(event) => setSelection(selectedIds.filter((id) => id !== event.detail.id))}
          on:clear={() => setSelection([])}
        />
      {/if}

      <slot name="state" slot="state" />

      {#if isDrilling}
        <ul class="drill-list" aria-label={currentLevel?.label ?? "Items"}>
          {#each drillItems as item (item.id)}
            <li class="drill-list__item">
              <button
                type="button"
                class="drill-list__button"
                on:click={() => drillSelect(item)}
              >
                <span class="drill-list__copy">
                  <strong>{item.label}</strong>
                  {#if item.description}
                    <small>{item.description}</small>
                  {/if}
                </span>
                <span class="drill-list__meta">
                  {#if item.count !== undefined}
                    <span class="drill-list__count">{item.count}</span>
                  {/if}
                  <Icon name="chevron-right" />
                </span>
              </button>
            </li>
          {/each}
          {#if drillItems.length === 0 && !drillLoading}
            <li class="drill-list__empty">No items found</li>
          {/if}
        </ul>
      {:else}
        <ul class="relation-picker__list" aria-label="Available candidates">
          {#each filteredItems as item, index}
            <li
              class="relation-picker__item"
              data-selected={selectedIds.includes(item.id)}
            >
              {#if selectionMode === "multiple"}
                <Checkbox
                  ariaLabel={`Select ${item.label}`}
                  checked={selectedIds.includes(item.id)}
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
      {/if}

      <div slot="footer">
        <FormActions align="between">
          <p class="relation-picker__footer-note">
            {selectionMode === "single" ? "Single-choice selection keeps the picker confirmable without inline radio-group chrome." : "Multi-selection stays explicit through selection summary and confirm/cancel actions."}
          </p>
          <div class="relation-picker__footer-actions">
            <Button variant="ghost" size={resolvedSize} on:click={() => dispatch("cancel")}>
              {cancelLabel}
            </Button>
            <Button variant="primary" size={resolvedSize} on:click={() => dispatch("confirm", { selectedIds })}>
              {confirmLabel}
            </Button>
          </div>
        </FormActions>
      </div>
    </PickerShell>
  </div>
</UiPresentationProvider>

<style>
  .relation-picker {
    --poodle-relation-picker-breadcrumb-control: 1.5rem;
    --poodle-relation-picker-breadcrumb-x: 0.375rem;
    --poodle-relation-picker-list-x: 0.625rem;
    --poodle-relation-picker-list-y: 0.5rem;
    --poodle-relation-picker-list-gap: 0.125rem;
    --poodle-relation-picker-item-y: 0.375rem;
    --poodle-relation-picker-item-x: 0.5rem;
    --poodle-relation-picker-item-gap: 0.5rem;
    --poodle-relation-picker-title-size: 0.8125rem;
    --poodle-relation-picker-desc-size: 0.6875rem;
  }

  .relation-picker[data-size="xs"] {
    --poodle-relation-picker-breadcrumb-control: 1.25rem;
    --poodle-relation-picker-breadcrumb-x: 0.25rem;
    --poodle-relation-picker-list-x: 0.5rem;
    --poodle-relation-picker-item-y: 0.25rem;
    --poodle-relation-picker-item-x: 0.375rem;
    --poodle-relation-picker-item-gap: 0.375rem;
    --poodle-relation-picker-title-size: 0.6875rem;
    --poodle-relation-picker-desc-size: 0.5625rem;
  }

  .relation-picker[data-size="sm"] {
    --poodle-relation-picker-breadcrumb-control: 1.5rem;
    --poodle-relation-picker-title-size: 0.75rem;
    --poodle-relation-picker-desc-size: 0.625rem;
  }

  .relation-picker[data-size="md"] {
    --poodle-relation-picker-breadcrumb-control: 1.75rem;
    --poodle-relation-picker-title-size: 0.8125rem;
    --poodle-relation-picker-desc-size: 0.6875rem;
  }

  .relation-picker[data-size="lg"] {
    --poodle-relation-picker-breadcrumb-control: 2rem;
    --poodle-relation-picker-breadcrumb-x: 0.5rem;
    --poodle-relation-picker-list-x: 0.75rem;
    --poodle-relation-picker-item-y: 0.5rem;
    --poodle-relation-picker-item-x: 0.625rem;
    --poodle-relation-picker-title-size: 0.875rem;
    --poodle-relation-picker-desc-size: 0.75rem;
  }

  .relation-picker[data-size="xl"] {
    --poodle-relation-picker-breadcrumb-control: 2.25rem;
    --poodle-relation-picker-breadcrumb-x: 0.625rem;
    --poodle-relation-picker-list-x: 0.875rem;
    --poodle-relation-picker-item-y: 0.5rem;
    --poodle-relation-picker-item-x: 0.75rem;
    --poodle-relation-picker-title-size: 0.9375rem;
    --poodle-relation-picker-desc-size: 0.8125rem;
  }

  .relation-picker[data-density="compact"] {
    --poodle-relation-picker-list-gap: 0.0625rem;
  }

  .relation-picker[data-density="comfortable"] {
    --poodle-relation-picker-list-gap: 0.1875rem;
  }

  /* Drill-down breadcrumbs */
  .drill-breadcrumbs {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding-bottom: var(--poodle-space-stack-sm);
  }

  .drill-breadcrumbs__back {
    display: flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-relation-picker-breadcrumb-control);
    height: var(--poodle-relation-picker-breadcrumb-control);
    padding: 0;
    border: none;
    border-radius: var(--poodle-radius-sm, 0.25rem);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
  }

  .drill-breadcrumbs__back:hover {
    background: var(--poodle-color-surface-hover, rgba(148, 163, 184, 0.12));
    color: var(--poodle-color-text-primary);
  }

  .drill-breadcrumbs__sep {
    color: var(--poodle-color-text-secondary);
    font-size: 0.6875rem;
    opacity: 0.6;
  }

  .drill-breadcrumbs__item {
    min-height: calc(var(--poodle-relation-picker-breadcrumb-control) - 0.25rem);
    padding: 0.125rem var(--poodle-relation-picker-breadcrumb-x);
    border: none;
    border-radius: var(--poodle-radius-sm, 0.25rem);
    background: transparent;
    color: var(--poodle-color-accent-base);
    font-size: var(--poodle-typography-label-size);
    font-weight: 500;
    cursor: pointer;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 8rem;
  }

  .drill-breadcrumbs__item:hover {
    background: var(--poodle-color-surface-hover, rgba(148, 163, 184, 0.12));
  }

  .drill-level-label {
    font-size: var(--poodle-typography-label-size);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--poodle-color-text-secondary);
    padding-bottom: 0.25rem;
  }

  /* Drill-down list */
  .drill-list {
    display: grid;
    gap: var(--poodle-relation-picker-list-gap);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .drill-list__item {
    display: flex;
  }

  .drill-list__button {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
    width: 100%;
    padding: var(--poodle-relation-picker-list-y) var(--poodle-relation-picker-list-x);
    border: none;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    text-align: left;
    font: inherit;
    font-size: var(--poodle-typography-body-size);
  }

  .drill-list__button:hover {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 60%, transparent);
  }

  .drill-list__button:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: -0.0625rem;
  }

  .drill-list__copy {
    display: grid;
    gap: 0.125rem;
    min-width: 0;
  }

  .drill-list__copy strong {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .drill-list__copy small {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .drill-list__meta {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    flex-shrink: 0;
    color: var(--poodle-color-text-secondary);
  }

  .drill-list__count {
    font-size: var(--poodle-typography-label-size);
    opacity: 0.7;
  }

  .drill-list__empty {
    padding: calc(var(--poodle-relation-picker-list-y) * 2.5);
    text-align: center;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }

  /* Existing flat picker styles */
  .relation-picker__list {
    display: grid;
    gap: var(--poodle-relation-picker-list-gap);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .relation-picker__item {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--poodle-relation-picker-item-gap);
    padding: var(--poodle-relation-picker-item-y) var(--poodle-relation-picker-item-x);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 86%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .relation-picker__item[data-selected="true"] {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 60%, transparent);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent);
  }

  .relation-picker__item-button {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    gap: 0.25rem;
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
    gap: 0.25rem;
  }

  .relation-picker__item-copy strong {
    margin: 0;
    font-size: var(--poodle-relation-picker-title-size);
    font-weight: 500;
  }

  .relation-picker__item-copy small {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-relation-picker-desc-size);
    line-height: 1.4;
  }

  .relation-picker__footer-note {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-relation-picker-desc-size);
    line-height: 1.5;
  }

  .relation-picker__item-button:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
    border-radius: var(--poodle-radius-control);
  }

  .relation-picker__footer-actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    justify-content: flex-end;
  }
</style>

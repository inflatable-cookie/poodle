<script lang="ts">
  import type { Snippet } from "svelte";

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

  interface Props {
    title?: string;
    description?: string | null;
    items?: PickerItem[];
    selectedIds?: string[];
    query?: string;
    selectionMode?: SelectionMode;
    variant?: PickerVariant;
    state?: BrowseState;
    ariaLabel?: string | null;
    confirmLabel?: string;
    cancelLabel?: string;
    drillDown?: DrillDownConfig | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onQueryChange?: ((value: string) => void) | undefined;
    onSelectionChange?: ((selectedIds: string[]) => void) | undefined;
    onConfirm?: ((selectedIds: string[]) => void) | undefined;
    onCancel?: (() => void) | undefined;
    onDrillContext?: ((context: DrillDownContext) => void) | undefined;
    stateContent?: Snippet<[]>;
  }

  let {
    title = "Select items",
    description = null,
    items = [],
    selectedIds = [],
    query = "",
    selectionMode = "multiple",
    variant = "inline",
    state: browseState = "ready",
    ariaLabel = null,
    confirmLabel = "Confirm selection",
    cancelLabel = "Cancel",
    drillDown = null,
    size = null,
    sizeRole = "control",
    density = null,
    onQueryChange = undefined,
    onSelectionChange = undefined,
    onConfirm = undefined,
    onCancel = undefined,
    onDrillContext = undefined,
    stateContent,
  }: Props = $props();

  const statusId = "relation-picker-status";
  let candidateButtons = $state<Array<HTMLButtonElement | null>>([]);
  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const currentSelectedIds = $derived(selectedIds);
  const currentQuery = $derived(query);

  // Drill-down state
  let drillDepth = $state(0);
  let drillSelections = $state<Record<string, DrillDownItem>>({});
  let drillSearchQuery = $state("");
  let drillItems = $state<DrillDownItem[]>([]);
  let drillLoading = $state(false);
  let finalItemsLoaded = $state<PickerItem[] | null>(null);
  let finalItemsLoading = $state(false);

  const isDrilling = $derived(drillDown !== null && drillDepth < (drillDown?.levels.length ?? 0));
  const hasDrillCompleted = $derived(drillDown !== null && drillDepth >= (drillDown?.levels.length ?? 0) && Object.keys(drillSelections).length > 0);
  const currentLevel = $derived(drillDown?.levels[drillDepth] ?? null);

  // Load final items when drill-down completes
  $effect(() => {
    if (hasDrillCompleted && drillDown?.finalItems) {
      loadFinalItems(drillDown.finalItems, drillContext, currentQuery);
    }
  });

  const drillContext = $derived(Object.fromEntries(
    Object.entries(drillSelections).map(([key, item]) => [key, item.id]),
  ) as DrillDownContext);

  const drillBreadcrumbs = $derived(drillDown
    ? drillDown.levels
        .slice(0, drillDepth)
        .map((level) => ({
          key: level.key,
          label: drillSelections[level.key]?.label ?? level.label,
          depth: drillDown!.levels.indexOf(level),
        }))
    : []);

  // Load drill-down items when level changes
  $effect(() => {
    if (currentLevel) {
      loadDrillItems(currentLevel, drillContext, drillSearchQuery);
    }
  });

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
    onDrillContext?.(ctx);
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
  const activeItems = $derived((hasDrillCompleted && finalItemsLoaded !== null) ? finalItemsLoaded : items);
  const filteredItems = $derived(activeItems.filter((item) =>
    currentQuery.trim().length === 0
      ? true
      : [item.label, item.description ?? "", item.meta ?? ""].some((value) =>
          value.toLowerCase().includes(currentQuery.trim().toLowerCase()),
        ),
  ));
  const selectedItems = $derived(activeItems
    .filter((item) => currentSelectedIds.includes(item.id))
    .map((item) => ({ id: item.id, label: item.label })));
  const pickerStatusText = $derived(
    browseState === "loading"
      ? "Picker results are loading."
      : browseState === "error"
        ? "Picker results are unavailable."
        : browseState === "empty"
          ? "No candidates are available."
          : browseState === "no-results"
            ? `No candidates match "${currentQuery}".`
            : `${filteredItems.length} candidate${filteredItems.length === 1 ? "" : "s"} available, ${currentSelectedIds.length} selected.`
  );

  function setSelection(nextIds: string[]): void {
    onSelectionChange?.(nextIds);
  }

  function toggleItem(id: string): void {
    if (selectionMode === "single") {
      setSelection([id]);
      return;
    }

    setSelection(
      currentSelectedIds.includes(id)
        ? currentSelectedIds.filter((selectedId) => selectedId !== id)
        : [...currentSelectedIds, id],
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

{#snippet toolbarContent()}
  {#if isDrilling}
    {#if drillBreadcrumbs.length > 0}
      <div class="poodle-drill-breadcrumbs">
        <button
          type="button"
          class="poodle-drill-breadcrumbs__back"
          onclick={drillBack}
          aria-label="Go back"
        >
          <Icon name="chevron-left" />
        </button>
        {#each drillBreadcrumbs as crumb, i}
          {#if i > 0}
            <span class="poodle-drill-breadcrumbs__sep">/</span>
          {/if}
          <button
            type="button"
            class="poodle-drill-breadcrumbs__item"
            onclick={() => drillNavigateTo(crumb.depth)}
          >
            {crumb.label}
          </button>
        {/each}
      </div>
    {/if}
    <div class="poodle-drill-level-label">{currentLevel?.label}</div>
    <TextInput
      id="drill-search"
      type="search"
      value={drillSearchQuery}
      ariaLabel={`Search ${currentLevel?.label ?? "items"}`}
      placeholder={currentLevel?.searchPlaceholder ?? `Search ${currentLevel?.label?.toLowerCase() ?? 'items'}...`}
      onValueChange={(nextValue) => { drillSearchQuery = nextValue; }}
      onClear={() => { drillSearchQuery = ""; }}
      onKeyDown={handleDrillSearchKeydown}
    />
  {:else}
    {#if drillDown && drillBreadcrumbs.length > 0}
      <div class="poodle-drill-breadcrumbs">
        <button
          type="button"
          class="poodle-drill-breadcrumbs__back"
          onclick={drillBack}
          aria-label="Go back"
        >
          <Icon name="chevron-left" />
        </button>
        {#each drillBreadcrumbs as crumb, i}
          {#if i > 0}
            <span class="poodle-drill-breadcrumbs__sep">/</span>
          {/if}
          <button
            type="button"
            class="poodle-drill-breadcrumbs__item"
            onclick={() => drillNavigateTo(crumb.depth)}
          >
            {crumb.label}
          </button>
        {/each}
      </div>
    {/if}
    <TextInput
      id="relation-picker-search"
      type="search"
      value={currentQuery}
      ariaLabel="Search picker results"
      describedBy={statusId}
      onValueChange={(nextValue) => onQueryChange?.(nextValue)}
      onClear={() => onQueryChange?.("")}
    />
  {/if}
{/snippet}

{#snippet selectionContent()}
  {#if !isDrilling}
    <SelectionSummary
      items={selectedItems}
      onRemove={(id) => setSelection(currentSelectedIds.filter((selectedId) => selectedId !== id))}
      onClear={() => setSelection([])}
    />
  {/if}
{/snippet}

{#snippet footerContent()}
  <FormActions align="between">
    <p class="poodle-relation-picker__footer-note">
      {selectionMode === "single" ? "Single-choice selection keeps the picker confirmable without inline radio-group chrome." : "Multi-selection stays explicit through selection summary and confirm/cancel actions."}
    </p>
    <div class="poodle-relation-picker__footer-actions">
      <Button variant="ghost" size={resolvedSize} onClick={() => onCancel?.()}>
        {cancelLabel}
      </Button>
      <Button variant="primary" size={resolvedSize} onClick={() => onConfirm?.(currentSelectedIds)}>
        {confirmLabel}
      </Button>
    </div>
  </FormActions>
{/snippet}

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div class="poodle-relation-picker" data-size={resolvedSize} data-density={resolvedDensity}>
    <PickerShell
      {title}
      {description}
      {variant}
      state={isDrilling ? (drillLoading ? "loading" : "ready") : browseState}
      {ariaLabel}
      resultCount={isDrilling ? drillItems.length : filteredItems.length}
      selectionCount={currentSelectedIds.length}
      statusText={isDrilling ? `${drillItems.length} item${drillItems.length === 1 ? "" : "s"}` : pickerStatusText}
      statusId={statusId}
      stateTitle={isDrilling ? "Loading" : (browseState === "loading" ? "Loading candidates" : browseState === "error" ? "Picker unavailable" : browseState === "empty" ? "No candidates available" : "No matching candidates")}
      stateMessage={isDrilling ? "Loading items..." : (browseState === "loading" ? "Picker results are loading while selection state stays host-owned." : browseState === "error" ? "Error handling remains host-owned, but the picker preserves its structure." : browseState === "empty" ? "This relation has no available candidates yet." : "Try widening the search query or clearing selection filters.")}
      toolbar={toolbarContent}
      selection={!isDrilling ? selectionContent : undefined}
      stateContent={stateContent}
      footer={footerContent}
    >
      {#if isDrilling}
        <ul class="poodle-drill-list" aria-label={currentLevel?.label ?? "Items"}>
          {#each drillItems as item (item.id)}
            <li class="poodle-drill-list__item">
              <button
                type="button"
                class="poodle-drill-list__button"
                onclick={() => drillSelect(item)}
              >
                <span class="poodle-drill-list__copy">
                  <strong>{item.label}</strong>
                  {#if item.description}
                    <small>{item.description}</small>
                  {/if}
                </span>
                <span class="poodle-drill-list__meta">
                  {#if item.count !== undefined}
                    <span class="poodle-drill-list__count">{item.count}</span>
                  {/if}
                  <Icon name="chevron-right" />
                </span>
              </button>
            </li>
          {/each}
          {#if drillItems.length === 0 && !drillLoading}
            <li class="poodle-drill-list__empty">No items found</li>
          {/if}
        </ul>
      {:else}
        <ul class="poodle-relation-picker__list" aria-label="Available candidates">
          {#each filteredItems as item, index}
            <li
              class="poodle-relation-picker__item"
              data-selected={currentSelectedIds.includes(item.id)}
            >
              {#if selectionMode === "multiple"}
                <Checkbox
                  ariaLabel={`Select ${item.label}`}
                  checked={currentSelectedIds.includes(item.id)}
                  onCheckedChange={() => toggleItem(item.id)}
                />
                <button
                  bind:this={candidateButtons[index]}
                  type="button"
                  class="poodle-relation-picker__item-button"
                  aria-pressed={currentSelectedIds.includes(item.id)}
                  aria-describedby={item.description || item.meta ? `relation-picker-item-${item.id}` : undefined}
                  onclick={() => toggleItem(item.id)}
                  onkeydown={(event) => handleCandidateKeydown(event, index)}
                >
                  <span class="poodle-relation-picker__item-copy">
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
                  class="poodle-relation-picker__item-button"
                  aria-pressed={currentSelectedIds.includes(item.id)}
                  aria-describedby={item.description || item.meta ? `relation-picker-item-${item.id}` : undefined}
                  onclick={() => toggleItem(item.id)}
                  onkeydown={(event) => handleCandidateKeydown(event, index)}
                >
                  <span class="poodle-relation-picker__item-copy">
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
    </PickerShell>
  </div>
</UiPresentationProvider>

<style>
  .poodle-relation-picker {
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

  .poodle-relation-picker[data-size="xs"] {
    --poodle-relation-picker-breadcrumb-control: 1.25rem;
    --poodle-relation-picker-breadcrumb-x: 0.25rem;
    --poodle-relation-picker-list-x: 0.5rem;
    --poodle-relation-picker-item-y: 0.25rem;
    --poodle-relation-picker-item-x: 0.375rem;
    --poodle-relation-picker-item-gap: 0.375rem;
    --poodle-relation-picker-title-size: 0.6875rem;
    --poodle-relation-picker-desc-size: 0.5625rem;
  }

  .poodle-relation-picker[data-size="sm"] {
    --poodle-relation-picker-breadcrumb-control: 1.5rem;
    --poodle-relation-picker-title-size: 0.75rem;
    --poodle-relation-picker-desc-size: 0.625rem;
  }

  .poodle-relation-picker[data-size="md"] {
    --poodle-relation-picker-breadcrumb-control: 1.75rem;
    --poodle-relation-picker-title-size: 0.8125rem;
    --poodle-relation-picker-desc-size: 0.6875rem;
  }

  .poodle-relation-picker[data-size="lg"] {
    --poodle-relation-picker-breadcrumb-control: 2rem;
    --poodle-relation-picker-breadcrumb-x: 0.5rem;
    --poodle-relation-picker-list-x: 0.75rem;
    --poodle-relation-picker-item-y: 0.5rem;
    --poodle-relation-picker-item-x: 0.625rem;
    --poodle-relation-picker-title-size: 0.875rem;
    --poodle-relation-picker-desc-size: 0.75rem;
  }

  .poodle-relation-picker[data-size="xl"] {
    --poodle-relation-picker-breadcrumb-control: 2.25rem;
    --poodle-relation-picker-breadcrumb-x: 0.625rem;
    --poodle-relation-picker-list-x: 0.875rem;
    --poodle-relation-picker-item-y: 0.5rem;
    --poodle-relation-picker-item-x: 0.75rem;
    --poodle-relation-picker-title-size: 0.9375rem;
    --poodle-relation-picker-desc-size: 0.8125rem;
  }

  .poodle-relation-picker[data-density="compact"] {
    --poodle-relation-picker-list-gap: 0.0625rem;
  }

  .poodle-relation-picker[data-density="comfortable"] {
    --poodle-relation-picker-list-gap: 0.1875rem;
  }

  /* Drill-down breadcrumbs */
  .poodle-drill-breadcrumbs {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding-bottom: var(--poodle-space-stack-sm);
  }

  .poodle-drill-breadcrumbs__back {
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

  .poodle-drill-breadcrumbs__back:hover {
    background: var(--poodle-color-surface-hover, rgba(148, 163, 184, 0.12));
    color: var(--poodle-color-text-primary);
  }

  .poodle-drill-breadcrumbs__sep {
    color: var(--poodle-color-text-secondary);
    font-size: 0.6875rem;
    opacity: 0.6;
  }

  .poodle-drill-breadcrumbs__item {
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

  .poodle-drill-breadcrumbs__item:hover {
    background: var(--poodle-color-surface-hover, rgba(148, 163, 184, 0.12));
  }

  .poodle-drill-level-label {
    font-size: var(--poodle-typography-label-size);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--poodle-color-text-secondary);
    padding-bottom: 0.25rem;
  }

  /* Drill-down list */
  .poodle-drill-list {
    display: grid;
    gap: var(--poodle-relation-picker-list-gap);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .poodle-drill-list__item {
    display: flex;
  }

  .poodle-drill-list__button {
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

  .poodle-drill-list__button:hover {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 60%, transparent);
  }

  .poodle-drill-list__button:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: -0.0625rem;
  }

  .poodle-drill-list__copy {
    display: grid;
    gap: 0.125rem;
    min-width: 0;
  }

  .poodle-drill-list__copy strong {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .poodle-drill-list__copy small {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .poodle-drill-list__meta {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    flex-shrink: 0;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-drill-list__count {
    font-size: var(--poodle-typography-label-size);
    opacity: 0.7;
  }

  .poodle-drill-list__empty {
    padding: calc(var(--poodle-relation-picker-list-y) * 2.5);
    text-align: center;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }

  /* Existing flat picker styles */
  .poodle-relation-picker__list {
    display: grid;
    gap: var(--poodle-relation-picker-list-gap);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .poodle-relation-picker__item {
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

  .poodle-relation-picker__item[data-selected="true"] {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 60%, transparent);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent);
  }

  .poodle-relation-picker__item-button {
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

  .poodle-relation-picker__item-copy {
    display: grid;
    gap: 0.25rem;
  }

  .poodle-relation-picker__item-copy strong {
    margin: 0;
    font-size: var(--poodle-relation-picker-title-size);
    font-weight: 500;
  }

  .poodle-relation-picker__item-copy small {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-relation-picker-desc-size);
    line-height: 1.4;
  }

  .poodle-relation-picker__footer-note {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-relation-picker-desc-size);
    line-height: 1.5;
  }

  .poodle-relation-picker__item-button:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
    border-radius: var(--poodle-radius-control);
  }

  .poodle-relation-picker__footer-actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    justify-content: flex-end;
  }
</style>

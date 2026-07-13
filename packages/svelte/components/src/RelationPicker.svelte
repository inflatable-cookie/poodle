<script lang="ts">
  import "@poodle/styles/relation-picker.css";
  import type { Snippet } from "svelte";

  import { default as Button } from "./Button.svelte";
  import { default as Checkbox } from "./Checkbox.svelte";
  import { default as FormActions } from "./FormActions.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as Select } from "./Select.svelte";
  import { default as TextInput } from "./TextInput.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  import { default as PickerShell } from "./PickerShell.svelte";
  import { default as SelectionSummary } from "./SelectionSummary.svelte";
  import type {
    BrowseState,
    DrillDownConfig,
    DrillDownContext,
    DrillDownItem,
    PickerFilterConfig,
    PickerItem,
    PickerVariant,
    SelectionMode,
  } from "./types";

  interface Props {
    title?: string;
    description?: string | null;
    items?: PickerItem[];
    selectedItems?: PickerItem[];
    selectedIds?: string[];
    query?: string | undefined;
    selectionMode?: SelectionMode;
    variant?: PickerVariant;
    state?: BrowseState;
    ariaLabel?: string | null;
    searchPlaceholder?: string;
    filters?: PickerFilterConfig[];
    filterValues?: Record<string, string | undefined>;
    stateTitle?: string | null;
    stateMessage?: string | null;
    confirmLabel?: string;
    cancelLabel?: string;
    footerNote?: string | null;
    showFooter?: boolean;
    showSelectionSummary?: boolean;
    drillDown?: DrillDownConfig | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onQueryChange?: ((value: string) => void) | undefined;
    onSelectionChange?: ((selectedIds: string[]) => void) | undefined;
    onFilterChange?: ((key: string, value: string | undefined) => void) | undefined;
    onConfirm?: ((selectedIds: string[]) => void) | undefined;
    onCancel?: (() => void) | undefined;
    onDrillContext?: ((context: DrillDownContext) => void) | undefined;
    renderItem?: Snippet<[item: PickerItem, selected: boolean]>;
    stateContent?: Snippet<[]>;
  }

  let {
    title = "Select items",
    description = null,
    items = [],
    selectedItems: providedSelectedItems = [],
    selectedIds = $bindable<string[] | undefined>(undefined),
    query = $bindable<string | undefined>(undefined),
    selectionMode = "multiple",
    variant = "inline",
    state: browseState = "ready",
    ariaLabel = null,
    searchPlaceholder = "Search picker results",
    filters = [],
    filterValues = {},
    stateTitle = null,
    stateMessage = null,
    confirmLabel = "Confirm selection",
    cancelLabel = "Cancel",
    footerNote = null,
    showFooter = true,
    showSelectionSummary = true,
    drillDown = null,
    size = null,
    sizeRole = "control",
    density = null,
    onQueryChange = undefined,
    onSelectionChange = undefined,
    onFilterChange = undefined,
    onConfirm = undefined,
    onCancel = undefined,
    onDrillContext = undefined,
    renderItem,
    stateContent,
  }: Props = $props();

  const statusId = "relation-picker-status";
  let candidateButtons = $state<Array<HTMLButtonElement | null>>([]);
  let selectedItemLabels = $state<Record<string, string>>({});
  let uncontrolledQuery = $state("");
  let seededUncontrolledQuery = $state(false);
  let uncontrolledSelectedIds = $state<string[]>([]);
  let seededUncontrolledSelectedIds = $state(false);
  const uiPresentation = getUiPresentation();

  $effect.pre(() => {
    if (seededUncontrolledQuery) {
      return;
    }

    uncontrolledQuery = query ?? "";
    seededUncontrolledQuery = true;
  });

  $effect.pre(() => {
    if (seededUncontrolledSelectedIds) {
      return;
    }

    uncontrolledSelectedIds = selectedIds ? [...selectedIds] : [];
    seededUncontrolledSelectedIds = true;
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasQueryProp = $derived(query !== undefined);
  const hasSelectedIdsProp = $derived(selectedIds !== undefined);
  const currentSelectedIds = $derived(hasSelectedIdsProp ? selectedIds ?? [] : uncontrolledSelectedIds);
  const currentQuery = $derived(hasQueryProp ? query ?? "" : uncontrolledQuery);
  const toolbarSnippet = $derived(toolbarContent as unknown as Snippet<[]>);
  const selectionSnippet = $derived(selectionContent as unknown as Snippet<[]>);
  const footerSnippet = $derived(footerContent as unknown as Snippet<[]>);

  // Drill-down state
  let drillDepth = $state(0);
  let drillSelections = $state<Record<string, DrillDownItem>>({});
  let drillSearchQuery = $state("");
  let drillItems = $state<DrillDownItem[]>([]);
  let drillLoading = $state(false);
  let finalItemsLoaded = $state<PickerItem[] | null>(null);
  let finalItemsLoading = $state(false);
  let lastDrillContextKey = "";

  const isDrilling = $derived(drillDown !== null && drillDepth < (drillDown?.levels.length ?? 0));
  const hasDrillCompleted = $derived(drillDown !== null && drillDepth >= (drillDown?.levels.length ?? 0) && Object.keys(drillSelections).length > 0);
  const currentLevel = $derived(drillDown?.levels[drillDepth] ?? null);

  // Load final items when drill-down completes
  $effect(() => {
    if (hasDrillCompleted && drillDown?.finalItems) {
      loadFinalItems(drillDown.finalItems, drillContext, currentQuery);
    }
  });

  const drillContext = $derived(buildDrillContext());

  $effect(() => {
    const nextKey = JSON.stringify(drillContext);
    if (nextKey === lastDrillContextKey) {
      return;
    }

    lastDrillContextKey = nextKey;
    onDrillContext?.(drillContext);
  });

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

  function buildDrillContext(): DrillDownContext {
    return Object.fromEntries(
      Object.entries(drillSelections).map(([key, item]) => [key, item.id]),
    ) as DrillDownContext;
  }

  function drillSelect(item: DrillDownItem): void {
    if (!drillDown) return;
    drillSelections[currentLevel!.key] = item;
    drillSelections = drillSelections;
    drillSearchQuery = "";
    drillDepth++;
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

  function handleDrillSearchKeydown(keyboardEvent: KeyboardEvent): void {

    if (keyboardEvent.key === "Escape" || (keyboardEvent.key === "Backspace" && !drillSearchQuery)) {
      if (drillDepth > 0) {
        keyboardEvent.preventDefault();
        drillBack();
      }
    }
  }

  // Flat picker logic — use finalItemsLoaded when drill-down provides items
  const activeItems = $derived((hasDrillCompleted && finalItemsLoaded !== null) ? finalItemsLoaded : items);

  $effect(() => {
    if (activeItems.length === 0 && providedSelectedItems.length === 0) {
      return;
    }

    const nextLabels = { ...selectedItemLabels };
    let changed = false;

    for (const item of [...activeItems, ...providedSelectedItems]) {
      if (nextLabels[item.id] !== item.label) {
        nextLabels[item.id] = item.label;
        changed = true;
      }
    }

    if (changed) {
      selectedItemLabels = nextLabels;
    }
  });

  const filteredItems = $derived(activeItems.filter((item) =>
    currentQuery.trim().length === 0
      ? true
      : [item.label, item.description ?? "", item.meta ?? ""].some((value) =>
          value.toLowerCase().includes(currentQuery.trim().toLowerCase()),
        ),
  ));
  const selectedItems = $derived(currentSelectedIds.map((id) => ({
    id,
    label: selectedItemLabels[id] ?? id,
  })));
  const shellState = $derived(
    isDrilling
      ? drillLoading ? "loading" : "ready"
      : finalItemsLoading
        ? "loading"
        : browseState,
  );
  const pickerStatusText = $derived(
    shellState === "loading"
      ? "Picker results are loading."
      : shellState === "error"
        ? "Picker results are unavailable."
        : shellState === "empty"
          ? "No candidates are available."
          : shellState === "no-results"
            ? `No candidates match "${currentQuery}".`
            : `${filteredItems.length} candidate${filteredItems.length === 1 ? "" : "s"} available, ${currentSelectedIds.length} selected.`
  );

  function setSelection(nextIds: string[]): void {
    if (hasSelectedIdsProp) {
      selectedIds = nextIds;
    } else {
      uncontrolledSelectedIds = nextIds;
    }

    onSelectionChange?.(nextIds);
  }

  function setQuery(nextQuery: string): void {
    if (hasQueryProp) {
      query = nextQuery;
    } else {
      uncontrolledQuery = nextQuery;
    }

    onQueryChange?.(nextQuery);
  }

  function toggleItem(id: string): void {
    const item = activeItems.find((candidate) => candidate.id === id);
    if (item?.disabled) {
      return;
    }

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

  function getFilterOptions(filter: PickerFilterConfig) {
    return [
      ...(filter.includeAll === false
        ? []
        : [{
            value: "__all__",
            label: filter.allLabel ?? "All",
          }]),
      ...filter.options.map((option) => ({
        value: option.id,
        label: option.label,
      })),
    ];
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
      placeholder={searchPlaceholder}
      describedBy={statusId}
      onValueChange={setQuery}
      onClear={() => setQuery("")}
    />
    {#if filters.length > 0}
      <div class="poodle-relation-picker__filters">
        {#each filters as filter (filter.key)}
          <Select
            value={filterValues[filter.key] ?? "__all__"}
            options={getFilterOptions(filter)}
            ariaLabel={`${filter.label} filter`}
            size={resolvedSize}
            density={resolvedDensity}
            onValueChange={(value) => {
              onFilterChange?.(filter.key, value === "__all__" ? undefined : value);
            }}
          />
        {/each}
      </div>
    {/if}
  {/if}
{/snippet}

{#snippet selectionContent()}
  {#if showSelectionSummary && currentSelectedIds.length > 0}
    <SelectionSummary
      items={selectedItems}
      onRemove={(id) => setSelection(currentSelectedIds.filter((selectedId) => selectedId !== id))}
      onClear={() => setSelection([])}
    />
  {/if}
{/snippet}

{#snippet footerContent()}
  {#if showFooter}
    <FormActions align="start">
    {#if footerNote !== null}
      <p class="poodle-relation-picker__footer-note">
        {footerNote}
      </p>
    {/if}
    <div class="poodle-relation-picker__footer-actions">
      <Button variant="ghost" size={resolvedSize} onClick={() => onCancel?.()}>
        {cancelLabel}
      </Button>
      <Button variant="primary" size={resolvedSize} onClick={() => onConfirm?.(currentSelectedIds)}>
        {confirmLabel}
      </Button>
    </div>
  </FormActions>
  {/if}
{/snippet}

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div class="poodle-relation-picker" data-size={resolvedSize} data-density={resolvedDensity} data-variant={variant}>
    <PickerShell
      {title}
      {description}
      {variant}
      state={shellState}
      {ariaLabel}
      resultCount={isDrilling ? drillItems.length : filteredItems.length}
      selectionCount={currentSelectedIds.length}
      statusText={isDrilling ? `${drillItems.length} item${drillItems.length === 1 ? "" : "s"}` : pickerStatusText}
      statusId={statusId}
      stateTitle={stateTitle ?? (isDrilling ? "Loading" : (browseState === "loading" ? "Loading candidates" : browseState === "error" ? "Picker unavailable" : browseState === "empty" ? "No candidates available" : "No matching candidates"))}
      stateMessage={stateMessage ?? (isDrilling ? "Loading items..." : (browseState === "loading" ? "Picker results are loading while selection state stays host-owned." : browseState === "error" ? "Error handling remains host-owned, but the picker preserves its structure." : browseState === "empty" ? "This relation has no available candidates yet." : "Try widening the search query or clearing selection filters."))}
      toolbar={toolbarSnippet}
      selection={selectionSnippet}
      stateContent={stateContent}
      footer={showFooter ? footerSnippet : undefined}
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
              data-selection-mode={selectionMode}
              data-selected={currentSelectedIds.includes(item.id)}
              data-disabled={item.disabled}
            >
              {#if selectionMode === "multiple"}
                <Checkbox
                  ariaLabel={`Select ${item.label}`}
                  checked={currentSelectedIds.includes(item.id)}
                  disabled={item.disabled}
                  onCheckedChange={() => toggleItem(item.id)}
                />
                <button
                  bind:this={candidateButtons[index]}
                  type="button"
                  class="poodle-relation-picker__item-button"
                  aria-pressed={currentSelectedIds.includes(item.id)}
                  aria-disabled={item.disabled}
                  aria-describedby={item.description || item.meta ? `relation-picker-item-${item.id}` : undefined}
                  onclick={() => toggleItem(item.id)}
                  onkeydown={(event) => handleCandidateKeydown(event, index)}
                  disabled={item.disabled}
                >
                  {#if renderItem}
                    {@render renderItem(item, currentSelectedIds.includes(item.id))}
                  {:else}
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
                  {/if}
                </button>
              {:else}
                <button
                  bind:this={candidateButtons[index]}
                  type="button"
                  class="poodle-relation-picker__item-button"
                  aria-pressed={currentSelectedIds.includes(item.id)}
                  aria-disabled={item.disabled}
                  aria-describedby={item.description || item.meta ? `relation-picker-item-${item.id}` : undefined}
                  onclick={() => toggleItem(item.id)}
                  onkeydown={(event) => handleCandidateKeydown(event, index)}
                  disabled={item.disabled}
                >
                  {#if renderItem}
                    {@render renderItem(item, currentSelectedIds.includes(item.id))}
                  {:else}
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
                  {/if}
                </button>
              {/if}
            </li>
          {/each}
        </ul>
      {/if}
    </PickerShell>
  </div>
</UiPresentationProvider>


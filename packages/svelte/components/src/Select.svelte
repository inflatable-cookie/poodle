<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/select.css";
  import {
    filterSelectGroups,
    flattenSelectOptions,
    isSelectOptionDisabled,
    layerContains,
    registerDismissLayer,
    selectTransition,
    type SelectContext,
    type SelectEvent,
    type SelectOptionState,
    type SelectResult,
  } from "@inflatable-cookie/poodle-core";
  import type { Snippet } from "svelte";

  import { anchored } from "./anchored";
  import { default as Icon } from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    OverlayPlacement,
    SemanticControlSizeRole,
    SelectEmptyRenderState,
    SelectLoadOptions,
    SelectItems,
    SelectOptionRenderState,
    SelectOption,
    SelectOptionGroup,
    SelectTriggerRenderState,
    ValidationState,
  } from "./types";

  interface Props {
    id?: string;
    name?: string;
    value?: string | null;
    defaultValue?: string | null;
    options?: SelectItems;
    loadOptions?: SelectLoadOptions | null;
    loadKey?: string | null;
    valueLabel?: string | null;
    placeholder?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    disabled?: boolean;
    required?: boolean;
    validationState?: ValidationState;
    clearable?: boolean;
    searchable?: boolean;
    freeform?: boolean;
    native?: boolean;
    emptyMessage?: string;
    variant?: "default" | "ghost";
    menuMinWidth?: string | null;
    dismissOnOutsideInteract?: boolean;
    ariaLabel?: string | null;
    describedBy?: string | null;
    onValueChange?: ((value: string) => void) | undefined;
    onQueryChange?: ((query: string) => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
    trigger?: Snippet<[SelectTriggerRenderState]>;
    option?: Snippet<[SelectOptionRenderState]>;
    empty?: Snippet<[SelectEmptyRenderState]>;
  }

  let {
    id = undefined,
    name = undefined,
    value = $bindable<string | null | undefined>(undefined),
    defaultValue = null,
    options = [],
    loadOptions = null,
    loadKey = null,
    valueLabel = null,
    placeholder = null,
    size = null,
    sizeRole = "control",
    density = null,
    disabled = false,
    required = false,
    validationState = "none",
    clearable = false,
    searchable = false,
    freeform = false,
    native = undefined,
    emptyMessage = "No matches",
    variant = "default",
    menuMinWidth = null,
    dismissOnOutsideInteract = true,
    ariaLabel = null,
    describedBy = null,
    onValueChange = undefined,
    onQueryChange = undefined,
    onOpenChange = undefined,
    trigger: triggerSnippet = undefined,
    option: optionSnippet = undefined,
    empty: emptySnippet = undefined,
  }: Props = $props();

  const generatedSelectId = `poodle-select-${crypto.randomUUID()}`;
  const uiPresentation = getUiPresentation();
  let rootElement: HTMLDivElement | null = $state(null);
  let listboxElement: HTMLDivElement | null = $state(null);
  let inputElement: HTMLInputElement | null = $state(null);
  let open = $state(false);
  let query = $state("");
  let highlightedValue = $state<string | null>(null);
  let skipBlurCommit = $state(false);
  // Reported by the anchored action once the listbox is measured; the classes
  // below only need the side and the alignment, not the full placement.
  let resolvedPlacement = $state<OverlayPlacement>("bottom-start");
  const placement = $derived<"below" | "above">(
    resolvedPlacement.startsWith("top") ? "above" : "below",
  );
  const alignEnd = $derived(resolvedPlacement.endsWith("-end"));
  let loadedOptions: SelectItems | null = $state(null);
  let loadState: "idle" | "loading" | "loaded" | "error" = $state("idle");
  let loadError: string | null = $state(null);
  let lastLoadKey: string | null = $state(null);
  let activeLoadRequestId = 0;
  let uncontrolledValue = $state("");
  let uncontrolledValueSeeded = $state(false);

  const selectId = $derived(id ?? generatedSelectId);
  const listboxId = $derived(`${selectId}-listbox`);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const currentValue = $derived(
    value !== undefined ? (value ?? "") : uncontrolledValue
  );
  const useCustom = $derived(
    native === true ? false : native === false ? true : searchable || Boolean(optionSnippet) || Boolean(triggerSnippet)
  );
  const isLazy = $derived(Boolean(loadOptions));
  const clearValue = $derived(defaultValue ?? "");
  const placeholderValue = $derived(clearable ? clearValue : "");
  const placeholderLabel = $derived(placeholder ?? (clearable ? valueLabel ?? "All" : null));
  const normalizedOptions = $derived(loadedOptions ?? options);
  const flatOptions = $derived(flattenOptions(normalizedOptions));
  const hasPlaceholderOption = $derived(flatOptions.some((entry) => entry.value === placeholderValue));
  const hasCurrentOption = $derived(flatOptions.some((entry) => entry.value === currentValue));
  const hasSelection = $derived(currentValue !== "" && currentValue !== placeholderValue);
  const showClear = $derived(clearable && hasSelection && !disabled);
  const isGrouped = $derived(normalizedOptions.length > 0 && "options" in normalizedOptions[0]);
  const normalizedGroups = $derived(isGrouped ? (normalizedOptions as SelectOptionGroup[]) : []);
  const selectedOption = $derived(flatOptions.find((entry) => entry.value === currentValue) ?? null);
  const filteredOptions = $derived(
    searchable && query.length > 0
      ? flatOptions.filter((entry) => entry.label.toLowerCase().includes(query.toLowerCase()))
      : flatOptions
  );
  const filteredGroups = $derived(
    searchable && query.length > 0 ? filterGroups(normalizedOptions, query) : normalizedOptions
  );
  const visibleGroups = $derived(isGrouped ? (filteredGroups as SelectOptionGroup[]) : []);
  const highlightedOptionIndex = $derived(
    highlightedValue === null ? -1 : filteredOptions.findIndex((entry) => entry.value === highlightedValue)
  );
  const highlightedOptionId = $derived(
    open && highlightedOptionIndex >= 0
      ? `${listboxId}-option-${highlightedOptionIndex}`
      : undefined
  );

  // ── Helpers ──────────────────────────────────────────────────

  function flattenOptions(source: SelectItems): SelectOption[] {
    return flattenSelectOptions(source as (SelectOption | SelectOptionGroup)[]) as SelectOption[];
  }

  function filterGroups(source: SelectItems, q: string): SelectItems {
    if (!isGrouped) return source;
    return filterSelectGroups(source as SelectOptionGroup[], q) as SelectItems;
  }

  function isOptionDisabled(o: SelectOption): boolean {
    return isSelectOptionDisabled(o);
  }

  function optionStates(source: SelectItems): SelectOptionState[] {
    return flattenOptions(source).map((option) => ({
      value: option.value,
      label: option.label,
      disabled: isOptionDisabled(option),
    }));
  }

  async function startLoad(nextQuery = query): Promise<void> {
    const requestId = ++activeLoadRequestId;
    loadState = "loading";
    loadError = null;
    try {
      const nextOptions = loadOptions
        ? await loadOptions({
            query: nextQuery.trim() || undefined,
            value: currentValue || null,
            loadKey
          })
        : [];
      if (requestId !== activeLoadRequestId) return;
      loadedOptions = nextOptions;
      loadState = "loaded";
      if (open) {
        dispatch({ type: "OPTIONS_CHANGED", options: optionStates(nextOptions) });
      }
    } catch (error) {
      if (requestId !== activeLoadRequestId) return;
      loadState = "error";
      loadError = error instanceof Error ? error.message : "Failed to load options";
    }
  }

  // ── Native mode handlers ─────────────────────────────────────

  function machineContext(): SelectContext {
    return {
      value: currentValue,
      open,
      query,
      highlightedValue,
      options: flatOptions.map((option) => ({
        value: option.value,
        label: option.label,
        disabled: isOptionDisabled(option),
      })),
      clearValue,
      searchable,
      freeform,
      disabled,
    };
  }

  function applyResult(result: SelectResult): SelectContext {
    open = result.context.open;
    query = result.context.query;
    highlightedValue = result.context.highlightedValue;

    for (const effect of result.effects) {
      if (effect.type === "openChanged") {
        onOpenChange?.(effect.open);
      } else if (effect.type === "queryChanged") {
        if (useCustom && searchable) {
          onQueryChange?.(effect.query);
        }
      } else if (effect.type === "valueChanged") {
        if (value !== undefined) {
          value = effect.value;
        } else {
          uncontrolledValue = effect.value;
        }
        onValueChange?.(effect.value);
      }
    }

    return result.context;
  }

  function dispatch(event: SelectEvent, from = machineContext()): SelectContext {
    return applyResult(selectTransition(from, event));
  }

  function handleNativeChange(event: Event): void {
    const nextValue = (event.currentTarget as HTMLSelectElement).value;
    const option = flatOptions.find((entry) => entry.value === nextValue);

    if (option) {
      dispatch({ type: "COMMIT_OPTION", value: nextValue });
    } else if (nextValue === clearValue) {
      dispatch({ type: "CLEAR" });
    }
  }

  // ── Custom mode handlers ─────────────────────────────────────

  function selectOption(option: SelectOption): void {
    dispatch({ type: "COMMIT_OPTION", value: option.value });
  }

  function handleTriggerClick(): void {
    dispatch({ type: "TOGGLE" });
  }

  function handleSearchableIndicatorClick(event: MouseEvent): void {
    event.stopPropagation();
    dispatch({ type: "TOGGLE" });
    inputElement?.focus();
  }

  function handleInputInput(event: Event): void {
    const nextQuery = (event.currentTarget as HTMLInputElement).value;
    dispatch({ type: "QUERY", query: nextQuery });

    if (isLazy) {
      void startLoad(nextQuery);
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === "ArrowDown") {
      event.preventDefault();
      dispatch({ type: "HIGHLIGHT_NEXT" });
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      dispatch({ type: "HIGHLIGHT_PREV" });
    }

    if (event.key === "Enter" && open) {
      event.preventDefault();
      dispatch({ type: "COMMIT_HIGHLIGHTED" });
    }

    if (event.key === "Escape" && open) {
      event.preventDefault();
      dispatch({ type: "CLOSE" });
    }

    if (event.key === "Tab" && open) {
      skipBlurCommit = true;
      dispatch({ type: "CLOSE" });
    }

    if (event.key === "Home" && open) {
      event.preventDefault();
      dispatch({ type: "HIGHLIGHT_FIRST" });
    }

    if (event.key === "End" && open) {
      event.preventDefault();
      dispatch({ type: "HIGHLIGHT_LAST" });
    }
  }

  function handleClear(event: MouseEvent): void {
    event.stopPropagation();
    dispatch({ type: "CLEAR" });
    if (isLazy) {
      void startLoad("");
    }
  }

  function handleControlFocusOut(event: FocusEvent): void {
    if (
      event.relatedTarget instanceof Node &&
      layerContains(event.relatedTarget, rootElement, listboxElement)
    ) {
      return;
    }

    if (skipBlurCommit) {
      skipBlurCommit = false;
      if (open) {
        dispatch({ type: "CLOSE" });
      }
      return;
    }

    const afterCommit = dispatch({ type: "COMMIT_FREEFORM" });
    if (afterCommit.open) {
      dispatch({ type: "CLOSE" }, afterCommit);
    }
  }

  function handleOptionPointerDown(event: MouseEvent): void {
    event.preventDefault();
  }

  function handleOptionEnter(option: SelectOption): void {
    dispatch({ type: "HIGHLIGHT", value: option.value });
  }

  $effect(() => {
    if (!uncontrolledValueSeeded) {
      uncontrolledValue = defaultValue ?? "";
      uncontrolledValueSeeded = true;
    }
  });

  $effect(() => {
    if (!open && !freeform) {
      query = hasSelection ? (selectedOption?.label ?? "") : "";
    }
  });

  $effect(() => {
    if (loadKey !== lastLoadKey) {
      activeLoadRequestId += 1;
      lastLoadKey = loadKey;
      loadedOptions = null;
      loadState = "idle";
      loadError = null;
    }
  });

  $effect(() => {
    if (isLazy && loadState === "idle") {
      void startLoad();
    }
  });

  $effect(() => {
    if (!open) {
      return;
    }

    return registerDismissLayer({
      // The listbox is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target, rootElement, listboxElement),
      dismissOnOutsideInteract,
      onDismiss: () => {
        skipBlurCommit = true;
        dispatch({ type: "CLOSE" });
      },
      // Host-aware so a parent composite that registers around this Select
      // (child effects can run first) still becomes the parent layer.
      hostElement: rootElement,
    });
  });
</script>

{#if useCustom}
  <!-- ═══ CUSTOM MODE ═══ -->
  <div
    bind:this={rootElement}
    class="poodle-select poodle-select--custom"
    data-open={open}
    data-placeholder={!hasSelection}
    data-variant={variant}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-validation-state={validationState}
    data-has-clear={showClear}
    aria-invalid={validationState === "invalid" ? "true" : undefined}
    onfocusout={handleControlFocusOut}
  >
    {#if searchable}
      <!-- Searchable: text input trigger -->
      <div
        class="poodle-select__trigger-area"
        role="combobox"
        tabindex="-1"
        aria-expanded={open ? "true" : "false"}
        aria-haspopup="listbox"
        aria-controls={open ? listboxId : undefined}
        aria-label={ariaLabel ?? undefined}
      >
        <input
          id={selectId}
          bind:this={inputElement}
          class="poodle-select__input"
          type="text"
          value={query}
          {disabled}
          placeholder={placeholder ?? undefined}
          aria-autocomplete="list"
          aria-activedescendant={highlightedOptionId}
          aria-describedby={describedBy ?? undefined}
          onfocus={() => { if (!open) dispatch({ type: "OPEN" }); }}
          oninput={handleInputInput}
          onkeydown={handleKeydown}
        />
        {#if showClear}
          <button
            type="button"
            class="poodle-select__clear"
            aria-label="Clear selection"
            onclick={handleClear}
          >
            <Icon name="x" size="xs" />
          </button>
        {/if}
        <button
          type="button"
          class="poodle-select__indicator-button"
          tabindex="-1"
          aria-hidden="true"
          aria-label={open ? "Close options" : "Open options"}
          onclick={handleSearchableIndicatorClick}
        >
          <Icon name="chevron-down" />
        </button>
      </div>
    {:else}
      <!-- Non-searchable: button trigger -->
      <div class="poodle-select__trigger-area">
        <button
          type="button"
          class="poodle-select__trigger"
          id={selectId}
          {disabled}
          aria-expanded={open ? "true" : "false"}
          aria-haspopup="listbox"
          aria-controls={open ? listboxId : undefined}
          aria-label={ariaLabel ?? undefined}
          aria-describedby={describedBy ?? undefined}
          onclick={handleTriggerClick}
          onkeydown={handleKeydown}
        >
          <span class="poodle-select__trigger-content">
            {#if triggerSnippet}
              {@render triggerSnippet({ selectedOption, open, placeholder })}
            {:else}
              <span class="poodle-select__value" data-placeholder={!hasSelection}>
                {hasSelection ? (selectedOption?.label ?? "") : (placeholder ?? selectedOption?.label ?? "")}
              </span>
            {/if}
          </span>
        </button>
        {#if showClear}
          <button
            type="button"
            class="poodle-select__clear"
            aria-label="Clear selection"
            onclick={handleClear}
          >
            <Icon name="x" size="xs" />
          </button>
        {/if}
        <button
          type="button"
          class="poodle-select__indicator-button"
          tabindex="-1"
          aria-hidden="true"
          aria-label={open ? "Close options" : "Open options"}
          onclick={handleTriggerClick}
        >
          <Icon name="chevron-down" />
        </button>
      </div>
    {/if}

    <!-- Hidden input for form submission -->
    {#if name}
      <input type="hidden" {name} value={currentValue} />
    {/if}

    <!-- Dropdown listbox -->
    {#if open}
      <div
        bind:this={listboxElement}
        use:anchored={{
          anchor: rootElement,
          placement: "bottom-start",
          // Ghost triggers sit tighter to their menu than bordered ones.
          offset: variant === "ghost" ? 6 : 4,
          // A fixed min-width means the listbox sizes to its content; without
          // one it tracks the trigger exactly, as the old absolute inset did.
          matchWidth: !menuMinWidth,
          onPlacement: (next) => (resolvedPlacement = next),
        }}
        id={listboxId}
        class="poodle-select__listbox"
        class:poodle-select__listbox--above={placement === "above"}
        class:poodle-select__listbox--auto-width={!!menuMinWidth}
        class:poodle-select__listbox--align-end={alignEnd}
        data-variant={variant}
        data-size={resolvedSize}
        data-density={resolvedDensity}
        role="listbox"
        aria-label={ariaLabel ?? undefined}
        style={menuMinWidth ? `min-width: ${menuMinWidth}` : undefined}
      >
        {#if isGrouped && !searchable}
          {#each normalizedGroups as group}
            {#if group.options.length > 0}
              <div class="poodle-select__group" role="group" aria-label={group.label || undefined}>
                {#if group.label}
                  <div class="poodle-select__group-label">{group.label}</div>
                {/if}
                {#each group.options as option, idx}
                  {@const flatIdx = flatOptions.indexOf(option)}
                  <button
                    type="button"
                  class="poodle-select__option"
                  tabindex="-1"
                  id={`${listboxId}-option-${flatIdx}`}
                  data-value={option.value}
                  role="option"
                  aria-selected={currentValue === option.value ? "true" : "false"}
                  data-highlighted={highlightedValue === option.value}
                  disabled={isOptionDisabled(option)}
                    onmousedown={handleOptionPointerDown}
                    onmouseenter={() => handleOptionEnter(option)}
                    onclick={() => selectOption(option)}
                  >
                    {#if optionSnippet}
                      <span class="poodle-select__option-content">
                        {@render optionSnippet({
                          option,
                          highlighted: highlightedValue === option.value,
                          selected: currentValue === option.value,
                          index: flatIdx,
                        })}
                      </span>
                    {:else}
                      {#if option.icon}
                        <span class="poodle-select__option-icon"><Icon icon={option.icon} size="sm" /></span>
                      {/if}
                      <span class="poodle-select__option-content">
                        <span class="poodle-select__option-body">
                          <span class="poodle-select__option-label">{option.label}</span>
                          {#if option.description}
                            <span class="poodle-select__option-description">{option.description}</span>
                          {/if}
                        </span>
                      </span>
                    {/if}
                  </button>
                {/each}
              </div>
            {/if}
          {/each}
        {:else if isGrouped && searchable}
          {#each visibleGroups as group}
            {#if group.options.length > 0}
              <div class="poodle-select__group" role="group" aria-label={group.label || undefined}>
                {#if group.label}
                  <div class="poodle-select__group-label">{group.label}</div>
                {/if}
                {#each group.options as option}
                  {@const flatIdx = filteredOptions.indexOf(option)}
                  <button
                    type="button"
                  class="poodle-select__option"
                  tabindex="-1"
                  id={`${listboxId}-option-${flatIdx}`}
                  data-value={option.value}
                  role="option"
                  aria-selected={currentValue === option.value ? "true" : "false"}
                  data-highlighted={highlightedValue === option.value}
                  disabled={isOptionDisabled(option)}
                    onmousedown={handleOptionPointerDown}
                    onmouseenter={() => handleOptionEnter(option)}
                    onclick={() => selectOption(option)}
                  >
                    {#if optionSnippet}
                      <span class="poodle-select__option-content">
                        {@render optionSnippet({
                          option,
                          highlighted: highlightedValue === option.value,
                          selected: currentValue === option.value,
                          index: flatIdx,
                        })}
                      </span>
                    {:else}
                      {#if option.icon}
                        <span class="poodle-select__option-icon"><Icon icon={option.icon} size="sm" /></span>
                      {/if}
                      <span class="poodle-select__option-content">
                        <span class="poodle-select__option-body">
                          <span class="poodle-select__option-label">{option.label}</span>
                          {#if option.description}
                            <span class="poodle-select__option-description">{option.description}</span>
                          {/if}
                        </span>
                      </span>
                    {/if}
                  </button>
                {/each}
              </div>
            {/if}
          {/each}
        {:else}
          {#each filteredOptions as option, index (option.value)}
            <button
              type="button"
              class="poodle-select__option"
              tabindex="-1"
              id={`${listboxId}-option-${index}`}
              data-value={option.value}
              role="option"
              aria-selected={currentValue === option.value ? "true" : "false"}
              data-highlighted={highlightedValue === option.value}
              disabled={isOptionDisabled(option)}
              onmousedown={handleOptionPointerDown}
              onmouseenter={() => handleOptionEnter(option)}
              onclick={() => selectOption(option)}
            >
              {#if optionSnippet}
                <span class="poodle-select__option-content">
                  {@render optionSnippet({
                    option,
                    highlighted: highlightedValue === option.value,
                    selected: currentValue === option.value,
                    index,
                  })}
                </span>
              {:else}
                {#if option.icon}
                  <span class="poodle-select__option-icon"><Icon icon={option.icon} size="sm" /></span>
                {/if}
                <span class="poodle-select__option-content">
                  <span class="poodle-select__option-body">
                    <span class="poodle-select__option-label">{option.label}</span>
                    {#if option.description}
                      <span class="poodle-select__option-description">{option.description}</span>
                    {/if}
                  </span>
                </span>
              {/if}
            </button>
          {/each}
        {/if}

        {#if filteredOptions.length === 0}
          {#if emptySnippet}
            {@render emptySnippet({ query })}
          {:else}
            <div class="poodle-select__empty">{emptyMessage}</div>
          {/if}
        {/if}
      </div>
    {/if}
  </div>

{:else}
  <!-- ═══ NATIVE MODE ═══ -->
  <div class="poodle-select" data-placeholder={!hasSelection} data-variant={variant} data-size={resolvedSize} data-density={resolvedDensity} data-validation-state={validationState}>
    <select
      id={selectId}
      {name}
      class="poodle-select__control"
      value={currentValue}
      {disabled}
      {required}
      aria-label={ariaLabel ?? undefined}
      aria-describedby={describedBy ?? undefined}
      aria-invalid={validationState === "invalid" ? "true" : undefined}
      onchange={handleNativeChange}
    >
      {#if placeholderLabel && !hasPlaceholderOption}
        <option value={placeholderValue} disabled={!clearable && required}>{placeholderLabel}</option>
      {/if}

      {#if isGrouped}
        {#each normalizedGroups as group}
          {#if group.label.trim().length === 0}
            {#each group.options as option (option.value)}
              <option value={option.value} disabled={isOptionDisabled(option)}>
                {option.label}
              </option>
            {/each}
          {:else}
            <optgroup label={group.label}>
              {#each group.options as option (option.value)}
                <option value={option.value} disabled={isOptionDisabled(option)}>
                  {option.label}
                </option>
              {/each}
            </optgroup>
          {/if}
        {/each}
      {:else if flatOptions.length > 0}
        {#each flatOptions as option}
          <option value={option.value} disabled={isOptionDisabled(option)}>
            {option.label}
          </option>
        {/each}
      {:else if isLazy && currentValue && valueLabel}
        <option value={currentValue}>{valueLabel}</option>
      {:else if isLazy && loadState === "loading"}
        <option value={placeholderValue} disabled>Loading…</option>
      {:else if isLazy && loadState === "error"}
        <option value={placeholderValue} disabled>{loadError ?? "Failed to load options"}</option>
      {:else if currentValue && !hasCurrentOption && valueLabel}
        <option value={currentValue}>{valueLabel}</option>
      {:else if currentValue && !hasCurrentOption}
        <option value={currentValue}>{currentValue}</option>
      {/if}
    </select>

    <span class="poodle-select__indicator" aria-hidden="true"><Icon name="chevron-down" /></span>
  </div>
{/if}

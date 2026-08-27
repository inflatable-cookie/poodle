<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/select.css";
  import {
    filterSelectGroups,
    flattenSelectOptions,
    isSelectOptionDisabled,
    layerContains,
    registerDismissLayer,
    selectOpenHighlightIndex,
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
  let highlightIndex = $state(0);
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
      ? flatOptions.filter((entry) => !isOptionDisabled(entry) && entry.label.toLowerCase().includes(query.toLowerCase()))
      : flatOptions.filter((entry) => !isOptionDisabled(entry))
  );
  const filteredGroups = $derived(
    searchable && query.length > 0 ? filterGroups(normalizedOptions, query) : normalizedOptions
  );
  const visibleGroups = $derived(isGrouped ? (filteredGroups as SelectOptionGroup[]) : []);
  const highlightedOptionId = $derived(
    open && filteredOptions.length > 0 && highlightIndex >= 0
      ? `${listboxId}-option-${highlightIndex}`
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
    } catch (error) {
      if (requestId !== activeLoadRequestId) return;
      loadState = "error";
      loadError = error instanceof Error ? error.message : "Failed to load options";
    }
  }

  // ── Native mode handlers ─────────────────────────────────────

  function handleNativeChange(event: Event): void {
    const nextValue = (event.currentTarget as HTMLSelectElement).value;
    commitValue(nextValue);
  }

  // ── Custom mode handlers ─────────────────────────────────────

  function commitValue(nextValue: string): void {
    if (value !== undefined) {
      value = nextValue;
    } else {
      uncontrolledValue = nextValue;
    }
    onValueChange?.(nextValue);
  }

  function setOpen(nextOpen: boolean): void {
    open = nextOpen;
    onOpenChange?.(nextOpen);

    if (nextOpen) {
      highlightIndex = selectOpenHighlightIndex(filteredOptions, selectedOption?.value ?? null);
    }
  }

  function selectOption(option: SelectOption): void {
    if (isOptionDisabled(option)) return;
    query = option.label;
    commitValue(option.value);
    setOpen(false);
  }

  function handleTriggerClick(): void {
    if (disabled) return;
    setOpen(!open);
  }

  function handleSearchableIndicatorClick(event: MouseEvent): void {
    event.stopPropagation();
    if (disabled) return;
    setOpen(!open);
    inputElement?.focus();
  }

  function handleInputInput(event: Event): void {
    query = (event.currentTarget as HTMLInputElement).value;
    highlightIndex = 0;
    if (!open) setOpen(true);
    onQueryChange?.(query);

    if (isLazy) {
      void startLoad(query);
    }

    if (freeform) {
      commitValue(query);
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === "ArrowDown") {
      event.preventDefault();
      if (!open) { setOpen(true); return; }
      highlightIndex = Math.min(highlightIndex + 1, filteredOptions.length - 1);
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      if (!open) { setOpen(true); return; }
      highlightIndex = Math.max(highlightIndex - 1, 0);
    }

    if (event.key === "Enter" && open && filteredOptions[highlightIndex]) {
      event.preventDefault();
      selectOption(filteredOptions[highlightIndex]);
    }

    if (event.key === "Escape" && open) {
      event.preventDefault();
      setOpen(false);
    }

    if (event.key === "Home" && open) {
      event.preventDefault();
      highlightIndex = 0;
    }

    if (event.key === "End" && open) {
      event.preventDefault();
      highlightIndex = Math.max(filteredOptions.length - 1, 0);
    }
  }

  function handleClear(event: MouseEvent): void {
    event.stopPropagation();
    query = "";
    commitValue(clearValue);
    if (isLazy) {
      void startLoad("");
    }
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
    if (filteredOptions.length > 0 && highlightIndex >= filteredOptions.length) {
      highlightIndex = 0;
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
      onDismiss: () => setOpen(false),
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
          onfocus={() => { if (!open) setOpen(true); }}
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
                  id={`${listboxId}-option-${flatIdx}`}
                  data-value={option.value}
                  role="option"
                  aria-selected={currentValue === option.value ? "true" : "false"}
                  data-highlighted={highlightIndex === flatIdx}
                  disabled={isOptionDisabled(option)}
                    onmouseenter={() => (highlightIndex = flatIdx)}
                    onclick={() => selectOption(option)}
                  >
                    {#if optionSnippet}
                      <span class="poodle-select__option-content">
                        {@render optionSnippet({
                          option,
                          highlighted: highlightIndex === flatIdx,
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
                  id={`${listboxId}-option-${flatIdx}`}
                  data-value={option.value}
                  role="option"
                  aria-selected={currentValue === option.value ? "true" : "false"}
                  data-highlighted={highlightIndex === flatIdx}
                  disabled={isOptionDisabled(option)}
                    onmouseenter={() => (highlightIndex = flatIdx)}
                    onclick={() => selectOption(option)}
                  >
                    {#if optionSnippet}
                      <span class="poodle-select__option-content">
                        {@render optionSnippet({
                          option,
                          highlighted: highlightIndex === flatIdx,
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
              id={`${listboxId}-option-${index}`}
              data-value={option.value}
              role="option"
              aria-selected={currentValue === option.value ? "true" : "false"}
              data-highlighted={highlightIndex === index}
              disabled={isOptionDisabled(option)}
              onmouseenter={() => (highlightIndex = index)}
              onclick={() => selectOption(option)}
            >
              {#if optionSnippet}
                <span class="poodle-select__option-content">
                  {@render optionSnippet({
                    option,
                    highlighted: highlightIndex === index,
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


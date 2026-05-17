<script lang="ts">
  import { onMount, type Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
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
  let inputElement: HTMLInputElement | null = $state(null);
  let open = $state(false);
  let query = $state("");
  let highlightIndex = $state(0);
  let placement: "below" | "above" = $state("below");
  let alignEnd = $state(false);
  let loadedOptions: SelectItems | null = $state(null);
  let loadState: "idle" | "loading" | "loaded" | "error" = $state("idle");
  let loadError: string | null = $state(null);
  let lastLoadKey: string | null = $state(null);
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
  const hasCurrentOption = $derived(flatOptions.some((entry) => entry.value === currentValue));
  const hasSelection = $derived(currentValue !== "" && currentValue !== clearValue);
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
    if (source.length === 0) return [];
    if ("options" in source[0]) {
      return (source as SelectOptionGroup[]).flatMap((g) => g.options);
    }
    return source as SelectOption[];
  }

  function filterGroups(source: SelectItems, q: string): SelectItems {
    if (!isGrouped) return source;
    const lq = q.toLowerCase();
    return (source as SelectOptionGroup[])
      .map((g) => ({
        ...g,
        options: g.options.filter((o) => o.label.toLowerCase().includes(lq)),
      }))
      .filter((g) => g.options.length > 0);
  }

  function isOptionDisabled(o: SelectOption): boolean {
    return o.disabled === true || o.isDisabled === true;
  }

  function parseMinWidth(value: string): number {
    const num = parseFloat(value);
    if (value.endsWith("rem")) return num * 16;
    return num;
  }

  async function startLoad(): Promise<void> {
    if (loadState === "loading") return;
    loadState = "loading";
    loadError = null;
    try {
      loadedOptions = loadOptions ? await loadOptions() : [];
      loadState = "loaded";
    } catch (error) {
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
    if (nextOpen && rootElement) {
      const rect = rootElement.getBoundingClientRect();
      const spaceBelow = window.innerHeight - rect.bottom;
      placement = spaceBelow < 280 ? "above" : "below";

      // Horizontal: if menuMinWidth is set, check if left-anchored would overflow
      if (menuMinWidth) {
        const spaceRight = window.innerWidth - rect.left;
        const minW = parseMinWidth(menuMinWidth);
        alignEnd = minW > spaceRight && rect.right > minW;
      } else {
        alignEnd = false;
      }
    }
    open = nextOpen;
    onOpenChange?.(nextOpen);

    if (nextOpen) {
      highlightIndex = selectedOption
        ? filteredOptions.findIndex((entry) => entry.value === selectedOption.value)
        : 0;
      if (highlightIndex < 0) highlightIndex = 0;
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
  }

  $effect(() => {
    if (!uncontrolledValueSeeded) {
      uncontrolledValue = defaultValue ?? "";
      uncontrolledValueSeeded = true;
    }
  });

  $effect(() => {
    if (!open && !freeform) {
      query = selectedOption?.label ?? "";
    }
  });

  $effect(() => {
    if (filteredOptions.length > 0 && highlightIndex >= filteredOptions.length) {
      highlightIndex = 0;
    }
  });

  $effect(() => {
    if (loadKey !== lastLoadKey) {
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

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!open || !rootElement) return;
      if (!rootElement.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    function handleDocKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && open) {
        event.preventDefault();
        setOpen(false);
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleDocKeydown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleDocKeydown);
    };
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
        {#if clearable && hasSelection && !disabled}
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
                {selectedOption?.label ?? placeholder ?? ""}
              </span>
            {/if}
          </span>
        </button>
        {#if clearable && hasSelection && !disabled}
          <button
            type="button"
            class="poodle-select__clear"
            aria-label="Clear selection"
            onclick={handleClear}
          >
            <Icon name="x" size="xs" />
          </button>
        {/if}
        {#if variant !== "ghost"}
          <button
            type="button"
            class="poodle-select__indicator-button"
            aria-label={open ? "Close options" : "Open options"}
            onclick={handleTriggerClick}
          >
            <Icon name="chevron-down" />
          </button>
        {/if}
      </div>
    {/if}

    <!-- Hidden input for form submission -->
    {#if name}
      <input type="hidden" {name} value={currentValue} />
    {/if}

    <!-- Dropdown listbox -->
    {#if open}
      <div
        id={listboxId}
        class="poodle-select__listbox"
        class:poodle-select__listbox--above={placement === "above"}
        class:poodle-select__listbox--auto-width={!!menuMinWidth}
        class:poodle-select__listbox--align-end={alignEnd}
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
  <div class="poodle-select" data-placeholder={!hasSelection} data-size={resolvedSize} data-density={resolvedDensity} data-validation-state={validationState}>
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
      {#if placeholderLabel}
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

<style>
  /* ═══ SHARED STYLES ═══ */

  .poodle-select {
    --poodle-select-inline-padding-size-adjust: 0rem;
    --poodle-select-inline-padding-density-adjust: 0rem;
    --poodle-select-inline-padding: calc(
      var(--poodle-space-control-x)
      + var(--poodle-select-inline-padding-size-adjust)
      + var(--poodle-select-inline-padding-density-adjust)
    );
    position: relative;
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    align-items: center;
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-select-inline-padding);
    border: 0.0625rem solid var(
      --poodle-treatment-interactive-subtle-border,
      var(--poodle-color-border-default)
    );
    border-radius: var(--poodle-treatment-interactive-subtle-radius, var(--poodle-radius-control));
    background: var(--poodle-treatment-interactive-subtle-fill, var(--poodle-color-background-surface));
    box-shadow: var(--poodle-treatment-interactive-subtle-shadow, none);
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-select[data-validation-state="invalid"] {
    border-color: var(--poodle-color-status-danger);
  }

  .poodle-select[data-validation-state="valid"] {
    border-color: var(--poodle-color-status-success);
  }

  .poodle-select[data-validation-state="pending"] {
    border-color: var(--poodle-color-accent-base);
  }

  .poodle-select:focus-within {
    border-color: var(--poodle-color-accent-focusRing);
    background: var(
      --poodle-treatment-interactive-subtle-fill-focus,
      var(--poodle-color-background-surface)
    );
    box-shadow: var(
      --poodle-treatment-interactive-subtle-shadow-focus,
      0 0 0 var(--poodle-border-width-focus)
        color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)
    );
  }

  .poodle-select__indicator {
    position: absolute;
    right: var(--poodle-select-inline-padding);
    top: 50%;
    transform: translateY(-50%);
    color: var(--poodle-color-icon-muted);
    font-size: 0.75rem;
    line-height: 1;
    pointer-events: none;
  }

  /* ═══ NATIVE MODE ═══ */

  .poodle-select:has(.poodle-select__control:disabled) {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-select__control {
    min-width: 0;
    width: 100%;
    height: calc(var(--poodle-size-control-height) - (var(--poodle-border-width-default) * 2));
    padding: 0;
    padding-right: 1.5rem;
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    outline: 0;
    appearance: none;
    cursor: pointer;
  }

  .poodle-select[data-placeholder="true"] .poodle-select__control {
    color: var(--poodle-color-text-secondary);
  }

  .poodle-select__control optgroup { font-weight: 600; color: var(--poodle-color-text-secondary); }
  .poodle-select__control option { font-weight: normal; color: var(--poodle-color-text-primary); }

  /* ═══ CUSTOM MODE ═══ */

  .poodle-select--custom {
    padding: 0;
  }

  .poodle-select--custom:has(:disabled) {
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Trigger area (searchable) */
  .poodle-select__trigger-area {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0 var(--poodle-select-inline-padding);
    min-height: var(--poodle-size-control-height);
  }

  .poodle-select__input {
    flex: 1;
    min-width: 0;
    width: 100%;
    height: var(--poodle-size-control-height);
    padding: 0;
    padding-right: 1.5rem;
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    outline: 0;
  }

  .poodle-select__input::placeholder {
    color: var(--poodle-color-text-secondary);
  }

  /* Trigger button (non-searchable custom) */
  .poodle-select__trigger {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    flex: 1;
    width: 100%;
    min-width: 0;
    min-height: var(--poodle-size-control-height);
    padding: 0;
    padding-right: 1.5rem;
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    text-align: left;
    cursor: pointer;
  }

  .poodle-select__trigger:disabled {
    cursor: not-allowed;
  }

  .poodle-select__trigger-content {
    display: flex;
    align-items: center;
    flex: 1;
    min-width: 0;
  }

  .poodle-select__value {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .poodle-select__value[data-placeholder="true"] {
    color: var(--poodle-color-text-secondary);
  }

  /* Clear button */
  .poodle-select__indicator-button {
    position: absolute;
    right: var(--poodle-select-inline-padding);
    top: 50%;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--poodle-color-icon-muted);
    transform: translateY(-50%);
    cursor: pointer;
  }

  .poodle-select__indicator-button:disabled {
    cursor: not-allowed;
  }

  .poodle-select__clear {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 1.125rem;
    height: 1.125rem;
    padding: 0;
    border: 0;
    border-radius: var(--poodle-radius-pill);
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 18%, transparent);
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
  }

  .poodle-select__clear:hover {
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 28%, transparent);
    color: var(--poodle-color-text-primary);
  }

  /* Listbox dropdown */
  .poodle-select__listbox {
    position: absolute;
    top: calc(100% + 0.25rem);
    left: -0.0625rem;
    right: -0.0625rem;
    z-index: var(--poodle-overlay-z-menu);
    display: grid;
    gap: 0.0625rem;
    max-height: 16rem;
    overflow-y: auto;
    padding: 0.25rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-elevated);
    box-shadow: var(--poodle-treatment-surface-elevated-shadow, var(--poodle-elevation-overlay));
  }

  .poodle-select__listbox--above {
    top: auto;
    bottom: calc(100% + 0.25rem);
  }

  /* Group */
  .poodle-select__group-label {
    padding: 0.375rem 0.5rem 0.1875rem;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .poodle-select__group .poodle-select__option {
    padding-left: 1rem;
  }

  /* Option */
  .poodle-select__option {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    width: 100%;
    padding: 0.375rem 0.5rem;
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-body-size);
    text-align: left;
  }

  .poodle-select__option[data-highlighted="true"],
  .poodle-select__option:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 14%, transparent);
  }

  .poodle-select__option[aria-selected="true"] {
    font-weight: 500;
  }

  .poodle-select__option:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-select__option-icon {
    display: inline-flex;
    flex-shrink: 0;
    margin-top: 0.125rem;
  }

  .poodle-select__option-content {
    display: block;
    flex: 1;
    min-width: 0;
  }

  .poodle-select__option-body {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    min-width: 0;
  }

  .poodle-select__option-label {
    display: block;
    min-width: 0;
  }

  .poodle-select__option-description {
    display: block;
    color: var(--poodle-color-text-secondary);
    font-size: 0.6875rem;
  }

  .poodle-select__empty {
    padding: 0.5rem;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }

  /* ═══ GHOST VARIANT ═══ */

  .poodle-select[data-variant="ghost"] {
    min-height: 0;
    padding: 0;
    border: 0;
    border-radius: 0;
    background: transparent;
    box-shadow: none;
  }

  .poodle-select[data-variant="ghost"]:focus-within {
    border-color: transparent;
    background: transparent;
    box-shadow: none;
  }

  .poodle-select[data-variant="ghost"] .poodle-select__trigger-area {
    padding: 0;
    min-height: 0;
  }

  .poodle-select[data-variant="ghost"] .poodle-select__trigger {
    min-height: 0;
    padding: 0;
    line-height: 1;
  }

  /* Auto-width listbox (used with menuMinWidth) */
  .poodle-select__listbox--auto-width {
    right: auto;
    width: max-content;
  }

  .poodle-select__listbox--align-end {
    left: auto;
    right: -0.0625rem;
  }

  /* ═══ DENSITY VARIANTS ═══ */
  .poodle-select[data-density="compact"] { --poodle-select-inline-padding-density-adjust: -0.125rem; }
  .poodle-select[data-density="comfortable"] { --poodle-select-inline-padding-density-adjust: 0.125rem; }
  .poodle-select--custom[data-density="compact"] { padding: 0; }
  .poodle-select--custom[data-density="comfortable"] { padding: 0; }

  /* ═══ SIZE VARIANTS ═══ */
  .poodle-select[data-size="xs"] { --poodle-select-inline-padding-size-adjust: -0.25rem; min-height: 1.5rem; }
  .poodle-select[data-size="xs"] .poodle-select__control { height: calc(1.5rem - (var(--poodle-border-width-default) * 2)); font-size: 0.75rem; }
  .poodle-select[data-size="xs"] .poodle-select__input,
  .poodle-select[data-size="xs"] .poodle-select__trigger { font-size: 0.75rem; min-height: 1.5rem; }
  .poodle-select--custom[data-size="xs"] { padding: 0; }

  .poodle-select[data-size="sm"] { --poodle-select-inline-padding-size-adjust: -0.125rem; min-height: 1.75rem; }
  .poodle-select[data-size="sm"] .poodle-select__control { height: calc(1.75rem - (var(--poodle-border-width-default) * 2)); }
  .poodle-select[data-size="sm"] .poodle-select__input,
  .poodle-select[data-size="sm"] .poodle-select__trigger { min-height: 1.75rem; }
  .poodle-select--custom[data-size="sm"] { padding: 0; }

  .poodle-select[data-size="md"] { --poodle-select-inline-padding-size-adjust: 0rem; }
  .poodle-select--custom[data-size="md"] { padding: 0; }

  .poodle-select[data-size="lg"] { --poodle-select-inline-padding-size-adjust: 0.125rem; min-height: 2.75rem; }
  .poodle-select[data-size="lg"] .poodle-select__control { height: calc(2.75rem - (var(--poodle-border-width-default) * 2)); font-size: 0.9375rem; }
  .poodle-select[data-size="lg"] .poodle-select__input,
  .poodle-select[data-size="lg"] .poodle-select__trigger { font-size: 0.9375rem; min-height: 2.75rem; }
  .poodle-select--custom[data-size="lg"] { padding: 0; }

  .poodle-select[data-size="xl"] { --poodle-select-inline-padding-size-adjust: 0.1875rem; min-height: 3.25rem; }
  .poodle-select[data-size="xl"] .poodle-select__control { height: calc(3.25rem - (var(--poodle-border-width-default) * 2)); font-size: 1rem; }
  .poodle-select[data-size="xl"] .poodle-select__input,
  .poodle-select[data-size="xl"] .poodle-select__trigger { font-size: 1rem; min-height: 3.25rem; }
  .poodle-select--custom[data-size="xl"] { padding: 0; }
</style>

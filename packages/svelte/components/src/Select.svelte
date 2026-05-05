<script context="module" lang="ts">
  let nextSelectId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import Icon from "./Icon.svelte";
  import { firstEnabledIndex } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    IconProp,
    SemanticControlSizeRole,
    SelectItems,
    SelectLoadOptions,
    SelectOption,
    SelectOptionGroup,
    ValidationState,
  } from "./types";

  // ── Legacy compat types ──────────────────────────────────────
  type LegacySelectItem = {
    value: string;
    label: string;
    disabled?: boolean;
    isDisabled?: boolean;
  };

  type LegacySelectGroup = {
    label: string;
    items?: LegacySelectItem[];
    groups?: LegacySelectGroup[];
  };

  // ── Props ────────────────────────────────────────────────────

  // Identity & form
  export let id: string | undefined = undefined;
  export let name: string | undefined = undefined;

  // Value
  export let value: string | null = null;
  export let defaultValue: string | null = null;

  // Options
  export let options: SelectItems = [];
  /** @deprecated Use `options` with flat or grouped array */
  export let items: LegacySelectItem[] | null = null;
  /** @deprecated Use `options` with SelectOptionGroup[] */
  export let groups: LegacySelectGroup[] | null = null;
  export let loadOptions: SelectLoadOptions | null = null;
  /** @deprecated Use `loadOptions` */
  export let loadItems: (() => Promise<LegacySelectItem[]>) | null = null;
  /** @deprecated Use `loadOptions` */
  export let loadGroups: (() => Promise<LegacySelectGroup[]>) | null = null;
  export let loadKey: string | null = null;
  export let valueLabel: string | null = null;

  // Presentation
  export let placeholder: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  // Behavior
  export let disabled = false;
  export let required = false;
  export let validationState: ValidationState = "none";
  export let clearable = false;
  /** When true, renders a custom dropdown with search/filter input. */
  export let searchable = false;
  /** When true with searchable, query text becomes the value if no option selected. */
  export let freeform = false;
  /**
   * Explicit native/custom mode override.
   * - `true`: always native `<select>` (searchable/slots ignored)
   * - `false`: always custom dropdown
   * - `undefined`: auto (native unless searchable or option slot provided)
   */
  export let native: boolean | undefined = undefined;
  export let emptyMessage = "No matches";

  // Appearance
  /**
   * Visual variant of the trigger.
   * - `"default"`: standard bordered input appearance
   * - `"ghost"`: no border, background, or chevron — bare trigger
   */
  export let variant: "default" | "ghost" = "default";
  /** Minimum width for the dropdown listbox (e.g. "12rem"). */
  export let menuMinWidth: string | null = null;

  // Accessibility
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;

  // Callbacks
  export let onchange: ((value: string) => void) | null = null;

  // ── Internal state ───────────────────────────────────────────

  const selectId = id ?? `poodle-select-${++nextSelectId}`;
  const listboxId = `${selectId}-listbox`;
  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    change: { value: string };
    queryChange: { query: string };
    openChange: { open: boolean };
  }>();

  const uiPresentation = getUiPresentation();
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = defaultValue;
  let open = false;
  let query = "";
  let highlightIndex = 0;
  let placement: "below" | "above" = "below";
  let alignEnd = false;

  // Async loading state
  let loadedOptions: SelectItems | null = null;
  let loadState: "idle" | "loading" | "loaded" | "error" = "idle";
  let loadError: string | null = null;
  let lastLoadKey: string | null = null;

  // Legacy compat loading
  let loadedItems: LegacySelectItem[] | null = null;
  let loadedGroups: LegacySelectGroup[] | null = null;

  // ── Derived state ────────────────────────────────────────────

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isControlled = value !== null;
  $: currentValue = (isControlled ? value : uncontrolledValue) ?? "";
  $: useCustom = native === true ? false : native === false ? true : searchable || $$slots.option;
  $: isLazy = Boolean(loadOptions || loadItems || loadGroups);
  $: clearValue = defaultValue ?? "";
  $: placeholderValue = clearable ? clearValue : "";
  $: placeholderLabel = placeholder ?? (clearable ? valueLabel ?? "All" : null);

  // Normalize options from all sources
  $: normalizedOptions = (() => {
    if (loadedOptions) return loadedOptions;
    if (loadedGroups) return normalizeGroups(loadedGroups);
    if (loadedItems) return normalizeItems(loadedItems);
    if (groups) return normalizeGroups(groups);
    return normalizeItems(items ?? options);
  })();

  $: flatOptions = flattenOptions(normalizedOptions);
  $: hasCurrentOption = flatOptions.some((o) => o.value === currentValue);
  $: hasSelection = currentValue !== "" && currentValue !== clearValue;
  $: isGrouped = normalizedOptions.length > 0 && "options" in normalizedOptions[0];
  $: selectedOption = flatOptions.find((o) => o.value === currentValue) ?? null;

  // Custom mode: filtering
  $: filteredOptions = searchable && query.length > 0
    ? flatOptions.filter((o) => !(o.disabled || o.isDisabled) && o.label.toLowerCase().includes(query.toLowerCase()))
    : flatOptions.filter((o) => !(o.disabled || o.isDisabled));

  $: filteredGroups = searchable && query.length > 0
    ? filterGroups(normalizedOptions, query)
    : normalizedOptions;

  // Keep query in sync with selected option when closed
  $: if (!open && selectedOption && !freeform) {
    query = selectedOption.label;
  } else if (!open && !selectedOption && !freeform) {
    query = "";
  }

  // Clamp highlight
  $: if (filteredOptions.length > 0 && highlightIndex >= filteredOptions.length) {
    highlightIndex = 0;
  }

  $: highlightedOptionId =
    open && filteredOptions.length > 0 && highlightIndex >= 0
      ? `${listboxId}-option-${highlightIndex}`
      : undefined;

  // Async loading
  $: if (loadKey !== lastLoadKey) {
    lastLoadKey = loadKey;
    loadedOptions = null;
    loadedItems = null;
    loadedGroups = null;
    loadState = "idle";
    loadError = null;
  }

  $: if (isLazy && loadState === "idle") {
    void startLoad();
  }

  // ── Helpers ──────────────────────────────────────────────────

  function normalizeItems(source: LegacySelectItem[] | SelectItems | null): SelectItems {
    if (!source || source.length === 0) return [];
    if ("options" in source[0]) return source as SelectItems;
    return (source as LegacySelectItem[]).map((o) => ({
      value: o.value,
      label: o.label,
      disabled: o.isDisabled ?? o.disabled ?? false,
    }));
  }

  function normalizeGroups(source: LegacySelectGroup[]): SelectOptionGroup[] {
    return source.flatMap((group) => {
      const ownOptions = (group.items ?? []).map((o) => ({
        value: o.value,
        label: o.label,
        disabled: o.isDisabled ?? o.disabled ?? false,
      }));
      const nested = group.groups?.length ? normalizeGroups(group.groups) : [];
      if (group.label.trim().length === 0) {
        return [...(ownOptions.length ? [{ label: "", options: ownOptions }] : []), ...nested];
      }
      return [...(ownOptions.length ? [{ label: group.label, options: ownOptions }] : []), ...nested];
    });
  }

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
      if (loadOptions) {
        loadedOptions = await loadOptions();
      } else if (loadGroups) {
        loadedGroups = await loadGroups();
      } else if (loadItems) {
        loadedItems = await loadItems();
      }
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
    if (!isControlled) {
      uncontrolledValue = nextValue;
    }
    dispatch("valueChange", { value: nextValue });
    dispatch("change", { value: nextValue });
    onchange?.(nextValue);
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
    dispatch("openChange", { open: nextOpen });

    if (nextOpen) {
      highlightIndex = selectedOption
        ? filteredOptions.findIndex((o) => o.value === selectedOption!.value)
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

  function handleInputInput(event: Event): void {
    query = (event.currentTarget as HTMLInputElement).value;
    highlightIndex = 0;
    if (!open) setOpen(true);
    dispatch("queryChange", { query });

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
        aria-expanded={open ? "true" : "false"}
        aria-haspopup="listbox"
        aria-controls={open ? listboxId : undefined}
        aria-label={ariaLabel ?? undefined}
      >
        <input
          id={selectId}
          class="poodle-select__input"
          type="text"
          value={query}
          {disabled}
          placeholder={placeholder ?? undefined}
          aria-autocomplete="list"
          aria-activedescendant={highlightedOptionId}
          aria-describedby={describedBy ?? undefined}
          on:focus={() => { if (!open) setOpen(true); }}
          on:input={handleInputInput}
          on:keydown={handleKeydown}
        />
        {#if clearable && hasSelection && !disabled}
          <button
            type="button"
            class="poodle-select__clear"
            aria-label="Clear selection"
            on:click={handleClear}
          >
            <Icon name="x" size="xs" />
          </button>
        {/if}
        <span class="poodle-select__indicator" aria-hidden="true">
          <Icon name="chevron-down" />
        </span>
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
          on:click={handleTriggerClick}
          on:keydown={handleKeydown}
        >
          {#if $$slots.trigger}
            <slot name="trigger" selectedOption={selectedOption} {open} {placeholder} />
          {:else}
            <span class="poodle-select__value" data-placeholder={!hasSelection}>
              {selectedOption?.label ?? placeholder ?? ""}
            </span>
          {/if}
        </button>
        {#if clearable && hasSelection && !disabled}
          <button
            type="button"
            class="poodle-select__clear"
            aria-label="Clear selection"
            on:click={handleClear}
          >
            <Icon name="x" size="xs" />
          </button>
        {/if}
        {#if variant !== "ghost"}
          <span class="poodle-select__indicator" aria-hidden="true">
            <Icon name="chevron-down" />
          </span>
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
          {#each normalizedOptions as group}
            {@const g = group as SelectOptionGroup}
            {#if g.options.length > 0}
              <div class="poodle-select__group" role="group" aria-label={g.label || undefined}>
                {#if g.label}
                  <div class="poodle-select__group-label">{g.label}</div>
                {/if}
                {#each g.options as option, idx}
                  {@const flatIdx = flatOptions.indexOf(option)}
                  <button
                    type="button"
                    class="poodle-select__option"
                    id={`${listboxId}-option-${flatIdx}`}
                    role="option"
                    aria-selected={currentValue === option.value ? "true" : "false"}
                    data-highlighted={highlightIndex === flatIdx}
                    disabled={isOptionDisabled(option)}
                    on:mouseenter={() => (highlightIndex = flatIdx)}
                    on:click={() => selectOption(option)}
                  >
                    {#if $$slots.option}
                      <slot name="option" {option} highlighted={highlightIndex === flatIdx} selected={currentValue === option.value} index={flatIdx} />
                    {:else}
                      {#if option.icon}
                        <span class="poodle-select__option-icon"><Icon icon={option.icon} size="sm" /></span>
                      {/if}
                      <span class="poodle-select__option-label">{option.label}</span>
                      {#if option.description}
                        <span class="poodle-select__option-description">{option.description}</span>
                      {/if}
                    {/if}
                  </button>
                {/each}
              </div>
            {/if}
          {/each}
        {:else if isGrouped && searchable}
          {#each filteredGroups as group}
            {@const g = group as SelectOptionGroup}
            {#if g.options.length > 0}
              <div class="poodle-select__group" role="group" aria-label={g.label || undefined}>
                {#if g.label}
                  <div class="poodle-select__group-label">{g.label}</div>
                {/if}
                {#each g.options as option}
                  {@const flatIdx = filteredOptions.indexOf(option)}
                  <button
                    type="button"
                    class="poodle-select__option"
                    id={`${listboxId}-option-${flatIdx}`}
                    role="option"
                    aria-selected={currentValue === option.value ? "true" : "false"}
                    data-highlighted={highlightIndex === flatIdx}
                    disabled={isOptionDisabled(option)}
                    on:mouseenter={() => (highlightIndex = flatIdx)}
                    on:click={() => selectOption(option)}
                  >
                    {#if $$slots.option}
                      <slot name="option" {option} highlighted={highlightIndex === flatIdx} selected={currentValue === option.value} index={flatIdx} />
                    {:else}
                      <span class="poodle-select__option-label">{option.label}</span>
                      {#if option.description}
                        <span class="poodle-select__option-description">{option.description}</span>
                      {/if}
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
              on:mouseenter={() => (highlightIndex = index)}
              on:click={() => selectOption(option)}
            >
              {#if $$slots.option}
                <slot name="option" {option} highlighted={highlightIndex === index} selected={currentValue === option.value} {index} />
              {:else}
                {#if option.icon}
                  <span class="poodle-select__option-icon"><Icon icon={option.icon} size="sm" /></span>
                {/if}
                <span class="poodle-select__option-label">{option.label}</span>
                {#if option.description}
                  <span class="poodle-select__option-description">{option.description}</span>
                {/if}
              {/if}
            </button>
          {/each}
        {/if}

        {#if filteredOptions.length === 0}
          {#if $$slots.empty}
            <slot name="empty" {query} />
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
      on:change={handleNativeChange}
    >
      {#if placeholderLabel}
        <option value={placeholderValue} disabled={!clearable && required}>{placeholderLabel}</option>
      {/if}

      {#if isGrouped}
        {#each normalizedOptions as group}
          {#if (group as SelectOptionGroup).label.trim().length === 0}
            {#each (group as SelectOptionGroup).options as option (option.value)}
              <option value={option.value} disabled={isOptionDisabled(option)}>
                {option.label}
              </option>
            {/each}
          {:else}
            <optgroup label={(group as SelectOptionGroup).label}>
              {#each (group as SelectOptionGroup).options as option (option.value)}
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
    position: relative;
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    align-items: center;
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
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
    right: var(--poodle-space-control-x);
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
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0 var(--poodle-space-control-x);
    min-height: var(--poodle-size-control-height);
  }

  .poodle-select__input {
    flex: 1;
    min-width: 0;
    height: var(--poodle-size-control-height);
    padding: 0;
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
    gap: 0.25rem;
    flex: 1;
    min-width: 0;
    min-height: var(--poodle-size-control-height);
    padding: 0;
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
    align-items: center;
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
  }

  .poodle-select__option-label {
    flex: 1;
    min-width: 0;
  }

  .poodle-select__option-description {
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
  .poodle-select[data-density="compact"] { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .poodle-select[data-density="comfortable"] { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
  .poodle-select--custom[data-density="compact"] { padding: 0; }
  .poodle-select--custom[data-density="compact"] .poodle-select__trigger-area,
  .poodle-select--custom[data-density="compact"] .poodle-select__trigger { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .poodle-select--custom[data-density="compact"] .poodle-select__trigger { padding: 0; }
  .poodle-select--custom[data-density="comfortable"] { padding: 0; }
  .poodle-select--custom[data-density="comfortable"] .poodle-select__trigger-area,
  .poodle-select--custom[data-density="comfortable"] .poodle-select__trigger { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
  .poodle-select--custom[data-density="comfortable"] .poodle-select__trigger { padding: 0; }

  /* ═══ SIZE VARIANTS ═══ */
  .poodle-select[data-size="xs"] { min-height: 1.5rem; padding: 0 0.5rem; }
  .poodle-select[data-size="xs"] .poodle-select__control { height: calc(1.5rem - (var(--poodle-border-width-default) * 2)); font-size: 0.75rem; }
  .poodle-select[data-size="xs"] .poodle-select__input,
  .poodle-select[data-size="xs"] .poodle-select__trigger { font-size: 0.75rem; min-height: 1.5rem; }
  .poodle-select--custom[data-size="xs"] { padding: 0; }

  .poodle-select[data-size="sm"] { min-height: 1.75rem; padding: 0 0.625rem; }
  .poodle-select[data-size="sm"] .poodle-select__control { height: calc(1.75rem - (var(--poodle-border-width-default) * 2)); }
  .poodle-select[data-size="sm"] .poodle-select__input,
  .poodle-select[data-size="sm"] .poodle-select__trigger { min-height: 1.75rem; }
  .poodle-select--custom[data-size="sm"] { padding: 0; }

  .poodle-select[data-size="lg"] { min-height: 2.75rem; padding: 0 1rem; }
  .poodle-select[data-size="lg"] .poodle-select__control { height: calc(2.75rem - (var(--poodle-border-width-default) * 2)); font-size: 0.9375rem; }
  .poodle-select[data-size="lg"] .poodle-select__input,
  .poodle-select[data-size="lg"] .poodle-select__trigger { font-size: 0.9375rem; min-height: 2.75rem; }
  .poodle-select--custom[data-size="lg"] { padding: 0; }

  .poodle-select[data-size="xl"] { min-height: 3.25rem; padding: 0 1.125rem; }
  .poodle-select[data-size="xl"] .poodle-select__control { height: calc(3.25rem - (var(--poodle-border-width-default) * 2)); font-size: 1rem; }
  .poodle-select[data-size="xl"] .poodle-select__input,
  .poodle-select[data-size="xl"] .poodle-select__trigger { font-size: 1rem; min-height: 3.25rem; }
  .poodle-select--custom[data-size="xl"] { padding: 0; }
</style>

<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    SelectItems,
    SelectOption,
    SelectOptionGroup,
  } from "./types";

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

  export let id: string | undefined = undefined;
  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let placeholder: string | null = null;
  export let options: SelectItems = [];
  export let items: LegacySelectItem[] | null = null;
  export let groups: LegacySelectGroup[] | null = null;
  export let disabled = false;
  export let required = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let name: string | undefined = undefined;
  export let clearable = false;
  export let valueLabel: string | null = null;
  export let loadItems: (() => Promise<LegacySelectItem[]>) | null = null;
  export let loadGroups: (() => Promise<LegacySelectGroup[]>) | null = null;
  export let loadKey: string | null = null;
  export let onchange: ((value: string) => void) | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    change: { value: string };
  }>();

  const uiPresentation = getUiPresentation();
  let uncontrolledValue = defaultValue;
  let loadedItems: LegacySelectItem[] | null = null;
  let loadedGroups: LegacySelectGroup[] | null = null;
  let loadState: "idle" | "loading" | "loaded" | "error" = "idle";
  let loadError: string | null = null;
  let lastLoadKey: string | null = null;

  function normalizeItems(source: LegacySelectItem[] | SelectItems | null): SelectItems {
    if (!source || source.length === 0) return [];
    if ("options" in source[0]) {
      return source as SelectItems;
    }
    return (source as LegacySelectItem[]).map((option) => ({
      value: option.value,
      label: option.label,
      isDisabled: option.isDisabled ?? option.disabled ?? false,
    }));
  }

  function normalizeGroups(source: LegacySelectGroup[]): SelectOptionGroup[] {
    return source.flatMap((group) => {
      const ownOptions = (group.items ?? []).map((option) => ({
        value: option.value,
        label: option.label,
        isDisabled: option.isDisabled ?? option.disabled ?? false,
      }));
      const nestedOptions = group.groups?.length ? normalizeGroups(group.groups) : [];

      if (group.label.trim().length === 0) {
        return [
          ...(ownOptions.length ? [{ label: "", options: ownOptions }] : []),
          ...nestedOptions,
        ];
      }

      return [
        ...(ownOptions.length ? [{ label: group.label, options: ownOptions }] : []),
        ...nestedOptions,
      ];
    });
  }

  function flattenOptions(source: SelectItems): SelectOption[] {
    if (source.length === 0) return [];
    if ("options" in source[0]) {
      return (source as SelectOptionGroup[]).flatMap((group) => group.options);
    }
    return source as SelectOption[];
  }

  async function startLoad(): Promise<void> {
    if (loadState === "loading" || (!loadItems && !loadGroups)) return;
    loadState = "loading";
    loadError = null;

    try {
      if (loadGroups) {
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

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isControlled = value !== null;
  $: currentValue = (isControlled ? value : uncontrolledValue) ?? "";
  $: isLazy = Boolean(loadItems || loadGroups);
  $: clearValue = defaultValue ?? "";
  $: placeholderValue = clearable ? clearValue : "";
  $: placeholderLabel = placeholder ?? (clearable ? valueLabel ?? "All" : null);
  $: normalizedOptions = loadedGroups
    ? normalizeGroups(loadedGroups)
    : loadedItems
      ? normalizeItems(loadedItems)
      : groups
        ? normalizeGroups(groups)
        : normalizeItems(items ?? options);
  $: flatOptions = flattenOptions(normalizedOptions);
  $: hasCurrentOption = flatOptions.some((option) => option.value === currentValue);
  $: hasSelection = currentValue !== "" && currentValue !== clearValue;
  $: isGrouped = normalizedOptions.length > 0 && "options" in normalizedOptions[0];

  $: if (loadKey !== lastLoadKey) {
    lastLoadKey = loadKey;
    loadedItems = null;
    loadedGroups = null;
    loadState = "idle";
    loadError = null;
  }

  $: if (isLazy && loadState === "idle") {
    void startLoad();
  }

  function handleChange(event: Event): void {
    const nextValue = (event.currentTarget as HTMLSelectElement).value;

    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
    dispatch("change", { value: nextValue });
    onchange?.(nextValue);
  }
</script>

<div class="select" data-placeholder={!hasSelection} data-size={resolvedSize} data-density={resolvedDensity}>
  <select
    {id}
    {name}
    class="select__control"
    value={currentValue}
    disabled={disabled}
    {required}
    aria-label={ariaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    on:change={handleChange}
  >
    {#if placeholderLabel}
      <option value={placeholderValue} disabled={!clearable && required}>{placeholderLabel}</option>
    {/if}

    {#if isGrouped}
      {#each normalizedOptions as group}
        {#if (group as SelectOptionGroup).label.trim().length === 0}
          {#each (group as SelectOptionGroup).options as option (option.value)}
            <option value={option.value} disabled={option.isDisabled === true}>
              {option.label}
            </option>
          {/each}
        {:else}
          <optgroup label={(group as SelectOptionGroup).label}>
            {#each (group as SelectOptionGroup).options as option (option.value)}
              <option value={option.value} disabled={option.isDisabled === true}>
                {option.label}
              </option>
            {/each}
          </optgroup>
        {/if}
      {/each}
    {:else if flatOptions.length > 0}
      {#each flatOptions as option}
        <option value={option.value} disabled={option.isDisabled === true}>
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

  <span class="select__indicator" aria-hidden="true"><Icon name="chevron-down" /></span>
</div>

<style>
  .select {
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

  .select:focus-within {
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

  .select:has(.select__control:disabled) {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .select__control {
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

  .select[data-placeholder="true"] .select__control {
    color: var(--poodle-color-text-secondary);
  }

  .select__indicator {
    position: absolute;
    right: var(--poodle-space-control-x);
    top: 50%;
    transform: translateY(-50%);
    color: var(--poodle-color-icon-muted);
    font-size: 0.75rem;
    line-height: 1;
    pointer-events: none;
  }

  .select__control optgroup {
    font-weight: 600;
    color: var(--poodle-color-text-secondary);
  }

  .select__control option {
    font-weight: normal;
    color: var(--poodle-color-text-primary);
  }

  /* Density variants */
  .select[data-density="compact"] {
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .select[data-density="comfortable"] {
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
  }

  /* Size variants */
  .select[data-size="xs"] {
    min-height: calc(var(--poodle-size-control-height) - 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .select[data-size="xs"] .select__control {
    height: calc(var(--poodle-size-control-height) - 0.5rem - (var(--poodle-border-width-default) * 2));
    font-size: 0.75rem;
  }

  .select[data-size="sm"] {
    min-height: calc(var(--poodle-size-control-height) - 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.0625rem);
  }

  .select[data-size="sm"] .select__control {
    height: calc(var(--poodle-size-control-height) - 0.375rem - (var(--poodle-border-width-default) * 2));
  }

  .select[data-size="lg"] {
    min-height: calc(var(--poodle-size-control-height) + 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
  }

  .select[data-size="lg"] .select__control {
    height: calc(var(--poodle-size-control-height) + 0.375rem - (var(--poodle-border-width-default) * 2));
    font-size: 0.9375rem;
  }

  .select[data-size="xl"] {
    min-height: calc(var(--poodle-size-control-height) + 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.1875rem);
  }

  .select[data-size="xl"] .select__control {
    height: calc(var(--poodle-size-control-height) + 0.5rem - (var(--poodle-border-width-default) * 2));
    font-size: 1rem;
  }
</style>

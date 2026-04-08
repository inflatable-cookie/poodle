<!--
  @deprecated Use <Select searchable> instead.
  This component is maintained for backward compatibility but will be
  removed in a future major release. Migrate to:
    <Select options={...} searchable placeholder="..." />
  For freeform text entry, use:
    <Select options={...} searchable freeform placeholder="..." />
-->
<script context="module" lang="ts">
  let nextComboboxId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  import { firstEnabledIndex } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ComboboxOption, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let options: ComboboxOption[] = [];
  export let placeholder: string | null = null;
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    queryChange: { query: string };
    openChange: { open: boolean };
  }>();

  const listboxId = `poodle-combobox-listbox-${++nextComboboxId}`;
  const uiPresentation = getUiPresentation();
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = defaultValue;
  let placement: "below" | "above" = "below";
  let query = "";
  let open = false;
  let highlightIndex = 0;

  $: controlled = value !== null;
  $: currentValue = controlled ? value : uncontrolledValue;
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: selectedOption = options.find((option) => option.value === currentValue) ?? null;
  $: filteredOptions = options.filter((option) =>
    option.label.toLowerCase().includes(query.toLowerCase())
  );
  $: if (!open && selectedOption) {
    query = selectedOption.label;
  }
  $: if (filteredOptions.length > 0 && highlightIndex >= filteredOptions.length) {
    highlightIndex = firstEnabledIndex(filteredOptions);
  }
  $: highlightedOptionId =
    open && filteredOptions.length > 0 && highlightIndex >= 0
      ? `${listboxId}-option-${highlightIndex}`
      : undefined;

  function setOpen(nextOpen: boolean): void {
    if (nextOpen && rootElement) {
      const rect = rootElement.getBoundingClientRect();
      const spaceBelow = window.innerHeight - rect.bottom;
      placement = spaceBelow < 280 ? "above" : "below";
    }
    open = nextOpen;
    dispatch("openChange", { open: nextOpen });
  }

  function selectOption(option: ComboboxOption): void {
    if (option.disabled) {
      return;
    }

    if (!controlled) {
      uncontrolledValue = option.value;
    }

    query = option.label;
    setOpen(false);
    dispatch("valueChange", { value: option.value });
  }

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!open || !rootElement) {
        return;
      }

      if (!rootElement.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    document.addEventListener("mousedown", handlePointerDown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
    };
  });
</script>

<div
  bind:this={rootElement}
  class="combobox"
  data-open={open}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  role="combobox"
  aria-label={ariaLabel ?? undefined}
  aria-expanded={open ? "true" : "false"}
  aria-haspopup="listbox"
  aria-controls={listboxId}
>
  <input
    class="combobox__input"
    type="text"
    value={query}
    disabled={disabled}
    aria-autocomplete="list"
    aria-activedescendant={highlightedOptionId}
    {placeholder}
    on:focus={() => setOpen(true)}
    on:input={(event) => {
      query = (event.currentTarget as HTMLInputElement).value;
      setOpen(true);
      dispatch("queryChange", { query });
    }}
    on:keydown={(event) => {
      if (event.key === "ArrowDown") {
        event.preventDefault();
        setOpen(true);
        highlightIndex = Math.min(highlightIndex + 1, Math.max(filteredOptions.length - 1, 0));
      }

      if (event.key === "ArrowUp") {
        event.preventDefault();
        setOpen(true);
        highlightIndex = Math.max(highlightIndex - 1, 0);
      }

      if (event.key === "Enter" && open && filteredOptions[highlightIndex]) {
        event.preventDefault();
        selectOption(filteredOptions[highlightIndex]);
      }

      if (event.key === "Escape") {
        event.preventDefault();
        setOpen(false);
      }
    }}
  />

  {#if open}
    <div id={listboxId} class="combobox__list" class:combobox__list--above={placement === "above"} role="listbox">
      {#if filteredOptions.length === 0}
        <div class="combobox__empty">No matches</div>
      {:else}
        {#each filteredOptions as option, index (option.value)}
          <button
            type="button"
            class="combobox__option"
            id={`${listboxId}-option-${index}`}
            data-highlighted={highlightIndex === index}
            disabled={option.disabled === true}
            role="option"
            aria-selected={selectedOption?.value === option.value ? "true" : "false"}
            on:mouseenter={() => (highlightIndex = index)}
            on:click={() => selectOption(option)}
          >
            <span class="combobox__label">{option.label}</span>
            {#if option.description}
              <span class="combobox__description">{option.description}</span>
            {/if}
          </button>
        {/each}
      {/if}
    </div>
  {/if}
</div>

<style>
  .combobox {
    position: relative;
    display: grid;
    min-width: 14rem;
  }

  .combobox__input {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(
      --poodle-treatment-interactive-subtle-border,
      var(--poodle-color-border-default)
    );
    border-radius: var(--poodle-treatment-interactive-subtle-radius, var(--poodle-radius-control));
    background: var(--poodle-treatment-interactive-subtle-fill, var(--poodle-color-background-surface));
    box-shadow: var(--poodle-treatment-interactive-subtle-shadow, none);
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .combobox__input:focus-visible {
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
    outline: none;
  }

  .combobox__list {
    position: absolute;
    top: calc(100% + 0.375rem);
    left: 0;
    right: 0;
    z-index: var(--poodle-overlay-z-menu);
    display: grid;
    gap: 0.125rem;
    padding: 0.25rem;
    border: 0.0625rem solid var(
      --poodle-treatment-surface-elevated-border,
      color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)
    );
    border-radius: var(--poodle-treatment-surface-elevated-radius, var(--poodle-radius-surface));
    background: var(
      --poodle-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))
    );
    box-shadow: var(--poodle-treatment-surface-elevated-shadow, var(--poodle-elevation-overlay));
  }

  .combobox__list--above {
    top: auto;
    bottom: calc(100% + 0.375rem);
  }

  .combobox__option {
    display: grid;
    gap: 0.125rem;
    width: 100%;
    padding: 0.375rem 0.5rem;
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
    text-align: left;
  }

  .combobox__option[data-highlighted="true"],
  .combobox__option:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
  }

  .combobox__option:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .combobox__description,
  .combobox__empty {
    color: var(--poodle-color-text-secondary);
    font-size: 0.6875rem;
    line-height: 1.35;
  }

  .combobox__empty {
    padding: 0.5rem;
  }

  /* Density variants */
  .combobox[data-density="compact"] .combobox__input {
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .combobox[data-density="comfortable"] .combobox__input {
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
  }

  /* Size variants */
  .combobox[data-size="xs"] .combobox__input {
    min-height: calc(var(--poodle-size-control-height) - 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
    font-size: 0.75rem;
  }

  .combobox[data-size="sm"] .combobox__input {
    min-height: calc(var(--poodle-size-control-height) - 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.0625rem);
  }

  .combobox[data-size="lg"] .combobox__input {
    min-height: calc(var(--poodle-size-control-height) + 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
    font-size: 0.9375rem;
  }

  .combobox[data-size="xl"] .combobox__input {
    min-height: calc(var(--poodle-size-control-height) + 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.1875rem);
    font-size: 1rem;
  }
</style>

<script context="module" lang="ts">
  let nextComboboxId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  import { firstEnabledIndex } from "./internal";

  import type { ComboboxOption } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let options: ComboboxOption[] = [];
  export let placeholder: string | null = null;
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    queryChange: { query: string };
    openChange: { open: boolean };
  }>();

  const listboxId = `flint-combobox-listbox-${++nextComboboxId}`;
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = defaultValue;
  let query = "";
  let open = false;
  let highlightIndex = 0;

  $: controlled = value !== null;
  $: currentValue = controlled ? value : uncontrolledValue;
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

  function setOpen(nextOpen: boolean): void {
    open = nextOpen;
    dispatch("openChange", { open: nextOpen });
  }

  function selectOption(option: ComboboxOption): void {
    if (option.isDisabled) {
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
    disabled={isDisabled}
    aria-autocomplete="list"
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
    <div id={listboxId} class="combobox__list" role="listbox">
      {#if filteredOptions.length === 0}
        <div class="combobox__empty">No matches</div>
      {:else}
        {#each filteredOptions as option, index (option.value)}
          <button
            type="button"
            class="combobox__option"
            data-highlighted={highlightIndex === index}
            disabled={option.isDisabled === true}
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
    min-height: var(--flint-size-control-height);
    padding: 0 var(--flint-space-control-x);
    border: 0.0625rem solid var(
      --flint-treatment-interactive-subtle-border,
      var(--flint-color-border-default)
    );
    border-radius: var(--flint-treatment-interactive-subtle-radius, var(--flint-radius-control));
    background: var(--flint-treatment-interactive-subtle-fill, var(--flint-color-background-surface));
    box-shadow: var(--flint-treatment-interactive-subtle-shadow, none);
    color: var(--flint-color-text-primary);
    font-family: var(--flint-typography-body-family);
    font-size: var(--flint-typography-body-size);
    line-height: var(--flint-typography-body-lineHeight);
    transition:
      border-color var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      box-shadow var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .combobox__input:focus-visible {
    border-color: var(--flint-color-accent-focusRing);
    background: var(
      --flint-treatment-interactive-subtle-fill-focus,
      var(--flint-color-background-surface)
    );
    box-shadow: var(
      --flint-treatment-interactive-subtle-shadow-focus,
      0 0 0 var(--flint-border-width-focus)
        color-mix(in srgb, var(--flint-color-accent-focusRing) 28%, transparent)
    );
    outline: none;
  }

  .combobox__list {
    position: absolute;
    top: calc(100% + 0.375rem);
    left: 0;
    right: 0;
    z-index: var(--flint-overlay-z-menu);
    display: grid;
    gap: 0.125rem;
    padding: 0.25rem;
    border: 0.0625rem solid var(
      --flint-treatment-surface-elevated-border,
      color-mix(in srgb, var(--flint-color-border-default) 72%, transparent)
    );
    border-radius: var(--flint-treatment-surface-elevated-radius, var(--flint-radius-surface));
    background: var(
      --flint-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--flint-color-background-elevated) 98%, var(--flint-color-background-panel))
    );
    box-shadow: var(--flint-treatment-surface-elevated-shadow, var(--flint-elevation-overlay));
  }

  .combobox__option {
    display: grid;
    gap: 0.125rem;
    width: 100%;
    padding: 0.375rem 0.5rem;
    border: 0;
    border-radius: calc(var(--flint-radius-control) - 0.125rem);
    background: transparent;
    color: var(--flint-color-text-primary);
    cursor: pointer;
    font: inherit;
    text-align: left;
  }

  .combobox__option[data-highlighted="true"],
  .combobox__option:hover:not(:disabled) {
    background: color-mix(in srgb, var(--flint-color-accent-base) 16%, transparent);
  }

  .combobox__option:disabled {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }

  .combobox__description,
  .combobox__empty {
    color: var(--flint-color-text-secondary);
    font-size: 0.6875rem;
    line-height: 1.35;
  }

  .combobox__empty {
    padding: 0.5rem;
  }
</style>

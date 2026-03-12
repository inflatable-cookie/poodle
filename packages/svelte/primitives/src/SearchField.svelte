<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import TextInput from "./TextInput.svelte";
  import type { ValidationState } from "./types";

  export let id: string;
  export let value: string | null = null;
  export let defaultValue = "";
  export let placeholder = "Search";
  export let ariaLabel = "Search";
  export let describedBy: string | null = null;
  export let isDisabled = false;
  export let isReadOnly = false;
  export let showClearButton = true;
  export let validationState: ValidationState = "none";

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    submit: { value: string };
    clear: void;
    cancel: void;
  }>();

  let uncontrolledValue = defaultValue;

  $: isControlled = value !== null;
  $: currentValue = isControlled ? value ?? "" : uncontrolledValue;
  $: canClear = showClearButton && !isDisabled && !isReadOnly && currentValue.length > 0;

  function handleValueChange(event: CustomEvent<{ value: string }>): void {
    const nextValue = event.detail.value;

    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function handleClear(): void {
    if (!isControlled) {
      uncontrolledValue = "";
    }

    dispatch("valueChange", { value: "" });
    dispatch("clear");
  }
</script>

<TextInput
  {id}
  type="search"
  {value}
  {defaultValue}
  {placeholder}
  {ariaLabel}
  {describedBy}
  {isDisabled}
  {isReadOnly}
  {validationState}
  on:valueChange={handleValueChange}
  on:submit={(event) => dispatch("submit", event.detail)}
  on:cancel={() => dispatch("cancel")}
>
  <span slot="leading" aria-hidden="true">⌕</span>
  <svelte:fragment slot="trailing">
    {#if canClear}
      <button
        class="search-field__clear"
        type="button"
        aria-label="Clear search query"
        on:click={handleClear}
      >
        ✕
      </button>
    {/if}
  </svelte:fragment>
</TextInput>

<style>
  .search-field__clear {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--pug-icon-size-default);
    height: var(--pug-icon-size-default);
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--pug-color-icon-muted);
    cursor: pointer;
    border-radius: calc(
      var(--pug-treatment-interactive-subtle-radius, var(--pug-radius-control)) - 0.0625rem
    );
  }

  .search-field__clear:hover {
    background: var(
      --pug-treatment-interactive-subtle-fill-hover,
      color-mix(in srgb, var(--pug-color-background-surface) 84%, transparent)
    );
    color: var(--pug-color-text-primary);
  }

  .search-field__clear:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>

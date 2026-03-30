<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import TextInput from "./TextInput.svelte";
  import { getUiPresentation } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState } from "./types";

  export let id: string;
  export let value: string | null = null;
  export let defaultValue = "";
  export let placeholder = "Search";
  export let ariaLabel = "Search";
  export let describedBy: string | null = null;
  export let disabled = false;
  export let readOnly = false;
  export let debounce: number | null = null;
  export let showClearButton = true;
  export let validationState: ValidationState = "none";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const uiPresentation = getUiPresentation();

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    submit: { value: string };
    clear: void;
    cancel: void;
    keydown: KeyboardEvent;
  }>();

  let uncontrolledValue = defaultValue;

  $: isControlled = value !== null;
  $: currentValue = isControlled ? value ?? "" : uncontrolledValue;
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: canClear = showClearButton && !disabled && !readOnly && currentValue.length > 0;

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
  {disabled}
  {readOnly}
  {debounce}
  {validationState}
  {size}
  {sizeRole}
  on:valueChange={handleValueChange}
  on:submit={(event) => dispatch("submit", event.detail)}
  on:cancel={() => dispatch("cancel")}
  on:keydown={(event) => dispatch("keydown", event.detail)}
>
  <span slot="leading" aria-hidden="true"><Icon name="search" /></span>
  <svelte:fragment slot="trailing">
    {#if canClear}
      <button
        class="search-field__clear"
        type="button"
        aria-label="Clear search query"
        on:click={handleClear}
      >
        <Icon name="x" />
      </button>
    {/if}
  </svelte:fragment>
</TextInput>

<style>
  .search-field__clear {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-icon-size-default);
    height: var(--poodle-icon-size-default);
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--poodle-color-icon-muted);
    cursor: pointer;
    border-radius: calc(
      var(--poodle-treatment-interactive-subtle-radius, var(--poodle-radius-control)) - 0.0625rem
    );
  }

  .search-field__clear:hover {
    background: var(
      --poodle-treatment-interactive-subtle-fill-hover,
      color-mix(in srgb, var(--poodle-color-background-surface) 84%, transparent)
    );
    color: var(--poodle-color-text-primary);
  }

  .search-field__clear:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>

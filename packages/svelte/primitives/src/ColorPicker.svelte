<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let value = "#6366f1";
  export let swatches: string[] = [];
  export let showInput = true;
  export let isDisabled = false;
  export let ariaLabel = "Color picker";

  const dispatch = createEventDispatcher<{
    change: { value: string };
  }>();

  let inputValue = value;

  $: if (value !== inputValue && isValidHex(value)) {
    inputValue = value;
  }

  function isValidHex(hex: string): boolean {
    return /^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/.test(hex);
  }

  function normalizeHex(hex: string): string {
    if (hex.length === 4) {
      return `#${hex[1]}${hex[1]}${hex[2]}${hex[2]}${hex[3]}${hex[3]}`;
    }
    return hex;
  }

  function handleColorInput(event: Event): void {
    const newValue = (event.currentTarget as HTMLInputElement).value;
    value = newValue;
    inputValue = newValue;
    dispatch("change", { value: newValue });
  }

  function handleTextInput(event: Event): void {
    const raw = (event.currentTarget as HTMLInputElement).value;
    inputValue = raw;
    const normalized = raw.startsWith("#") ? raw : `#${raw}`;
    if (isValidHex(normalized)) {
      value = normalizeHex(normalized);
      dispatch("change", { value });
    }
  }

  function handleTextBlur(): void {
    inputValue = value;
  }

  function selectSwatch(hex: string): void {
    if (isDisabled) return;
    value = hex;
    inputValue = hex;
    dispatch("change", { value: hex });
  }
</script>

<div class="color-picker" aria-label={ariaLabel} data-disabled={isDisabled}>
  <div class="color-picker__controls">
    <div class="color-picker__preview-wrap">
      <input
        type="color"
        class="color-picker__native"
        value={value}
        disabled={isDisabled}
        aria-label="Select color"
        on:input={handleColorInput}
      />
      <div
        class="color-picker__preview"
        style="background: {value}"
        aria-hidden="true"
      ></div>
    </div>

    {#if showInput}
      <input
        type="text"
        class="color-picker__input"
        value={inputValue}
        disabled={isDisabled}
        maxlength="9"
        aria-label="Hex color value"
        on:input={handleTextInput}
        on:blur={handleTextBlur}
      />
    {/if}
  </div>

  {#if swatches.length > 0}
    <div class="color-picker__swatches" role="listbox" aria-label="Color swatches">
      {#each swatches as hex (hex)}
        <button
          type="button"
          class="color-picker__swatch"
          class:color-picker__swatch--active={value === hex}
          style="background: {hex}"
          disabled={isDisabled}
          role="option"
          aria-selected={value === hex ? "true" : "false"}
          aria-label={hex}
          on:click={() => selectSwatch(hex)}
        ></button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .color-picker {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-width: 16rem;
  }

  .color-picker[data-disabled="true"] {
    opacity: var(--pug-state-opacity-disabled);
    pointer-events: none;
  }

  .color-picker__controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .color-picker__preview-wrap {
    position: relative;
    width: 2.25rem;
    height: 2.25rem;
    flex-shrink: 0;
  }

  .color-picker__native {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
    border: 0;
    padding: 0;
  }

  .color-picker__preview {
    width: 100%;
    height: 100%;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 62%, transparent);
    border-radius: var(--pug-radius-control);
    pointer-events: none;
  }

  .color-picker__input {
    flex: 1;
    min-width: 0;
    height: 2.25rem;
    padding: 0 var(--pug-space-control-x);
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-code-family);
    font-size: 0.8125rem;
    outline: none;
    transition:
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      box-shadow var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .color-picker__input:focus {
    border-color: var(--pug-color-accent-focusRing);
    box-shadow: 0 0 0 var(--pug-border-width-focus)
      color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent);
  }

  .color-picker__swatches {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
  }

  .color-picker__swatch {
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: 0.125rem solid transparent;
    border-radius: 0.25rem;
    cursor: pointer;
    transition:
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      transform var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .color-picker__swatch:hover:not(:disabled) {
    transform: scale(1.15);
  }

  .color-picker__swatch:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .color-picker__swatch--active {
    border-color: var(--pug-color-text-primary);
    box-shadow: 0 0 0 0.0625rem var(--pug-color-background-surface);
  }
</style>

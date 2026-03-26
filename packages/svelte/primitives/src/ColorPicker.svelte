<script context="module" lang="ts">
  let nextColorPickerId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  import Slider from "./Slider.svelte";
  import SegmentedControl from "./SegmentedControl.svelte";
  import NumberEntry from "./NumberEntry.svelte";
  import {
    hexToHsv,
    hsvToHex,
    hsvToRgb,
    rgbToHsv,
    hsvToHsl,
    hslToHsv,
    isValidHex,
    normalizeHex,
  } from "./color-utils";
  import type { ColorInputMode } from "./types";

  export let value = "#6366f1";
  export let swatches: string[] = [];
  export let showInput = true;
  export let showAlpha = false;
  export let disabled = false;
  export let ariaLabel = "Color picker";
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let defaultMode: ColorInputMode = "hex";

  const dispatch = createEventDispatcher<{
    change: { value: string };
    openChange: { open: boolean };
  }>();

  const pickerId = ++nextColorPickerId;
  const surfaceId = `poodle-color-picker-surface-${pickerId}`;

  let rootElement: HTMLDivElement | null = null;
  let gradientElement: HTMLDivElement | null = null;
  let uncontrolledOpen = defaultOpen;
  let inputMode: ColorInputMode = defaultMode;

  // Internal HSV state
  let h = 0;
  let s = 0;
  let v = 0;
  let alpha = 1;

  // Tracks the authoritative hex to avoid HSV round-trip drift.
  // Set when syncing from an external hex; cleared when the user
  // changes colour via gradient/slider (which must recompute from HSV).
  let pinnedHex: string | null = null;

  // Text input state
  let hexInput = value;

  // Initialise from prop
  syncFromHex(value);

  $: isOpen = open ?? uncontrolledOpen;

  // Keep internal state in sync when value prop changes externally
  $: if (isValidHex(value)) {
    syncFromHex(value);
  }

  // Derived colours — use pinnedHex when available to avoid rounding drift
  $: computedHex = hsvToHex(h, s, v, showAlpha && alpha < 1 ? alpha : undefined);
  $: currentHex = pinnedHex ?? computedHex;
  $: currentRgb = hsvToRgb(h, s, v);
  $: currentHsl = hsvToHsl(h, s, v);
  $: previewColor = showAlpha
    ? `rgba(${currentRgb.r}, ${currentRgb.g}, ${currentRgb.b}, ${alpha})`
    : currentHex;

  function syncFromHex(hex: string): void {
    if (!isValidHex(hex)) return;
    const norm = normalizeHex(hex);
    const hsv = hexToHsv(norm);
    h = hsv.h;
    s = hsv.s;
    v = hsv.v;

    // Parse alpha from 8-digit hex
    const stripped = norm.replace("#", "");
    if (stripped.length === 8) {
      alpha = parseInt(stripped.slice(6, 8), 16) / 255;
    } else {
      alpha = 1;
    }
    pinnedHex = norm;
    hexInput = norm;
  }

  function commitColor(): void {
    // Clear pinned hex — the user changed colour via controls,
    // so we must derive hex from the current HSV state.
    pinnedHex = null;
    const out = hsvToHex(h, s, v, showAlpha && alpha < 1 ? alpha : undefined);
    value = out;
    hexInput = out;
    dispatch("change", { value: out });
  }

  /** Commit using the pinned hex (preserves exact hex from text input / swatch). */
  function commitFromPinned(): void {
    const out = pinnedHex ?? hsvToHex(h, s, v, showAlpha && alpha < 1 ? alpha : undefined);
    value = out;
    hexInput = out;
    dispatch("change", { value: out });
  }

  function setOpen(next: boolean): void {
    if (open === null) {
      uncontrolledOpen = next;
    }
    dispatch("openChange", { open: next });
  }

  function toggleOpen(): void {
    if (disabled) return;
    setOpen(!isOpen);
  }

  // ── Gradient pad pointer tracking ────────────────────────────
  let dragging = false;

  function updateFromPointer(event: PointerEvent): void {
    if (!gradientElement) return;
    const rect = gradientElement.getBoundingClientRect();
    const x = Math.max(0, Math.min(event.clientX - rect.left, rect.width));
    const y = Math.max(0, Math.min(event.clientY - rect.top, rect.height));
    s = Math.round((x / rect.width) * 100);
    v = Math.round((1 - y / rect.height) * 100);
    commitColor();
  }

  function onGradientPointerDown(event: PointerEvent): void {
    if (disabled) return;
    event.preventDefault();
    dragging = true;
    (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
    updateFromPointer(event);
  }

  function onGradientPointerMove(event: PointerEvent): void {
    if (!dragging) return;
    updateFromPointer(event);
  }

  function onGradientPointerUp(): void {
    dragging = false;
  }

  function onGradientKeydown(event: KeyboardEvent): void {
    const step = event.shiftKey ? 10 : 1;
    let handled = true;

    switch (event.key) {
      case "ArrowRight":
        s = Math.min(100, s + step);
        break;
      case "ArrowLeft":
        s = Math.max(0, s - step);
        break;
      case "ArrowUp":
        v = Math.min(100, v + step);
        break;
      case "ArrowDown":
        v = Math.max(0, v - step);
        break;
      default:
        handled = false;
    }

    if (handled) {
      event.preventDefault();
      commitColor();
    }
  }

  // ── Hue slider ───────────────────────────────────────────────
  function onHueChange(event: CustomEvent<{ value: number }>): void {
    h = event.detail.value;
    commitColor();
  }

  // ── Alpha slider ─────────────────────────────────────────────
  function onAlphaChange(event: CustomEvent<{ value: number }>): void {
    alpha = event.detail.value / 100;
    commitColor();
  }

  // ── Mode toggle ──────────────────────────────────────────────
  const modeOptions = [
    { value: "hex", label: "Hex" },
    { value: "rgb", label: "RGB" },
    { value: "hsl", label: "HSL" },
  ];

  function onModeChange(event: CustomEvent<{ value: string }>): void {
    inputMode = event.detail.value as ColorInputMode;
  }

  // ── Hex text input ───────────────────────────────────────────
  function onHexInput(event: Event): void {
    const raw = (event.currentTarget as HTMLInputElement).value;
    hexInput = raw;
    const normalized = raw.startsWith("#") ? raw : `#${raw}`;
    if (isValidHex(normalized)) {
      syncFromHex(normalizeHex(normalized));
      commitFromPinned();
    }
  }

  function onHexBlur(): void {
    hexInput = currentHex;
  }

  // ── Trigger text input (inline hex field) ────────────────────
  let triggerHexInput = value;

  $: triggerHexInput = currentHex;

  function onTriggerHexInput(event: Event): void {
    const raw = (event.currentTarget as HTMLInputElement).value;
    triggerHexInput = raw;
    const normalized = raw.startsWith("#") ? raw : `#${raw}`;
    if (isValidHex(normalized)) {
      syncFromHex(normalizeHex(normalized));
      commitFromPinned();
    }
  }

  function onTriggerHexBlur(): void {
    triggerHexInput = currentHex;
  }

  // ── RGB inputs ───────────────────────────────────────────────
  function onRgbChange(channel: "r" | "g" | "b", event: CustomEvent<{ value: number | null }>): void {
    const val = event.detail.value ?? 0;
    const rgb = { ...currentRgb };
    rgb[channel] = val;
    const hsv = rgbToHsv(rgb.r, rgb.g, rgb.b);
    h = hsv.h;
    s = hsv.s;
    v = hsv.v;
    commitColor();
  }

  // ── HSL inputs ───────────────────────────────────────────────
  function onHslChange(channel: "h" | "s" | "l", event: CustomEvent<{ value: number | null }>): void {
    const val = event.detail.value ?? 0;
    const hsl = { ...currentHsl };
    hsl[channel] = val;
    const hsv = hslToHsv(hsl.h, hsl.s, hsl.l);
    h = hsv.h;
    s = hsv.s;
    v = hsv.v;
    commitColor();
  }

  // ── Alpha input (shared across modes) ────────────────────────
  function onAlphaInputChange(event: CustomEvent<{ value: number | null }>): void {
    alpha = (event.detail.value ?? 100) / 100;
    commitColor();
  }

  // ── Swatches ─────────────────────────────────────────────────
  function selectSwatch(hex: string): void {
    if (disabled) return;
    syncFromHex(hex);
    commitFromPinned();
  }

  // ── Document listeners (outside click & escape) ──────────────
  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!isOpen || !rootElement) return;
      if (!rootElement.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && isOpen) {
        event.preventDefault();
        setOpen(false);
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeydown);
    };
  });
</script>

<div
  class="color-picker"
  aria-label={ariaLabel}
  data-disabled={disabled || undefined}
  bind:this={rootElement}
>
  <!-- Trigger row -->
  <div class="color-picker__controls">
    <button
      type="button"
      class="color-picker__trigger"
      aria-label="Open color picker"
      aria-haspopup="dialog"
      aria-expanded={isOpen}
      aria-controls={surfaceId}
      disabled={disabled}
      on:click={toggleOpen}
    >
      <span
        class="color-picker__preview"
        style="background: {previewColor}"
        aria-hidden="true"
      ></span>
    </button>

    {#if showInput}
      <input
        type="text"
        class="color-picker__input"
        value={triggerHexInput}
        disabled={disabled}
        maxlength="9"
        aria-label="Hex color value"
        on:input={onTriggerHexInput}
        on:blur={onTriggerHexBlur}
      />
    {/if}
  </div>

  <!-- Surface (popover) -->
  {#if isOpen}
    <div
      id={surfaceId}
      class="color-picker__surface"
      role="dialog"
      aria-label="Color picker"
    >
      <div class="color-picker__picker-area">
        <!-- Gradient pad (saturation × value) -->
        <div
          class="color-picker__gradient"
          style="background-color: hsl({h}, 100%, 50%)"
          role="slider"
          tabindex="0"
          aria-label="Saturation and brightness"
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={Math.round(s)}
          aria-valuetext="Saturation {s}%, Brightness {v}%"
          bind:this={gradientElement}
          on:pointerdown={onGradientPointerDown}
          on:pointermove={onGradientPointerMove}
          on:pointerup={onGradientPointerUp}
          on:keydown={onGradientKeydown}
        >
          <div
            class="color-picker__gradient-thumb"
            style="left: {s}%; top: {100 - v}%; background: {currentHex}"
            aria-hidden="true"
          ></div>
        </div>

        <!-- Controls panel (right of gradient) -->
        <div class="color-picker__controls-panel">
          <!-- Hue slider -->
          <div class="color-picker__hue-wrap">
            <Slider
              value={h}
              min={0}
              max={360}
              step={1}
              ariaLabel="Hue"
              on:valueChange={onHueChange}
            />
          </div>

          <!-- Alpha slider -->
          {#if showAlpha}
            <div
              class="color-picker__alpha-wrap"
              style="--poodle-cp-alpha-color: {currentHex}"
            >
              <Slider
                value={Math.round(alpha * 100)}
                min={0}
                max={100}
                step={1}
                ariaLabel="Opacity"
                on:valueChange={onAlphaChange}
              />
            </div>
          {/if}

          <!-- Mode toggle + inputs -->
          <div class="color-picker__mode-section">
        <div class="color-picker__mode-toggle">
          <SegmentedControl
            value={inputMode}
            options={modeOptions}
            ariaLabel="Color input mode"
            on:valueChange={onModeChange}
          />
        </div>

        <div class="color-picker__inputs">
          {#if inputMode === "hex"}
            <div class="color-picker__hex-field">
              <input
                type="text"
                class="color-picker__text-input"
                value={hexInput}
                maxlength="9"
                aria-label="Hex color"
                on:input={onHexInput}
                on:blur={onHexBlur}
              />
              <span class="color-picker__input-label" aria-hidden="true">Hex</span>
            </div>
            {#if showAlpha}
              <div class="color-picker__channel-field">
                <NumberEntry
                  id="cp-{pickerId}-a-hex"
                  value={Math.round(alpha * 100)}
                  min={0}
                  max={100}
                  step={1}
                  ariaLabel="Alpha"
                  on:valueChange={onAlphaInputChange}
                />
                <span class="color-picker__input-label" aria-hidden="true">A</span>
              </div>
            {/if}
          {:else if inputMode === "rgb"}
            <div class="color-picker__channel-field">
              <NumberEntry
                id="cp-{pickerId}-r"
                value={currentRgb.r}
                min={0}
                max={255}
                step={1}
                ariaLabel="Red"
                on:valueChange={(e) => onRgbChange("r", e)}
              />
              <span class="color-picker__input-label" aria-hidden="true">R</span>
            </div>
            <div class="color-picker__channel-field">
              <NumberEntry
                id="cp-{pickerId}-g"
                value={currentRgb.g}
                min={0}
                max={255}
                step={1}
                ariaLabel="Green"
                on:valueChange={(e) => onRgbChange("g", e)}
              />
              <span class="color-picker__input-label" aria-hidden="true">G</span>
            </div>
            <div class="color-picker__channel-field">
              <NumberEntry
                id="cp-{pickerId}-b"
                value={currentRgb.b}
                min={0}
                max={255}
                step={1}
                ariaLabel="Blue"
                on:valueChange={(e) => onRgbChange("b", e)}
              />
              <span class="color-picker__input-label" aria-hidden="true">B</span>
            </div>
            {#if showAlpha}
              <div class="color-picker__channel-field">
                <NumberEntry
                  id="cp-{pickerId}-a-rgb"
                  value={Math.round(alpha * 100)}
                  min={0}
                  max={100}
                  step={1}
                  ariaLabel="Alpha"
                  on:valueChange={onAlphaInputChange}
                />
                <span class="color-picker__input-label" aria-hidden="true">A</span>
              </div>
            {/if}
          {:else}
            <div class="color-picker__channel-field">
              <NumberEntry
                id="cp-{pickerId}-hsl-h"
                value={currentHsl.h}
                min={0}
                max={360}
                step={1}
                ariaLabel="Hue"
                on:valueChange={(e) => onHslChange("h", e)}
              />
              <span class="color-picker__input-label" aria-hidden="true">H</span>
            </div>
            <div class="color-picker__channel-field">
              <NumberEntry
                id="cp-{pickerId}-hsl-s"
                value={currentHsl.s}
                min={0}
                max={100}
                step={1}
                ariaLabel="Saturation"
                on:valueChange={(e) => onHslChange("s", e)}
              />
              <span class="color-picker__input-label" aria-hidden="true">S</span>
            </div>
            <div class="color-picker__channel-field">
              <NumberEntry
                id="cp-{pickerId}-hsl-l"
                value={currentHsl.l}
                min={0}
                max={100}
                step={1}
                ariaLabel="Lightness"
                on:valueChange={(e) => onHslChange("l", e)}
              />
              <span class="color-picker__input-label" aria-hidden="true">L</span>
            </div>
            {#if showAlpha}
              <div class="color-picker__channel-field">
                <NumberEntry
                  id="cp-{pickerId}-a-hsl"
                  value={Math.round(alpha * 100)}
                  min={0}
                  max={100}
                  step={1}
                  ariaLabel="Alpha"
                  on:valueChange={onAlphaInputChange}
                />
                <span class="color-picker__input-label" aria-hidden="true">A</span>
              </div>
            {/if}
          {/if}
        </div>
      </div>
        </div>
      </div>

      <!-- Swatches -->
      {#if swatches.length > 0}
        <div class="color-picker__swatches" role="listbox" aria-label="Color swatches">
          {#each swatches as hex (hex)}
            <button
              type="button"
              class="color-picker__swatch"
              class:color-picker__swatch--active={currentHex === hex}
              style="background: {hex}"
              role="option"
              aria-selected={currentHex === hex ? "true" : "false"}
              aria-label={hex}
              on:click={() => selectSwatch(hex)}
            ></button>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .color-picker {
    position: relative;
    display: inline-flex;
    flex-direction: column;
  }

  .color-picker[data-disabled] {
    opacity: var(--poodle-state-opacity-disabled);
    pointer-events: none;
  }

  .color-picker__controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  /* ── Trigger button ──────────────────────────────────────── */

  .color-picker__trigger {
    position: relative;
    width: 2.25rem;
    height: 2.25rem;
    min-height: 0;
    flex-shrink: 0;
    padding: 0;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 62%, transparent);
    border-radius: var(--poodle-radius-control);
    background: transparent;
    cursor: pointer;
    overflow: hidden;
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .color-picker__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .color-picker__preview {
    position: absolute;
    inset: 0;
    border-radius: inherit;
  }

  /* ── Inline hex input ────────────────────────────────────── */

  .color-picker__input {
    width: 6.5rem;
    height: 2.25rem;
    min-height: 0;
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
    outline: none;
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .color-picker__input:focus {
    border-color: var(--poodle-color-accent-focusRing);
    box-shadow: 0 0 0 var(--poodle-border-width-focus)
      color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent);
  }

  /* ── Surface (popover) ───────────────────────────────────── */

  .color-picker__surface {
    position: absolute;
    top: 100%;
    left: 0;
    z-index: 50;
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
    width: 24rem;
    margin-top: 0.25rem;
    padding: 0.75rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-elevated);
    box-shadow: var(--poodle-shadow-lg);
  }

  /* ── Picker area (gradient + controls side by side) ────── */

  .color-picker__picker-area {
    display: flex;
    gap: 0.625rem;
    align-items: stretch;
  }

  .color-picker__controls-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    min-width: 0;
  }

  /* ── Gradient pad ────────────────────────────────────────── */

  .color-picker__gradient {
    position: relative;
    width: 10rem;
    flex-shrink: 0;
    aspect-ratio: 1;
    border-radius: 0.25rem;
    cursor: crosshair;
    touch-action: none;
    overflow: hidden;
  }

  .color-picker__gradient::before {
    content: "";
    position: absolute;
    inset: 0;
    background: linear-gradient(to right, #fff, transparent);
  }

  .color-picker__gradient::after {
    content: "";
    position: absolute;
    inset: 0;
    background: linear-gradient(to bottom, transparent, #000);
  }

  .color-picker__gradient:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .color-picker__gradient-thumb {
    position: absolute;
    z-index: 1;
    width: 0.875rem;
    height: 0.875rem;
    min-height: 0;
    margin-left: -0.4375rem;
    margin-top: -0.4375rem;
    border: 0.125rem solid #fff;
    border-radius: 50%;
    box-shadow: 0 0 0 0.0625rem rgba(0, 0, 0, 0.3), inset 0 0 0 0.0625rem rgba(0, 0, 0, 0.1);
    pointer-events: none;
  }

  /* ── Hue slider ──────────────────────────────────────────── */

  .color-picker__hue-wrap {
    min-height: 0;
  }

  .color-picker__hue-wrap :global(.slider__track) {
    background: linear-gradient(
      to right,
      #f00 0%,
      #ff0 17%,
      #0f0 33%,
      #0ff 50%,
      #00f 67%,
      #f0f 83%,
      #f00 100%
    ) !important;
  }

  .color-picker__hue-wrap :global(.slider__fill) {
    display: none;
  }

  /* ── Alpha slider ────────────────────────────────────────── */

  .color-picker__alpha-wrap {
    position: relative;
    min-height: 0;
  }

  .color-picker__alpha-wrap :global(.slider__track) {
    background:
      linear-gradient(
        to right,
        transparent,
        var(--poodle-cp-alpha-color, #000)
      ),
      repeating-conic-gradient(
        #d0d0d0 0% 25%,
        #fff 0% 50%
      ) 0 0 / 0.5rem 0.5rem !important;
  }

  .color-picker__alpha-wrap :global(.slider__fill) {
    display: none;
  }

  /* ── Mode section ────────────────────────────────────────── */

  .color-picker__mode-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .color-picker__mode-toggle {
    min-height: 0;
  }

  .color-picker__inputs {
    display: flex;
    gap: 0.375rem;
    align-items: flex-start;
  }

  .color-picker__hex-field {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    min-width: 0;
  }

  .color-picker__channel-field {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    min-width: 0;
  }

  .color-picker__channel-field :global(input) {
    min-height: 0;
  }

  .color-picker__text-input {
    width: 100%;
    height: 2rem;
    min-height: 0;
    padding: 0 0.375rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    outline: none;
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .color-picker__text-input:focus {
    border-color: var(--poodle-color-accent-focusRing);
    box-shadow: 0 0 0 var(--poodle-border-width-focus)
      color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent);
  }

  .color-picker__input-label {
    display: block;
    font-family: var(--poodle-typography-label-family);
    font-size: 0.625rem;
    font-weight: var(--poodle-typography-label-weight);
    color: var(--poodle-color-text-secondary);
    text-align: center;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    user-select: none;
  }

  /* ── Swatches ────────────────────────────────────────────── */

  .color-picker__swatches {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
    padding-top: 0.25rem;
    border-top: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 42%, transparent);
  }

  .color-picker__swatch {
    width: 1.25rem;
    height: 1.25rem;
    min-height: 0;
    padding: 0;
    border: 0.125rem solid transparent;
    border-radius: 0.1875rem;
    cursor: pointer;
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      transform var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .color-picker__swatch:hover {
    transform: scale(1.15);
  }

  .color-picker__swatch:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .color-picker__swatch--active {
    border-color: var(--poodle-color-text-primary);
    box-shadow: 0 0 0 0.0625rem var(--poodle-color-background-surface);
  }
</style>

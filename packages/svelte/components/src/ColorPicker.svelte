<script module lang="ts">
  let nextColorPickerId = 0;
</script>

<script lang="ts">
  import { default as Slider } from "./Slider.svelte";
  import { default as SegmentedControl } from "./SegmentedControl.svelte";
  import { default as NumberInput } from "./NumberInput.svelte";
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
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ColorInputMode, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    value?: string | undefined;
    swatches?: string[];
    showInput?: boolean;
    showAlpha?: boolean;
    disabled?: boolean;
    ariaLabel?: string;
    open?: boolean | null | undefined;
    defaultOpen?: boolean;
    defaultMode?: ColorInputMode;
    onChange?: ((value: string) => void) | null;
    onOpenChange?: ((open: boolean) => void) | null;
  }

  let {
    size = null,
    sizeRole = "control",
    density = null,
    value = $bindable<string | undefined>(undefined),
    swatches = [],
    showInput = true,
    showAlpha = false,
    disabled = false,
    ariaLabel = "Color picker",
    open = undefined,
    defaultOpen = false,
    defaultMode = "hex",
    onChange = null,
    onOpenChange = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const pickerId = ++nextColorPickerId;
  const surfaceId = `poodle-color-picker-surface-${pickerId}`;

  let rootElement = $state<HTMLDivElement | null>(null);
  let gradientElement = $state<HTMLDivElement | null>(null);
  let uncontrolledValue = $state("#6366f1");
  let uncontrolledOpen = $state(false);
  let placement = $state<"below" | "above">("below");
  let inputMode = $state<ColorInputMode>("hex");
  let h = $state(0);
  let s = $state(0);
  let v = $state(0);
  let alpha = $state(1);
  let pinnedHex = $state<string | null>(null);
  let hexInput = $state("#6366f1");
  let triggerHexInput = $state("#6366f1");
  let dragging = $state(false);
  let seededDefaults = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasControlledValue = $derived(value !== undefined);
  const currentValue = $derived(hasControlledValue ? value ?? "#6366f1" : uncontrolledValue);
  const isOpen = $derived(open === undefined ? uncontrolledOpen : open === true);
  const computedHex = $derived(hsvToHex(h, s, v, showAlpha && alpha < 1 ? alpha : undefined));
  const currentHex = $derived(pinnedHex ?? computedHex);
  const currentRgb = $derived(hsvToRgb(h, s, v));
  const currentHsl = $derived(hsvToHsl(h, s, v));
  const previewColor = $derived(
    showAlpha ? `rgba(${currentRgb.r}, ${currentRgb.g}, ${currentRgb.b}, ${alpha})` : currentHex,
  );

  $effect.pre(() => {
    if (seededDefaults) {
      return;
    }

    uncontrolledValue = value ?? "#6366f1";
    uncontrolledOpen = defaultOpen;
    inputMode = defaultMode;
    syncFromHex(value ?? "#6366f1");
    seededDefaults = true;
  });

  $effect(() => {
    if (isValidHex(currentValue)) {
      syncFromHex(currentValue);
    }
  });

  $effect(() => {
    triggerHexInput = currentHex;
  });

  function syncFromHex(hex: string): void {
    if (!isValidHex(hex)) {
      return;
    }

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
    triggerHexInput = norm;
  }

  function commitColor(): void {
    pinnedHex = null;
    const out = hsvToHex(h, s, v, showAlpha && alpha < 1 ? alpha : undefined);
    if (hasControlledValue) {
      value = out;
    } else {
      uncontrolledValue = out;
    }
    hexInput = out;
    onChange?.(out);
  }

  function commitFromPinned(): void {
    const out = pinnedHex ?? hsvToHex(h, s, v, showAlpha && alpha < 1 ? alpha : undefined);
    if (hasControlledValue) {
      value = out;
    } else {
      uncontrolledValue = out;
    }
    hexInput = out;
    onChange?.(out);
  }

  function setOpen(next: boolean): void {
    if (next && rootElement) {
      const rect = rootElement.getBoundingClientRect();
      const spaceBelow = window.innerHeight - rect.bottom;
      placement = spaceBelow < 360 ? "above" : "below";
    }
    if (open === undefined) {
      uncontrolledOpen = next;
    }
    onOpenChange?.(next);
  }

  function toggleOpen(): void {
    if (disabled) return;
    setOpen(!isOpen);
  }

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

  function onHueChange(value: number): void {
    h = value;
    commitColor();
  }

  function onAlphaChange(value: number): void {
    alpha = value / 100;
    commitColor();
  }

  const modeOptions = [
    { value: "hex", label: "Hex" },
    { value: "rgb", label: "RGB" },
    { value: "hsl", label: "HSL" },
  ];

  function onModeChange(value: string): void {
    inputMode = value as ColorInputMode;
  }

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

  function toNumericInputValue(value: string | number | null): number | null {
    const nextValue = value;

    if (nextValue === null || nextValue === "") {
      return null;
    }

    if (typeof nextValue === "number") {
      return Number.isFinite(nextValue) ? nextValue : null;
    }

    const parsedValue = Number(nextValue);
    return Number.isFinite(parsedValue) ? parsedValue : null;
  }

  function onRgbChange(channel: "r" | "g" | "b", value: string | number | null): void {
    const val = toNumericInputValue(value) ?? 0;
    const rgb = { ...currentRgb };
    rgb[channel] = val;
    const hsv = rgbToHsv(rgb.r, rgb.g, rgb.b);
    h = hsv.h;
    s = hsv.s;
    v = hsv.v;
    commitColor();
  }

  function onHslChange(channel: "h" | "s" | "l", value: string | number | null): void {
    const val = toNumericInputValue(value) ?? 0;
    const hsl = { ...currentHsl };
    hsl[channel] = val;
    const hsv = hslToHsv(hsl.h, hsl.s, hsl.l);
    h = hsv.h;
    s = hsv.s;
    v = hsv.v;
    commitColor();
  }

  function onAlphaInputChange(value: string | number | null): void {
    alpha = (toNumericInputValue(value) ?? 100) / 100;
    commitColor();
  }

  function selectSwatch(hex: string): void {
    if (disabled) return;
    syncFromHex(hex);
    commitFromPinned();
  }

  $effect(() => {
    if (!isOpen) {
      return;
    }

    function handlePointerDown(event: MouseEvent): void {
      if (!rootElement) return;
      if (!rootElement.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape") {
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
  class="poodle-color-picker"
  aria-label={ariaLabel}
  data-disabled={disabled || undefined}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  bind:this={rootElement}
>
  <!-- Trigger row -->
  <div class="poodle-color-picker__controls">
    <button
      type="button"
      class="poodle-color-picker__trigger"
      aria-label="Open color picker"
      aria-haspopup="dialog"
      aria-expanded={isOpen}
      aria-controls={surfaceId}
      disabled={disabled}
      onclick={toggleOpen}
    >
      <span
        class="poodle-color-picker__preview"
        style="background: {previewColor}"
        aria-hidden="true"
      ></span>
    </button>

    {#if showInput}
      <input
        type="text"
        class="poodle-color-picker__input"
        value={triggerHexInput}
        disabled={disabled}
        maxlength="9"
        aria-label="Hex color value"
        oninput={onTriggerHexInput}
        onblur={onTriggerHexBlur}
      />
    {/if}
  </div>

  <!-- Surface (popover) -->
  {#if isOpen}
    <div
      id={surfaceId}
      class="poodle-color-picker__surface"
      class:poodle-color-picker__surface--above={placement === "above"}
      role="dialog"
      aria-label="Color picker"
    >
      <div class="poodle-color-picker__picker-area">
        <!-- Gradient pad (saturation × value) -->
        <div
          class="poodle-color-picker__gradient"
          style="background-color: hsl({h}, 100%, 50%)"
          role="slider"
          tabindex="0"
          aria-label="Saturation and brightness"
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={Math.round(s)}
          aria-valuetext="Saturation {s}%, Brightness {v}%"
          bind:this={gradientElement}
          onpointerdown={onGradientPointerDown}
          onpointermove={onGradientPointerMove}
          onpointerup={onGradientPointerUp}
          onkeydown={onGradientKeydown}
        >
          <div
            class="poodle-color-picker__gradient-thumb"
            style="left: {s}%; top: {100 - v}%; background: {currentHex}"
            aria-hidden="true"
          ></div>
        </div>

        <!-- Controls panel (right of gradient) -->
        <div class="poodle-color-picker__controls-panel">
          <!-- Hue slider -->
          <div class="poodle-color-picker__hue-wrap">
            <Slider
              value={h}
              min={0}
              max={360}
              step={1}
              ariaLabel="Hue"
              size={resolvedSize}
              density={resolvedDensity}
              onValueChange={onHueChange}
            />
          </div>

          <!-- Alpha slider -->
          {#if showAlpha}
            <div
              class="poodle-color-picker__alpha-wrap"
              style="--poodle-cp-alpha-color: {currentHex}"
            >
              <Slider
                value={Math.round(alpha * 100)}
                min={0}
                max={100}
                step={1}
                ariaLabel="Opacity"
                size={resolvedSize}
                density={resolvedDensity}
                onValueChange={onAlphaChange}
              />
            </div>
          {/if}

          <!-- Mode toggle + inputs -->
          <div class="poodle-color-picker__mode-section">
        <div class="poodle-color-picker__mode-toggle">
          <SegmentedControl
            value={inputMode}
            options={modeOptions}
            ariaLabel="Color input mode"
            size={resolvedSize}
            density={resolvedDensity}
            onValueChange={onModeChange}
          />
        </div>

        <div class="poodle-color-picker__inputs">
          {#if inputMode === "hex"}
            <div class="poodle-color-picker__hex-field">
              <input
                type="text"
                class="poodle-color-picker__text-input"
                value={hexInput}
                maxlength="9"
                aria-label="Hex color"
                oninput={onHexInput}
                onblur={onHexBlur}
              />
              <span class="poodle-color-picker__input-label" aria-hidden="true">Hex</span>
            </div>
            {#if showAlpha}
              <div class="poodle-color-picker__channel-field">
                <NumberInput
                  id="cp-{pickerId}-a-hex"
                  value={Math.round(alpha * 100)}
                  min={0}
                  max={100}
                  step={1}
                  ariaLabel="Alpha"
                  size={resolvedSize}
                  density={resolvedDensity}
                  onValueChange={onAlphaInputChange}
                />
                <span class="poodle-color-picker__input-label" aria-hidden="true">A</span>
              </div>
            {/if}
          {:else if inputMode === "rgb"}
            <div class="poodle-color-picker__channel-field">
              <NumberInput
                id="cp-{pickerId}-r"
                value={currentRgb.r}
                min={0}
                max={255}
                step={1}
                ariaLabel="Red"
                size={resolvedSize}
                density={resolvedDensity}
                onValueChange={(value) => onRgbChange("r", value)}
              />
              <span class="poodle-color-picker__input-label" aria-hidden="true">R</span>
            </div>
            <div class="poodle-color-picker__channel-field">
              <NumberInput
                id="cp-{pickerId}-g"
                value={currentRgb.g}
                min={0}
                max={255}
                step={1}
                ariaLabel="Green"
                size={resolvedSize}
                density={resolvedDensity}
                onValueChange={(value) => onRgbChange("g", value)}
              />
              <span class="poodle-color-picker__input-label" aria-hidden="true">G</span>
            </div>
            <div class="poodle-color-picker__channel-field">
              <NumberInput
                id="cp-{pickerId}-b"
                value={currentRgb.b}
                min={0}
                max={255}
                step={1}
                ariaLabel="Blue"
                size={resolvedSize}
                density={resolvedDensity}
                onValueChange={(value) => onRgbChange("b", value)}
              />
              <span class="poodle-color-picker__input-label" aria-hidden="true">B</span>
            </div>
            {#if showAlpha}
              <div class="poodle-color-picker__channel-field">
                <NumberInput
                  id="cp-{pickerId}-a-rgb"
                  value={Math.round(alpha * 100)}
                  min={0}
                  max={100}
                  step={1}
                  ariaLabel="Alpha"
                  size={resolvedSize}
                  density={resolvedDensity}
                  onValueChange={onAlphaInputChange}
                />
                <span class="poodle-color-picker__input-label" aria-hidden="true">A</span>
              </div>
            {/if}
          {:else}
            <div class="poodle-color-picker__channel-field">
              <NumberInput
                id="cp-{pickerId}-hsl-h"
                value={currentHsl.h}
                min={0}
                max={360}
                step={1}
                ariaLabel="Hue"
                size={resolvedSize}
                density={resolvedDensity}
                onValueChange={(value) => onHslChange("h", value)}
              />
              <span class="poodle-color-picker__input-label" aria-hidden="true">H</span>
            </div>
            <div class="poodle-color-picker__channel-field">
              <NumberInput
                id="cp-{pickerId}-hsl-s"
                value={currentHsl.s}
                min={0}
                max={100}
                step={1}
                ariaLabel="Saturation"
                size={resolvedSize}
                density={resolvedDensity}
                onValueChange={(value) => onHslChange("s", value)}
              />
              <span class="poodle-color-picker__input-label" aria-hidden="true">S</span>
            </div>
            <div class="poodle-color-picker__channel-field">
              <NumberInput
                id="cp-{pickerId}-hsl-l"
                value={currentHsl.l}
                min={0}
                max={100}
                step={1}
                ariaLabel="Lightness"
                size={resolvedSize}
                density={resolvedDensity}
                onValueChange={(value) => onHslChange("l", value)}
              />
              <span class="poodle-color-picker__input-label" aria-hidden="true">L</span>
            </div>
            {#if showAlpha}
              <div class="poodle-color-picker__channel-field">
                <NumberInput
                  id="cp-{pickerId}-a-hsl"
                  value={Math.round(alpha * 100)}
                  min={0}
                  max={100}
                  step={1}
                  ariaLabel="Alpha"
                  size={resolvedSize}
                  density={resolvedDensity}
                  onValueChange={onAlphaInputChange}
                />
                <span class="poodle-color-picker__input-label" aria-hidden="true">A</span>
              </div>
            {/if}
          {/if}
        </div>
      </div>
        </div>
      </div>

      <!-- Swatches -->
      {#if swatches.length > 0}
        <div class="poodle-color-picker__swatches" role="listbox" aria-label="Color swatches">
          {#each swatches as hex (hex)}
            <button
              type="button"
              class="poodle-color-picker__swatch"
              class:poodle-color-picker__swatch--active={currentHex === hex}
              style="background: {hex}"
              role="option"
              aria-selected={currentHex === hex ? "true" : "false"}
              aria-label={hex}
              onclick={() => selectSwatch(hex)}
            ></button>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .poodle-color-picker {
    position: relative;
    display: inline-flex;
    flex-direction: column;
  }

  .poodle-color-picker[data-disabled] {
    opacity: var(--poodle-state-opacity-disabled);
    pointer-events: none;
  }

  .poodle-color-picker__controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  /* ── Trigger button ──────────────────────────────────────── */

  .poodle-color-picker__trigger {
    position: relative;
    width: 2.25rem;
    height: 2.25rem;
    min-height: 0;
    flex-shrink: 0;
    padding: 0;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 62%, transparent);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-recipe-color-picker-trigger-fill, transparent);
    cursor: pointer;
    overflow: hidden;
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-color-picker__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .poodle-color-picker__preview {
    position: absolute;
    inset: 0;
    border-radius: inherit;
  }

  /* ── Inline hex input ────────────────────────────────────── */

  .poodle-color-picker__input {
    width: 6.5rem;
    height: 2.25rem;
    min-height: 0;
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-recipe-color-picker-input-fill, var(--poodle-color-background-surface));
    color: var(--poodle-recipe-color-picker-input-text, var(--poodle-color-text-primary));
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
    outline: none;
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-color-picker__input:focus {
    border-color: var(--poodle-recipe-color-picker-focus-input-border, var(--poodle-color-accent-focusRing));
    box-shadow: var(--poodle-recipe-color-picker-focus-input-shadow, 0 0 0 var(--poodle-border-width-focus)
      color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent));
  }

  /* ── Surface (popover) ───────────────────────────────────── */

  .poodle-color-picker__surface {
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
    background: var(--poodle-recipe-color-picker-surface-fill, var(--poodle-color-background-elevated));
    box-shadow: var(--poodle-recipe-color-picker-surface-shadow, var(--poodle-shadow-lg));
  }

  .poodle-color-picker__surface--above {
    top: auto;
    bottom: 100%;
    margin-top: 0;
    margin-bottom: 0.25rem;
  }

  /* ── Picker area (gradient + controls side by side) ────── */

  .poodle-color-picker__picker-area {
    display: flex;
    gap: 0.625rem;
    align-items: stretch;
  }

  .poodle-color-picker__controls-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    min-width: 0;
  }

  /* ── Gradient pad ────────────────────────────────────────── */

  .poodle-color-picker__gradient {
    position: relative;
    width: 10rem;
    flex-shrink: 0;
    aspect-ratio: 1;
    border-radius: 0.25rem;
    cursor: crosshair;
    touch-action: none;
    overflow: hidden;
  }

  .poodle-color-picker__gradient::before {
    content: "";
    position: absolute;
    inset: 0;
    background: var(--poodle-recipe-color-picker-gradient-fill, linear-gradient(to right, #fff, transparent));
  }

  .poodle-color-picker__gradient::after {
    content: "";
    position: absolute;
    inset: 0;
    background: var(--poodle-recipe-color-picker-gradient-after-fill, linear-gradient(to bottom, transparent, #000));
  }

  .poodle-color-picker__gradient:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .poodle-color-picker__gradient-thumb {
    position: absolute;
    z-index: 1;
    width: 0.875rem;
    height: 0.875rem;
    min-height: 0;
    margin-left: -0.4375rem;
    margin-top: -0.4375rem;
    border: 0.125rem solid #fff;
    border-radius: 50%;
    box-shadow: var(--poodle-recipe-color-picker-gradient-thumb-shadow, 0 0 0 0.0625rem rgba(0, 0, 0, 0.3), inset 0 0 0 0.0625rem rgba(0, 0, 0, 0.1));
    pointer-events: none;
  }

  /* ── Hue slider ──────────────────────────────────────────── */

  .poodle-color-picker__hue-wrap {
    min-height: 0;
  }

  .poodle-color-picker__hue-wrap :global(.poodle-slider__track) {
    background: var(--poodle-recipe-color-picker-track-fill, linear-gradient(
      to right,
      #f00 0%,
      #ff0 17%,
      #0f0 33%,
      #0ff 50%,
      #00f 67%,
      #f0f 83%,
      #f00 100%
    ) !important);
  }

  .poodle-color-picker__hue-wrap :global(.poodle-slider__fill) {
    display: none;
  }

  /* ── Alpha slider ────────────────────────────────────────── */

  .poodle-color-picker__alpha-wrap {
    position: relative;
    min-height: 0;
  }

  .poodle-color-picker__alpha-wrap :global(.poodle-slider__track) {
    background: var(--poodle-recipe-color-picker-track-fill, linear-gradient(
        to right,
        transparent,
        var(--poodle-cp-alpha-color, #000)
      ),
      repeating-conic-gradient(
        #d0d0d0 0% 25%,
        #fff 0% 50%
      ) 0 0 / 0.5rem 0.5rem !important);
  }

  .poodle-color-picker__alpha-wrap :global(.poodle-slider__fill) {
    display: none;
  }

  /* ── Mode section ────────────────────────────────────────── */

  .poodle-color-picker__mode-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .poodle-color-picker__mode-toggle {
    min-height: 0;
  }

  .poodle-color-picker__inputs {
    display: flex;
    gap: 0.375rem;
    align-items: flex-start;
  }

  .poodle-color-picker__hex-field {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    min-width: 0;
  }

  .poodle-color-picker__channel-field {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    min-width: 0;
  }

  .poodle-color-picker__channel-field :global(.poodle-number-input__field) {
    height: 2rem;
    min-height: 0;
  }

  .poodle-color-picker__channel-field :global(input) {
    min-height: 0;
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    text-align: center;
  }

  .poodle-color-picker__text-input {
    width: 100%;
    height: 2rem;
    min-height: 0;
    padding: 0 0.375rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-recipe-color-picker-text-input-fill, var(--poodle-color-background-surface));
    color: var(--poodle-recipe-color-picker-text-input-text, var(--poodle-color-text-primary));
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    outline: none;
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-color-picker__text-input:focus {
    border-color: var(--poodle-recipe-color-picker-focus-text-input-border, var(--poodle-color-accent-focusRing));
    box-shadow: var(--poodle-recipe-color-picker-focus-text-input-shadow, 0 0 0 var(--poodle-border-width-focus)
      color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent));
  }

  .poodle-color-picker__input-label {
    display: block;
    font-family: var(--poodle-typography-label-family);
    font-size: 0.625rem;
    font-weight: var(--poodle-typography-label-weight);
    color: var(--poodle-recipe-color-picker-input-label-text, var(--poodle-color-text-secondary));
    text-align: center;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    user-select: none;
  }

  /* ── Swatches ────────────────────────────────────────────── */

  .poodle-color-picker__swatches {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
    padding-top: 0.25rem;
    border-top: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 42%, transparent);
  }

  .poodle-color-picker__swatch {
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

  .poodle-color-picker__swatch:hover {
    transform: scale(1.15);
  }

  .poodle-color-picker__swatch:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .poodle-color-picker__swatch--active {
    border-color: var(--poodle-recipe-color-picker-swatch--active-border, var(--poodle-color-text-primary));
    box-shadow: var(--poodle-recipe-color-picker-swatch--active-shadow, 0 0 0 0.0625rem var(--poodle-color-background-surface));
  }

  /* Size variants */
  .poodle-color-picker[data-size="xs"] .poodle-color-picker__trigger {
    width: 1.75rem;
    height: 1.75rem;
  }

  .poodle-color-picker[data-size="xs"] .poodle-color-picker__input {
    height: 1.75rem;
    font-size: 0.6875rem;
  }

  .poodle-color-picker[data-size="sm"] .poodle-color-picker__trigger {
    width: 2rem;
    height: 2rem;
  }

  .poodle-color-picker[data-size="sm"] .poodle-color-picker__input {
    height: 2rem;
    font-size: 0.75rem;
  }

  .poodle-color-picker[data-size="lg"] .poodle-color-picker__trigger {
    width: 2.5rem;
    height: 2.5rem;
  }

  .poodle-color-picker[data-size="lg"] .poodle-color-picker__input {
    height: 2.5rem;
    font-size: 0.875rem;
  }

  .poodle-color-picker[data-size="xl"] .poodle-color-picker__trigger {
    width: 2.75rem;
    height: 2.75rem;
  }

  .poodle-color-picker[data-size="xl"] .poodle-color-picker__input {
    height: 2.75rem;
    font-size: 0.9375rem;
  }

  /* Density variants */
  .poodle-color-picker[data-density="compact"] .poodle-color-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .poodle-color-picker[data-density="comfortable"] .poodle-color-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
</style>

<script module lang="ts">
  let nextColorPickerId = 0;
</script>

<script lang="ts">
  import { layerContains } from "@poodle/headless";
  import "@poodle/styles/color-picker.css";
  import { anchored } from "./anchored";
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
  let surfaceElement = $state<HTMLDivElement | null>(null);
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
      // The surface is portalled out of the root, so both count as inside.
      if (!layerContains(event.target as Node, rootElement, surfaceElement)) {
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
      bind:this={surfaceElement}
      use:anchored={{
        anchor: rootElement,
        placement: "bottom-start",
        offset: 4,
        onPlacement: (next) => (placement = next.startsWith("top") ? "above" : "below"),
      }}
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


import "@inflatable-cookie/poodle-core/styles/color-picker.css";

import {
  useEffect,
  useId,
  useRef,
  useState,
  type ChangeEvent,
  type KeyboardEvent as ReactKeyboardEvent,
  type PointerEvent as ReactPointerEvent,
} from "react";

import {
  hexToHsv,
  hslToHsv,
  hsvToHex,
  hsvToHsl,
  hsvToRgb,
  isValidHex,
  normalizeHex,
  rgbToHsv,
  layerContains,
} from "@inflatable-cookie/poodle-core";

import { AnchoredSurface } from "./AnchoredSurface";
import { NumberInput } from "./NumberInput";
import { SegmentedControl } from "./SegmentedControl";
import { Slider } from "./Slider";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ColorInputMode, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface ColorPickerProps {
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

const modeOptions = [
  { value: "hex", label: "Hex" },
  { value: "rgb", label: "RGB" },
  { value: "hsl", label: "HSL" },
];

function toNumericInputValue(value: string | number | null): number | null {
  if (value === null || value === "") {
    return null;
  }

  if (typeof value === "number") {
    return Number.isFinite(value) ? value : null;
  }

  const parsedValue = Number(value);
  return Number.isFinite(parsedValue) ? parsedValue : null;
}

export function ColorPicker({
  size = null,
  sizeRole = "control",
  density = null,
  value,
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
}: ColorPickerProps) {
  const uiPresentation = useUiPresentation();

  const pickerId = useId();
  const surfaceId = `poodle-color-picker-surface-${pickerId}`;

  // The root is state, not a ref: the portalled surface has to re-render
  // once it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const surfaceRef = useRef<HTMLDivElement | null>(null);
  const gradientRef = useRef<HTMLDivElement | null>(null);
  const draggingRef = useRef(false);

  const initialHex = isValidHex(value ?? "") ? normalizeHex(value ?? "#6366f1") : "#6366f1";
  const initialHsv = hexToHsv(initialHex);

  const [uncontrolledValue, setUncontrolledValue] = useState(value ?? "#6366f1");
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [placement, setPlacement] = useState<"below" | "above">("below");
  const [inputMode, setInputMode] = useState<ColorInputMode>(defaultMode);
  const [h, setH] = useState(initialHsv.h);
  const [s, setS] = useState(initialHsv.s);
  const [v, setV] = useState(initialHsv.v);
  const [alpha, setAlpha] = useState(1);
  const [pinnedHex, setPinnedHex] = useState<string | null>(initialHex);
  const [hexInput, setHexInput] = useState(initialHex);
  const [triggerHexInput, setTriggerHexInput] = useState(initialHex);

  const hasControlledValue = value !== undefined;
  const currentValue = hasControlledValue ? (value ?? "#6366f1") : uncontrolledValue;
  const isOpen = open === undefined ? uncontrolledOpen : open === true;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const computedHex = hsvToHex(h, s, v, showAlpha && alpha < 1 ? alpha : undefined);
  const currentHex = pinnedHex ?? computedHex;
  const currentRgb = hsvToRgb(h, s, v);
  const currentHsl = hsvToHsl(h, s, v);
  const previewColor = showAlpha ? `rgba(${currentRgb.r}, ${currentRgb.g}, ${currentRgb.b}, ${alpha})` : currentHex;

  const hsvRef = useRef({ h, s, v, alpha });
  hsvRef.current = { h, s, v, alpha };

  function syncFromHex(hex: string): void {
    if (!isValidHex(hex)) {
      return;
    }

    const norm = normalizeHex(hex);
    const hsv = hexToHsv(norm);
    setH(hsv.h);
    setS(hsv.s);
    setV(hsv.v);

    // Parse alpha from 8-digit hex
    const stripped = norm.replace("#", "");
    if (stripped.length === 8) {
      setAlpha(parseInt(stripped.slice(6, 8), 16) / 255);
    } else {
      setAlpha(1);
    }
    setPinnedHex(norm);
    setHexInput(norm);
    setTriggerHexInput(norm);
  }

  const lastSyncedValue = useRef(currentValue);
  useEffect(() => {
    if (currentValue !== lastSyncedValue.current && isValidHex(currentValue)) {
      lastSyncedValue.current = currentValue;
      syncFromHex(currentValue);
    }
  }, [currentValue]);

  useEffect(() => {
    setTriggerHexInput(currentHex);
  }, [currentHex]);

  function emitValue(out: string): void {
    lastSyncedValue.current = out;
    if (!hasControlledValue) {
      setUncontrolledValue(out);
    }
    setHexInput(out);
    onChange?.(out);
  }

  function commitColor(nextH = hsvRef.current.h, nextS = hsvRef.current.s, nextV = hsvRef.current.v, nextAlpha = hsvRef.current.alpha): void {
    setPinnedHex(null);
    const out = hsvToHex(nextH, nextS, nextV, showAlpha && nextAlpha < 1 ? nextAlpha : undefined);
    emitValue(out);
  }

  function commitFromPinned(pinned: string): void {
    emitValue(pinned);
  }

  function setOpenState(next: boolean): void {
    if (open === undefined) {
      setUncontrolledOpen(next);
    }
    onOpenChange?.(next);
  }

  function toggleOpen(): void {
    if (disabled) return;
    setOpenState(!isOpen);
  }

  function updateFromPointer(event: ReactPointerEvent | PointerEvent): void {
    const gradient = gradientRef.current;
    if (!gradient) return;
    const rect = gradient.getBoundingClientRect();
    const x = Math.max(0, Math.min(event.clientX - rect.left, rect.width));
    const y = Math.max(0, Math.min(event.clientY - rect.top, rect.height));
    const nextS = Math.round((x / rect.width) * 100);
    const nextV = Math.round((1 - y / rect.height) * 100);
    setS(nextS);
    setV(nextV);
    commitColor(hsvRef.current.h, nextS, nextV);
  }

  function onGradientPointerDown(event: ReactPointerEvent): void {
    if (disabled) return;
    event.preventDefault();
    draggingRef.current = true;
    (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
    updateFromPointer(event);
  }

  function onGradientPointerMove(event: ReactPointerEvent): void {
    if (!draggingRef.current) return;
    updateFromPointer(event);
  }

  function onGradientPointerUp(): void {
    draggingRef.current = false;
  }

  function onGradientKeydown(event: ReactKeyboardEvent): void {
    const step = event.shiftKey ? 10 : 1;
    let nextS = hsvRef.current.s;
    let nextV = hsvRef.current.v;
    let handled = true;

    switch (event.key) {
      case "ArrowRight":
        nextS = Math.min(100, nextS + step);
        break;
      case "ArrowLeft":
        nextS = Math.max(0, nextS - step);
        break;
      case "ArrowUp":
        nextV = Math.min(100, nextV + step);
        break;
      case "ArrowDown":
        nextV = Math.max(0, nextV - step);
        break;
      default:
        handled = false;
    }

    if (handled) {
      event.preventDefault();
      setS(nextS);
      setV(nextV);
      commitColor(hsvRef.current.h, nextS, nextV);
    }
  }

  function onHueChange(nextHue: number): void {
    setH(nextHue);
    commitColor(nextHue);
  }

  function onAlphaChange(nextAlphaPct: number): void {
    const nextAlpha = nextAlphaPct / 100;
    setAlpha(nextAlpha);
    commitColor(hsvRef.current.h, hsvRef.current.s, hsvRef.current.v, nextAlpha);
  }

  function applyHexText(raw: string, setText: (text: string) => void): void {
    setText(raw);
    const normalized = raw.startsWith("#") ? raw : `#${raw}`;
    if (isValidHex(normalized)) {
      const norm = normalizeHex(normalized);
      syncFromHex(norm);
      commitFromPinned(norm);
    }
  }

  function onHexInputChange(event: ChangeEvent<HTMLInputElement>): void {
    applyHexText(event.currentTarget.value, setHexInput);
  }

  function onTriggerHexInputChange(event: ChangeEvent<HTMLInputElement>): void {
    applyHexText(event.currentTarget.value, setTriggerHexInput);
  }

  function onRgbChange(channel: "r" | "g" | "b", nextValue: string | number | null): void {
    const val = toNumericInputValue(nextValue) ?? 0;
    const rgb = { ...currentRgb };
    rgb[channel] = val;
    const hsv = rgbToHsv(rgb.r, rgb.g, rgb.b);
    setH(hsv.h);
    setS(hsv.s);
    setV(hsv.v);
    commitColor(hsv.h, hsv.s, hsv.v);
  }

  function onHslChange(channel: "h" | "s" | "l", nextValue: string | number | null): void {
    const val = toNumericInputValue(nextValue) ?? 0;
    const hsl = { ...currentHsl };
    hsl[channel] = val;
    const hsv = hslToHsv(hsl.h, hsl.s, hsl.l);
    setH(hsv.h);
    setS(hsv.s);
    setV(hsv.v);
    commitColor(hsv.h, hsv.s, hsv.v);
  }

  function onAlphaInputChange(nextValue: string | number | null): void {
    const nextAlpha = (toNumericInputValue(nextValue) ?? 100) / 100;
    setAlpha(nextAlpha);
    commitColor(hsvRef.current.h, hsvRef.current.s, hsvRef.current.v, nextAlpha);
  }

  function selectSwatch(hex: string): void {
    if (disabled) return;
    const norm = normalizeHex(hex);
    syncFromHex(norm);
    commitFromPinned(norm);
  }

  useEffect(() => {
    if (!isOpen) {
      return;
    }

    function handlePointerDown(event: MouseEvent): void {
      if (!rootElement) return;
      // The surface is portalled out of the root, so both count as inside.
      if (!layerContains(event.target as Node, rootElement, surfaceRef.current)) {
        setOpenState(false);
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape") {
        event.preventDefault();
        setOpenState(false);
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeydown);
    };
  }, [isOpen]);

  const alphaField = (suffix: string) => (
    <div className="poodle-color-picker__channel-field">
      <NumberInput
        id={`cp-${pickerId}-a-${suffix}`}
        value={Math.round(alpha * 100)}
        min={0}
        max={100}
        step={1}
        ariaLabel="Alpha"
        size={resolvedSize}
        density={resolvedDensity}
        onValueChange={onAlphaInputChange}
      />
      <span className="poodle-color-picker__input-label" aria-hidden="true">
        A
      </span>
    </div>
  );

  return (
    <div
      className="poodle-color-picker"
      aria-label={ariaLabel}
      data-disabled={disabled || undefined}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      ref={setRootElement}
    >
      <div className="poodle-color-picker__controls">
        <button
          type="button"
          className="poodle-color-picker__trigger"
          aria-label="Open color picker"
          aria-haspopup="dialog"
          aria-expanded={isOpen}
          aria-controls={surfaceId}
          disabled={disabled}
          onClick={toggleOpen}
        >
          <span className="poodle-color-picker__preview" style={{ background: previewColor }} aria-hidden="true" />
        </button>

        {showInput ? (
          <input
            type="text"
            className="poodle-color-picker__input"
            value={triggerHexInput}
            disabled={disabled}
            maxLength={9}
            aria-label="Hex color value"
            onChange={onTriggerHexInputChange}
            onBlur={() => setTriggerHexInput(currentHex)}
          />
        ) : null}
      </div>

      {isOpen ? (
        <AnchoredSurface
          ref={surfaceRef}
          anchor={rootElement}
          placement="bottom-start"
          offset={4}
          onPlacement={(next) => setPlacement(next.startsWith("top") ? "above" : "below")}
          id={surfaceId}
          className={
            placement === "above"
              ? "poodle-color-picker__surface poodle-color-picker__surface--above"
              : "poodle-color-picker__surface"
          }
          role="dialog"
          aria-label="Color picker"
        >
          <div className="poodle-color-picker__picker-area">
            <div
              className="poodle-color-picker__gradient"
              style={{ backgroundColor: `hsl(${h}, 100%, 50%)` }}
              role="slider"
              tabIndex={0}
              aria-label="Saturation and brightness"
              aria-valuemin={0}
              aria-valuemax={100}
              aria-valuenow={Math.round(s)}
              aria-valuetext={`Saturation ${s}%, Brightness ${v}%`}
              ref={gradientRef}
              onPointerDown={onGradientPointerDown}
              onPointerMove={onGradientPointerMove}
              onPointerUp={onGradientPointerUp}
              onKeyDown={onGradientKeydown}
            >
              <div
                className="poodle-color-picker__gradient-thumb"
                style={{ left: `${s}%`, top: `${100 - v}%`, background: currentHex }}
                aria-hidden="true"
              />
            </div>

            <div className="poodle-color-picker__controls-panel">
              <div className="poodle-color-picker__hue-wrap">
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

              {showAlpha ? (
                <div
                  className="poodle-color-picker__alpha-wrap"
                  style={{ "--poodle-cp-alpha-color": currentHex } as React.CSSProperties}
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
              ) : null}

              <div className="poodle-color-picker__mode-section">
                <div className="poodle-color-picker__mode-toggle">
                  <SegmentedControl
                    value={inputMode}
                    options={modeOptions}
                    ariaLabel="Color input mode"
                    size={resolvedSize}
                    density={resolvedDensity}
                    onValueChange={(mode) => setInputMode(mode as ColorInputMode)}
                  />
                </div>

                <div className="poodle-color-picker__inputs">
                  {inputMode === "hex" ? (
                    <>
                      <div className="poodle-color-picker__hex-field">
                        <input
                          type="text"
                          className="poodle-color-picker__text-input"
                          value={hexInput}
                          maxLength={9}
                          aria-label="Hex color"
                          onChange={onHexInputChange}
                          onBlur={() => setHexInput(currentHex)}
                        />
                        <span className="poodle-color-picker__input-label" aria-hidden="true">
                          Hex
                        </span>
                      </div>
                      {showAlpha ? alphaField("hex") : null}
                    </>
                  ) : inputMode === "rgb" ? (
                    <>
                      {(["r", "g", "b"] as const).map((channel) => (
                        <div key={channel} className="poodle-color-picker__channel-field">
                          <NumberInput
                            id={`cp-${pickerId}-${channel}`}
                            value={currentRgb[channel]}
                            min={0}
                            max={255}
                            step={1}
                            ariaLabel={channel === "r" ? "Red" : channel === "g" ? "Green" : "Blue"}
                            size={resolvedSize}
                            density={resolvedDensity}
                            onValueChange={(nextValue) => onRgbChange(channel, nextValue)}
                          />
                          <span className="poodle-color-picker__input-label" aria-hidden="true">
                            {channel.toUpperCase()}
                          </span>
                        </div>
                      ))}
                      {showAlpha ? alphaField("rgb") : null}
                    </>
                  ) : (
                    <>
                      {(["h", "s", "l"] as const).map((channel) => (
                        <div key={channel} className="poodle-color-picker__channel-field">
                          <NumberInput
                            id={`cp-${pickerId}-hsl-${channel}`}
                            value={currentHsl[channel]}
                            min={0}
                            max={channel === "h" ? 360 : 100}
                            step={1}
                            ariaLabel={channel === "h" ? "Hue" : channel === "s" ? "Saturation" : "Lightness"}
                            size={resolvedSize}
                            density={resolvedDensity}
                            onValueChange={(nextValue) => onHslChange(channel, nextValue)}
                          />
                          <span className="poodle-color-picker__input-label" aria-hidden="true">
                            {channel.toUpperCase()}
                          </span>
                        </div>
                      ))}
                      {showAlpha ? alphaField("hsl") : null}
                    </>
                  )}
                </div>
              </div>
            </div>
          </div>

          {swatches.length > 0 ? (
            <div className="poodle-color-picker__swatches" role="listbox" aria-label="Color swatches">
              {swatches.map((hex) => (
                <button
                  key={hex}
                  type="button"
                  className={
                    currentHex === hex
                      ? "poodle-color-picker__swatch poodle-color-picker__swatch--active"
                      : "poodle-color-picker__swatch"
                  }
                  style={{ background: hex }}
                  role="option"
                  aria-selected={currentHex === hex ? "true" : "false"}
                  aria-label={hex}
                  onClick={() => selectSwatch(hex)}
                />
              ))}
            </div>
          ) : null}
        </AnchoredSurface>
      ) : null}
    </div>
  );
}

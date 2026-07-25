import { useEffect, useId, useRef, useState, type CSSProperties } from "react";
import { layerContains, registerDismissLayer } from "@poodle/headless";

import "@poodle/styles/theme-select.css";

import { AnchoredSurface } from "./AnchoredSurface";
import { Icon } from "./Icon";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import { useThemeController } from "./theme-controller";
import type { ControlDensity, ControlSize, SemanticControlSizeRole, ThemeOption } from "./types";

export interface ThemeSelectProps {
  themes?: ThemeOption[];
  value?: string;
  onChange?: ((value: string) => void) | null;
  ariaLabel?: string;
  disabled?: boolean;
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  columns?: number;
  showLabel?: boolean;
}

function swatchStyle(option: ThemeOption): CSSProperties {
  return { background: option.swatch.canvas, borderColor: option.swatch.border };
}

export function ThemeSelect({
  themes,
  value,
  onChange = null,
  ariaLabel = "Theme",
  disabled = false,
  sizeRole = "control",
  size = null,
  density = null,
  columns = 3,
  showLabel = true,
}: ThemeSelectProps) {
  const controller = useThemeController();
  const uiPresentation = useUiPresentation();
  const panelId = useId();

  const [open, setOpen] = useState(false);
  const [uncontrolledValue, setUncontrolledValue] = useState("");
  // The root is state, not a ref: the portalled surface has to re-render
  // once it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const panelRef = useRef<HTMLDivElement | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const options = themes ?? controller?.themes ?? [];
  const hasValueProp = value !== undefined;
  const currentValue = hasValueProp ? (value ?? "") : controller ? controller.current : uncontrolledValue;
  const currentOption = options.find((option) => option.value === currentValue);
  const triggerLabel = currentOption?.label ?? "Theme";

  useEffect(() => {
    if (!open) return;
    const selected = panelRef.current?.querySelector<HTMLElement>('[data-selected="true"]');
    const first = panelRef.current?.querySelector<HTMLElement>("button:not([disabled])");
    (selected ?? first)?.focus();
  }, [open]);

  useEffect(() => {
    if (!open) return;
    return registerDismissLayer({
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target as Node, rootElement, panelRef.current),
      dismissOnOutsideInteract: true,
      onDismiss: () => setOpen(false),
    });
  }, [open]);

  function select(next: string): void {
    if (disabled) return;
    setOpen(false);
    if (!hasValueProp && !controller) setUncontrolledValue(next);
    else if (controller && !hasValueProp) controller.setTheme(next);
    onChange?.(next);
  }

  return (
    <div
      ref={setRootElement}
      className="poodle-theme-select"
      role="group"
      aria-label={ariaLabel}
      data-disabled={disabled}
      data-open={open}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <button
        type="button"
        className="poodle-theme-select__trigger"
        disabled={disabled}
        aria-label={`${ariaLabel}: ${triggerLabel}`}
        aria-haspopup="dialog"
        aria-expanded={open}
        aria-controls={open ? panelId : undefined}
        onClick={() => {
          if (!disabled) setOpen((o) => !o);
        }}
      >
        {currentOption ? (
          <span className="poodle-theme-select__swatch" style={swatchStyle(currentOption)} aria-hidden="true">
            <span className="poodle-theme-select__swatch-surface" style={{ background: currentOption.swatch.surface }} />
            <span className="poodle-theme-select__swatch-accent" style={{ background: currentOption.swatch.accent }} />
            <span className="poodle-theme-select__swatch-text" style={{ background: currentOption.swatch.text }} />
          </span>
        ) : null}
        {showLabel ? <span className="poodle-theme-select__label">{triggerLabel}</span> : null}
        <span className="poodle-theme-select__chevron" aria-hidden="true">
          ▾
        </span>
      </button>

      {open ? (
        <AnchoredSurface
          ref={panelRef}
          anchor={rootElement}
          placement="bottom-start"
          offset={8}
          id={panelId}
          className="poodle-theme-select__surface"
          role="dialog"
          aria-label={ariaLabel}
          tabIndex={-1}
        >
          <div
            className="poodle-theme-select__grid"
            role="listbox"
            aria-label={ariaLabel}
            style={{ ["--poodle-theme-select-columns" as string]: columns }}
          >
            {options.map((option) => (
              <button
                key={option.value}
                type="button"
                className="poodle-theme-select__tile"
                role="option"
                aria-selected={option.value === currentValue}
                data-selected={option.value === currentValue}
                title={option.description ?? option.label}
                onClick={() => select(option.value)}
              >
                <span
                  className="poodle-theme-select__swatch poodle-theme-select__swatch--tile"
                  style={swatchStyle(option)}
                  aria-hidden="true"
                >
                  <span className="poodle-theme-select__swatch-surface" style={{ background: option.swatch.surface }} />
                  <span className="poodle-theme-select__swatch-accent" style={{ background: option.swatch.accent }} />
                  <span className="poodle-theme-select__swatch-text" style={{ background: option.swatch.text }} />
                  {option.value === currentValue ? (
                    <span className="poodle-theme-select__check" style={{ color: option.swatch.accent }}>
                      <Icon name="check" size="xs" />
                    </span>
                  ) : null}
                </span>
                <span className="poodle-theme-select__tile-label">{option.label}</span>
              </button>
            ))}
          </div>
        </AnchoredSurface>
      ) : null}
    </div>
  );
}

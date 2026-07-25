import { Fragment, useEffect, useId, useRef, useState, type KeyboardEvent } from "react";
import { layerContains, registerDismissLayer } from "@poodle/headless";

import "@poodle/styles/model-picker.css";

import { AnchoredSurface } from "./AnchoredSurface";
import { Icon } from "./Icon";
import { SegmentedControl } from "./SegmentedControl";
import { Switch } from "./Switch";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import {
  applicableAxes,
  axisControlKind,
  axisValue,
  groupHeadingFor,
  initialSelection,
  modelLabel,
  resolveSelection,
  summaryText,
} from "./model-picker-model";
import type {
  ControlDensity,
  ControlSize,
  ModelAxisValue,
  ModelCapabilityAxis,
  ModelOption,
  ModelSelection,
  SemanticControlSizeRole,
} from "./types";

export interface ModelPickerProps {
  models?: ModelOption[];
  axes?: ModelCapabilityAxis[];
  value?: ModelSelection;
  placeholder?: string;
  ariaLabel?: string;
  disabled?: boolean;
  showAxisSummary?: boolean;
  showModelDescriptions?: boolean;
  /** `bare` is the borderless inline trigger used in composer toolbars;
   * `outlined` draws the standard control border and fill. */
  variant?: "bare" | "outlined";
  /** `default` is full-strength trigger text. `subdued` dims the label and
   * summary so the picker recedes beside a more important control (its home in
   * `AgentChatInput`, where the editor should hold the eye); hover and focus
   * bring it back to full strength. */
  emphasis?: "default" | "subdued";
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  onChange?: ((value: ModelSelection) => void) | null;
}

export function ModelPicker({
  models = [],
  axes = [],
  value,
  placeholder = "Select model",
  ariaLabel = "Model",
  disabled = false,
  showAxisSummary = true,
  showModelDescriptions = true,
  variant = "bare",
  emphasis = "default",
  sizeRole = "control",
  size = null,
  density = null,
  onChange = null,
}: ModelPickerProps) {
  const uiPresentation = useUiPresentation();
  const panelId = useId();

  const [open, setOpen] = useState(false);
  const [uncontrolledValue, setUncontrolledValue] = useState<ModelSelection | null>(null);
  // The picker's home is a composer toolbar pinned to the bottom of a
  // viewport, so it prefers to open upward and flips only when it must.
  const [placement, setPlacement] = useState<"top" | "bottom">("top");
  // The root is state, not a ref: the portalled surface has to re-render once
  // it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const panelRef = useRef<HTMLDivElement | null>(null);
  const triggerRef = useRef<HTMLButtonElement | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const hasValueProp = value !== undefined;
  const effectiveValue: ModelSelection = hasValueProp
    ? value ?? { model: "", axes: {} }
    : uncontrolledValue ?? initialSelection(models, axes);

  const selectedModel = models.find((model) => model.value === effectiveValue.model);
  const triggerLabel = modelLabel(models, effectiveValue, placeholder);
  const axisSummary = showAxisSummary ? summaryText(models, axes, effectiveValue) : "";
  const visibleAxes = applicableAxes(models, axes, effectiveValue.model);
  // Two columns (models | axes) whenever the selected model has applicable
  // axes; a plain list otherwise.
  const panelLayout = visibleAxes.length > 0 ? "split" : "single";
  const triggerAriaLabel = axisSummary
    ? `${ariaLabel}: ${triggerLabel}, ${axisSummary}`
    : `${ariaLabel}: ${triggerLabel}`;

  useEffect(() => {
    if (!open) return;
    const panel = panelRef.current;
    if (!panel) return;
    const selected = panel.querySelector<HTMLElement>('[data-selected="true"]:not([disabled])');
    const first = panel.querySelector<HTMLElement>(".poodle-model-picker__option:not([disabled])");
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
  }, [open, rootElement]);

  function sync(next: ModelSelection): void {
    // Every emission is normalised, so a scoped-out axis value never leaks.
    const resolved = resolveSelection(models, axes, next);
    if (!hasValueProp) setUncontrolledValue(resolved);
    onChange?.(resolved);
  }

  function selectModel(model: string): void {
    if (disabled) return;
    // The popover stays open: the axes belong to the model just chosen, so
    // closing here would force a second trip to adjust them. Escape or an
    // outside interaction dismisses.
    sync({ model, axes: { ...effectiveValue.axes } });
  }

  function setAxis(key: string, next: ModelAxisValue): void {
    if (disabled) return;
    // Changing an axis leaves the popover open — only a model choice closes it.
    sync({ model: effectiveValue.model, axes: { ...effectiveValue.axes, [key]: next } });
  }

  function handleOptionKeydown(event: KeyboardEvent<HTMLButtonElement>): void {
    if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;
    event.preventDefault();
    const options = Array.from(
      panelRef.current?.querySelectorAll<HTMLButtonElement>(
        ".poodle-model-picker__option:not([disabled])",
      ) ?? [],
    );
    if (options.length === 0) return;
    const current = options.indexOf(event.currentTarget);
    const delta = event.key === "ArrowDown" ? 1 : -1;
    const next = (current + delta + options.length) % options.length;
    options[next]?.focus();
  }

  return (
    <div
      ref={setRootElement}
      className="poodle-model-picker"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-variant={variant}
      data-emphasis={emphasis}
      data-open={open}
      data-disabled={disabled}
    >
      <button
        ref={triggerRef}
        type="button"
        className="poodle-model-picker__trigger"
        disabled={disabled}
        aria-label={triggerAriaLabel}
        aria-haspopup="dialog"
        aria-expanded={open ? "true" : "false"}
        aria-controls={open ? panelId : undefined}
        onClick={() => {
          if (disabled) return;
          setOpen((current) => !current);
        }}
      >
        {selectedModel?.image ? (
          <span className="poodle-model-picker__icon">
            <img
              className="poodle-model-picker__image"
              src={selectedModel.image.src}
              alt={selectedModel.image.alt ?? ""}
            />
          </span>
        ) : selectedModel?.icon ? (
          <span className="poodle-model-picker__icon">
            <Icon name={selectedModel.icon} size="xs" />
          </span>
        ) : null}
        <span className="poodle-model-picker__label" data-placeholder={!effectiveValue.model}>
          {triggerLabel}
        </span>
        {axisSummary ? (
          <span className="poodle-model-picker__summary" aria-hidden="true">
            {axisSummary}
          </span>
        ) : null}
        <span className="poodle-model-picker__chevron" aria-hidden="true">
          ▾
        </span>
      </button>

      {open ? (
        <AnchoredSurface
          ref={panelRef}
          anchor={rootElement}
          placement="top-start"
          offset={8}
          onPlacement={(next) => setPlacement(next.startsWith("top") ? "top" : "bottom")}
          id={panelId}
          className="poodle-model-picker__surface"
          data-layout={panelLayout}
          data-placement={placement}
          data-size={resolvedSize}
          data-density={resolvedDensity}
          role="dialog"
          aria-label={ariaLabel}
          tabIndex={-1}
        >
          <div className="poodle-model-picker__panel">
            <div className="poodle-model-picker__models" role="radiogroup" aria-label="Model">
              {models.map((model, index) => {
                const heading = groupHeadingFor(models, index);
                const isSelected = model.value === effectiveValue.model;
                return (
                  <Fragment key={model.value}>
                    {heading ? (
                      <span className="poodle-model-picker__group">{heading}</span>
                    ) : null}
                    <button
                      type="button"
                      className="poodle-model-picker__option"
                      role="radio"
                      aria-checked={isSelected ? "true" : "false"}
                      data-selected={isSelected}
                      data-disabled={model.disabled ?? false}
                      disabled={disabled || model.disabled}
                      onClick={() => selectModel(model.value)}
                      onKeyDown={handleOptionKeydown}
                    >
                      {model.image ? (
                        <span className="poodle-model-picker__option-icon">
                          <img
                            className="poodle-model-picker__option-image"
                            src={model.image.src}
                            alt={model.image.alt ?? ""}
                          />
                        </span>
                      ) : model.icon ? (
                        <span className="poodle-model-picker__option-icon">
                          <Icon name={model.icon} size="sm" />
                        </span>
                      ) : null}
                      <span className="poodle-model-picker__option-text">
                        <span className="poodle-model-picker__option-label">{model.label}</span>
                        {showModelDescriptions && model.description ? (
                          <span className="poodle-model-picker__option-description">
                            {model.description}
                          </span>
                        ) : null}
                      </span>
                      {model.badge ? (
                        <span className="poodle-model-picker__option-badge">{model.badge}</span>
                      ) : null}
                      {isSelected ? (
                        <span className="poodle-model-picker__option-check">
                          <Icon name="check" size="xs" />
                        </span>
                      ) : null}
                    </button>
                  </Fragment>
                );
              })}
            </div>

            {visibleAxes.length > 0 ? (
              <div className="poodle-model-picker__axes">
                {visibleAxes.map((axis) => {
                  const current = axisValue(axis, effectiveValue);
                  return (
                    <div
                      key={axis.key}
                      className="poodle-model-picker__axis"
                      data-kind={axis.kind}
                      data-control={axisControlKind(axis)}
                    >
                      <span className="poodle-model-picker__axis-label">{axis.label}</span>
                      {axis.description ? (
                        <span className="poodle-model-picker__axis-description">
                          {axis.description}
                        </span>
                      ) : null}
                      {axis.kind === "select" && axisControlKind(axis) === "list" ? (
                        <div
                          className="poodle-model-picker__axis-list"
                          role="radiogroup"
                          aria-label={axis.label}
                        >
                          {(axis.options ?? []).map((option) => (
                            <button
                              key={option.value}
                              type="button"
                              className="poodle-model-picker__axis-option"
                              role="radio"
                              aria-checked={current === option.value ? "true" : "false"}
                              data-selected={current === option.value}
                              data-disabled={option.disabled ?? false}
                              disabled={disabled || axis.disabled || option.disabled}
                              onClick={() => setAxis(axis.key, option.value)}
                            >
                              <span className="poodle-model-picker__axis-option-label">
                                {option.label}
                              </span>
                              {current === option.value ? (
                                <span className="poodle-model-picker__axis-option-check">
                                  <Icon name="check" size="xs" />
                                </span>
                              ) : null}
                            </button>
                          ))}
                        </div>
                      ) : axis.kind === "select" ? (
                        <SegmentedControl
                          options={(axis.options ?? []).map((option) => ({
                            value: option.value,
                            label: option.label,
                            disabled: option.disabled,
                          }))}
                          value={typeof current === "string" ? current : ""}
                          ariaLabel={axis.label}
                          size={resolvedSize}
                          density={resolvedDensity}
                          equalWidth
                          disabled={disabled || axis.disabled}
                          onValueChange={(next) => setAxis(axis.key, next)}
                        />
                      ) : (
                        <Switch
                          checked={current === true}
                          ariaLabel={axis.label}
                          size={resolvedSize}
                          density={resolvedDensity}
                          disabled={disabled || axis.disabled}
                          onCheckedChange={(checked) => setAxis(axis.key, checked)}
                        />
                      )}
                    </div>
                  );
                })}
              </div>
            ) : null}
          </div>
        </AnchoredSurface>
      ) : null}
    </div>
  );
}

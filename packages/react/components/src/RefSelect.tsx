import { Fragment, useEffect, useId, useRef, useState, type KeyboardEvent } from "react";
import { layerContains, registerDismissLayer } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/ref-select.css";

import { AnchoredSurface } from "./AnchoredSurface";
import { Icon } from "./Icon";
import { TextInput } from "./TextInput";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import { filterRefs, groupHeadingFor, refIcon, refKindIcon, refLabel } from "./ref-select-model";
import type { ControlDensity, ControlSize, RefOption, SemanticControlSizeRole } from "./types";

export interface RefSelectProps {
  refs?: RefOption[];
  value?: string;
  /** The checked-out ref, marked in the list. Often equals `value`, but a host
   * browsing another ref keeps the marker where it belongs. */
  currentRef?: string | null;
  currentLabel?: string;
  placeholder?: string;
  searchable?: boolean;
  /** Controlled query. When supplied the component stops filtering — the host
   * owns which refs to pass. */
  searchValue?: string | null;
  searchPlaceholder?: string;
  searchLabel?: string;
  loading?: boolean;
  loadingLabel?: string;
  emptyLabel?: string;
  ariaLabel?: string;
  disabled?: boolean;
  variant?: "bare" | "outlined";
  emphasis?: "default" | "subdued";
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  onChange?: ((value: string) => void) | null;
  onSearchChange?: ((query: string) => void) | null;
}

export function RefSelect({
  refs = [],
  value,
  currentRef = null,
  currentLabel = "current",
  placeholder = "Select ref",
  searchable = true,
  searchValue = null,
  searchPlaceholder = "Search refs…",
  searchLabel = "Search refs",
  loading = false,
  loadingLabel = "Loading more refs…",
  emptyLabel = "No refs found",
  ariaLabel = "Ref",
  disabled = false,
  variant = "bare",
  emphasis = "default",
  sizeRole = "control",
  size = null,
  density = null,
  onChange = null,
  onSearchChange = null,
}: RefSelectProps) {
  const uiPresentation = useUiPresentation();
  const panelId = useId();

  const [open, setOpen] = useState(false);
  // Its home is a composer footer pinned to the bottom of a viewport, so it
  // prefers to open upward and flips only when it must.
  const [placement, setPlacement] = useState<"top" | "bottom">("top");
  const [localQuery, setLocalQuery] = useState("");
  const [uncontrolledValue, setUncontrolledValue] = useState("");
  // The root is state, not a ref: the portalled surface has to re-render once
  // it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const panelRef = useRef<HTMLDivElement | null>(null);
  const triggerRef = useRef<HTMLButtonElement | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const effectiveValue = value ?? uncontrolledValue;
  const hostDrivesSearch = searchValue !== null;
  const query = hostDrivesSearch ? searchValue ?? "" : localQuery;
  // A host-supplied query means the passed list is already the answer.
  const visibleRefs = hostDrivesSearch ? refs : filterRefs(refs, query);
  const selected = refs.find((option) => option.value === effectiveValue);
  const triggerLabel = refLabel(refs, effectiveValue, placeholder);
  const triggerIcon = selected ? refIcon(selected) : refKindIcon(undefined);

  useEffect(() => {
    if (!open) return;
    const panel = panelRef.current;
    if (!panel) return;
    const search = panel.querySelector<HTMLInputElement>("input");
    const first = panel.querySelector<HTMLElement>(".poodle-ref-select__option:not([disabled])");
    (search ?? first)?.focus();
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

  function toggleOpen(): void {
    if (disabled) return;
    setOpen((current) => !current);
  }

  function selectRef(next: string): void {
    if (disabled) return;
    if (value === undefined) setUncontrolledValue(next);
    onChange?.(next);
    // Choosing a ref is the terminal action here — unlike ModelPicker, nothing
    // follows it in the panel.
    setOpen(false);
    triggerRef.current?.focus();
  }

  function setQuery(next: string): void {
    if (!hostDrivesSearch) setLocalQuery(next);
    onSearchChange?.(next);
  }

  /** Arrow keys move through the filtered rows from anywhere in the panel, so
   * typing and choosing are one gesture. */
  function moveFocus(event: KeyboardEvent<HTMLDivElement>, delta: number): void {
    event.preventDefault();
    const options = Array.from(
      panelRef.current?.querySelectorAll<HTMLButtonElement>(
        ".poodle-ref-select__option:not([disabled])",
      ) ?? [],
    );
    if (options.length === 0) return;
    const current = options.indexOf(document.activeElement as HTMLButtonElement);
    const next =
      current === -1
        ? delta > 0
          ? 0
          : options.length - 1
        : (current + delta + options.length) % options.length;
    options[next]?.focus();
  }

  function handlePanelKeyDown(event: KeyboardEvent<HTMLDivElement>): void {
    if (event.key === "ArrowDown") moveFocus(event, 1);
    else if (event.key === "ArrowUp") moveFocus(event, -1);
  }

  return (
    <div
      ref={setRootElement}
      className="poodle-ref-select"
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
        className="poodle-ref-select__trigger"
        disabled={disabled}
        aria-label={`${ariaLabel}: ${triggerLabel}`}
        aria-haspopup="dialog"
        aria-expanded={open ? "true" : "false"}
        aria-controls={open ? panelId : undefined}
        onClick={toggleOpen}
      >
        <span className="poodle-ref-select__icon">
          <Icon name={triggerIcon} size="xs" />
        </span>
        <span className="poodle-ref-select__label" data-placeholder={!effectiveValue}>
          {triggerLabel}
        </span>
        <span className="poodle-ref-select__chevron" aria-hidden="true">
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
          className="poodle-ref-select__surface"
          data-placement={placement}
          data-size={resolvedSize}
          data-density={resolvedDensity}
          role="dialog"
          aria-label={ariaLabel}
          tabIndex={-1}
          onKeyDown={handlePanelKeyDown}
        >
          {searchable ? (
            <div className="poodle-ref-select__search">
              <TextInput
                type="search"
                value={query}
                placeholder={searchPlaceholder}
                ariaLabel={searchLabel}
                size={resolvedSize}
                density={resolvedDensity}
                disabled={disabled}
                onValueChange={setQuery}
              />
            </div>
          ) : null}

          <div className="poodle-ref-select__list" role="listbox" aria-label={ariaLabel}>
            {visibleRefs.map((option, index) => {
              const heading = groupHeadingFor(visibleRefs, index);
              const isSelected = option.value === effectiveValue;
              return (
                <Fragment key={option.value}>
                  {heading ? <span className="poodle-ref-select__group">{heading}</span> : null}
                  <button
                    type="button"
                    className="poodle-ref-select__option"
                    role="option"
                    aria-selected={isSelected ? "true" : "false"}
                    data-selected={isSelected}
                    data-current={option.value === currentRef}
                    data-disabled={option.disabled ?? false}
                    data-kind={option.kind}
                    disabled={disabled || option.disabled}
                    onClick={() => selectRef(option.value)}
                  >
                    <span className="poodle-ref-select__option-icon">
                      <Icon name={refIcon(option)} size="xs" />
                    </span>
                    <span className="poodle-ref-select__option-text">
                      <span className="poodle-ref-select__option-label">{option.label}</span>
                      {option.description ? (
                        <span className="poodle-ref-select__option-description">
                          {option.description}
                        </span>
                      ) : null}
                    </span>
                    {option.value === currentRef ? (
                      <span className="poodle-ref-select__option-marker">{currentLabel}</span>
                    ) : null}
                  </button>
                </Fragment>
              );
            })}
          </div>

          {visibleRefs.length === 0 && !loading ? (
            <p className="poodle-ref-select__empty">{emptyLabel}</p>
          ) : null}

          {loading ? (
            <p className="poodle-ref-select__loading" role="status">
              {loadingLabel}
            </p>
          ) : null}
        </AnchoredSurface>
      ) : null}
    </div>
  );
}
